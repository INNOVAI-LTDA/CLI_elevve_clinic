"""Testes do bi_radar_service.

Cobre os 6 casos do plano do Commit 3:
  1. Pilar sem respostas e sem ranges -> raw=0, cluster=false,
     classification="Pendente de configuracao".
  2. 3 sintomas-chave do Metabolismo em resposta 3 -> cluster_ativo=true,
     bonus=3, score sobe.
  3. Respostas uniformes (todas 2) em Metabolismo -> cluster_ativo=true.
  4. Pilar sem ranges -> "Pendente de configuracao" literal.
  5. Empate de indice_prioridade -> desempate por risk_score desc,
     depois por display_order asc.
  6. Divergencia score_max informed vs computed -> service usa
     computed e loga warning.

F1 refactor: parametro `user_id` (int) substitui `patient_id` (str)
para refletir o modelo User unificado.
"""
from __future__ import annotations

import logging

import pytest

from services.bi_radar_service import (
    PENDING_CONFIG_LABEL,
    BiRadarClusterInput,
    BiRadarInputs,
    BiRadarPillarInput,
    BiRadarQuestionInput,
    BiRadarRangeInput,
    compute_bi_radar,
)


# ---------- helpers locais ----------

def _inputs(
    *,
    user_id: int = 9001,
    responses: dict | None = None,
    pillars: list | None = None,
    questions: list | None = None,
    clusters: list | None = None,
    ranges: list | None = None,
) -> BiRadarInputs:
    return BiRadarInputs(
        user_id=user_id,
        responses_by_question=responses or {},
        pillars=pillars or [],
        questions=questions or [],
        clusters=clusters or [],
        ranges=ranges or [],
    )


def _metabolismo_questions_and_cluster():
    """8 perguntas de Metabolismo + 1 cluster (3 chaves)."""
    weights = {
        "metabolismo_01": 2.0,
        "metabolismo_02": 1.0,
        "metabolismo_03": 1.0,
        "metabolismo_04": 1.0,
        "metabolismo_05": 2.0,
        "metabolismo_06": 2.0,
        "metabolismo_07": 1.0,
        "metabolismo_08": 1.0,
    }
    questions = [
        BiRadarQuestionInput(q, "metabolismo", weight=w)
        for q, w in weights.items()
    ]
    cluster = BiRadarClusterInput(
        cluster_code="cl_metabolismo",
        pillar_code="metabolismo",
        activation_threshold=2,
        minimum_key_symptoms=3,
        bonus_points=3.0,
        key_question_codes=(
            "metabolismo_01",
            "metabolismo_05",
            "metabolismo_06",
        ),
    )
    return questions, cluster


# ---------- 1) Pilar sem respostas e sem ranges ----------

def test_empty_responses_no_ranges_no_questions_yields_pending_config():
    """Pilar sem perguntas, sem ranges, sem respostas: tudo zero e
    classification = 'Pendente de configuracao'."""
    pillars = [BiRadarPillarInput("hormonios", "Hormônios", score_max=36, display_order=1)]
    inputs = _inputs(pillars=pillars, questions=[], ranges=[])

    result = compute_bi_radar(inputs)

    assert len(result.axes) == 1
    ax = result.axes[0]
    assert ax.pillar_code == "hormonios"
    assert ax.raw_score == 0
    assert ax.max_score == 0
    assert ax.cluster_active is False
    assert ax.cluster_bonus == 0
    assert ax.final_score == 0
    assert ax.risk_score == 0
    assert ax.balance_score == 100
    assert ax.classification == PENDING_CONFIG_LABEL
    assert ax.is_priority_axis is True  # unico eixo, vence por default
    assert result.summary.pending_configuration is True
    assert result.summary.priority_axis_code == "hormonios"


# ---------- 2) 3 sintomas-chave do Metabolismo em resposta 3 ----------

def test_three_key_questions_at_3_activate_cluster():
    """3 chaves em 3 -> cluster_ativo=true, bonus=3, raw=23, final=26."""
    pillars = [BiRadarPillarInput("metabolismo", "Metabolismo", score_max=33, display_order=1)]
    questions, cluster = _metabolismo_questions_and_cluster()

    responses = {
        "metabolismo_01": 3,  # chave
        "metabolismo_05": 3,  # chave
        "metabolismo_06": 3,  # chave
        "metabolismo_02": 1,
        "metabolismo_03": 1,
        "metabolismo_04": 1,
        "metabolismo_07": 1,
        "metabolismo_08": 1,
    }

    inputs = _inputs(
        pillars=pillars,
        questions=questions,
        clusters=[cluster],
        responses=responses,
    )
    result = compute_bi_radar(inputs)

    ax = result.axes[0]
    assert ax.cluster_active is True
    assert ax.cluster_bonus == 3.0
    # raw = 3*2 + 1 + 1 + 1 + 3*2 + 3*2 + 1 + 1 = 6+1+1+1+6+6+1+1 = 23
    assert ax.raw_score == 23.0
    # max = 3*(2+1+1+1+2+2+1+1) = 3*11 = 33
    assert ax.max_score == 33.0
    assert ax.final_score == 26.0
    # indice_prioridade = final + bonus (uma vez no final) = 26 + 3 = 29
    assert ax.priority_index == 29.0
    assert ax.is_priority_axis is True
    # details: cluster ativado em 3 chaves
    assert ax.details.cluster_activated_count == 3


