# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

This repo, `CLI_elevve_clinic`, is a delivery package for the **Elevve Clinic** client of `innovai`. The work is delivered as a self-contained `migration-kit/` that can be lifted into the client's target repository. The kit ships three runnable components plus a reference SQL/contracts layer:

- `migration-kit/admin-tui/` — Rust + Ratatui TUI admin front-end (talks to a backend via HTTP; does **not** touch SQLite directly even though it has a `rusqlite` dep for the schema introspection helper).
- `migration-kit/backend-integration/` — FastAPI Python backend (admin DB API, runtime config, error envelope, schemas, services, repositories). The `admin_api.py` module is the SQLite-backed adapter the TUI consumes.
- `migration-kit/frontend-app/` — React 18 + Vite + TypeScript (strict) admin/mentor/student SPA.
- `migration-kit/sql/` — schema migrations (numbered 001..009), SQLite seed DB (`elevve_clinic_deva_db.db`), full schema dump (`sql_create_database.sql`), and `runtime-stores/*.json` initial-load payloads.
- `migration-kit/contracts/` — frozen v1 contract specs (markdown) plus frontend DTOs (`contracts/frontend/*.ts`) that mirror them.

All tables, users, and migrations are prefixed `deva_elevveclinic_` to avoid collision when lifted into the client's repo.

## Common commands

### Frontend (`migration-kit/frontend-app/`)

```bash
npm install
cp .env.example .env
# dev
npm run dev                       # Vite dev on 127.0.0.1:5173
# tests
npm test                         # vitest run (single-shot, jsdom)
npm run test:watch               # vitest watch
# build
VITE_DEPLOY_TARGET=local npm run build
# client-safe build (requires absolute VITE_API_BASE_URL + VITE_CLIENT_CODE)
VITE_DEPLOY_TARGET=client VITE_CLIENT_CODE=<client_code> VITE_API_BASE_URL=https://<backend-domain> VITE_APP_BASE_PATH=/ npm run build
```

`vite.config.ts` validates env contracts at build time via `src/shared/config/envContract.ts` and fails the build on missing/invalid `VITE_DEPLOY_TARGET`, `VITE_API_BASE_URL` (client only), `VITE_CLIENT_CODE` (client only), or malformed `VITE_APP_BASE_PATH`. Keep that contract intact.

### Admin TUI (`migration-kit/admin-tui/`)

```bash
cargo run                        # builds and launches the TUI (default binary)
cargo run --bin db_query_test    # smoke test reading SQLite via rusqlite
```

TUI environment variables (all optional, with defaults):
- `TUI_API_BASE_URL` (default `http://127.0.0.1:8000`)
- `TUI_API_TOKEN` (optional bearer)
- `TUI_SQLITE_DB_PATH` (default `migration-kit/sql/elevve_clinic_deva_db.db`)
- `TUI_SCHEMA_SQL_PATH` (default `migration-kit/sql/sql_create_database.sql`)

The TUI never writes SQLite directly — it only calls the backend over HTTP, even for CSV/SQL load flows. The bundled SQLite DB and `sql_create_database.sql` are reference sources for the schema diagram view.

### Backend (`migration-kit/backend-integration/`)

```bash
# from migration-kit/backend-integration
uvicorn admin_api:app --reload --host 127.0.0.1 --port 8000
```

Backend env (from `migration-kit/env.example`):
- `APP_ENV` (required, `local|development|dev|test` for non-prod)
- `CLIENT_CODE` (required everywhere)
- `APP_AUTH_SECRET`
- `CORS_ALLOW_ORIGINS` (required in production-like envs; must be bare http(s) origins, no `*` in prod)
- `ENABLE_MENTOR_ROUTES` (also accepts legacy `ENABLE_MENTOR_DEMO_ROUTES`)
- `STORAGE_BACKUP_DIR`
- `SUPABASE_DB_URL` (Postgres connection string, used by postgres indicator repos)
- `DB_PROVIDER` (default `sqlite`, used by `admin_api.py`)

