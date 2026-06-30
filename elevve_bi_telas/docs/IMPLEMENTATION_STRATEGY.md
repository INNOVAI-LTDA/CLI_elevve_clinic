# Estratégia de Implementação — Migration-kit, sem Streamlit

## Objetivo

Validar BI + Banco + Visual com uma fatia vertical real no `migration-kit`.

## Por que não Streamlit

O repo já possui frontend React, backend FastAPI, contratos, migrations e camadas de domínio. Usar Streamlit criaria uma arquitetura paralela e pouco reaproveitável.

## Vertical Slice

```text
Banco
  ↓
Repository
  ↓
Service de cálculo
  ↓
Schema/DTO
  ↓
Endpoint FastAPI
  ↓
Frontend service/adapter/hook
  ↓
Tela React
```

## Sprint 1 — Radar ponta a ponta

Entregáveis:

- migration SQL;
- seed;
- service de cálculo;
- endpoint;
- tela Radar;
- croquis traduzidos para componentes.

## Sprint 2 — Expansão visual

Entregáveis:

- Visão Geral;
- Histórico/Evolução stub;
- Relatório/Output stub;
- Matriz/Centro de Comando se compatível com features existentes;
- refinamento visual.

## Ordem obrigatória para Claude Code

1. Inspecionar estrutura existente.
2. Identificar features já existentes de radar/matrix/command-center.
3. Propor plano de alteração.
4. Aguardar confirmação antes de codar.
5. Implementar em pequenos commits lógicos.
