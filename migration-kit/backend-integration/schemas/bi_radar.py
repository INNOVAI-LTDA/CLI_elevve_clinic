"""Pydantic DTOs para o BI Radar de Longevidade.

Mirror do contract TypeScript em migration-kit/contracts/frontend/biRadar.ts
(espelho manual; nao ha geracao automatica de TS).

Pydantic v2 com field names em camelCase para casar com o JSON
de saida do backend e com o tipo TypeScript do frontend.
"""
from __future__ import annotations

from typing import Optional

from pydantic import BaseModel


class BiRadarPillarDto(BaseModel):
    """Metadado de um pilar (usado em /bi/radar/pillars)."""
    pillarCode: str
    name: str
    description: Optional[str] = None
    scoreMax: Optional[int] = None
    displayOrder: int


class BiRadarPatientDto(BaseModel):
    """Item da lista de pacientes (usado em /bi/radar/patients)."""
    patientId: str
    patientCode: str
    name: str
    programName: Optional[str] = None


class BiRadarAxisScoreDto(BaseModel):
    """Score calculado de 1 pilar."""
    axisKey: str
    axisLabel: str
    axisSub: Optional[str] = None
    rawScore: float
    scoreMax: float
    clusterActive: bool
    clusterBonus: float
    finalScore: float
    riskScore: float
    balanceScore: float
    classification: str
    priorityIndex: float
    isPriorityAxis: bool


class BiRadarSummaryDto(BaseModel):
    """Resumo agregado do calculo."""
    risk: float
    balance: float
    pendingConfiguration: bool


class BiRadarComputationDto(BaseModel):
    """Resposta completa do calculo do Radar para 1 paciente."""
    patientId: str
    calculatedAt: str
    axes: list[BiRadarAxisScoreDto]
    priorityAxisKey: str
    summary: BiRadarSummaryDto
