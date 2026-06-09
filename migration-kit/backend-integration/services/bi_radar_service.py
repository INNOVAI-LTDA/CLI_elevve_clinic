"""
BI Radar de Longevidade (Elevve Clinic) - regra de calculo pura.

Este modulo implementa a regra de negocio do Radar de Longevidade
sem dependencia de FastAPI, SQLite, HTTP ou frontend. Recebe
dicionarios/dataclasses como entrada e devolve dataclasses como
saida. A camada de repository (commit 4) sera responsavel por
buscar as entradas no banco e persistir o snapshot.

F1 refactor: o parametro antes chamado `patient_id` agora se chama
`user_id` e tem tipo `int`, refletindo que o cliente do BI e' um User
de tipo Client em `deva_elevveclinic_users` (modelo unificado).

Regra (fonte: docs/BI_RULE_RADAR_CANONICAL.md do bundle):

  5 pilares fixos: hormonios, intestino, recuperacao, estrutura, metabolismo.
  Respostas 0..3 por pergunta, com peso por pergunta.

    score_bruto_eixo   = sum(resposta * peso) para todas as perguntas do pilar
    score_maximo_eixo  = sum(3 * peso) para as perguntas ativas do pilar
                         (esta e a fonte de verdade; o score_max informado na
                         tabela de pilares e' metadata; divergencia => logger.warning)

    cluster_ativo      = (count de key_questions com resposta >= activation_threshold)
                         >= minimum_key_symptoms
    bonus_cluster      = bonus_points se cluster_ativo, senao 0
    score_final_eixo   = score_bruto_eixo + bonus_cluster
    score_risco        = (score_final / score_maximo) * 100
    score_equilibrio   = 100 - score_risco

    indice_prioridade  = score_final_eixo + bonus_cluster_ativo
                         (soma o bonus uma vez no final, alem do bonus ja' somado
                         ao score_final; em nosso modelo isso significa
                         final + 3 quando cluster ativo, final + 0 quando nao)

    classification     = primeira faixa de interpretation_ranges
                         (ordenada por priority_level) cuja min_score <= raw_score
                         e (max_score is None ou raw_score <= max_score).
                         Se o pilar nao tem ranges, devolve literalmente
                         "Pendente de configuracao".

    Eixo prioritario   = pilar com maior indice_prioridade; empate: maior
                         score_risco; ainda empate: menor display_order.
"""

from __future__ import annotations

import logging
from collections.abc import Mapping, Sequence
from dataclasses import dataclass, replace
from datetime import datetime, timezone


logger = logging.getLogger(__name__)


# String canonica retornada quando a classificacao nao pode ser
# determinada por falta de faixas. Mantida como constante para que
# testes e frontend possam comparar exatamente.
PENDING_CONFIG_LABEL = "Pendente de configuração"


# ============================================================
# Inputs
# ============================================================

@dataclass(frozen=True)
class BiRadarPillarInput:
    """Pilar do Radar.

    score_max: valor informado pela spec (metadata). O service
    usa a soma real de 3*peso como denominador de score_risco;
    se houver divergencia, loga warning.
    """
    pillar_code: str
    name: str
    score_max: int | None
    display_order: int


@dataclass(frozen=True)
class BiRadarQuestionInput:
    """Pergunta do questionario. weight e' o peso definido pela spec."""
    question_code: str
    pillar_code: str
    weight: float
    is_active: bool = True


@dataclass(frozen=True)
class BiRadarClusterInput:
    """Cluster (gatilho de bonus) vinculado a um pilar.

    key_question_codes: tuple com as perguntas-chave; o cluster
    ativa quando pelo menos minimum_key_symptoms delas tem
    resposta >= activation_threshold.
    """
    cluster_code: str
    pillar_code: str
    activation_threshold: int
    minimum_key_symptoms: int
    bonus_points: float
    key_question_codes: tuple[str, ...]


@dataclass(frozen=True)
class BiRadarRangeInput:
    """Faixa de interpretacao. max_score=None significa faixa aberta (ex.: > 22)."""
    pillar_code: str
    label: str
    min_score: float
    max_score: float | None
    priority_level: int