### SQL

Apply migrations in numeric order from `migration-kit/sql/migrations/` (001 through 009) — see `migration-kit/sql/README.md`. `runtime-stores/*.json` are the seed/initial-load payloads. `import_pacientes.py` + `run_import_pacientes.ps1` are one-off CSV importers for the SQLite DB.

## Architecture & key contracts

### Error envelope (mandatory)

All HTTP errors from the backend, regardless of status, return:

```json
{ "error": { "status": 409, "code": "MENTORIA_CONFLICT", "message": "...", "details": null } }
```

The `migration-kit/backend-integration/api/errors.py` module exposes `build_error_payload`, `api_error`, and FastAPI exception handlers that enforce this shape. The frontend `src/shared/api/httpClient.ts` parses the same envelope into `AppError` (`src/shared/api/types.ts`). The frozen v1 contract is documented in `migration-kit/contracts/backend/contracts-freeze-v1.md` — do not break field names or types without bumping the contract version.

### Domain language

The contract is named in mentor-domain language: `mentor`, `aluno`, `mentoria`, `metodo`. Legacy technical keys (`organization_id`, `protocol_id`) remain stable for v1. User roles in the system are `admin`, `provider`, `client` (the TUI/internal code still uses `provider`/`client`); the SPA maps to `admin | mentor | aluno` via `src/shared/auth/roleRouting.ts`.

### Frontend layering

`src/app/` — routes (`routes.tsx`), layout (`layout/AppLayout.tsx`), auth provider (`providers/AuthProvider.tsx`). Route guards: `RequireAuth`, `RequireAdmin`, `RequireMentorWorkspace`, `RequireStudentWorkspace`.

`src/domain/` — the slice closest to data:
- `models.ts` — domain types (`StudentListItem`, `StudentDetail`, `CommandCenterStudentCollection`, `RadarAxis`, `StudentRadar`, etc.).
- `services/` — pure service functions (e.g. `commandCenterService.ts`, `matrixService.ts`, `radarService.ts`, `authService.ts`).
- `adapters/` — translate backend DTOs into domain models (`commandCenterAdapter.ts`, `matrixAdapter.ts`, `radarAdapter.ts`).
- `hooks/` — React Query-style async hooks (`useCommandCenter`, `useMatrix`, `useRadar`, `useStudentWorkspace`, plus admin CRUD hooks).

`src/features/` — UI per feature (`command-center/`, `matrix/`, `radar/`, `admin/`, `mentor/`, `student/`, plus entity lists for `users`, `organizations`, `enrollments`, `protocols`, `measurements`, `checkpoints`). Each feature owns its `pages/`, `components/`, `hooks/`, and `*.css`. Trash/scratch work lives in `src/features/_trash/` (excluded from `tsconfig`).

`src/contracts/` — re-exports DTOs that mirror `migration-kit/contracts/frontend/*.ts` (auth, commandCenter, matrix, radar, plus admin*). Keep these in lock-step with the frozen backend contracts.

`src/shared/`:
- `config/env.ts` and `envContract.ts` — env loading, brand pack (`BrandPack`), and validators. `env` is the only export the rest of the app reads.
- `api/httpClient.ts` and `types.ts` — the single HTTP entry point; produces `AppError` from the standard error envelope, injects the bearer token, runs abort/timeout via `AbortController`.
- `auth/` — `tokenStorage.ts`, `authEvents.ts` (emits `unauthorized`), `roleRouting.ts` (role → default route, role allow-list).
- `formatters/` — `currency.ts`, `percent.ts`.
- `ui/ResourceStatePanel.tsx` — shared loading/error/empty state.

`src/main.tsx` sets CSS custom properties from `env.themeColors` (and `* -rgb` channel variants) on `document.documentElement`, so feature CSS consumes them via `rgba(var(--color-*-rgb), α)`.

### Frontend deploy targets

