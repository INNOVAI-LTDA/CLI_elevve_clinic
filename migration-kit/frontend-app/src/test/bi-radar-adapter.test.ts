import { describe, expect, it } from "vitest";
import { adaptBiRadarPayload } from "../domain/adapters/biRadarAdapter";
import type { BiRadarComputationDto } from "../contracts/biRadar";

describe("adaptBiRadarPayload", () => {
  it("retorna defaults seguros para payload vazio", () => {
    const result = adaptBiRadarPayload({});
    expect(result.patientId).toBe("");
    expect(result.calculatedAt).toBe("");
    expect(result.priorityAxisKey).toBe("");
    expect(result.axes).toEqual([]);
    expect(result.summary.risk).toBe(0);
    expect(result.summary.balance).toBe(0);
    expect(result.summary.pendingConfiguration).toBe(false);
  });

  it("trata payload null/undefined sem quebrar", () => {
    expect(adaptBiRadarPayload(null).axes).toEqual([]);
    expect(adaptBiRadarPayload(undefined).axes).toEqual([]);
  });

  it("preserva campos validos do payload", () => {
    const payload: Partial<BiRadarComputationDto> = {
      patientId: "pat_x",
      calculatedAt: "2026-06-08T12:00:00Z",
      priorityAxisKey: "metabolismo",
      axes: [
        {
          axisKey: "metabolismo",
          axisLabel: "Metabolismo",
          axisSub: null,
          rawScore: 26,
          scoreMax: 33,
          clusterActive: true,
          clusterBonus: 3,
          finalScore: 29,
          riskScore: 87.88,
          balanceScore: 12.12,
          classification: "Eixo prioritário",
          priorityIndex: 32,
          isPriorityAxis: true,
        },
      ],
      summary: {
        risk: 55.8,
        balance: 44.2,
        pendingConfiguration: true,
      },
    };
    const result = adaptBiRadarPayload(payload);
    expect(result.patientId).toBe("pat_x");
    expect(result.priorityAxisKey).toBe("metabolismo");
    expect(result.axes).toHaveLength(1);
    const ax = result.axes[0];
    expect(ax.axisKey).toBe("metabolismo");
    expect(ax.clusterActive).toBe(true);
    expect(ax.clusterBonus).toBe(3);
    expect(ax.riskScore).toBe(87.88);
    expect(ax.classification).toBe("Eixo prioritário");
    expect(ax.isPriorityAxis).toBe(true);
    expect(result.summary.pendingConfiguration).toBe(true);
  });

  it("coerce strings numericas em eixos via coerceNumber", () => {
    // O backend emite strings em alguns pontos do contrato
    // (ex.: quando vem de mock ou formatos legados); o adapter
    // precisa normalizar.
    const result = adaptBiRadarPayload({
      axes: [
        {
          axisKey: "hormonios",
          axisLabel: "Hormônios",
          rawScore: "20",
          scoreMax: "36",
          clusterBonus: "0",
          finalScore: "20",
          riskScore: "55.5",
          balanceScore: "44.4",
          priorityIndex: "20",
          classification: "Pendente de configuração",
          isPriorityAxis: false,
        },
      ],
    });
    expect(result.axes[0].rawScore).toBe(20);
    expect(result.axes[0].scoreMax).toBe(36);
    expect(result.axes[0].riskScore).toBe(55.5);
    expect(result.axes[0].balanceScore).toBe(44.4);
  });

  it("defaulta axes ausentes para lista vazia", () => {
    const result = adaptBiRadarPayload({ patientId: "p1" });
    expect(result.axes).toEqual([]);
    expect(result.priorityAxisKey).toBe("");
  });

  it("defaulta summary ausente para zeros", () => {
    const result = adaptBiRadarPayload({ patientId: "p1" });
    expect(result.summary.risk).toBe(0);
    expect(result.summary.balance).toBe(0);
    expect(result.summary.pendingConfiguration).toBe(false);
  });
});
