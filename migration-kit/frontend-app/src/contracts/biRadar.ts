/**
 * BI Radar de Longevidade - contract DTO (espelho do
 * migration-kit/contracts/frontend/biRadar.ts).
 *
 * Manter este arquivo em sincronia manual com o spec autoritativo
 * em migration-kit/contracts/frontend/biRadar.ts.
 *
 * Convencoes:
 * - camelCase para todos os campos (mirror do Pydantic v2).
 * - Optional/nullable: description, axisSub, scoreMax, programName
 *   podem ser null.
 * - Envelope de erro canonico: { "error": { "status", "code",
 *   "message", "details" } } (api/errors.py).
 */

export type BiRadarPillarDto = {
  pillarCode: string;
  name: string;
  description?: string | null;
  scoreMax?: number | null;
  displayOrder: number;
};

export type BiRadarPatientDto = {
  patientId: string;
  patientCode: string;
  name: string;
  programName?: string | null;
};

export type BiRadarAxisScoreDto = {
  axisKey: string;
  axisLabel: string;
  axisSub?: string | null;
  rawScore: number;
  scoreMax: number;
  clusterActive: boolean;
  clusterBonus: number;
  finalScore: number;
  riskScore: number;
  balanceScore: number;
  classification: string;
  priorityIndex: number;
  isPriorityAxis: boolean;
};

export type BiRadarSummaryDto = {
  risk: number;
  balance: number;
  pendingConfiguration: boolean;
};

export type BiRadarComputationDto = {
  patientId: string;
  calculatedAt: string;
  axes: BiRadarAxisScoreDto[];
  priorityAxisKey: string;
  summary: BiRadarSummaryDto;
};

export type BiRadarErrorPayload = {
  error: {
    status: number;
    code: string;
    message: string;
    details: unknown;
  };
};