@dataclass(frozen=True)
class BiRadarInputs:
    """Inputs puros para o calculo. Repository popula, service consome.

    F1 refactor: user_id e' int (era patient_id str). Reflete que o
    cliente do BI e' um User de tipo Client.
    """
    user_id: int
    responses_by_question: Mapping[str, int]
    pillars: Sequence[BiRadarPillarInput]
    questions: Sequence[BiRadarQuestionInput]
    clusters: Sequence[BiRadarClusterInput]
    ranges: Sequence[BiRadarRangeInput]


# ============================================================
# Outputs
# ============================================================

@dataclass(frozen=True)
class BiRadarAxisDetails:
    """Detalhes do calculo por eixo, para auditoria e details_json do snapshot."""
    score_max_informed: int | None
    score_max_computed: float
    score_max_diverges: bool
    key_question_codes: tuple[str, ...]
    key_question_responses: tuple[tuple[str, int], ...]
    cluster_activated_count: int


@dataclass(frozen=True)
class BiRadarAxisComputation:
    """Resultado do calculo para 1 eixo."""
    pillar_code: str
    name: str
    display_order: int
    raw_score: float
    max_score: float
    cluster_active: bool
    cluster_bonus: float
    final_score: float
    risk_score: float
    balance_score: float
    classification: str
    priority_index: float
    is_priority_axis: bool
    details: BiRadarAxisDetails


@dataclass(frozen=True)
class BiRadarSummary:
    """Resumo agregado."""
    risk: float
    balance: float
    pending_configuration: bool
    axis_count: int
    axes_with_cluster: int
    priority_axis_code: str | None


@dataclass(frozen=True)
class BiRadarComputation:
    """Resultado final do calculo para 1 cliente (User de tipo Client)."""
    user_id: int
    calculated_at: str
    axes: tuple[BiRadarAxisComputation, ...]
    summary: BiRadarSummary


# ============================================================
# Public API
# ============================================================

def compute_bi_radar(inputs: BiRadarInputs) -> BiRadarComputation:
    """Calcula o Radar de Longevidade para o cliente (User de tipo Client).

    Funcao pura: nao toca em IO, nao le banco, nao faz HTTP. Recebe
    as entradas ja materializadas e devolve um BiRadarComputation
    com todos os eixos, o eixo prioritario e o resumo agregado.
    """
    qs_by_pillar: dict[str, list[BiRadarQuestionInput]] = {}
    for q in inputs.questions:
        if not q.is_active:
            continue
        qs_by_pillar.setdefault(q.pillar_code, []).append(q)

    cl_by_pillar: dict[str, BiRadarClusterInput] = {}
    for c in inputs.clusters:
        cl_by_pillar[c.pillar_code] = c

    rng_by_pillar: dict[str, list[BiRadarRangeInput]] = {}
    for r in inputs.ranges:
        rng_by_pillar.setdefault(r.pillar_code, []).append(r)
    for pillar_code in rng_by_pillar:
        # Garante a ordem: menor priority_level = primeira faixa.
        rng_by_pillar[pillar_code].sort(key=lambda r: r.priority_level)

    axes: list[BiRadarAxisComputation] = []
    for p in sorted(inputs.pillars, key=lambda p: p.display_order):
        axis = _compute_axis(
            pillar=p,
            questions=qs_by_pillar.get(p.pillar_code, []),
            cluster=cl_by_pillar.get(p.pillar_code),
            ranges=rng_by_pillar.get(p.pillar_code, []),
            responses=inputs.responses_by_question,
        )
        axes.append(axis)

    priority_code = _resolve_priority_axis(axes)
    # Substitui is_priority_axis em cada eixo. dataclasses.replace
    # eh a forma canonica de "mutar" um frozen dataclass.
    axes = tuple(
        replace(a, is_priority_axis=(a.pillar_code == priority_code))
        for a in axes
    )

    summary = _build_summary(axes, priority_code)
    calculated_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%fZ")

    return BiRadarComputation(
        user_id=inputs.user_id,
        calculated_at=calculated_at,
        axes=axes,
        summary=summary,
    )


# ============================================================
# Helpers
# ============================================================

