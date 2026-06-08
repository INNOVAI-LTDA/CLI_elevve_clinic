import type { BiRadarComputationDto } from "../../contracts/biRadar";
import { coerceNumber } from "./domainAdapter";

/**
 * Normaliza o payload cru do backend (ou mock) para o DTO canonico
 * do frontend, com defaults seguros para campos faltantes.
 */
export function adaptBiRadarPayload(payload: unknown): BiRadarComputationDto {
  const dto = (payload ?? {}) as Partial<BiRadarComputationDto>;
  const axes = Array.isArray(dto.axes) ? dto.axes : [];
  const summary = (dto.summary ?? {}) as Partial<
    NonNullable<BiRadarComputationDto["summary"]>
  >;

  return {
    patientId: String(dto.patientId ?? ""),
    calculatedAt: String(dto.calculatedAt ?? ""),
    priorityAxisKey: String(dto.priorityAxisKey ?? ""),
    axes: axes.map((axis) => ({
      axisKey: String(axis.axisKey ?? ""),
      axisLabel: String(axis.axisLabel ?? axis.axisKey ?? "Eixo"),
      axisSub: axis.axisSub ?? null,
      rawScore: coerceNumber(axis.rawScore),
      scoreMax: coerceNumber(axis.scoreMax),
      clusterActive: Boolean(axis.clusterActive),
      clusterBonus: coerceNumber(axis.clusterBonus),
      finalScore: coerceNumber(axis.finalScore),
      riskScore: coerceNumber(axis.riskScore),
      balanceScore: coerceNumber(axis.balanceScore),
      classification: String(axis.classification ?? "Pendente de configuração"),
      priorityIndex: coerceNumber(axis.priorityIndex),
      isPriorityAxis: Boolean(axis.isPriorityAxis),
    })),
    summary: {
      risk: coerceNumber(summary.risk),
      balance: coerceNumber(summary.balance),
      pendingConfiguration: Boolean(summary.pendingConfiguration),
    },
  };
}
