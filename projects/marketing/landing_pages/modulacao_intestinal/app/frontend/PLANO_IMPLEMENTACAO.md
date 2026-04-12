# Plano de Implementação - Landing Page Elevve Clinic
## Subdomínio: www.elevveclinic.com.br/intestino

---

## 📋 Visão Geral do Projeto

**Objetivo:** Preparar a landing page de Modulação Intestinal para deploy na Vercel no subdomínio `www.elevveclinic.com.br/intestino`.

**Arquivo Original:** `projects/marketing/landing_pages/modulacao_intestinal/app/frontend/web-dev_lp_ElevveClinic_v0.html`

**Domínio de Produção:** `elevveclinic.com.br`

**URL Final:** `https://www.elevveclinic.com.br/intestino`

---

## 🔴 FASE 1 - CRÍTICO (Bloqueadores de Deploy)

### Tarefa 1.1: Reestruturação do Projeto

#### Escopo
Reorganizar a estrutura de pastas e arquivos para atender aos requisitos da Vercel.

#### Elementos Envolvidos
- Arquivo: `web-dev_lp_ElevveClinic_v0.html`
- Pasta: `/workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend/`
- Novo arquivo: `vercel.json`

#### Alterações Necessárias

1. **Renomear arquivo principal**
   - De: `web-dev_lp_ElevveClinic_v0.html`
   - Para: `index.html`

2. **Criar estrutura de pastas organizada**
   ```
   /frontend/
   ├── index.html
   ├── css/
   │   └── styles.css
   ├── js/
   │   └── main.js
   ├── images/
   │   ├── hero-microbiome.webp
   │   ├── intestino-biologia.webp
   │   ├── yoga-mulher.webp
   │   └── dra-cristal-portrait.webp
   └── vercel.json
   ```

3. **Criar arquivo `vercel.json` na raiz do frontend**
   ```json
   {
     "version": 2,
     "builds": [
       {
         "src": "**/*",
         "use": "@vercel/static"
       }
     ],
     "routes": [
       {
         "src": "/intestino/(.*)",
         "dest": "/$1"
       },
       {
         "src": "/(.*)",
         "dest": "/$1"
       }
     ],
     "headers": [
       {
         "source": "/(.*)",
         "headers": [
           {
             "key": "X-Content-Type-Options",
             "value": "nosniff"
           },
           {
             "key": "X-Frame-Options",
             "value": "DENY"
           },
           {
             "key": "X-XSS-Protection",
             "value": "1; mode=block"
           },
           {
             "key": "Referrer-Policy",
             "value": "strict-origin-when-cross-origin"
           }
         ]
       }
     ]
   }
   ```

#### Plano de Testes
- [ ] Verificar se o arquivo `index.html` existe na pasta `/frontend`
- [ ] Confirmar que a Vercel reconhece `index.html` como entry point
- [ ] Testar deploy local com `vercel dev` (se aplicável)
- [ ] Validar estrutura de pastas após reorganização

---

### Tarefa 1.2: Substituição do Tailwind CDN por CSS Compilado

#### Escopo
Remover o script CDN do Tailwind e compilar CSS estático para produção.

#### Elementos Envolvidos
- Script CDN: `<script src="https://cdn.tailwindcss.com?plugins=forms,container-queries"></script>` (linha 5)
- Configuração Tailwind inline (linhas 8-71)
- Novo arquivo: `tailwind.config.js`
- Novo arquivo: `input.css`
- Arquivo compilado: `css/styles.css`

#### Alterações Necessárias

1. **Instalar dependências do Tailwind (desenvolvimento)**
   ```bash
   npm install -D tailwindcss postcss autoprefixer
   npx tailwindcss init
   ```

2. **Criar `tailwind.config.js`**
   ```javascript
   /** @type {import('tailwindcss').Config} */
   module.exports = {
     content: ["./index.html"],
     darkMode: 'class',
     theme: {
       extend: {
         colors: {
           secondary: "#735c00",
           "surface-container": "#edefe7",
           error: "#ba1a1a",
           "on-secondary": "#ffffff",
           "secondary-container": "#fed65b",
           "surface-dim": "#d9dbd3",
           "on-secondary-fixed-variant": "#574500",
           "error-container": "#ffdad6",
           "on-secondary-fixed": "#241a00",
           "primary-fixed-dim": "#a5d0b9",
           "on-tertiary-container": "#65b68e",
           "on-tertiary-fixed": "#002113",
           "secondary-fixed": "#ffe088",
           "surface-variant": "#e2e3db",
           "secondary-fixed-dim": "#e9c349",
           "surface-bright": "#f9faf2",
           "tertiary-container": "#00452d",
           "surface-container-lowest": "#ffffff",
           "outline-variant": "#c1c8c2",
           "surface-tint": "#3f6653",
           "primary-container": "#1b4332",
           "inverse-primary": "#a5d0b9",
           "on-primary": "#ffffff",
           "on-primary-container": "#86af99",
           "on-background": "#1a1c18",
           "on-primary-fixed-variant": "#274e3d",
           "on-secondary-container": "#745c00",
           primary: "#012d1d",
           background: "#f9faf2",
           tertiary: "#002d1b",
           "tertiary-fixed": "#a1f4c8",
           "on-surface": "#1a1c18",
           "primary-fixed": "#c1ecd4",
           "surface-container-low": "#f3f4ec",
           "surface-container-highest": "#e2e3db",
           "inverse-on-surface": "#f0f1e9",
           "on-tertiary": "#ffffff",
           "on-tertiary-fixed-variant": "#005236",
           surface: "#f9faf2",
           "on-error": "#ffffff",
           "inverse-surface": "#2f312c",
           "tertiary-fixed-dim": "#86d7ad",
           "on-error-container": "#93000a",
           outline: "#717973",
           "surface-container-high": "#e8e9e1",
           "on-primary-fixed": "#002114",
           "on-surface-variant": "#414844"
         },
         fontFamily: {
           headline: ["Noto Serif", "serif"],
           body: ["Manrope", "sans-serif"],
           label: ["Manrope", "sans-serif"]
         },
         borderRadius: {
           DEFAULT: "0.25rem",
           lg: "0.5rem",
           xl: "0.75rem",
           full: "9999px"
         }
       }
     },
     plugins: [
       require('@tailwindcss/forms'),
       require('@tailwindcss/container-queries')
     ]
   }
   ```

