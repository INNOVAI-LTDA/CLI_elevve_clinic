# Manual de Troubleshooting — Erro 404 em `/intestino` (Vercel)

Este manual foi feito para **este repositório** e cobre os cenários mais comuns de 404 na landing page:

- `https://www.elevveclinic.com.br/intestino`
- URLs de preview da Vercel (`*.vercel.app/intestino`)

---

## 1) Pré-check: este manual é aplicável ao seu repo?

Sim, porque este repositório possui:

1. **Dois contextos de deploy** (um na raiz e outro no frontend Vite).
2. Deploy da LP em **subpath** (`/intestino/`).
3. Regras de `rewrites/redirects` que precisam bater com o `base` do Vite.

### Evidências no repo

- `projects/.../app/frontend/vite.config.ts` usa `base: "/intestino/"`.
- `projects/.../app/frontend/vercel.json` contém redirects/rewrites para `/intestino`.
- `vercel.json` na raiz usa outra estratégia de rewrite (`/public/intestino/index.html`).

---

## 2) Sintomas típicos e causa provável

### Sintoma A
- `/intestino` retorna 404, mas `/intestino/` abre.

**Causa provável:** normalização de URL + regra de redirect/rewrite não aplicada pelo `vercel.json` esperado.

### Sintoma B
- Tudo em `/intestino` retorna 404 no preview.

**Causa provável:** Vercel está buildando a pasta errada (ex.: raiz), então usa config incorreta.

### Sintoma C
- HTML abre, mas assets retornam 404.

**Causa provável:** `VITE_BASE_PATH` divergente do subpath real (`/intestino/`).

---

## 3) Checklist rápido no painel da Vercel

## Projeto
- [ ] Root Directory = `projects/marketing/landing_pages/modulacao_intestinal/app/frontend`
- [ ] Framework Preset = `Vite`
- [ ] Build Command = `npm run build`
- [ ] Output Directory = `dist`

## Variáveis de ambiente (Preview + Production)
- [ ] `VITE_BASE_PATH=/intestino/`
- [ ] `VITE_BASE_URL=https://www.elevveclinic.com.br/intestino`
- [ ] `VITE_WHATSAPP_NUMBER=5511944885013` (ou número oficial)

## Pós-ajuste
- [ ] Fazer redeploy (não apenas salvar)

---

## 4) Script de validação (curl)

Rode no seu terminal local (substitua `HOST`):

```bash
HOST="https://SEU_HOST"

for path in "/" "/intestino" "/intestino/"; do
  echo "\n### $path"
  curl -sSI "$HOST$path" | sed -n '1,12p'
done
```

### Resultado esperado

- `/` → redireciona para `/intestino/`
- `/intestino` → redireciona para `/intestino/` ou responde corretamente com fallback
- `/intestino/` → `200 OK`

---

## 5) Diagnóstico por camada

### Camada 1 — Build
No diretório `app/frontend`:

```bash
npm install
npm run build
```

Se falhar aqui, o problema é de build/dependência.

### Camada 2 — Preview local

```bash
npm run preview
```

Testar:
- `http://localhost:4173/intestino/`
- `http://localhost:4173/intestino/qualquer-rota`

Se essas rotas funcionam localmente, foco vai para configuração da Vercel.

### Camada 3 — Deploy Vercel
Verifique qual `vercel.json` está efetivamente em uso:

- **Correto (frontend):** `projects/.../app/frontend/vercel.json`
- **Risco de conflito (raiz):** `vercel.json`

---

## 6) Matriz de decisão rápida

1. `/intestino/` funciona e `/intestino` não:
   - revisar redirects da rota sem slash.

2. ambos falham (`/intestino` e `/intestino/`):
   - revisar Root Directory e `vercel.json` efetivo.

3. HTML abre mas JS/CSS 404:
   - revisar `VITE_BASE_PATH` + paths de assets.

---

## 7) Perguntas frequentes

### "Trocar apenas VITE_WHATSAPP_NUMBER resolve 404?"
Não. Isso corrige CTA de WhatsApp, mas **não** corrige roteamento/deploy.

### "Posso manter arquivos antigos fora de `projects/`?"
Pode, mas aumenta chance de deploy no contexto errado. O ideal é garantir Root Directory correto e reduzir ambiguidade de configuração.

---

## 8) Comandos úteis de investigação

```bash
# Verifica arquivos de configuração críticos
rg --files | rg "vercel.json|vite.config.ts|package.json"

# Procura referências de /intestino e rewrites
rg -n "intestino|rewrite|redirect|VITE_BASE_PATH|BASE_URL"
```

---

## 9) Ação recomendada para estabilizar

1. Padronizar deploy da LP apenas pelo diretório `app/frontend`.
2. Tratar `vercel.json` da raiz como legado (ou documentar que não deve ser usado para esta LP).
3. Manter este checklist no processo de release para evitar regressões de rota.
