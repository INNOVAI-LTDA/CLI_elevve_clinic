import { adaptBiRadarPayload } from "../adapters/biRadarAdapter";
import { httpClient } from "../../shared/api/httpClient";
import type {
  BiRadarComputationDto,
  BiRadarPatientDto,
  BiRadarPillarDto,
} from "../../contracts/biRadar";

const BI_BASE = "/bi/radar";

/**
 * Lista os 5 pilares do BI Radar de Longevidade.
 */
export async function listBiPillars(): Promise<BiRadarPillarDto[]> {
  try {
    const payload = await httpClient.get<unknown>(`${BI_BASE}/pillars`);
    if (!Array.isArray(payload)) {
      return [];
    }
    return payload as BiRadarPillarDto[];
  } catch {
    return getBiPillarsMock();
  }
}

/**
 * Lista os pacientes do BI (com fallback para mock em dev).
 */
export async function listBiPatients(): Promise<BiRadarPatientDto[]> {
  try {
    const payload = await httpClient.get<unknown>(`${BI_BASE}/patients`);
    if (!Array.isArray(payload)) {
      return [];
    }
    return payload as BiRadarPatientDto[];
  } catch {
    return getBiPatientsMock();
  }
}

/**
 * Calcula o BI Radar de Longevidade para 1 paciente.
 * Fallback para mock em dev quando a API esta fora.
 */
export async function getPatientBiRadar(
  patientId: string
): Promise<BiRadarComputationDto> {
  if (!patientId) {
    return adaptBiRadarPayload({});
  }
  try {
    const payload = await httpClient.get<unknown>(
      `${BI_BASE}/patients/${encodeURIComponent(patientId)}`
    );
    return adaptBiRadarPayload(payload);
  } catch {
    return getPatientBiRadarMock(patientId);
  }
}

// ---------- Mocks (dev/offline) ----------

function getBiPillarsMock(): BiRadarPillarDto[] {
  return [
    {
      pillarCode: "hormonios",
      name: "Hormônios",
      description: "Estabilidade hormonal e neuroendócrina.",
      scoreMax: 36,
      displayOrder: 1,
    },
    {
      pillarCode: "intestino",
      name: "Intestino",
      description: "Digestão, absorção e tolerância alimentar adequadas.",
      scoreMax: 36,
      displayOrder: 2,
    },
    {
      pillarCode: "recuperacao",
      name: "Recuperação",
      description: "Capacidade de recuperação e adaptação ao estresse.",
      scoreMax: 39,
      displayOrder: 3,
    },
    {
      pillarCode: "estrutura",
      name: "Estrutura",
      description: "Força, resistência e integridade física ao longo do tempo.",
      scoreMax: 39,
      displayOrder: 4,
    },
    {
      pillarCode: "metabolismo",
      name: "Metabolismo",
      description: "Produção e estabilidade de energia metabólica.",
      scoreMax: 33,
      displayOrder: 5,
    },
  ];
}

function getBiPatientsMock(): BiRadarPatientDto[] {
  return [
    {
      patientId: "bi_pat_demo_01",
      patientCode: "BI-DEMO-01",
      name: "Paciente Demo 01",
      programName: "Mentoria Acelerador Medico",
    },
    {
      patientId: "bi_pat_demo_02",
      patientCode: "BI-DEMO-02",
      name: "Paciente Demo 02",
      programName: "Mentoria Acelerador Medico",
    },
  ];
}

/**
 * Mock do calculo - replica os scores esperados do seed do
 * Commit 2 para os 2 pacientes demo. Em caso de API fora, o
 * frontend continua exibindo os mesmos numeros que o backend
 * produz quando o seed esta aplicado.
 */
