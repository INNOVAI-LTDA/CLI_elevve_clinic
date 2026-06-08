/**
 * BI Radar de Longevidade — contract DTO (espelho do backend
 * FastAPI em migration-kit/backend-integration/schemas/bi_radar.py).
 *
 * Quando o frontend precisar deste contract, criar copia em
 * migration-kit/frontend-app/src/contracts/biRadar.ts (Commit 5).
 *
 * Convencoes:
 * - camelCase para todos os campos (Pydantic emite JSON com o nome
 *   do field por default; field names ja estao em camelCase).
 * - Optional/nullable: `description`, `axisSub`, `scoreMax`,
 *   `programName` podem ser null.
 * - Envelope de erro segue o contrato canonico (api/errors.py):
 *     { "error": { "status", "code", "message", "details" } }
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