3. **Criar `input.css`**
   ```css
   @tailwind base;
   @tailwind components;
   @tailwind utilities;

   body {
     font-family: 'Manrope', sans-serif;
   }

   h1, h2, h3 {
     font-family: 'Noto Serif', serif;
   }

   .material-symbols-outlined {
     font-variation-settings: 'FILL' 0, 'wght' 300, 'GRAD' 0, 'opsz' 24;
   }

   .glass-nav {
     backdrop-filter: blur(20px);
   }
   ```

4. **Compilar CSS para produção**
   ```bash
   npx tailwindcss -i ./input.css -o ./css/styles.css --minify
   ```

5. **Atualizar `index.html`**
   - Remover linha 5: `<script src="https://cdn.tailwindcss.com?plugins=forms,container-queries"></script>`
   - Remover linhas 8-71 (script tailwind-config)
   - Adicionar no `<head>`: `<link rel="stylesheet" href="./css/styles.css">`

#### Plano de Testes
- [ ] Verificar se CSS foi compilado sem erros
- [ ] Validar visual da página após substituição (deve ser idêntico)
- [ ] Testar modo dark/light
- [ ] Medir redução no tamanho do bundle (comparar ~300KB CDN vs CSS estático)
- [ ] Verificar performance no Lighthouse (melhoria esperada em Performance)

---

### Tarefa 1.3: Download e Otimização de Imagens

#### Escopo
Baixar todas as imagens hospedadas no Google, converter para WebP e servir localmente.

#### Elementos Envolvidos
- Imagem 1 (Hero): `https://lh3.googleusercontent.com/aida-public/AB6AXuAT8XzGcvGbEccp7VefEarQK7xBmpBjWZW--pPYhQQNwEPlzol-UG-Pte5aED_9LReXGXQBNU2Vux6mOsVap_uKt4R5uL4GasSPyxN0QRm7UbWjNzeQ52U1u0bgdTeOOtkRxK_iqSQG-YvsP8a6Dclv4E4Py59hNhyfq6ZR6k-IZnMy2vQ9MEfpsJh9UYypXVpjW_hWffa-OaT4Rn5HX0ERFq-lUg0SHvhvK3gQ1m-3vvNQsQZdyhxo5IywO3ZWvoHfyjaOWGcyPPA`
- Imagem 2 (Intestino): `https://lh3.googleusercontent.com/aida-public/AB6AXuCAr6_EFiKbFo4zScp-dmmLCUk_Jji1psyTE1r_76JasjCt2zbzQ9MSJ4qyjOCoESx5qEHcNKtOzhSLw2om0kXio8icO0k_9U8iHCQJvyd8aSczXZ-JSJZf2A-89cPWh2Cb0Eea9Oxie77--thwPzZ92zOTcs26eZKhSg0OigZyzZuYKemdpBRuwGRvsVYJcLlpS8phYtlsO26bXbW2zbsprEz4z-1mdkk8jsElUkjb8YszWpKD_fNQUTBxhRq7K3p7LnmBvqdToDY`
- Imagem 3 (Yoga): `https://lh3.googleusercontent.com/aida-public/AB6AXuBE1jr0ornsbhcfpCDcrLdhyy-q8hF_oYPEYJJt6_nn5o-c1s3XTxO7akArGhQ6Pey9eTuN7mIELaLGG38FBtaAIy3X9vY5VZrmOkyF7ZxPgL2SJXagdWijOma137HmI_Nx6CDWXEaskjcx6S_N_F4MTYe1sjcxW90A3hXjFfRyubd08xqYxVJtMOSTCpkyNQlkxxYDSRdHnPNBIA3-LaPXJ8CHX-TT30YLmecPVhOfyU84JdiuxAFD5QSPnpahgRXwuWSaH0A2Lys`
- Imagem 4 (Dra. Cristal): `https://lh3.googleusercontent.com/aida/ADBb0uhx6IaPhPIh4OzaqAbxMALmpSjfCPgeNC0jTVe390Yeb6JQVOw5wRjrkRZEyc6Z0LfpdnWWCkcD9q6JI_XHvH-VvTi65nEQKgwhR6LK1miP3LexMcC0Hbt_rLS-RhhJB3QceP8vXEOrl2FIGq32ROV7VLe3P5B8QF-QVcRUKrmnUgkjgCSnC9Y4JcLH9uESqQA_8MVT9eTkArHts6Ryws2cUSYe5FAyGiliV5ku0sPrwEoTN3vmvHtt5Ht2e9PHmc0eq2gR2jMD`

