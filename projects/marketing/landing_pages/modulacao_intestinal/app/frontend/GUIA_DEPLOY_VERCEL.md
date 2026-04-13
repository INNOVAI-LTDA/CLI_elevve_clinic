# 🚀 Guia de Deploy Vercel (Frontend Vite) — Elevve Clinic / Intestino

Este guia cobre o deploy do **novo frontend (Vite + React)** para servir em:

- `https://elevveclinic.com.br/intestino`
- `https://www.elevveclinic.com.br/intestino`

---

## 1) Estrutura oficial do projeto

```txt
projects/marketing/landing_pages/modulacao_intestinal/app/
├── frontend/      # NOVA versão (Vite + React)  ✅
└── frontend_v0/   # Versão antiga (HTML estático)
```

O deploy novo deve usar a pasta `frontend`.

---

## 2) Configuração recomendada na Vercel

### Opção A (recomendada): Root Directory = `.../app/frontend`

No projeto da Vercel, configure:

- **Root Directory**: `projects/marketing/landing_pages/modulacao_intestinal/app/frontend`
- **Framework Preset**: `Vite`
- **Install Command**: `npm install`
- **Build Command**: `npm run build`
- **Output Directory**: `dist`

Essa opção é a mais simples e reduz risco de erro de path.

### Opção B: Root Directory = `.../app`

Se o time precisar manter Root Directory na pasta `app`, será necessário ajustar comandos para entrar em `frontend` antes do build (menos recomendado para manutenção).

---

## 3) Rotas para subpath `/intestino`

Como a LP roda em subcaminho (e não na raiz do domínio), há dois pontos críticos:

1. **Vite `base`** deve ser `/intestino/` para gerar assets corretos.
2. **Vercel rewrites** devem mapear `/intestino/*` para os arquivos da build.

Este repositório já está com:

- `vite.config.ts` usando `base: process.env.VITE_BASE_PATH || "/intestino/"`
- `BrowserRouter` com `basename={import.meta.env.BASE_URL}`
- `vercel.json` com rewrites para assets e fallback SPA em `/intestino`

---

## 4) Variáveis de ambiente (Vite)

Na Vercel (Project Settings → Environment Variables), configure no mínimo:

- `VITE_BASE_PATH=/intestino/`
- `VITE_BASE_URL=https://www.elevveclinic.com.br/intestino`
- `VITE_MODE=production`
- `VITE_WHATSAPP_NUMBER=...`
- `VITE_WHATSAPP_MESSAGE=...`

O arquivo `.env.example` já contém o template padrão.

> Lembrete: em Vite, somente variáveis com prefixo `VITE_` ficam disponíveis no client.

---

## 5) Domínio e URL final

No projeto Vercel:

1. Vincule o domínio principal `elevveclinic.com.br`.
2. Garanta que `www` redirecione corretamente (ou vice-versa, conforme política do time).
3. Após deploy, valide:
   - `/intestino`
   - `/intestino/`
   - refresh de rota interna SPA
   - carregamento de assets (`/intestino/assets/...`)

---

## 6) Checklist de validação pós-deploy

- [ ] `npm run build` gera `dist/` sem erros.
- [ ] Página abre em `/intestino`.
- [ ] Refresh (F5) em rota interna não retorna 404.
- [ ] Imagens/CSS/JS carregam sem 404 no DevTools Network.
- [ ] Meta tags (canonical/og:url) apontam para `/intestino`.
- [ ] Botões de CTA (WhatsApp/Calendly) usam variáveis corretas.

---

## 7) Troubleshooting rápido

### Erro: assets quebrados (404)

- Verifique `VITE_BASE_PATH=/intestino/`.
- Verifique se o build foi feito com essa variável.
- Confirme rewrites no `vercel.json`.

### Erro: rota interna SPA volta 404 ao atualizar

- Fallback SPA para `/index.html` deve estar ativo em `/intestino/(.*)`.

### Erro: Vercel builda projeto errado

- Revise **Root Directory** para `.../app/frontend`.

---

## 8) Estratégia de organização para A/B tests (próximas LPs)

Sugestão de estrutura (escalável):

```txt
app/
├── frontend/                 # versão atualmente em produção
├── frontend_v0/              # legado
└── experiments/
    └── intestino/
        ├── control/          # variação A (controle)
        ├── variant-b/        # variação B
        └── variant-c/        # variação C
```

Regras práticas:

1. Cada variação com `package.json` + `vercel.json` próprios.
2. Nomenclatura padronizada (`control`, `variant-b`, `variant-c`).
3. Branch de release por experimento (`release/intestino-variant-b`).
4. Métricas por variação via parâmetro de campanha + evento analytics.
5. Promover para `frontend/` apenas a variação vencedora.

---

## 9) Comandos locais úteis

```bash
# no diretório app/frontend
npm install
npm run dev
npm run build
npm run preview
npm run test
```

---

**Última atualização:** 13 de abril de 2026
