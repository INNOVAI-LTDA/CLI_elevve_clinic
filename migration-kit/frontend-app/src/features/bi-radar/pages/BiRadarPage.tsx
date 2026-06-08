import { useEffect, useState } from "react";
import { useBiPatientList, usePatientBiRadar } from "../../../domain/hooks/useBiRadar";
import { MentorShell } from "../../mentor/components/MentorShell";
import "../bi-radar.css";

/**
 * Tela principal do BI Radar de Longevidade (Commit 5).
 *
 * Sem grafico ainda (vai no Commit 6). Lista os 5 pilares com
 * scores brutos, cluster, score final, risco, equilibrio e
 * classification. Destaque do eixo prioritario e badges
 * "Pendente de configuracao" onde a spec nao tem faixa.
 */
export function BiRadarPage() {
  const patientsResource = useBiPatientList();
  const [selectedPatientId, setSelectedPatientId] = useState<string | null>(null);

  useEffect(() => {
    if (!selectedPatientId && patientsResource.data.length > 0) {
      setSelectedPatientId(patientsResource.data[0].patientId);
    }
  }, [selectedPatientId, patientsResource.data]);

  const radarResource = usePatientBiRadar(selectedPatientId);

  const selectedPatient =
    patientsResource.data.find((p) => p.patientId === selectedPatientId) ?? null;

  const axisCount = radarResource.data.axes.length;
  const clusterCount = radarResource.data.axes.filter(
    (a) => a.clusterActive
  ).length;
  const pendingCount = radarResource.data.axes.filter(
    (a) => a.classification === "Pendente de configuração"
  ).length;
  const priorityAxis =
    radarResource.data.axes.find((a) => a.isPriorityAxis) ?? null;

  return (
    <MentorShell
      activeView="bi-radar"
      brandLabel="BI Elevve"
      brandTitle="Radar de Longevidade"
      metrics={[
        { label: "Eixos ativos", value: String(axisCount) },
        {
          label: "Risco médio",
          value: `${radarResource.data.summary.risk.toFixed(1)}%`,
          tone: "warning",
        },
        {
          label: "Equilíbrio médio",
          value: `${radarResource.data.summary.balance.toFixed(1)}%`,
          tone: "success",
        },
        {
          label: "Pendências",
          value: String(pendingCount),
          tone: pendingCount > 0 ? "warning" : "neutral",
        },
      ]}
    >
      <section className="bi-radar-page">
        <section className="bi-radar-control-row">
          <label className="bi-radar-select-wrap">
            <span>Paciente</span>
            <select
              value={selectedPatientId ?? ""}
              onChange={(event) =>
                setSelectedPatientId(event.target.value || null)
              }
              disabled={
                patientsResource.loading || patientsResource.data.length === 0
              }
            >
              {patientsResource.data.length === 0 && (
                <option value="">Sem pacientes</option>
              )}
              {patientsResource.data.map((p) => (
                <option key={p.patientId} value={p.patientId}>
                  {p.name} ({p.patientCode})
                </option>
              ))}
            </select>
          </label>

          <article className="bi-radar-patient-chip">
            <p>Paciente selecionado</p>
            <strong>{selectedPatient?.name ?? "Sem seleção"}</strong>
            <span>
              {selectedPatient?.programName ?? "Programa não informado"}
            </span>
            {priorityAxis && (
              <span>
                Eixo prioritário: <strong>{priorityAxis.axisLabel}</strong>
              </span>
            )}
          </article>
        </section>

        {patientsResource.loading && patientsResource.data.length === 0 && (
          <p className="bi-radar-state">Carregando pacientes para o BI Radar...</p>
        )}
        {patientsResource.error && patientsResource.data.length === 0 && (
          <div className="bi-radar-state bi-radar-state--error">
            <p>{patientsResource.error}</p>
            <button type="button" onClick={() => void patientsResource.refresh()}>
              Tentar novamente
            </button>
          </div>
        )}

        <section className="bi-radar-panel">
          <header>
            <h2>Pilares do Radar de Longevidade</h2>
            <p>
              Score bruto, bônus de cluster, score final, risco,
              equilíbrio e classificação por pilar.
            </p>
          </header>

          {radarResource.loading && radarResource.data.axes.length === 0 && (
            <p className="bi-radar-state">Carregando radar do paciente...</p>
          )}
          {radarResource.error && radarResource.data.axes.length === 0 && (
            <div className="bi-radar-state bi-radar-state--error">
              <p>{radarResource.error}</p>
              <button type="button" onClick={() => void radarResource.refresh()}>
                Tentar novamente
              </button>
            </div>
          )}
          {radarResource.data.axes.length === 0 &&
            !radarResource.loading &&
            !radarResource.error && (
              <p className="bi-radar-state">
                Sem dados de BI Radar para este paciente.
              </p>
            )}

          {radarResource.data.axes.length > 0 && (
            <ul className="bi-radar-pillar-list">
              {radarResource.data.axes.map((axis) => (
                <li
                  key={axis.axisKey}
                  className={
                    axis.isPriorityAxis
                      ? "bi-radar-pillar-item is-priority"
                      : "bi-radar-pillar-item"
                  }
                >
                  <div className="bi-radar-pillar-top">
                    <strong>{axis.axisLabel}</strong>
                    {axis.isPriorityAxis && <small>Prioritário</small>}
                  </div>
                  <div className="bi-radar-pillar-values">
                    <span>
                      bruto{" "}
                      <strong>
                        {axis.rawScore.toFixed(1)}/{axis.scoreMax.toFixed(0)}
                      </strong>
                    </span>
                    <span>
                      bônus{" "}
                      <strong>
                        {axis.clusterActive
                          ? `+${axis.clusterBonus.toFixed(0)} (cluster)`
                          : "—"}
                      </strong>
                    </span>
                    <span>
                      final{" "}
                      <strong>{axis.finalScore.toFixed(1)}</strong>
                    </span>
                    <span>
                      risco{" "}
                      <strong>{axis.riskScore.toFixed(1)}%</strong> ·{" "}
                      equilíbrio{" "}
                      <strong>{axis.balanceScore.toFixed(1)}%</strong>
                    </span>
                  </div>
                  <ClassificationBadge classification={axis.classification} />
                </li>
              ))}
            </ul>
          )}
        </section>
      </section>
    </MentorShell>
  );
}

function ClassificationBadge({ classification }: { classification: string }) {
  const cssClass = badgeClassFor(classification);
  return (
    <span className={cssClass}>
      {classification}
    </span>
  );
}

function badgeClassFor(classification: string): string {
  // Normaliza acentos/caixa para mapear em classes CSS tematicas
  const key = classification
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .toLowerCase();
  if (key.includes("pendente")) return "bi-radar-classification bi-radar-classification--pending";
  if (key.includes("prioritario") || key.includes("critico"))
    return "bi-radar-classification bi-radar-classification--prioritario";
  if (key.includes("desregulacao") || key.includes("atencao"))
    return "bi-radar-classification bi-radar-classification--alerta";
  if (key.includes("estavel"))
    return "bi-radar-classification bi-radar-classification--estavel";
  return "bi-radar-classification";
}