#### Alterações Necessárias

1. **Baixar imagens**
   ```bash
   mkdir -p /workspace/projects/marketing/landing_pages/modulacao_intestinal/app/frontend/images
   
   curl -o hero-microbiome.jpg "URL_IMAGEM_1"
   curl -o intestino-biologia.jpg "URL_IMAGEM_2"
   curl -o yoga-mulher.jpg "URL_IMAGEM_3"
   curl -o dra-cristal-portrait.jpg "URL_IMAGEM_4"
   ```

2. **Converter para WebP e otimizar**
   ```bash
   # Usando imagem (ImageMagick) ou sharp
   convert hero-microbiome.jpg -quality 80 -resize 1200x800 hero-microbiome.webp
   convert intestino-biologia.jpg -quality 80 -resize 800x800 intestino-biologia.webp
   convert yoga-mulher.jpg -quality 80 -resize 1200x800 yoga-mulher.webp
   convert dra-cristal-portrait.jpg -quality 85 -resize 600x800 dra-cristal-portrait.webp
   ```

3. **Atualizar referências no HTML**
   - Linha 113: `src="./images/hero-microbiome.webp"`
   - Linha 171: `src="./images/intestino-biologia.webp"`
   - Linha 280: `src="./images/yoga-mulher.webp"`
   - Linha 307: `src="./images/dra-cristal-portrait.webp"`

4. **Adicionar atributos de performance nas imagens**
   ```html
   <!-- Hero image (above-the-fold) -->
   <img 
     src="./images/hero-microbiome.webp" 
     alt="Visualização científica do microbioma humano saudável"
     width="1200" 
     height="800"
     class="w-full h-[600px] object-cover rounded-[2rem] shadow-2xl"
   >
   
   <!-- Imagens below-the-fold com lazy loading -->
   <img 
     src="./images/intestino-biologia.webp" 
     alt="Estruturas biológicas abstratas representando microbiota vibrante"
     width="800" 
     height="800"
     loading="lazy"
     class="w-full h-full object-cover mix-blend-multiply opacity-80"
   >
   ```

#### Plano de Testes
- [ ] Verificar se todas as imagens foram baixadas com sucesso
- [ ] Validar conversão para WebP (qualidade visual mantida)
- [ ] Comparar tamanho dos arquivos (redução esperada de 30-50%)
- [ ] Testar carregamento das imagens locais
- [ ] Validar atributos width/height (sem CLS - Cumulative Layout Shift)
- [ ] Verificar lazy loading em imagens below-the-fold
- [ ] Testar em diferentes dispositivos e resoluções

---

### Tarefa 1.4: Conexão dos CTAs (Call-to-Action)

#### Escopo
Implementar funcionalidade nos botões de agendamento e links do footer.

#### Elementos Envolvidos
- Botão Hero (linha 105-108)
- Botão Final CTA (linha 335-338)
- Links do footer (linhas 353-363)

#### Alterações Necessárias

1. **Definir destino dos CTAs**
   - Opção A: WhatsApp Business
   - Opção B: Calendly
   - Opção C: Formulário de contato
   
   *Recomendação inicial: WhatsApp + Calendly como fallback*

2. **Atualizar botão do Hero (linha 105)**
   ```html
   <!-- Opção WhatsApp -->
   <a 
     href="https://wa.me/5511999999999?text=Olá!%20Gostaria%20de%20agendar%20uma%20avaliação%20metabólica."
     target="_blank"
     rel="noopener noreferrer"
     class="bg-primary text-on-primary px-10 py-5 rounded-xl text-lg font-semibold shadow-xl hover:bg-primary-container transition-all duration-300 flex items-center gap-3 inline-block"
   >
     Agendar avaliação metabólica
     <span class="material-symbols-outlined">arrow_forward</span>
   </a>
   
   <!-- OU Opção Calendly -->
   <a 
     href="https://calendly.com/elevveclinic/avaliacao-metabolica"
     target="_blank"
     rel="noopener noreferrer"
     class="bg-primary text-on-primary px-10 py-5 rounded-xl text-lg font-semibold shadow-xl hover:bg-primary-container transition-all duration-300 flex items-center gap-3 inline-block"
   >
     Agendar avaliação metabólica
     <span class="material-symbols-outlined">arrow_forward</span>
   </a>
   ```

