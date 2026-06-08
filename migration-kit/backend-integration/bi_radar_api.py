"""BI Radar de Longevidade - FastAPI router (prefixo /bi).

Endpoints:
  GET  /bi/health
  GET  /bi/radar/pillars                   -> list[BiRadarPillarDto]
  GET  /bi/radar/patients                  -> list[BiRadarPatientDto]
  GET  /bi/radar/patients/{patient_id}     -> BiRadarComputationDto

Erros 4xx: devolvidos via JSONResponse no formato canonico
{ "error": { "status", "code", "message", "details" } }. 422 (Pydantic)
cai no handler default do FastAPI ({"detail": [...]}) - documentado
como pendencia para sprint futura.

Cada chamada a /bi/radar/patients/{id} calcula o Radar via
services.bi_radar_service.compute_bi_radar (funcao pura) e
persiste 1 snapshot por pilar em
deva_elevveclinic_bi_radar_score_snapshots (auditoria).
"""
from __future__ import annotations

import json
import logging
from datetime import datetime, timezone

from fastapi import APIRouter
from fastapi.responses import JSONResponse

from schemas.bi_radar import (
    BiRadarAxisScoreDto,
    BiRadarComputationDto,
    BiRadarPatientDto,
    BiRadarPillarDto,
    BiRadarSummaryDto,
)
from services.bi_radar_service import (
    BiRadarClusterInput,
    BiRadarInputs,
    BiRadarPillarInput,
    BiRadarQuestionInput,
    BiRadarRangeInput,
    compute_bi_radar,
)
from storage.bi_radar_repository import BiRadarRepository


logger = logging.getLogger(__name__)


router = APIRouter(prefix="/bi", tags=["bi-radar"])


# ---------- helpers ----------

def _get_repository() -> BiRadarRepository:
    """Factory local. Permite override em testes."""
    return BiRadarRepository()


def _to_inputs(
    patient_id: str,
    repo: BiRadarRepository,
) -> BiRadarInputs:
    """Materializa os inputs do service a partir do banco."""
    pillars_rows = repo.list_pillars()
    questions_rows = repo.list_questions()
    clusters_rows = repo.list_clusters()
    cluster_q_rows = repo.list_cluster_questions()
    ranges_rows = repo.list_interpretation_ranges()
    responses_rows = repo.list_responses(patient_id)

    pillars = [
        BiRadarPillarInput(
            pillar_code=r["pillar_code"],
            name=r["name"],
            score_max=r.get("score_max"),
            display_order=r["display_order"],
        )
        for r in pillars_rows
    ]
    questions = [
        BiRadarQuestionInput(
            question_code=r["question_code"],
            pillar_code=r["pillar_code"],
            weight=r["weight"],
            is_active=bool(r.get("is_active", 1)),
        )
        for r in questions_rows
    ]
    ranges = [
        BiRadarRangeInput(
            pillar_code=r["pillar_code"],
            label=r["label"],
            min_score=r["min_score"],
            max_score=r.get("max_score"),
            priority_level=r["priority_level"],
        )
        for r in ranges_rows
    ]

    keys_by_cluster: dict[str, list[str]] = {}
    for r in cluster_q_rows:
        keys_by_cluster.setdefault(r["cluster_code"], []).append(r["question_code"])

    clusters: list[BiRadarClusterInput] = []
    for cr in clusters_rows:
        clusters.append(
            BiRadarClusterInput(
                cluster_code=cr["cluster_code"],
                pillar_code=cr["pillar_code"],
                activation_threshold=cr["activation_threshold"],
                minimum_key_symptoms=cr["minimum_key_symptoms"],
                bonus_points=cr["bonus_points"],
                key_question_codes=tuple(
                    keys_by_cluster.get(cr["cluster_code"], [])
                ),
            )
        )

    responses_by_q = {
        r["question_code"]: r["response_value"] for r in responses_rows
    }

    return BiRadarInputs(
        patient_id=patient_id,
        responses_by_question=responses_by_q,
        pillars=pillars,
        questions=questions,
        clusters=clusters,
        ranges=ranges,
    )