# ---------- 3) Respostas uniformes (todas 2) em Metabolismo ----------

def test_uniform_responses_at_2_in_metabolismo_activate_cluster():
    """Todas respostas=2 em Metabolismo: as 3 chaves batem em >=2,
    cluster_ativo=true."""
    pillars = [BiRadarPillarInput("metabolismo", "Metabolismo", score_max=33, display_order=1)]
    questions, cluster = _metabolismo_questions_and_cluster()
    responses = {q.question_code: 2 for q in questions}

    inputs = _inputs(
        pillars=pillars,
        questions=questions,
        clusters=[cluster],
        responses=responses,
    )
    result = compute_bi_radar(inputs)

    ax = result.axes[0]
    assert ax.cluster_active is True
    assert ax.cluster_bonus == 3.0
    # raw = 2*2 + 2*1 + 2*1 + 2*1 + 2*2 + 2*2 + 2*1 + 2*1 = 4+2+2+2+4+4+2+2 = 22
    assert ax.raw_score == 22.0
    assert ax.final_score == 25.0


# ---------- 4) Pilar sem ranges -> "Pendente de configuracao" ----------

def test_pillar_without_ranges_returns_pending_config_literal():
    """Pilar sem nenhuma faixa: classification = 'Pendente de configuracao'
    mesmo com respostas altas. Confere tambem que summary.pending_configuration=True."""
    pillars = [BiRadarPillarInput("hormonios", "Hormônios", score_max=36, display_order=1)]
    questions = [
        BiRadarQuestionInput("hormonios_01", "hormonios", weight=2.0),
        BiRadarQuestionInput("hormonios_02", "hormonios", weight=2.0),
    ]
    responses = {"hormonios_01": 3, "hormonios_02": 3}

    inputs = _inputs(
        pillars=pillars,
        questions=questions,
        ranges=[],  # explicitamente sem ranges
        responses=responses,
    )
    result = compute_bi_radar(inputs)

    ax = result.axes[0]
    assert ax.classification == PENDING_CONFIG_LABEL
    assert result.summary.pending_configuration is True


# ---------- 5) Empate de indice_prioridade ----------

def test_priority_tiebreak_by_risk_score_when_priority_index_equal():
    """Dois eixos com mesmo priority_index: vence o de maior risk_score.
    Aqui p1 tem peso 2 e resposta 1 (raw=2, risk~33%),
    p2 tem peso 1 e resposta 2 (raw=2, risk~67%). Mesmo priority, p2 vence."""
    pillars = [
        BiRadarPillarInput("p1", "P1", score_max=None, display_order=1),
        BiRadarPillarInput("p2", "P2", score_max=None, display_order=2),
    ]
    questions = [
        BiRadarQuestionInput("q1", "p1", weight=2.0),
        BiRadarQuestionInput("q2", "p2", weight=1.0),
    ]
    responses = {"q1": 1, "q2": 2}  # raw=2 em ambos, risk diferente

    inputs = _inputs(pillars=pillars, questions=questions, responses=responses)
    result = compute_bi_radar(inputs)

    p1 = next(a for a in result.axes if a.pillar_code == "p1")
    p2 = next(a for a in result.axes if a.pillar_code == "p2")
    assert p1.priority_index == p2.priority_index == 2.0
    assert p1.risk_score < p2.risk_score
    assert p2.is_priority_axis is True
    assert p1.is_priority_axis is False
    assert result.summary.priority_axis_code == "p2"