3. **Atualizar botão Final CTA (linha 335)**
   ```html
   <a 
     href="https://wa.me/5511999999999?text=Olá!%20Estou%20pronto(a)%20para%20restaurar%20minha%20vitalidade."
     target="_blank"
     rel="noopener noreferrer"
     class="bg-primary text-on-primary px-12 py-6 rounded-full text-xl font-bold shadow-2xl hover:scale-105 transition-transform duration-300 inline-block"
   >
     Agendar avaliação metabólica
   </a>
   ```

4. **Atualizar links do footer**
   ```html
   <!-- Termos de Uso -->
   <a 
     href="/termos-de-uso" 
     class="text-[#1a1c18]/70 dark:text-[#f9faf2]/70 hover:text-[#012d1d] dark:hover:text-white transition-colors text-sm"
   >
     Termos de Uso
   </a>
   
   <!-- Política de Privacidade -->
   <a 
     href="/politica-de-privacidade" 
     class="text-[#1a1c18]/70 dark:text-[#f9faf2]/70 hover:text-[#012d1d] dark:hover:text-white transition-colors text-sm"
   >
     Política de Privacidade
   </a>
   
   <!-- Contato -->
   <a 
     href="mailto:contato@elevveclinic.com.br" 
     class="text-[#1a1c18]/70 dark:text-[#f9faf2]/70 hover:text-[#012d1d] dark:hover:text-white transition-colors text-sm"
   >
     Contato
   </a>
   
   <!-- Agendar Consulta -->
   <a 
     href="https://wa.me/5511999999999" 
     class="text-[#735c00] dark:text-[#fed65b] font-sans text-sm font-bold uppercase hover:opacity-80 transition-opacity"
   >
     Agendar Consulta
   </a>
   ```

5. **Criar páginas estáticas para Termos e Privacidade (opcional)**
   - Criar `termos-de-uso.html`
   - Criar `politica-de-privacidade.html`

#### Plano de Testes
- [ ] Clicar em todos os botões CTA e validar redirecionamento
- [ ] Testar links do WhatsApp (abrir app/mobile)
- [ ] Testar link do Calendly (se aplicável)
- [ ] Validar abertura em nova aba (`target="_blank"`)
- [ ] Verificar segurança (`rel="noopener noreferrer"`)
- [ ] Testar links de email (mailto:)
- [ ] Validar páginas de Termos e Privacidade (se criadas)

---

## 🟡 FASE 2 - IMPORTANTE (Qualidade e SEO)

### Tarefa 2.1: Implementação de Meta Tags de SEO

#### Escopo
Adicionar todas as meta tags necessárias para SEO, Open Graph e redes sociais.

#### Elementos Envolvidos
- Seção `<head>` do `index.html` (linhas 1-78)

#### Alterações Necessárias

1. **Substituir `<head>` atual por versão completa**
   ```html
   <head>
     <meta charset="utf-8">
     <meta name="viewport" content="width=device-width, initial-scale=1.0">
     
     <!-- SEO Básico -->
     <title>Elevve Clinic - Protocolo de Restauração Intestinal | Saúde Metabólica</title>
     <meta name="description" content="Recupere sua vitalidade com o Protocolo de Restauração Intestinal da Elevve Clinic. Investigação clínica profunda para tratar cansaço, inflamação e desequilíbrios metabólicos.">
     <meta name="keywords" content="restauração intestinal, saúde metabólica, microbioma, disbiose, SIBO, medicina integrativa, Dra. Cristal Cabral">
     <meta name="author" content="Elevve Clinic">
     <link rel="canonical" href="https://www.elevveclinic.com.br/intestino">
     
     <!-- Open Graph / Facebook -->
     <meta property="og:type" content="website">
     <meta property="og:url" content="https://www.elevveclinic.com.br/intestino">
     <meta property="og:title" content="Elevve Clinic - Protocolo de Restauração Intestinal">
     <meta property="og:description" content="Se você sente cansaço constante, o problema pode estar no seu intestino. Conheça nosso protocolo de restauração intestinal premium.">
     <meta property="og:image" content="https://www.elevveclinic.com.br/intestino/images/hero-microbiome.webp">
     <meta property="og:image:width" content="1200">
     <meta property="og:image:height" content="630">
     <meta property="og:locale" content="pt_BR">
     <meta property="og:site_name" content="Elevve Clinic">
     
     <!-- Twitter Card -->
     <meta name="twitter:card" content="summary_large_image">
     <meta name="twitter:url" content="https://www.elevveclinic.com.br/intestino">
     <meta name="twitter:title" content="Elevve Clinic - Protocolo de Restauração Intestinal">
     <meta name="twitter:description" content="Recupere sua vitalidade com investigação clínica profunda do seu metabolismo e saúde intestinal.">
     <meta name="twitter:image" content="https://www.elevveclinic.com.br/intestino/images/hero-microbiome.webp">
     
     <!-- Favicon -->
     <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
     <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
     <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
     <link rel="manifest" href="/site.webmanifest">
     
     <!-- Fonts -->
     <link rel="preconnect" href="https://fonts.googleapis.com">
     <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
     <link href="https://fonts.googleapis.com/css2?family=Noto+Serif:wght@400;700&family=Manrope:wght@300;400;500;600;700;800&display=swap" rel="stylesheet">
     <link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap" rel="stylesheet">
     
     <!-- Structured Data (JSON-LD) -->
     <script type="application/ld+json">
     {
       "@context": "https://schema.org",
       "@type": "MedicalClinic",
       "name": "Elevve Clinic",
       "alternateName": "Clínica Elevve",
       "url": "https://www.elevveclinic.com.br",
       "logo": "https://www.elevveclinic.com.br/logo.png",
       "description": "Clínica especializada em medicina integrativa e restauração intestinal",
       "address": {
         "@type": "PostalAddress",
         "addressCountry": "BR"
       },
       "geo": {
         "@type": "GeoCoordinates",
         "latitude": "",
         "longitude": ""
       },
       "telephone": "+5511999999999",
       "email": "contato@elevveclinic.com.br",
       "founder": {
         "@type": "Person",
         "name": "Dra. Cristal Cabral",
         "jobTitle": "Médica Especialista em Restauração Intestinal"
       },
       "medicalSpecialty": ["Nutrition", "Gastroenterology", "IntegrativeMedicine"],
       "sameAs": [
         "https://www.instagram.com/elevveclinic",
         "https://www.facebook.com/elevveclinic"
       ]
     }
     </script>
     
     <!-- CSS Local -->
     <link rel="stylesheet" href="./css/styles.css">
   </head>
   ```