def _persist_snapshots(
    patient_id: str,
    calculated_at: str,
    axes,
    repo: BiRadarRepository,
) -> None:
    """Persiste 1 linha por pilar em score_snapshots (auditoria)."""
    for ax in axes:
        details = {
            "score_max_informed": ax.details.score_max_informed,
            "score_max_computed": ax.details.score_max_computed,
            "score_max_diverges": ax.details.score_max_diverges,
            "key_question_codes": list(ax.details.key_question_codes),
            "key_question_responses": [
                list(kv) for kv in ax.details.key_question_responses
            ],
            "cluster_activated_count": ax.details.cluster_activated_count,
        }
        snapshot_id = (
            f"snap_{ax.pillar_code}_"
            f"{datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S%f')}_"
            f"{patient_id[:8]}"
        )
        repo.insert_snapshot({
            "id": snapshot_id,
            "patient_id": patient_id,
            "calculated_at": calculated_at,
            "pillar_code": ax.pillar_code,
            "raw_score": ax.raw_score,
            "cluster_bonus": ax.cluster_bonus,
            "final_score": ax.final_score,
            "max_score": ax.max_score,
            "risk_score": ax.risk_score,
            "balance_score": ax.balance_score,
            "classification": ax.classification,
            "priority_index": ax.priority_index,
            "is_priority_axis": 1 if ax.is_priority_axis else 0,
            "details_json": json.dumps(details, ensure_ascii=False),
        })


def _error_response(status: int, code: str, message: str, details=None) -> JSONResponse:
    """Constrói o envelope canônico { "error": { ... } }."""
    return JSONResponse(
        status_code=status,
        content={
            "error": {
                "status": status,
                "code": code,
                "message": message,
                "details": details,
            }
        },
    )


# ---------- routes ----------

@router.get("/health")
def health():
    return {"ok": True, "scope": "bi-radar"}


@router.get("/radar/pillars", response_model=list[BiRadarPillarDto])
def list_pillars():
    repo = _get_repository()
    rows = repo.list_pillars()
    return [
        BiRadarPillarDto(
            pillarCode=r["pillar_code"],
            name=r["name"],
            description=r.get("description"),
            scoreMax=r.get("score_max"),
            displayOrder=r["display_order"],
        )
        for r in rows
    ]


@router.get("/radar/patients", response_model=list[BiRadarPatientDto])
def list_patients():
    repo = _get_repository()
    rows = repo.list_patients()
    return [
        BiRadarPatientDto(
            patientId=r["id"],
            patientCode=r["patient_code"],
            name=r["name"],
            programName=r.get("program_name"),
        )
        for r in rows
    ]


@router.get(
    "/radar/patients/{patient_id}",
    response_model=BiRadarComputationDto,
)
def get_patient_radar(patient_id: str):
    repo = _get_repository()
    patient = repo.get_patient(patient_id)
    if patient is None:
        return _error_response(
            status=404,
            code="PATIENT_NOT_FOUND",
            message=f"Paciente '{patient_id}' nao encontrado.",
            details={"patient_id": patient_id},
        )

    inputs = _to_inputs(patient_id, repo)
    result = compute_bi_radar(inputs)
    _persist_snapshots(
        patient_id=patient_id,
        calculated_at=result.calculated_at,
        axes=result.axes,
        repo=repo,
    )

    axes_dto = [
        BiRadarAxisScoreDto(
            axisKey=ax.pillar_code,
            axisLabel=ax.name,
            axisSub="",
            rawScore=ax.raw_score,
            scoreMax=ax.max_score,
            clusterActive=ax.cluster_active,
            clusterBonus=ax.cluster_bonus,
            finalScore=ax.final_score,
            riskScore=ax.risk_score,
            balanceScore=ax.balance_score,
            classification=ax.classification,
            priorityIndex=ax.priority_index,
            isPriorityAxis=ax.is_priority_axis,
        )
        for ax in result.axes
    ]

    summary = BiRadarSummaryDto(
        risk=result.summary.risk,
        balance=result.summary.balance,
        pendingConfiguration=result.summary.pending_configuration,
    )

    return BiRadarComputationDto(
        patientId=result.patient_id,
        calculatedAt=result.calculated_at,
        axes=axes_dto,
        priorityAxisKey=result.summary.priority_axis_code or "",
        summary=summary,
    )