`VITE_DEPLOY_TARGET` is `local` (dev) or `client` (published). The contract:
- `client` builds **require** `VITE_CLIENT_CODE` and an absolute, credential-free `VITE_API_BASE_URL` without query/fragment.
- `local` builds fall back to `http://127.0.0.1:8000` and disable `VITE_ENABLE_DEMO_MODE` and `VITE_ENABLE_INTERNAL_MENTOR_SURFACE` regardless of the env value (see `parseBoolean` semantics in `env.ts`).
- Demo/preview login and the internal mentor surface must stay off in any client build.

### TUI architecture

`migration-kit/admin-tui/src/main.rs` is a single ~3.9k-line Ratatui app. Top-level menus: `1 Manage DB` (with `Schema Diagram`, `1.1 Load from CSV`, `1.2 Load from SQL`), `2 View Matrix Decision`, `3 View Command Center`, `4 View Radar as Provider`, `5 View Radar as Client`. The TUI delegates to the backend at the endpoints listed in `migration-kit/admin-tui/README.md` (admin schema, load-csv, sql list/preview/load, mentor command center, matriz, radar). The DB query smoke binary `src/db_query_test.rs` is a hard-coded read against `deva_elevveclinic_users` for quick verification of the bundled SQLite.

### Backend layering

`backend-integration/` follows a `routes → services → storage` separation with explicit DTOs in `schemas/` and a uniform error envelope from `api/errors.py`. `storage/` holds repositories (json-backed and Postgres-backed depending on the entity), an `io_gate.py` for env-gated persistence, and a `store_registry.py` for runtime stores. `services/` contains both admin CRUD services and mentor-facing services (centro de comando, radar, matriz, carga de indicadores, etc.). `config/runtime.py` is the single source of truth for env parsing, CORS normalization, and the mentor-route policy (`resolve_mentor_route_policy` reads `ENABLE_MENTOR_ROUTES` with `ENABLE_MENTOR_DEMO_ROUTES` as a legacy alias).

## Conventions

- Domain code uses Portuguese where it's user-facing or mirrors the spec, and English in code identifiers. Don't translate `mentor/aluno/mentoria/metodo` — they're the v1 contract vocabulary.
- All migration SQL objects use the `deva_elevveclinic_` prefix.
- The bundled SQLite DB (`migration-kit/sql/elevve_clinic_deva_db.db`) and `sql_create_database.sql` are authoritative references; `runtime-stores/*.json` is the initial-load payload set but is **not** the runtime source for the TUI.
- Tests for the frontend live in `src/test/` and run via `vitest` with `jsdom`. The setup file (`src/test/setup.ts`) stubs `VITE_DEPLOY_TARGET=local` so env contract failures are predictable in tests.
- New env files for client deployments should follow the `migration-kit/frontend-app/.env.example` template (see also `migration-kit/env.example` at the kit root) and must keep `VITE_ENABLE_DEMO_MODE=false` and `VITE_ENABLE_INTERNAL_MENTOR_SURFACE=false`.

## Bootstrap order when lifting to a target repo

See `migration-kit/README-migracao.md`. In short: copy the kit → apply SQL migrations 001..009 in order → import `runtime-stores/*.json` (seed or runtime load) → wire `backend-integration/` into the target backend preserving `route → service → repository` separation → import `contracts/` for payload/DTO compatibility → drop `frontend-app/` into the target frontend, run `npm install` → fill in `env.example` variables → run a client build (`VITE_DEPLOY_TARGET=client`) and the smoke tests in `README-migracao.md` (auth 401 envelope, command center list, radar per student, renewal matrix, initial load, `npm run build`).

## Out of scope for this repo

- `build_output.txt` (repo root) is a leftover from an earlier `cargo run --bin admin-tui` attempt run from the wrong directory; ignore unless debugging that history.
- There is no `Cargo.toml` at the repo root — Rust work lives entirely in `migration-kit/admin-tui/`.
