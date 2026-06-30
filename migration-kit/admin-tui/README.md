# Admin TUI (Ratatui)

Frontend administrativo em terminal construido com Rust + `ratatui`.

Arquitetura:
- TUI (Rust) isolado do banco
- comunicacao apenas via API HTTP com backend
- backend responsavel por acessar o SQLite (DB Browser)

Fonte de referencia de schema:
- schema SQL de `migration-kit/sql/sql_create_database.sql`
- schema atual via endpoint de API (`/admin/db/schema`) lendo o SQLite
- `runtime-stores/*.json` nao sao usados como fonte principal do TUI

## Login

- `username`: `root`
- `password`: `toor`

## Painel principal

- `1 - Manage DB`
- `2 - View Matrix Decision`
- `3 - View Command Center`
- `4 - View Radar as Provider`
- `5 - View Radar as Client`

## Execucao

Na pasta `migration-kit/admin-tui`:

```powershell
cargo run
```

Variaveis de ambiente:

```powershell
$env:TUI_API_BASE_URL="http://127.0.0.1:8000"
$env:TUI_API_TOKEN="seu_token_bearer_opcional"
$env:TUI_SQLITE_DB_PATH="migration-kit/sql/elevve_clinic_deva_db.db"
$env:TUI_SCHEMA_SQL_PATH="migration-kit/sql/sql_create_database.sql"
```

## Navegacao

- `UP/DOWN`: navegar listas
- `ENTER`: abrir opcao selecionada
- `Q` ou `ESC`: voltar/sair da tela atual
- `TAB` (no login): alternar entre username e password

## Acoes extras

- `Manage DB`:
  - submenu:
    - `Schema Diagram`: exibe schema das tabelas (estilo diagrama de classes) com relacoes FK
    - `1.1 - Load from CSV`
    - `1.2 - Load from SQL`
  - em `Schema Diagram`:
    - `B`: gerar backup do SQLite + schema SQL em `sql/backups/<timestamp>`
    - `R`: recarregar schema e caches pela API
- `Matrix`:
  - `LEFT/RIGHT`: trocar filtro (`all`, `topRight`, `critical`, `rescue`)

## 1.1 Load from CSV

Fluxo:
- escolher tabela de destino
- informar caminho do CSV
- app detecta schema pelo cabecalho e abre de-para manual
- `V`: validar mapeamento (com alertas)
- `L`: enviar carga via API (backend executa no SQLite)

Observacoes:
- o parser CSV detecta delimitador (`;`, `,`, `TAB`, `|`)
- revisar sempre o de-para e tipos antes de confirmar carga
- validacao de constraints/tipos ocorre no backend e no banco

## 1.2 Load from SQL

Fluxo:
- informar diretorio com scripts `.sql`
- selecionar script
- app solicita preview do schema pela API para confirmacao
- confirmar com `Y` para executar via backend
- app exibe resultado da execucao

## Endpoints esperados pelo TUI

- `GET /admin/db/schema`
- `POST /admin/db/load-csv`
- `POST /admin/db/sql/list`
- `POST /admin/db/sql/preview`
- `POST /admin/db/load-sql`
- `GET /mentor/centro-comando/alunos`
- `GET /mentor/centro-comando/alunos/{student_id}`
- `GET /mentor/centro-comando/alunos/{student_id}/timeline-anomalias`
- `GET /mentor/matriz-renovacao?filter=...`
- `GET /mentor/radar/alunos/{student_id}`

Observacao:
- para `View Radar as Provider`, o TUI tenta `GET /admin/mentores`; se indisponivel, usa o `context` retornado por `GET /mentor/centro-comando/alunos`.