#### Plano de Testes
- [ ] Validar meta description no Google Search Console
- [ ] Testar preview de compartilhamento no Facebook (Facebook Sharing Debugger)
- [ ] Testar preview no Twitter (Twitter Card Validator)
- [ ] Verificar canonical URL
- [ ] Validar structured data no Google Rich Results Test
- [ ] Confirmar favicon aparecendo em múltiplas abas
- [ ] Testar em diferentes navegadores

---

### Tarefa 2.2: Implementação de Menu Mobile (Hamburger)

#### Escopo
Criar menu responsivo para dispositivos móveis, já que o menu atual desaparece em telas menores que 768px.

#### Elementos Envolvidos
- Navegação (linhas 81-93)
- Novo arquivo: `js/main.js`

#### Alterações Necessárias

1. **Atualizar navegação no HTML**
   ```html
   <nav class="fixed top-0 w-full z-50 bg-[#f9faf2]/80 dark:bg-[#012d1d]/80 backdrop-blur-md shadow-sm dark:shadow-none">
     <div class="flex justify-between items-center px-8 py-4 max-w-7xl mx-auto">
       <div class="text-2xl font-serif font-bold text-[#012d1d] dark:text-[#fed65b]">Elevve Clinic</div>
       
       <!-- Desktop Menu -->
       <div class="hidden md:flex items-center space-x-8 font-serif text-lg tracking-tight">
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors duration-300" href="#problema">O Problema</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors duration-300" href="#intestino">O Intestino</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors duration-300" href="#metodo">Método</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors duration-300" href="#jornada">Jornada</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors duration-300" href="#dra-cristal">Dra. Cristal</a>
       </div>
       
       <div class="flex items-center gap-4">
         <a class="hidden md:inline-block px-6 py-2 bg-primary text-on-primary rounded-xl font-medium hover:opacity-80 transition-opacity" href="#faq">FAQ</a>
         
         <!-- Hamburger Button -->
         <button 
           id="mobile-menu-button" 
           class="md:hidden p-2 rounded-lg hover:bg-surface-container focus:outline-none"
           aria-label="Abrir menu"
           aria-expanded="false"
         >
           <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
             <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
           </svg>
         </button>
       </div>
     </div>
     
     <!-- Mobile Menu Panel -->
     <div 
       id="mobile-menu" 
       class="hidden md:hidden absolute top-full left-0 w-full bg-[#f9faf2] dark:bg-[#012d1d] shadow-lg border-t border-outline-variant/30"
     >
       <div class="flex flex-col px-8 py-6 space-y-4">
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#problema">O Problema</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#intestino">O Intestino</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#metodo">Método</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#jornada">Jornada</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#dra-cristal">Dra. Cristal</a>
         <a class="text-[#1a1c18] dark:text-[#f9faf2] hover:text-[#735c00] transition-colors py-2" href="#faq">FAQ</a>
         <a class="inline-block text-center px-6 py-3 bg-primary text-on-primary rounded-xl font-medium hover:opacity-80 transition-opacity mt-4" href="#cta">Agendar Avaliação</a>
       </div>
     </div>
   </nav>
   ```

