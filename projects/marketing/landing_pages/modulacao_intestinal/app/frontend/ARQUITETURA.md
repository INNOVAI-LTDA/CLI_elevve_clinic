# 🏗️ Arquitetura da Landing Page - Protocolo de Restauração Intestinal

## Visão Geral

Esta documentação descreve a nova arquitetura da Landing Page de Modulação Intestinal da Elevve Clinic, desenvolvida com **React + Vite + TypeScript** e componentes **shadcn/ui**.

---

## 📁 Estrutura do Projeto

```
frontend/
├── public/                     # Assets estáticos servidos diretamente
│   ├── favicon.ico
│   ├── placeholder.svg
│   ├── robots.txt
│   └── *.webp                  # Imagens otimizadas (hero, dra-cristal, etc.)
│
├── src/
│   ├── assets/                 # Imports de imagens no build do Vite
│   │   ├── dra-cristal-1.jpeg
│   │   ├── dra-cristal-2.jpeg
│   │   ├── prova-social.jpeg
│   │   └── *.webp              # Cópias das imagens da versão antiga
│   │
│   ├── components/             # Componentes React da Landing Page
│   │   ├── ui/                 # Componentes base do shadcn/ui (50+)
│   │   │   ├── accordion.tsx
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   └── ... (47 arquivos)
│   │   │
│   │   ├── HeroSection.tsx     # Seção principal com CTA
│   │   ├── SymptomsSection.tsx # Lista de sintomas
│   │   ├── ExplanationSection.tsx # Explicação do protocolo
│   │   ├── SocialProofSection.tsx # Prova social
│   │   ├── SystemSection.tsx   # Como funciona o sistema
│   │   ├── MethodSection.tsx   # Metodologia
│   │   ├── ConsultationSection.tsx # Chamada para consulta
│   │   ├── TreatmentSection.tsx # Detalhes do tratamento
│   │   ├── WarningSection.tsx  # Aviso importante (novo)
│   │   ├── ForYouSection.tsx   # Para quem é indicado
│   │   ├── ResultsSection.tsx  # Resultados esperados
│   │   ├── DoctorSection.tsx   # Sobre a Dra. Cristal
│   │   └── FAQSection.tsx      # Perguntas frequentes (novo)
│   │
│   ├── pages/
│   │   ├── Index.tsx           # Página principal (orquestra todas as seções)
│   │   └── NotFound.tsx        # Página 404
│   │
│   ├── hooks/                  # Custom React hooks
│   ├── lib/                    # Utilitários e configurações
│   │   └── utils.ts            # Função cn() para classNames
│   │
│   ├── App.tsx                 # Componente raiz com roteamento
│   ├── App.css                 # Estilos globais do App
│   ├── main.tsx                # Ponto de entrada da aplicação
│   ├── index.css               # Estilos globais e Tailwind
│   └── vite-env.d.ts           # Typescript declarations para Vite
│
├── index.html                  # HTML base com SEO completo
├── package.json                # Dependências e scripts
├── vite.config.ts              # Configuração do Vite
├── tailwind.config.ts          # Configuração do Tailwind CSS
├── tsconfig.json               # Configuração TypeScript
├── vercel.json                 # Configuração de deploy na Vercel
├── .env.example                # Template de variáveis de ambiente
└── .gitignore                  # Arquivos ignorados pelo Git
```

---

## 🎯 Fluxo de Renderização

```
main.tsx
  ↓
App.tsx (BrowserRouter + Routes)
  ↓
Index.tsx (Página Principal)
  ↓
13 Seções em Sequência:
  1. HeroSection
  2. SymptomsSection
  3. ExplanationSection
  4. SocialProofSection
  5. SystemSection
  6. MethodSection
  7. ConsultationSection
  8. TreatmentSection
  9. WarningSection ⭐ NOVO
  10. ForYouSection
  11. ResultsSection
  12. DoctorSection
  13. FAQSection ⭐ NOVO
```

---

## 🔧 Tecnologias Utilizadas

### Core
- **React 18.3.1** - Biblioteca UI
- **Vite 5.4.19** - Build tool e dev server
- **TypeScript 5.8.3** - Tipagem estática
- **React Router DOM 6.30.1** - Roteamento

### Estilização
- **Tailwind CSS 3.4.17** - Framework CSS utilitário
- **shadcn/ui** - Coleção de componentes reutilizáveis
- **Radix UI** - Primitivos de UI acessíveis (50+ pacotes)

### Utilidades
- **class-variance-authority** - Classes condicionais
- **clsx** + **tailwind-merge** - Merge de classes
- **lucide-react** - Ícones
- **date-fns** - Manipulação de datas

### Formulários & Validação
- **react-hook-form** - Gerenciamento de formulários
- **zod** - Validação de schemas
- **@hookform/resolvers** - Integração Zod + React Hook Form

### Estado & Data Fetching
- **@tanstack/react-query** - Gerenciamento de estado assíncrono

### Testes
- **Vitest** - Framework de testes
- **@testing-library/react** - Testes de componentes

---

## 🚀 Scripts Disponíveis

```bash
npm run dev          # Inicia servidor de desenvolvimento (porta 5173)
npm run build        # Build para produção
npm run build:dev    # Build modo desenvolvimento
npm run lint         # Linting com ESLint
npm run preview      # Preview do build em produção
npm run test         # Executa testes com Vitest
npm run test:watch   # Testes em modo watch
```

