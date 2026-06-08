import { useCallback } from "react";
import { useAsyncResource } from "./useAsyncResource";
import {
  getPatientBiRadar,
  listBiPatients,
  listBiPillars,
} from "../services/biRadarService";
import { adaptBiRadarPayload } from "../adapters/biRadarAdapter";
import type {
  BiRadarComputationDto,
  BiRadarPatientDto,
  BiRadarPillarDto,
} from "../../contracts/biRadar";

const EMPTY_COMPUTATION: BiRadarComputationDto = adaptBiRadarPayload({});

/**
 * Hook que carrega o calculo do BI Radar de Longevidade para 1 paciente.
 * Usa fallback de mock no service quando a API esta fora.
 */
export function usePatientBiRadar(patientId: string | null) {
  const loader = useCallback(() => {
    if (!patientId) {
      return Promise.resolve(EMPTY_COMPUTATION);
    }
    return getPatientBiRadar(patientId);
  }, [patientId]);

  return useAsyncResource<BiRadarComputationDto>(loader, [loader], {
    enabled: true,
    initialData: EMPTY_COMPUTATION,
    isEmpty: (data) => data.axes.length === 0,
    resourceName: "BI Radar do paciente",
  });
}

export function useBiPatientList() {
  const loader = useCallback(() => listBiPatients(), []);
  return useAsyncResource<BiRadarPatientDto[]>(loader, [loader], {
    enabled: true,
    initialData: [],
    isEmpty: (data) => data.length === 0,
    resourceName: "lista de pacientes BI",
  });
}

export function useBiPillars() {
  const loader = useCallback(() => listBiPillars(), []);
  return useAsyncResource<BiRadarPillarDto[]>(loader, [loader], {
    enabled: true,
    initialData: [],
    isEmpty: (data) => data.length === 0,
    resourceName: "pilares do BI Radar",
  });
}
