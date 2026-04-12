# 🚀 Guia de Execução - Landing Page Elevve Clinic

## 📋 Visão Geral

Este projeto implementa a landing page de **Restauração Intestinal** da Elevve Clinic, acessível em:
- **Produção**: `https://www.elevveclinic.com.br/intestino`
- **Local**: `http://localhost:3000`

---

## 🔄 Modos de Execução

### 🏠 Modo Local (Desenvolvimento/Testes)

Ideal para fazer ajustes, testar novas funcionalidades e validar mudanças antes do deploy.

**Características:**
- ✅ Tailwind CSS em watch mode com hot reload
- ✅ Servidor local com recarregamento automático
- ✅ CTAs apontam para WhatsApp de teste
- ✅ Google Analytics desativado
- ✅ Debug mode ativado
- ✅ URLs relativas funcionais

**Como rodar:**

```bash
# 1. Navegue até o diretório do frontend
cd /workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend

# 2. Instale as dependências (apenas na primeira vez)
npm install

# 3. Inicie o servidor de desenvolvimento
npm run dev
```

A página estará disponível em: **http://localhost:3000**

**Comandos úteis:**
```bash
# Apenas compilar o CSS
npm run build:css

# Preview da versão buildada
npm run preview
```

---

### 🌐 Modo Remoto (Produção)

Versão implantada na Vercel, acessível publicamente.

**Características:**
- ✅ Tailwind CSS compilado e minificado (25KB)
- ✅ Imagens WebP otimizadas servidas localmente
- ✅ CTAs oficiais (WhatsApp production)
- ✅ Google Analytics 4 ativo (com ID de produção)
- ✅ Headers de segurança completos
- ✅ SSL automático via Let's Encrypt
- ✅ Cache otimizado para estáticos

**Como fazer deploy:**

```bash
# 1. Certifique-se de que tudo foi commitado
git add .
git commit -m "feat: atualizações na landing page"

# 2. Faça o deploy na Vercel
vercel --prod
```

**Configuração necessária na Vercel:**
1. Conectar repositório GitHub
2. Definir Root Directory: `projects/marketing/landing_pages/modulacao_intestinal/app`
3. Build Command: `npm run build` (no diretório frontend)
4. Output Directory: `frontend`
5. Adicionar domínio: `elevveclinic.com.br`
6. Configurar DNS no registrador do domínio

---

## 📁 Estrutura do Projeto

```
frontend/
├── index.html              # Página principal (entry point)
├── package.json            # Dependências e scripts
├── tailwind.config.js      # Configuração do Tailwind
├── css/
│   ├── input.css           # Arquivo fonte do Tailwind
│   └── output.css          # CSS compilado (produção)
├── js/
│   └── main.js             # JavaScript (menu mobile, animations)
├── images/
│   ├── hero-microbiome.webp
│   ├── intestino-center.webp
│   ├── yoga-balance.webp
│   └── dra-cristal.webp
└── vercel.json             # Configuração Vercel (na pasta app/)
```

---

## 🔧 Variáveis de Ambiente

### `.env.local` (Desenvolvimento)
```env
MODE=local
BASE_URL=http://localhost:3000
ANALYTICS_ID=G-TESTE123456
WHATSAPP_NUMBER=5511999999999
```

### `.env.production` (Produção)
```env
MODE=production
BASE_URL=https://www.elevveclinic.com.br/intestino
ANALYTICS_ID=G-XXXXXXXXXX
WHATSAPP_NUMBER=5511999999999
```

---

## ✅ Checklist de Validação

### Antes do Deploy (Local)
- [ ] Testar menu mobile (hamburger)
- [ ] Verificar todos os links de navegação
- [ ] Validar CTAs (WhatsApp abrindo corretamente)
- [ ] Checar responsive design (mobile, tablet, desktop)
- [ ] Testar FAQ accordion
- [ ] Validar scroll suave entre seções
- [ ] Verificar carregamento de imagens
- [ ] Rodar Lighthouse (performance, SEO, accessibility)

### Após Deploy (Produção)
- [ ] Acessar `https://www.elevveclinic.com.br/intestino`
- [ ] Validar SSL (cadeado verde)
- [ ] Testar CTAs em dispositivo móvel
- [ ] Verificar Google Analytics (real-time)
- [ ] Validar meta tags (Open Graph, Twitter Cards)
- [ ] Testar compartilhamento em redes sociais
- [ ] Verificar headers de segurança
- [ ] Validar structured data (Google Rich Results Test)

---

## 🎯 Links Úteis

### Ferramentas de Validação
- **Google Lighthouse**: DevTools > Lighthouse
- **Rich Results Test**: https://search.google.com/test/rich-results
- **Mobile-Friendly Test**: https://search.google.com/test/mobile-friendly
- **PageSpeed Insights**: https://pagespeed.web.dev/

### Documentação
- **Tailwind CSS**: https://tailwindcss.com/docs
- **Vercel**: https://vercel.com/docs
- **Google Analytics 4**: https://support.google.com/analytics/answer/10089681

---

## 🆘 Solução de Problemas

### CSS não atualiza
```bash
npm run build:css
```

### Erro no deploy Vercel
Verifique se o `vercel.json` está na pasta correta (`app/`) e se o `outputDirectory` está configurado como `frontend`.

### Menu mobile não funciona
Verifique se o `js/main.js` está sendo carregado no final do `index.html`.

### Imagens não carregam
Confirme que as imagens estão na pasta `images/` e os caminhos no HTML estão relativos (`./images/...`).

---

## 📞 Contato

Em caso de dúvidas, consulte a documentação completa em `PLANO_IMPLEMENTACAO.md`.
