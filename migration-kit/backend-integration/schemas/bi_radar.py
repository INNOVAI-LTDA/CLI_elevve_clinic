"""Pydantic DTOs para o BI Radar de Longevidade.

Mirror do contract TypeScript em migration-kit/contracts/frontend/biRadar.ts
(espelho manual; nao ha geracao automatica de TS).

Pydantic v2 com field names em camelCase para casar com o JSON
de saida do backend e com o tipo TypeScript do frontend.

F1 refactor: clientes do BI sao User de tipo Client. BiRadarClientDto
substitui o antigo BiRadarPatientDto.
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


class BiRadarClientDto(BaseModel):
    """Cliente (User de tipo Client) usado no BI Radar.

    F1 refactor: substitui BiRadarPatientDto. Os campos sao derivados
    de deva_elevveclinic_users (role='client').
    """
    userId: int
    email: str
    fullName: str
    role: str  # sempre "client" para este DTO
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
    """Resposta completa do calculo do Radar para 1 cliente."""
    userId: int
    calculatedAt: str
    axes: list[BiRadarAxisScoreDto]
    priorityAxisKey: str
    summary: BiRadarSummaryDto
