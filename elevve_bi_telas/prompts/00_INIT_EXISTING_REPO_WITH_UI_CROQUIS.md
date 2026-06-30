# Prompt para /init — Repo existente + Croquis UI + Regra BI

Você está em um repositório existente que já possui `CLAUDE.md` e um `migration-kit/` com backend FastAPI, frontend React/Vite/TypeScript, SQL migrations, contratos e admin TUI.

Leia:

- `CLAUDE.md`
- `CLAUDE_ADDENDUM_ELEVVE_VERTICAL_SLICE.md`
- `docs/BI_RULE_RADAR_CANONICAL.md`
- `docs/DB_VERTICAL_SLICE_SPEC.md`
- `docs/UI_SCREEN_MAPPING_FROM_CROQUIS.md`
- `docs/IMPLEMENTATION_STRATEGY.md`
- `docs/ACCEPTANCE_CRITERIA_VERTICAL_SLICE.md`

Objetivo:

Construir uma fatia vertical do BI Elevve para validar Banco de Dados, regra de BI e visual, sem usar Streamlit.

Instruções:

1. Não sobrescreva o `CLAUDE.md` existente.
2. Não crie app Streamlit.
3. Não crie app paralelo.
4. Inspecione a arquitetura existente antes de alterar arquivos.
5. Reaproveite features existentes de radar, matrix e command-center se existirem.
6. Use os croquis em `assets/croquis/` como referência visual.
7. Use os ícones em `assets/icons/` apenas se forem úteis.
8. Implemente primeiro o Radar de Longevidade.
9. Mantenha regra de BI isolada do frontend.
10. Antes de codar, apresente:
    - arquivos que pretende ler;
    - arquivos que pretende alterar;
    - plano de implementação;
    - riscos;
    - comandos de teste.

Não implemente ainda. Aguarde confirmação.
