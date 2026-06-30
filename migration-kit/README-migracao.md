# Migration Kit (para viagem)

Este pacote isola o núcleo portátil para bootstrap em outro repositório.

## Estrutura

- `sql/`
  - `migrations/`: schema SQL versionado
  - `runtime-stores/`: seeds e stores de runtime em JSON
- `backend-integration/`
  - `services/`: regras de negócio
  - `storage/`: persistência e repositórios
  - `schemas/`: DTOs/schemas backend
  - `config/runtime.py`: contrato de ambiente/runtime
  - `api/errors.py`: envelope padronizado de erro
- `contracts/`
  - `backend/`: contratos congelados v1
  - `frontend/`: DTOs de contrato do frontend
- `frontend-app/`
  - `src/`: código-fonte React + TypeScript strict
  - `public/`: estáticos públicos, incluindo `public/branding/*`
  - `index.html`, `vite.config.ts`, `tsconfig.json`: runtime/build do Vite
  - `package.json`, `package-lock.json`: dependências e scripts
  - `.env.example`: template de ambiente alinhado ao contrato atual
  - `README.md`: guia de setup/build/branding/portabilidade
- `env.example`: variáveis centralizadas para backend/frontend

## Ordem recomendada de bootstrap no novo repositório

1. Copiar `migration-kit/` para a raiz do repositório destino.
2. Aplicar SQL de `sql/migrations/` em ordem numérica.
3. Importar dados iniciais de `sql/runtime-stores/` conforme a estratégia do destino (seed ou carga de runtime).
4. Integrar `backend-integration/` no backend destino mantendo a separação rota -> serviço -> repositório.
5. Integrar `contracts/` para manter compatibilidade de payload e DTO.
6. Copiar `frontend-app/` para o diretório de frontend do destino e instalar dependências (`npm install`).
7. Configurar variáveis do `env.example` (backend + frontend) no ambiente alvo.
8. Executar build client-safe do frontend e smoke tests integrados.

## Contrato de erros (obrigatório)

Manter envelope padrão:

```json
{
  "error": {
    "status": 409,
    "code": "MENTORIA_CONFLICT",
    "message": "...",
    "details": null
  }
}
```

## Remoção de acoplamentos locais no destino

Antes de publicar:

- remover branding e cópias específicas do projeto de origem
- remover defaults locais de demo/preview
- substituir URLs hardcoded por variáveis de ambiente
- validar conflitos de nomes de tabela/chave

## Smoke tests mínimos de portabilidade

1. **Auth**
   - login válido retorna token
   - token inválido retorna `401` no envelope padrão
2. **Endpoints críticos**
   - listar alunos no centro de comando
   - consultar radar por aluno
   - consultar matriz de renovação
3. **Carga inicial**
   - endpoint de carga inicial persiste dados esperados
   - validação de erro para payload inválido retorna `422` padrão
4. **Frontend**
   - `npm run build` com `VITE_DEPLOY_TARGET=client`
   - frontend sobe sem fallback de localhost em ambiente cliente
   - assets de `public/branding` resolvem corretamente no base path configurado

## Publicação no outro repositório

1. Subir o kit como módulo/pasta inicial (`migration-kit/`) na branch de implantação.
2. Documentar no README do destino a sequência oficial de setup (SQL -> seeds -> backend/contracts -> frontend -> env -> smoke).
3. Executar smoke tests em ambiente integrado antes de merge para trunk.

## Frontend Admin em TUI

Este kit inclui uma interface de administracao em terminal em `migration-kit/admin-tui/`:

- `src/main.rs` (Rust + Ratatui): login `root/toor` e painel principal com:
  - `1 - Manage DB`
  - `2 - View Matrix Decision`
  - `3 - View Command Center`
  - `4 - View Radar as Provider`
  - `5 - View Radar as Client`
- `README.md`: guia rapido de uso e execucao via `cargo run`.

Observacao: o Admin TUI (`migration-kit/admin-tui`) usa o schema de `sql/sql_create_database.sql` e o estado real do SQLite (`sql/elevve_clinic_deva_db.db`) via API HTTP (`TUI -> backend -> SQLite`); `runtime-stores` nao sao usados como fonte principal dessa interface.
