/**
 * BI Radar de Longevidade - contract DTO (espelho do backend
 * FastAPI em migration-kit/backend-integration/schemas/bi_radar.py).
 *
 * Manter este arquivo em sincronia manual com o spec autoritativo
 * em migration-kit/contracts/frontend/biRadar.ts.
 *
 * F1 refactor: clientes do BI sao User de tipo Client em
 * deva_elevveclinic_users. BiRadarClientDto substitui o antigo
 * BiRadarPatientDto.
 *
 * Convencoes:
 * - camelCase para todos os campos (Pydantic emite JSON com o nome
 *   do field por default; field names ja estao em camelCase).
 * - Optional/nullable: description, axisSub, scoreMax, programName
 *   podem ser null.
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

export type BiRadarClientDto = {
  userId: number;
  email: string;
  fullName: string;
  role: string;
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
  userId: number;
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
