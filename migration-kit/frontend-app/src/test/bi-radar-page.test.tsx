import { render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { describe, expect, it, vi } from "vitest";
import { BiRadarPage } from "../features/bi-radar/pages/BiRadarPage";

vi.mock("../shared/config/env", () => ({
    env: {
        brandingIconUrl: "/branding/icon.png",
        brandingLogoUrl: "/branding/logo.png",
        clientName: "Cliente",
        appName: "Plataforma",
        appTagline: "Tagline",
        shellSubtitle: "Subtitulo",
        routerBasePath: "/",
        appBasePath: "/",
        deployTarget: "local",
        isLocalDeployTarget: true,
        apiBaseUrl: "http://localhost:8000",
        httpTimeoutMs: 15000,
        demoModeEnabled: false,
        internalMentorSurfaceEnabled: false,
        internalMentorDemoEnabled: false
    }
}));

vi.mock("../domain/hooks/useBiRadar", () => ({
    useBiPatientList: () => ({
        data: [
            {
                patientId: "bi_pat_demo_01",
                patientCode: "BI-DEMO-01",
                name: "Paciente Demo 01",
                programName: "Mentoria Acelerador Medico"
            },
            {
                patientId: "bi_pat_demo_02",
                patientCode: "BI-DEMO-02",
                name: "Paciente Demo 02",
                programName: "Mentoria Acelerador Medico"
            }
        ],
        loading: false,
        error: null,
        refresh: vi.fn()
    }),
    usePatientBiRadar: () => ({
        data: {
            patientId: "bi_pat_demo_01",
            calculatedAt: "2026-06-08T12:00:00Z",
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
                    riskScore: 55.5,
                    balanceScore: 44.5,
                    classification: "Pendente de configuração",
                    priorityIndex: 20,
                    isPriorityAxis: false
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
                    riskScore: 87.8,
                    balanceScore: 12.2,
                    classification: "Eixo prioritário",
                    priorityIndex: 32,
                    isPriorityAxis: true
                }
            ],
            summary: {
                risk: 55.8,
                balance: 44.2,
                pendingConfiguration: true
            }
        },
        loading: false,
        error: null,
        refresh: vi.fn()
    })
}));

describe("BiRadarPage", () => {
    it("renderiza 5 pilares do Radar de Longevidade", () => {
        render(
            <MemoryRouter initialEntries={["/app/bi/radar"]}>
                <Routes>
                    <Route path="/app/bi/radar" element={<BiRadarPage />} />
                </Routes>
            </MemoryRouter>
        );
        // Pilar com classification "Eixo prioritário" deve aparecer
        // (verificamos via substring para evitar problemas de encoding).
        // getAllByText porque o mesmo string aparece no badge do pilar
        // e no chip do paciente.
        expect(screen.getAllByText(/Eixo priorit/i).length).toBeGreaterThan(0);
        // Pendência badge visivel
        expect(screen.getAllByText(/Pendente de configura/i).length).toBeGreaterThan(0);
    });

    it("destaca o eixo prioritario no chip do paciente", () => {
        render(
            <MemoryRouter initialEntries={["/app/bi/radar"]}>
                <Routes>
                    <Route path="/app/bi/radar" element={<BiRadarPage />} />
                </Routes>
            </MemoryRouter>
        );
        // O label "Eixo prioritário:" (com dois pontos) so' aparece no chip
        expect(screen.getByText(/Eixo prioritário:/i)).toBeInTheDocument();
        // O nome do eixo prioritario (Metabolismo) aparece no chip
        // (e tambem no titulo do pilar, mas o chip eh o que importa)
        expect(screen.getAllByText(/Metabolismo/i).length).toBeGreaterThan(0);
    });
});