---

## 🌐 Variáveis de Ambiente

Todas as variáveis usam o prefixo `VITE_` para exposição no client-side:

| Variável | Descrição | Exemplo |
|----------|-----------|---------|
| `VITE_MODE` | Ambiente atual | `local`, `production` |
| `VITE_BASE_URL` | URL base da aplicação | `http://localhost:5173` |
| `VITE_WHATSAPP_NUMBER` | Número do WhatsApp | `5511999999999` |
| `VITE_WHATSAPP_MESSAGE` | Mensagem padrão | `Olá! Vim pela LP...` |
| `VITE_CALENDLY_URL` | Link do Calendly | `https://calendly.com/...` |
| `VITE_GA4_ID` | Google Analytics 4 | `G-XXXXXXXXXX` |
| `VITE_GTM_ID` | Google Tag Manager | `GTM-XXXXXXX` |
| `VITE_DEBUG_MODE` | Modo debug | `true`, `false` |
| `VITE_ANALYTICS_ENABLED` | Habilita analytics | `true`, `false` |

**Uso no código:**
```typescript
const whatsappNumber = import.meta.env.VITE_WHATSAPP_NUMBER;
const isProduction = import.meta.env.VITE_MODE === 'production';
```

---

## 📊 Diferenças da Versão Antiga

| Aspecto | Versão Antiga (HTML Estático) | Nova Versão (React + Vite) |
|---------|-------------------------------|----------------------------|
| **Tecnologia** | HTML + Tailwind CLI | React + Vite + TypeScript |
| **Arquitetura** | Monolítico (index.html único) | Modular (13 componentes) |
| **Build Time** | ~2s | ~5-8s (com otimizações) |
| **Hot Reload** | live-server (básico) | Vite HMR (instantâneo) |
| **Tipagem** | Nenhuma | TypeScript completo |
| **Componentes** | N/A | 50+ componentes shadcn/ui |
| **SEO** | Manual no HTML | JSON-LD + Meta tags dinâmicas |
| **Manutenibilidade** | Baixa (HTML gigante) | Alta (componentes isolados) |
| **Testes** | N/A | Vitest + Testing Library |
| **Escalabilidade** | Limitada | Alta (fácil adicionar features) |

---

## 🔒 Segurança & Performance

### Headers de Segurança (vercel.json)
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Strict-Transport-Security: max-age=31536000`

### Otimizações
- **Cache imutável** para assets estáticos (1 ano)
- **Clean URLs** habilitadas
- **Imagens WebP** otimizadas
- **Code splitting** automático do Vite
- **Tree shaking** de dependências

---

## 🔄 Processo de Build

```
1. npm run build
   ↓
2. Vite compila TypeScript → JavaScript
   ↓
3. Tailwind purga CSS não utilizado
   ↓
4. Minificação e tree-shaking
   ↓
5. Geração de hashes nos filenames
   ↓
6. Output em /dist pronto para deploy
```

**Estrutura do build:**
```
dist/
├── index.html
├── assets/
│   ├── index-[hash].js
│   ├── index-[hash].css
│   └── [imagens]-[hash].webp
└── public/ (copiado integralmente)
```

---

## 📝 Adições da Nova Versão

### ✅ Componentes Novos
1. **WarningSection** - Alerta sobre expectativas reais do tratamento
2. **FAQSection** - 7 perguntas frequentes com accordion

### ✅ Melhorias de SEO
- JSON-LD estruturado para MedicalClinic
- Open Graph completo
- Twitter Cards
- Canonical URL
- Meta tags dinâmicas

### ✅ Configurações
- `.env.example` com prefixo VITE
- `vercel.json` otimizado para Vite
- `.gitignore` expandido
- TypeScript configurado com paths (@/)

---

## 🎨 Padrões de Código

### Imports de Componentes
```typescript
import { Button } from "@/components/ui/button";
import HeroSection from "@/components/HeroSection";
```

### Estrutura de Componente
```typescript
import { useState } from "react";
import { SomeIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

interface Props {
  title: string;
  onAction?: () => void;
}

const ComponentName = ({ title, onAction }: Props) => {
  return (
    <section className="py-20 bg-gray-50">
      {/* Conteúdo */}
    </section>
  );
};

export default ComponentName;
```

### Uso de Variáveis de Ambiente
```typescript
const GA_ID = import.meta.env.VITE_GA4_ID;
const isProd = import.meta.env.MODE === 'production';
```

---

## 🚨 Pontos de Atenção

1. **Imagens**: Usar `/assets/` para imports no código, `/public/` para referências diretas no HTML
2. **Environment**: Sempre usar `import.meta.env.VITE_*` (não `process.env`)
3. **Aliases**: O alias `@/` resolve para `./src/`
4. **Build**: O output vai para `/dist`, não `/build`
5. **Deploy**: A Vercel detecta automaticamente o framework Vite

---

## 📞 Suporte

Para dúvidas sobre esta arquitetura, consulte:
- [Documentação Vite](https://vitejs.dev/)
- [Documentação React](https://react.dev/)
- [shadcn/ui Docs](https://ui.shadcn.com/)
- [Tailwind CSS](https://tailwindcss.com/docs)

---

**Última atualização:** Abril 2025  
**Versão:** 2.0.0  
**Status:** ✅ Produção Ready
