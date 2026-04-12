# 🚀 Guia Passo a Passo: Deploy na Vercel

Este guia detalha como adicionar este repositório na Vercel e colocar a Landing Page no ar em `https://www.elevveclinic.com.br/intestino`.

---

## 📋 Pré-requisitos

1. **Conta na Vercel**: Acesse [vercel.com](https://vercel.com) e faça login (recomendado usar GitHub).
2. **Acesso ao Repositório**: Certifique-se de que o código está commitado e pushado no GitHub/GitLab/Bitbucket.
3. **Domínio Configurado**: Acesso ao DNS do domínio `elevveclinic.com.br`.

---

## 🏗️ Passo 1: Configuração do Projeto na Vercel

### 1.1. Importar o Repositório
1. No Dashboard da Vercel, clique em **"Add New..."** > **"Project"**.
2. Selecione a origem do código (ex: **GitHub**).
3. Busque pelo repositório contendo este código e clique em **"Import"**.

### 1.2. Configurar o Build (CRÍTICO)
Como o código fonte está aninhado em uma subpasta, a configuração padrão não funcionará. Preencha os campos conforme abaixo:

| Campo | Valor a Ser Inserido |
| :--- | :--- |
| **Framework Preset** | `Other` (ou Vite se houver framework futuro) |
| **Root Directory** | `projects/marketing/landing_pages/modulacao_intestinal/app` |
| **Build Command** | `npm run build` |
| **Output Directory** | `frontend/dist` (ou `frontend` se não houver build step complexo) |
| **Install Command** | `npm install` |

> **⚠️ Atenção ao Root Directory:**
> O campo **Root Directory** deve apontar exatamente para a pasta `app`. Isso diz à Vercel para ignorar todo o resto do monorepo e focar apenas onde está o `vercel.json` e o `package.json` do frontend.

### 1.3. Variáveis de Ambiente
Na etapa "Environment Variables", adicione as variáveis necessárias para o modo de produção:

| Nome | Valor (Exemplo) |
| :--- | :--- |
| `NEXT_PUBLIC_MODE` | `production` |
| `NEXT_PUBLIC_SITE_URL` | `https://www.elevveclinic.com.br/intestino` |
| `NEXT_PUBLIC_WHATSAPP_NUMBER` | `5511999999999` (Substituir pelo real) |
| `NEXT_PUBLIC_GA_ID` | `G-XXXXXXXXXX` (Se usar GA4) |

*Clique em "Save" e depois em "Deploy".*

---

## 🌐 Passo 2: Configuração de Domínio Customizado

Para que a página acesse `www.elevveclinic.com.br/intestino`:

### 2.1. Adicionar Domínio no Projeto
1. No projeto criado na Vercel, vá na aba **Domains**.
2. Clique em **Add** e digite: `elevveclinic.com.br`.
3. Adicione também: `www.elevveclinic.com.br`.

### 2.2. Configurar DNS no Registrador
A Vercel fornecerá os registros necessários. Geralmente são:

| Tipo | Nome/Host | Valor/Destino |
| :--- | :--- | :--- |
| `A` | `@` | `76.76.21.21` (IP da Vercel) |
| `CNAME` | `www` | `cname.vercel-dns.com` |

> **Nota sobre Subdomínios (/intestino):**
> A Vercel gerencia rotas via `vercel.json`. Como configuramos o rewrite no arquivo do projeto (`/intestino` → `index.html`), basta apontar o domínio raiz (`elevveclinic.com.br`) para a Vercel. O caminho `/intestino` será tratado automaticamente pela aplicação.

### 2.3. Aguardar Propagação
O status mudará de "Configuring" para "Active" assim que o DNS propagar (pode levar de alguns minutos a 48h, mas geralmente é rápido). O SSL (HTTPS) é provisionado automaticamente.

---

## 🧪 Passo 3: Validação e Testes

Após o deploy concluir (status "Ready"):

1. **Acesse a URL de Produção**:
   - `https://www.elevveclinic.com.br/intestino`
   
2. **Checklist de Validação**:
   - [ ] **Carregamento**: A página carrega sem erros de console (F12).
   - [ ] **Imagens**: Todas as imagens (Hero, Intestino, Yoga, Dra) aparecem (não estão quebradas).
   - [ ] **CTAs**: Ao clicar em "Agendar avaliação", o WhatsApp abre com a mensagem correta.
   - [ ] **Mobile**: O menu hamburger funciona em telas pequenas.
   - [ ] **SSL**: O cadeado de segurança aparece no navegador.
   - [ ] **Redirecionamento**: Acessar `.../intestino` mantém a URL limpa ou faz o rewrite correto.

---

## 🔄 Passo 4: Fluxo de Atualizações (CI/CD)

A Vercel possui integração contínua nativa.

1. **Desenvolvimento**:
   - Crie uma branch: `git checkout -b feature/nova-ajuste`
   - Faça alterações locais e teste com `npm run dev`.
   - Commit e Push: `git push origin feature/nova-ajuste`.

2. **Preview Automático**:
   - Ao abrir um **Pull Request**, a Vercel cria automaticamente uma URL de preview (ex: `project-name-git-feature.vercel.app`).
   - Use essa URL para validar mudanças antes de ir para produção.

3. **Produção**:
   - Ao fazer merge da branch na `main` (ou `master`), a Vercel detecta a mudança e executa o deploy de produção automaticamente.
   - Não é necessário rodar `vercel --prod` manualmente se a integração Git estiver ativa.

---

## 🛠️ Solução de Problemas Comuns

### Erro: "Build Failed" ou "Page Not Found"
- **Causa**: O `Root Directory` foi configurado incorretamente.
- **Solução**: Verifique nas configurações do projeto na Vercel se o **Root Directory** está exatamente como: `projects/marketing/landing_pages/modulacao_intestinal/app`.

### Erro: Imagens não carregam (404)
- **Causa**: Caminhos das imagens no HTML estão incorretos ou arquivos não foram commitados.
- **Solução**: Verifique se a pasta `frontend/images` foi para o repositório e se no `index.html` o src é relativo (ex: `src="./images/hero.webp"`).

### Erro: Tailwind não aplica estilos
- **Causa**: O comando de build não gerou o CSS final.
- **Solução**: Verifique se o script `npm run build` no `package.json` executa o comando de compilação do Tailwind (`npx tailwindcss -i ... -o ...`).

---

## 📞 Suporte

Em caso de dúvidas técnicas específicas sobre o código, consulte o arquivo `PLANO_IMPLEMENTACAO.md` ou entre em contato com a equipe de engenharia.