function getPatientBiRadarMock(
  patientId: string
): BiRadarComputationDto {
  if (patientId === "bi_pat_demo_01") {
    return {
      patientId,
      calculatedAt: new Date().toISOString(),
      priorityAxisKey: "metabolismo",
      axes: [
        {
          axisKey: "hormonios",
          axisLabel: "Hormônios",
          axisSub: "",
          rawScore: 20,
          scoreMax: 36,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 20,
          riskScore: 55.555556,
          balanceScore: 44.444444,
          classification: "Pendente de configuração",
          priorityIndex: 20,
          isPriorityAxis: false,
        },
        {
          axisKey: "intestino",
          axisLabel: "Intestino",
          axisSub: "",
          rawScore: 19,
          scoreMax: 36,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 19,
          riskScore: 52.777778,
          balanceScore: 47.222222,
          classification: "Desregulação importante",
          priorityIndex: 19,
          isPriorityAxis: false,
        },
        {
          axisKey: "recuperacao",
          axisLabel: "Recuperação",
          axisSub: "",
          rawScore: 20,
          scoreMax: 45,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 20,
          riskScore: 44.444444,
          balanceScore: 55.555556,
          classification: "Pendente de configuração",
          priorityIndex: 20,
          isPriorityAxis: false,
        },
        {
          axisKey: "estrutura",
          axisLabel: "Estrutura",
          axisSub: "",
          rawScore: 15,
          scoreMax: 39,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 15,
          riskScore: 38.461538,
          balanceScore: 61.538462,
          classification: "Pendente de configuração",
          priorityIndex: 15,
          isPriorityAxis: false,
        },
        {
          axisKey: "metabolismo",
          axisLabel: "Metabolismo",
          axisSub: "",
          rawScore: 26,
          scoreMax: 33,
          clusterActive: true,
          clusterBonus: 3,
          finalScore: 29,
          riskScore: 87.878788,
          balanceScore: 12.121212,
          classification: "Eixo prioritário",
          priorityIndex: 32,
          isPriorityAxis: true,
        },
      ],
      summary: {
        risk: 55.823621,
        balance: 44.176379,
        pendingConfiguration: true,
      },
    };
  }
  if (patientId === "bi_pat_demo_02") {
    return {
      patientId,
      calculatedAt: new Date().toISOString(),
      priorityAxisKey: "recuperacao",
      axes: [
        {
          axisKey: "hormonios",
          axisLabel: "Hormônios",
          axisSub: "",
          rawScore: 19,
          scoreMax: 36,
          clusterActive: true,
          clusterBonus: 3,
          finalScore: 22,
          riskScore: 61.111111,
          balanceScore: 38.888889,
          classification: "Pendente de configuração",
          priorityIndex: 25,
          isPriorityAxis: false,
        },
        {
          axisKey: "intestino",
          axisLabel: "Intestino",
          axisSub: "",
          rawScore: 12,
          scoreMax: 36,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 12,
          riskScore: 33.333333,
          balanceScore: 66.666667,
          classification: "Atenção funcional",
          priorityIndex: 12,
          isPriorityAxis: false,
        },
        {
          axisKey: "recuperacao",
          axisLabel: "Recuperação",
          axisSub: "",
          rawScore: 22,
          scoreMax: 45,
          clusterActive: true,
          clusterBonus: 3,
          finalScore: 25,
          riskScore: 55.555556,
          balanceScore: 44.444444,
          classification: "Pendente de configuração",
          priorityIndex: 28,
          isPriorityAxis: true,
        },
        {
          axisKey: "estrutura",
          axisLabel: "Estrutura",
          axisSub: "",
          rawScore: 13,
          scoreMax: 39,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 13,
          riskScore: 33.333333,
          balanceScore: 66.666667,
          classification: "Pendente de configuração",
          priorityIndex: 13,
          isPriorityAxis: false,
        },
        {
          axisKey: "metabolismo",
          axisLabel: "Metabolismo",
          axisSub: "",
          rawScore: 11,
          scoreMax: 33,
          clusterActive: false,
          clusterBonus: 0,
          finalScore: 11,
          riskScore: 33.333333,
          balanceScore: 66.666667,
          classification: "Atenção funcional",
          priorityIndex: 11,
          isPriorityAxis: false,
        },
      ],
      summary: {
        risk: 43.333333,
        balance: 56.666667,
        pendingConfiguration: true,
      },
    };
  }
  // Fallback generico
  return adaptBiRadarPayload({});
}