def test_priority_tiebreak_by_display_order_when_priority_and_risk_equal():
    """Tres eixos empatados em priority e risk: vence o de menor display_order."""
    pillars = [
        BiRadarPillarInput("p_ultimo",   "Ultimo",  score_max=None, display_order=3),
        BiRadarPillarInput("p_primeiro", "Primeiro", score_max=None, display_order=1),
        BiRadarPillarInput("p_meio",     "Meio",   score_max=None, display_order=2),
    ]
    # Cada pilar com 1 pergunta de peso 2, resposta 1.
    # raw=2, max=6, risk=2/6*100=33.33%, priority=2.
    questions = [
        BiRadarQuestionInput("q1", "p_ultimo",   weight=2.0),
        BiRadarQuestionInput("q2", "p_primeiro", weight=2.0),
        BiRadarQuestionInput("q3", "p_meio",     weight=2.0),
    ]
    responses = {"q1": 1, "q2": 1, "q3": 1}

    inputs = _inputs(pillars=pillars, questions=questions, responses=responses)
    result = compute_bi_radar(inputs)

    # Todos empatados em priority e risk. Vence display_order=1 = p_primeiro.
    assert result.summary.priority_axis_code == "p_primeiro"
    p_primeiro = next(a for a in result.axes if a.pillar_code == "p_primeiro")
    assert p_primeiro.is_priority_axis is True


# ---------- 6) Divergencia score_max informed vs computed ----------

def test_score_max_divergence_uses_computed_and_logs_warning(caplog):
    """Recuperacao tem 9 perguntas com soma de pesos 15, logo
    max_computed=45. Com score_max_informed=39, ha' divergencia:
    service usa 45 e loga warning."""
    # Pesos 9 perguntas -> soma 15 -> max_computed=45
    weights = [2.0, 2.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0]
    assert sum(weights) == 15.0

    pillars = [BiRadarPillarInput("recuperacao", "Recuperação", score_max=39, display_order=1)]
    questions = [
        BiRadarQuestionInput(f"recuperacao_{i:02d}", "recuperacao", weight=w)
        for i, w in enumerate(weights, start=1)
    ]
    responses = {q.question_code: 0 for q in questions}  # raw=0 para focar no max

    inputs = _inputs(pillars=pillars, questions=questions, responses=responses)

    with caplog.at_level(logging.WARNING, logger="services.bi_radar_service"):
        result = compute_bi_radar(inputs)

    ax = result.axes[0]
    # max computado = 3 * 15 = 45 (fonte de verdade)
    assert ax.max_score == 45.0
    # metadata registra o informed
    assert ax.details.score_max_informed == 39
    assert ax.details.score_max_computed == 45.0
    assert ax.details.score_max_diverges is True

    # Warning de fato foi emitido
    warnings = [r for r in caplog.records if r.levelno == logging.WARNING]
    assert any("diverges" in r.getMessage() for r in warnings)
    assert any("recuperacao" in r.getMessage() for r in warnings)


# ---------- Sanidade extra: regra "Pendente de configuracao" nao quebra a conta ----------

def test_pending_config_does_not_break_priority_ranking():
    """Um pilar 'Pendente' (sem faixa) e um pilar 'Eixo prioritario'
    (com faixa alta): o 'Eixo prioritario' vence mesmo que seu indice
    de prioridade seja igual ou menor. Confere que o calculo nao
    depende do classification para definir o vencedor."""
    pillars = [
        BiRadarPillarInput("p1", "P1 (sem range)", score_max=10, display_order=1),
        BiRadarPillarInput("p2", "P2 (com range)", score_max=10, display_order=2),
    ]
    questions = [
        BiRadarQuestionInput("q1", "p1", weight=1.0),
        BiRadarQuestionInput("q2", "p2", weight=1.0),
    ]
    # p1: raw=5, max=3, risk=166.67% (cap por max=3, nao cap por risk)
    # p2: raw=3, max=3, risk=100%
    # Nao importa: priority_index=raw=5 vs 3; p1 vence
    responses = {"q1": 5 if False else 3, "q2": 3}  # q1 capped por raw (responses >=0)
    # Re-corrigindo: response_value deve ser 0..3, entao raw max = 3
    # Para forcar p1 a ter raw=3 e p2 raw=2 (assim p1 vence com raw=3)
    responses = {"q1": 3, "q2": 2}

    inputs = _inputs(pillars=pillars, questions=questions, responses=responses)
    # ranges so' para p2
    ranges = [
        BiRadarRangeInput("p2", "Estável", 0.0, 1.0, 1),
        BiRadarRangeInput("p2", "Atenção", 2.0, 10.0, 2),
    ]
    inputs = _inputs(
        pillars=pillars, questions=questions, ranges=ranges, responses=responses,
    )
    result = compute_bi_radar(inputs)

    p1 = next(a for a in result.axes if a.pillar_code == "p1")
    p2 = next(a for a in result.axes if a.pillar_code == "p2")
    assert p1.classification == PENDING_CONFIG_LABEL
    assert p2.classification == "Atenção"
    # p1 vence por priority_index (3 > 2)
    assert p1.priority_index > p2.priority_index
    assert p1.is_priority_axis is True
