# 📁 Guia de Arquivos .env - Modos Local e Remoto

## 📍 Localização dos Arquivos

Todos os arquivos de configuração de ambiente estão localizados em:

```
/workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend/
├── .env.local        # Configuração para desenvolvimento local
├── .env.production   # Configuração para produção (Vercel)
├── .env.example      # Template de exemplo (pode ser commitado)
├── .env              # Arquivo ativo (NÃO commitar - gerado automaticamente)
└── .gitignore        # Regras para ignorar arquivos sensíveis
```

---

## 🔧 Como Usar

### Modo Local (Desenvolvimento)

Para rodar em localhost:

```bash
cd /workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend

# Copiar configuração local
cp .env.local .env

# Instalar dependências
npm install

# Rodar em modo desenvolvimento
npm run dev
```

**Acesso:** http://localhost:3000

**Características do modo local:**
- ✅ Debug mode ativado
- ✅ Analytics desativado
- ✅ Tailwind em watch mode (hot reload)
- ✅ CTAs apontam para números de teste
- ✅ Logs detalhados no console

---

### Modo Remoto (Produção)

Para deploy em produção na Vercel:

```bash
cd /workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend

# Copiar configuração de produção
cp .env.production .env

# Build para produção
npm run build

# Deploy na Vercel
vercel --prod
```

**Acesso:** https://www.elevveclinic.com.br/intestino

**Características do modo remoto:**
- ✅ Debug mode desativado
- ✅ Analytics ativado (GA4/GTM)
- ✅ CSS minificado e otimizado
- ✅ Headers de segurança completos
- ✅ SSL automático (HTTPS forçado)

---

## 📊 Comparação de Variáveis

| Variável | Local (.env.local) | Produção (.env.production) |
|----------|-------------------|---------------------------|
| `NODE_ENV` | `development` | `production` |
| `MODE` | `local` | `production` |
| `BASE_URL` | `http://localhost:3000` | `https://www.elevveclinic.com.br/intestino` |
| `WHATSAPP_NUMBER` | `5511999999999` (teste) | **SUBSTITUIR pelo número real** |
| `GA4_ID` | (vazio) | **SUBSTITUIR por G-XXXXXXXXXX** |
| `GTM_ID` | (vazio) | **SUBSTITUIR por GTM-XXXXXXX** |
| `DEBUG_MODE` | `true` | `false` |
| `ANALYTICS_ENABLED` | `false` | `true` |
| `ENABLE_NEWSLETTER` | `false` | `true` |

---

## ⚠️ Importante: Dados Sensíveis

### NUNCA commitar no Git:
- ❌ `.env` (arquivo ativo)
- ❌ `.env.production` (contém dados reais)
- ❌ Qualquer arquivo com IDs reais do Google Analytics
- ❌ Números de WhatsApp reais

### PODE ser commitado:
- ✅ `.env.example` (template sem dados sensíveis)
- ✅ `.env.local` (apenas se não tiver dados reais)
- ✅ `.gitignore`

---

## 🔄 Fluxo de Trabalho Recomendado

### 1. Desenvolvimento Local
```bash
# Sempre começar copiando .env.local
cp .env.local .env
npm run dev
```

### 2. Antes do Deploy
```bash
# Editar .env.production com dados REAIS
# - WhatsApp number correto
# - GA4 ID da propriedade de produção
# - GTM ID correto
# - Calendly URL real

# Testar build localmente
cp .env.production .env
npm run build
npm run preview
```

### 3. Deploy em Produção
```bash
# Na Vercel, configurar variáveis de ambiente no dashboard:
# - NODE_ENV=production
# - WHATSAPP_NUMBER=5511XXXXXXXXX
# - GA4_ID=G-XXXXXXXXXX
# - GTM_ID=GTM-XXXXXXX

# Ou usar o arquivo .env.production antes do deploy
cp .env.production .env
vercel --prod
```

---

## 🛡️ Segurança

O arquivo `.gitignore` já está configurado para:
- Ignorar `.env`, `.env.local`, `.env.production`
- Permitir apenas `.env.example` no Git
- Ignorar `node_modules/` e arquivos de build

**Verificação:**
```bash
# Verificar se .env está sendo ignorado
git check-ignore .env
# Deve retornar: .env

# Verificar quais arquivos .env serão ignorados
git check-ignore -v .env .env.local .env.production
```

---

## 📝 Checklist Pré-Deploy

Antes de fazer deploy em produção, verificar:

- [ ] `.env.production` tem o número de WhatsApp correto
- [ ] `.env.production` tem o GA4 ID real
- [ ] `.env.production` tem o GTM ID real (se usar)
- [ ] `.env.production` tem a URL correta do Calendly
- [ ] `.env` NÃO foi commitado no Git
- [ ] `.env.production` NÃO foi commitado no Git
- [ ] Variáveis sensíveis estão configuradas no dashboard da Vercel (recomendado)

---

## 🎯 Variáveis de Ambiente na Vercel

Recomenda-se configurar as variáveis diretamente no dashboard da Vercel:

1. Acessar: https://vercel.com/dashboard
2. Selecionar o projeto
3. Ir em **Settings** → **Environment Variables**
4. Adicionar:
   - `WHATSAPP_NUMBER` (Production)
   - `GA4_ID` (Production)
   - `GTM_ID` (Production)
   - `CALENDLY_URL` (Production)
   - `NODE_ENV=production` (Production)

**Vantagem:** As variáveis ficam seguras no dashboard da Vercel e não precisam estar em arquivos locais.

---

## 🐛 Troubleshooting

### Problema: Variáveis não estão sendo lidas
```bash
# Verificar se o arquivo .env existe
ls -la .env

# Verificar conteúdo
cat .env | grep MODE

# Reiniciar o servidor de desenvolvimento
npm run dev
```

### Problema: Build falha em produção
```bash
# Testar build localmente com config de produção
cp .env.production .env
npm run build

# Verificar erros
npm run build -- --debug
```

### Problema: Analytics não funciona
```bash
# Verificar se GA4_ID está configurado
cat .env | grep GA4_ID

# Verificar se ANALYTICS_ENABLED=true
cat .env | grep ANALYTICS_ENABLED

# Inspecionar o código fonte da página (Ctrl+U)
# Buscar por "G-" ou "GTM-"
```

---

## 📚 Referências

- [Vercel Environment Variables](https://vercel.com/docs/environment-variables)
- [Next.js Environment Variables](https://nextjs.org/docs/basic-features/environment-variables)
- [dotenv Documentation](https://github.com/motdotla/dotenv)
