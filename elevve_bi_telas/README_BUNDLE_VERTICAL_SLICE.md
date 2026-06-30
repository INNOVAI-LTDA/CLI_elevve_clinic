# Bundle Claude Code — Elevve BI Vertical Slice com Croquis de UI

## Objetivo

Este bundle atualiza o contexto do Claude Code para construir uma **fatia vertical real** do BI Elevve dentro do `migration-kit`, validando:

1. Banco de Dados
2. Regra de BI
3. Backend FastAPI
4. Frontend React/Vite
5. Visual baseado nos croquis enviados

## Decisão técnica

**Não usar Streamlit.**

Motivo: o repositório existente já possui `migration-kit` com backend FastAPI, frontend React/Vite/TypeScript, contratos, SQL e camadas de domínio. O piloto deve validar a arquitetura real do produto, não uma prova paralela.

## Como usar

1. Copie o conteúdo deste bundle para a raiz do repo.
2. Não sobrescreva o `CLAUDE.md` atual.
3. Adicione `CLAUDE_ADDENDUM_ELEVVE_VERTICAL_SLICE.md` na raiz.
4. Execute Claude Code na raiz.
5. Rode `/init`.
6. Cole o prompt em:

```text
prompts/00_INIT_EXISTING_REPO_WITH_UI_CROQUIS.md
```

## Estrutura

```text
CLAUDE_ADDENDUM_ELEVVE_VERTICAL_SLICE.md
docs/
  BI_RULE_RADAR_CANONICAL.md
  DB_VERTICAL_SLICE_SPEC.md
  UI_SCREEN_MAPPING_FROM_CROQUIS.md
  IMPLEMENTATION_STRATEGY.md
  ACCEPTANCE_CRITERIA_VERTICAL_SLICE.md

prompts/
  00_INIT_EXISTING_REPO_WITH_UI_CROQUIS.md
  01_INSPECT_EXISTING_FEATURES_AND_CONTRACTS.md
  02_CREATE_RADAR_DB_MIGRATION_AND_SEED.md
  03_IMPLEMENT_RADAR_SCORING_BACKEND.md
  04_IMPLEMENT_RADAR_API_CONTRACT.md
  05_IMPLEMENT_RADAR_FRONTEND_FROM_CROQUIS.md
  06_IMPLEMENT_HISTORY_AND_REPORT_STUBS.md
  07_REVIEW_VERTICAL_SLICE.md

assets/
  croquis/
  icons/

tasks/
  WBS_VERTICAL_SLICE.md
```

## Artefatos visuais incorporados

Os croquis e a paleta de ícones do arquivo `EIXO_UI_Bundle.zip` foram extraídos para:

```text
assets/croquis/
assets/icons/
```

## Regra principal

A regra oficial do BI está documentada em:

```text
docs/BI_RULE_RADAR_CANONICAL.md
```

A fonte bruta, quando disponível, foi copiada para:

```text
docs/raw_BI_ElevveClinic.txt
```

## Resultado esperado

Ao final da primeira fatia vertical:

- banco tem tabelas/seed do Radar;
- backend calcula o Radar;
- API expõe o resultado por paciente;
- frontend renderiza a tela Radar baseada nos croquis;
- histórico e relatório ficam pelo menos stubados/estruturados;
- nada fora do escopo é implementado.