2. **Criar `js/main.js`**
   ```javascript
   document.addEventListener('DOMContentLoaded', function() {
     const mobileMenuButton = document.getElementById('mobile-menu-button');
     const mobileMenu = document.getElementById('mobile-menu');
     const mobileLinks = mobileMenu.querySelectorAll('a');
     
     // Toggle mobile menu
     mobileMenuButton.addEventListener('click', function() {
       const isExpanded = this.getAttribute('aria-expanded') === 'true';
       this.setAttribute('aria-expanded', !isExpanded);
       mobileMenu.classList.toggle('hidden');
       
       // Update icon
       const svg = this.querySelector('svg');
       if (!isExpanded) {
         svg.innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>';
       } else {
         svg.innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>';
       }
     });
     
     // Close menu when clicking a link
     mobileLinks.forEach(link => {
       link.addEventListener('click', function() {
         mobileMenu.classList.add('hidden');
         mobileMenuButton.setAttribute('aria-expanded', 'false');
         mobileMenuButton.querySelector('svg').innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>';
       });
     });
     
     // Close menu on escape key
     document.addEventListener('keydown', function(e) {
       if (e.key === 'Escape' && !mobileMenu.classList.contains('hidden')) {
         mobileMenu.classList.add('hidden');
         mobileMenuButton.setAttribute('aria-expanded', 'false');
         mobileMenuButton.querySelector('svg').innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>';
       }
     });
   });
   ```

3. **Adicionar script no HTML antes de `</body>`**
   ```html
   <script src="./js/main.js" defer></script>
   ```

#### Plano de Testes
- [ ] Testar em dispositivo móvel (ou DevTools mobile view)
- [ ] Verificar abertura/fechamento do menu hamburger
- [ ] Validar transição de ícone (hamburger ↔ X)
- [ ] Testar fechamento ao clicar em link
- [ ] Testar fechamento com tecla Escape
- [ ] Verificar acessibilidade (ARIA attributes)
- [ ] Validar navegação por teclado (tab navigation)
- [ ] Testar em diferentes tamanhos de tela (breakpoint 768px)

---

### Tarefa 2.3: Otimização de Performance de Imagens

#### Escopo
Melhorar Core Web Vitals através de otimizações de carregamento de imagens.

#### Elementos Envolvidos
- Todas as tags `<img>` no HTML

#### Alterações Necessárias

1. **Adicionar width/height explícitos em todas as imagens**
   - Já coberto na Tarefa 1.3

2. **Implementar lazy loading estratégico**
   ```html
   <!-- Hero (above-the-fold) - EAGER loading -->
   <img 
     src="./images/hero-microbiome.webp" 
     alt="..."
     width="1200" 
     height="800"
     fetchpriority="high"
     class="..."
   >
   
   <!-- Imagens secundárias - LAZY loading -->
   <img 
     src="./images/intestino-biologia.webp" 
     alt="..."
     width="800" 
     height="800"
     loading="lazy"
     decoding="async"
     class="..."
   >
   ```

3. **Adicionar pré-carregamento para fontes críticas**
   ```html
   <link rel="preload" as="style" href="./css/styles.css">
   <link rel="preload" as="font" type="font/woff2" href="https://fonts.gstatic.com/s/manrope/v15/xn7gYHE41bi...woff2" crossorigin>
   ```

#### Plano de Testes
- [ ] Executar Google Lighthouse (esperado: Performance > 90)
- [ ] Verificar CLS (Cumulative Layout Shift) < 0.1
- [ ] Medir LCP (Largest Contentful Paint) < 2.5s
- [ ] Validar no WebPageTest.org
- [ ] Testar em conexão 3G simulada

---

### Tarefa 2.4: Atualização do Copyright

#### Escopo
Atualizar ano do copyright para 2025 ou implementar dinamicamente.

#### Elementos Envolvidos
- Footer (linha 368)

#### Alterações Necessárias

**Opção A: Estático (recomendado para simplicidade)**
```html
<p class="text-[#1a1c18]/70 dark:text-[#f9faf2]/70 font-sans text-xs uppercase tracking-widest">
  © 2025 Elevve Clinic. Todos os direitos reservados.
</p>
```

**Opção B: Dinâmico com JavaScript**
```html
<p class="text-[#1a1c18]/70 dark:text-[#f9faf2]/70 font-sans text-xs uppercase tracking-widest">
  © <span id="current-year"></span> Elevve Clinic. Todos os direitos reservados.
</p>

<script>
  document.getElementById('current-year').textContent = new Date().getFullYear();
</script>
```

#### Plano de Testes
- [ ] Verificar ano exibido corretamente
- [ ] Validar em janeiro do próximo ano (se dinâmico)

---

## 🟢 FASE 3 - RECOMENDADO (Boas Práticas)

### Tarefa 3.1: Limpeza de Código

#### Escopo
Remover atributos `style=""` vazios e melhorar legibilidade do código.

#### Elementos Envolvidos
- Todo o arquivo `index.html`
- Ocorrências: ~40+ atributos `style=""` vazios

#### Alterações Necessárias

1. **Script de limpeza (opcional)**
   ```bash
   # Usando sed para remover style="" vazio
   sed -i 's/ style=""//g' index.html
   ```

2. **Ou manualmente via busca/substituição no editor**
   - Buscar: ` style=""`
   - Substituir por: `` (vazio)

#### Plano de Testes
- [ ] Validar que página renderiza identicamente
- [ ] Verificar redução no tamanho do arquivo HTML
- [ ] Executar validador HTML (W3C Validator)

---