def _compute_axis(
    *,
    pillar: BiRadarPillarInput,
    questions: list[BiRadarQuestionInput],
    cluster: BiRadarClusterInput | None,
    ranges: list[BiRadarRangeInput],
    responses: Mapping[str, int],
) -> BiRadarAxisComputation:
    """Calcula 1 eixo. Funcao pura local."""
    raw = sum(responses.get(q.question_code, 0) * q.weight for q in questions)
    max_computed = sum(3 * q.weight for q in questions)

    key_q_codes: tuple[str, ...] = cluster.key_question_codes if cluster else ()
    key_responses = tuple((kq, responses.get(kq, 0)) for kq in key_q_codes)

    threshold = cluster.activation_threshold if cluster else 99
    min_symptoms = cluster.minimum_key_symptoms if cluster else 99
    bonus_value = cluster.bonus_points if cluster else 0.0

    activated = sum(1 for _kq, v in key_responses if v >= threshold)
    cluster_active = bool(cluster) and activated >= min_symptoms
    bonus = bonus_value if cluster_active else 0.0

    final = raw + bonus
    if max_computed > 0:
        risk = (final / max_computed) * 100
    else:
        risk = 0.0
    balance = 100 - risk

    classification = _lookup_classification(raw, ranges)

    # indice_prioridade: final + bonus (uma vez so' no final, alem do bonus
    # ja' somado ao final). Quando o cluster nao ativa, bonus=0 e
    # indice_prioridade == score_final_eixo == raw.
    priority_index = final + (bonus_value if cluster_active else 0.0)

    diverges = (
        pillar.score_max is not None
        and abs(max_computed - pillar.score_max) > 0.01
    )
    if diverges:
        logger.warning(
            "BI Radar score_max diverges for pillar=%s: "
            "informed=%s computed=%s; using computed.",
            pillar.pillar_code, pillar.score_max, max_computed,
        )

    details = BiRadarAxisDetails(
        score_max_informed=pillar.score_max,
        score_max_computed=round(max_computed, 6),
        score_max_diverges=diverges,
        key_question_codes=key_q_codes,
        key_question_responses=key_responses,
        cluster_activated_count=activated,
    )

    return BiRadarAxisComputation(
        pillar_code=pillar.pillar_code,
        name=pillar.name,
        display_order=pillar.display_order,
        raw_score=round(raw, 6),
        max_score=round(max_computed, 6),
        cluster_active=cluster_active,
        cluster_bonus=bonus,
        final_score=round(final, 6),
        risk_score=round(risk, 6),
        balance_score=round(balance, 6),
        classification=classification,
        priority_index=round(priority_index, 6),
        is_priority_axis=False,  # preenchido depois por compute_bi_radar
        details=details,
    )


def _lookup_classification(raw: float, ranges: list[BiRadarRangeInput]) -> str:
    """Devolve o label da primeira faixa que casa com raw, ou
    "Pendente de configuracao" se o pilar nao tem faixas ou
    nenhuma delas casa (improvavel, mas defensivo)."""
    if not ranges:
        return PENDING_CONFIG_LABEL
    for r in ranges:
        if raw >= r.min_score and (r.max_score is None or raw <= r.max_score):
            return r.label
    return PENDING_CONFIG_LABEL


def _resolve_priority_axis(axes: list[BiRadarAxisComputation]) -> str | None:
    """Desempate: priority_index desc, risk_score desc, display_order asc."""
    if not axes:
        return None
    return max(
        axes,
        key=lambda a: (a.priority_index, a.risk_score, -a.display_order),
    ).pillar_code


def _build_summary(
    axes: tuple[BiRadarAxisComputation, ...],
    priority_code: str | None,
) -> BiRadarSummary:
    if axes:
        avg_risk = sum(a.risk_score for a in axes) / len(axes)
        avg_balance = 100 - avg_risk
    else:
        avg_risk = 0.0
        avg_balance = 100.0
    return BiRadarSummary(
        risk=round(avg_risk, 6),
        balance=round(avg_balance, 6),
        pending_configuration=any(
            a.classification == PENDING_CONFIG_LABEL for a in axes
        ),
        axis_count=len(axes),
        axes_with_cluster=sum(1 for a in axes if a.cluster_active),
        priority_axis_code=priority_code,
    )