### Tarefa 3.2: Headers de Segurança Avançados

#### Escopo
Adicionar headers de segurança adicionais no `vercel.json`.

#### Elementos Envolvidos
- Arquivo: `vercel.json`

#### Alterações Necessárias

```json
{
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "X-Content-Type-Options",
          "value": "nosniff"
        },
        {
          "key": "X-Frame-Options",
          "value": "DENY"
        },
        {
          "key": "X-XSS-Protection",
          "value": "1; mode=block"
        },
        {
          "key": "Referrer-Policy",
          "value": "strict-origin-when-cross-origin"
        },
        {
          "key": "Permissions-Policy",
          "value": "camera=(), microphone=(), geolocation=()"
        },
        {
          "key": "Content-Security-Policy",
          "value": "default-src 'self'; script-src 'self' 'unsafe-inline' https://fonts.googleapis.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https:; connect-src 'self' https://wa.me https://calendly.com;"
        },
        {
          "key": "Strict-Transport-Security",
          "value": "max-age=31536000; includeSubDomains"
        }
      ]
    }
  ]
}
```

#### Plano de Testes
- [ ] Verificar headers com `curl -I https://www.elevveclinic.com.br/intestino`
- [ ] Validar no securityheaders.com (buscar rating A+)
- [ ] Testar CSP sem quebrar funcionalidades

---

### Tarefa 3.3: Integração de Analytics

#### Escopo
Implementar Google Analytics 4 e/ou Google Tag Manager para tracking de conversões.

#### Elementos Envolvidos
- Seção `<head>` do `index.html`

#### Alterações Necessárias

**Opção A: Google Analytics 4 Direto**
```html
<!-- Google tag (gtag.js) -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-XXXXXXXXXX');
  
  // Track CTA clicks
  document.querySelectorAll('a[href*="wa.me"], a[href*="calendly"]').forEach(function(link) {
    link.addEventListener('click', function() {
      gtag('event', 'cta_click', {
        'event_category': 'conversion',
        'event_label': 'Agendar Avaliação',
        'transport_type': 'beacon'
      });
    });
  });
</script>
```

**Opção B: Google Tag Manager (Recomendado)**
```html
<!-- Google Tag Manager -->
<script>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','GTM-XXXXXXX');</script>
<!-- End Google Tag Manager -->
```

E adicionar no-body:
```html
<!-- Google Tag Manager (noscript) -->
<noscript><iframe src="https://www.googletagmanager.com/ns.html?id=GTM-XXXXXXX"
height="0" width="0" style="display:none;visibility:hidden"></iframe></noscript>
```

#### Plano de Testes
- [ ] Verificar instalação no GA4 Real-Time
- [ ] Testar eventos de clique nos CTAs
- [ ] Validar no Google Tag Assistant
- [ ] Configurar conversões no GA4

---

## 🚀 FASE 4 - DEPLOY E CONFIGURAÇÃO

### Tarefa 4.1: Configuração do Domínio na Vercel

#### Escopo
Configurar subdomínio `www.elevveclinic.com.br/intestino` na Vercel.

#### Elementos Envolvidos
- Dashboard da Vercel
- DNS do domínio `elevveclinic.com.br`

#### Alterações Necessárias

1. **No Dashboard da Vercel:**
   - Acessar projeto da landing page
   - Settings → Domains
   - Adicionar domínio: `elevveclinic.com.br`
   - Configurar redirect de `elevveclinic.com.br/intestino`

2. **Configuração DNS no Registrador:**
   ```
   Type: CNAME
   Name: www
   Value: cname.vercel-dns.com
   TTL: Auto
   ```
   
   Ou para domínio raíz:
   ```
   Type: A
   Name: @
   Value: 76.76.21.21
   TTL: 3600
   ```

3. **Configurar Rewrite no vercel.json para subdomínio:**
   ```json
   {
     "rewrites": [
       {
         "source": "/intestino",
         "destination": "/index.html"
       },
       {
         "source": "/intestino/:path*",
         "destination": "/:path*"
       }
     ]
   }
   ```

#### Plano de Testes
- [ ] Verificar propagação DNS (pode levar até 48h)
- [ ] Testar acesso via `https://www.elevveclinic.com.br/intestino`
- [ ] Validar certificado SSL (automático via Let's Encrypt)
- [ ] Testar redirect HTTP → HTTPS
- [ ] Verificar no SSL Labs (ssllabs.com/ssltest)

---

### Tarefa 4.2: Deploy na Vercel

#### Escopo
Realizar deploy final e validação em produção.

#### Elementos Envolvidos
- Repositório Git
- Vercel CLI ou Dashboard

#### Passos de Deploy

1. **Commit e push das alterações**
   ```bash
   git add .
   git commit -m "feat: prepara landing page para deploy na Vercel"
   git push origin main
   ```

2. **Deploy via Vercel Dashboard ou CLI**
   ```bash
   vercel --prod
   ```

3. **Configurar variáveis de ambiente (se necessário)**
   - No dashboard: Settings → Environment Variables

#### Plano de Testes

**Testes Funcionais:**
- [ ] Todas as seções carregam corretamente
- [ ] Navegação entre âncoras funciona
- [ ] Menu mobile opera normalmente
- [ ] Todos os CTAs redirecionam corretamente
- [ ] Imagens carregam sem erros

**Testes de Performance:**
- [ ] Lighthouse Score > 90 (Performance)
- [ ] Lighthouse Score > 90 (SEO)
- [ ] Lighthouse Score > 90 (Accessibility)
- [ ] Lighthouse Score > 90 (Best Practices)
- [ ] Tempo de carregamento < 3s em 4G

**Testes Cross-Browser:**
- [ ] Chrome (última versão)
- [ ] Firefox (última versão)
- [ ] Safari (última versão)
- [ ] Edge (última versão)
- [ ] Mobile Safari (iOS)
- [ ] Chrome Mobile (Android)

**Testes Responsivos:**
- [ ] Desktop (1920px, 1366px, 1024px)
- [ ] Tablet (768px, 834px)
- [ ] Mobile (375px, 414px)

**Testes de SEO:**
- [ ] Submit no Google Search Console
- [ ] Validar sitemap (se aplicável)
- [ ] Verificar indexabilidade (robots.txt)

---

## 📊 RESUMO DO CHECKLIST

### Fase 1 - Crítico
| # | Tarefa | Status | Prioridade |
|---|--------|--------|------------|
| 1.1 | Reestruturação do Projeto | ⬜ Pendente | 🔴 |
| 1.2 | Substituição Tailwind CDN | ⬜ Pendente | 🔴 |
| 1.3 | Download e Otimização de Imagens | ⬜ Pendente | 🔴 |
| 1.4 | Conexão dos CTAs | ⬜ Pendente | 🔴 |

### Fase 2 - Importante
| # | Tarefa | Status | Prioridade |
|---|--------|--------|------------|
| 2.1 | Meta Tags de SEO | ⬜ Pendente | 🟡 |
| 2.2 | Menu Mobile | ⬜ Pendente | 🟡 |
| 2.3 | Performance de Imagens | ⬜ Pendente | 🟡 |
| 2.4 | Atualização Copyright | ⬜ Pendente | 🟡 |

### Fase 3 - Recomendado
| # | Tarefa | Status | Prioridade |
|---|--------|--------|------------|
| 3.1 | Limpeza de Código | ⬜ Pendente | 🟢 |
| 3.2 | Headers de Segurança | ⬜ Pendente | 🟢 |
| 3.3 | Integração Analytics | ⬜ Pendente | 🟢 |

### Fase 4 - Deploy
| # | Tarefa | Status | Prioridade |
|---|--------|--------|------------|
| 4.1 | Configuração Domínio | ⬜ Pendente | 🔴 |
| 4.2 | Deploy e Validação | ⬜ Pendente | 🔴 |

---

## 🎯 CRITÉRIOS DE ACEITE

### Técnicos
- ✅ Página carrega em < 3 segundos em conexão 4G
- ✅ Lighthouse Performance Score ≥ 90
- ✅ Zero erros no console do navegador
- ✅ Todas as imagens otimizadas (WebP, lazy loading)
- ✅ CSS estático (sem CDN em produção)
- ✅ Estrutura compatível com Vercel

### Funcionais
- ✅ Todos os CTAs funcionais (WhatsApp/Calendly)
- ✅ Menu mobile operacional
- ✅ Navegação por âncoras funcionando
- ✅ Links do footer válidos

### SEO & Acessibilidade
- ✅ Meta tags completas (description, OG, Twitter)
- ✅ Structured data válido (JSON-LD)
- ✅ Favicon configurado
- ✅ Canonical URL definida
- ✅ ARIA labels no menu mobile
- ✅ Navegação por teclado funcional

### Segurança
- ✅ HTTPS ativo
- ✅ Headers de segurança configurados
- ✅ CSP básico implementado
- ✅ Links externos com `rel="noopener noreferrer"`

---

## 📝 NOTAS ADICIONAIS

### Dependências Externas
- Google Fonts (Noto Serif, Manrope, Material Symbols)
- WhatsApp Business API (para CTAs)
- Calendly (opcional, para agendamentos)
- Google Analytics 4 / GTM

### Considerações Futuras
1. **Páginas do Footer:** Criar páginas estáticas para Termos de Uso e Política de Privacidade
2. **Formulário de Contato:** Implementar formulário próprio (ex: Formspree, EmailJS)
3. **Blog/Conteúdo:** Integrar seção de blog para SEO contínuo
4. **A/B Testing:** Configurar experimentos para otimizar conversão
5. **Pixel do Facebook:** Adicionar para remarketing

### Contatos Úteis
- **Suporte Vercel:** https://vercel.com/support
- **Documentação Tailwind:** https://tailwindcss.com/docs
- **Google Search Console:** https://search.google.com/search-console
- **Lighthouse:** https://developer.chrome.com/docs/lighthouse/overview/

---

**Documento criado em:** Janeiro 2025  
**Versão:** 1.0  
**Responsável:** Equipe de Engenharia de Software  
**Status:** Aprovação Pendente
