# Guia de Landing Pages - Elevve Clinic

## Estrutura Implementada

Este projeto agora suporta múltiplas landing pages com URLs amigáveis para diferentes temas e testes A/B.

## URLs Configuradas

Atualmente, as seguintes rotas estão configuradas:

- `elevveclinic.com.br/intestino` → Landing page de Modulação Intestinal
- `elevveclinic.com.br/emagrecimento` → Landing page de Emagrecimento (template base)
- `elevveclinic.com.br/harmonizacao` → Landing page de Harmonização (template base)

## Estrutura de Diretórios

```
/workspace/
├── public/
│   ├── intestino/
│   │   └── index.html          # Landing page intestinal
│   ├── emagrecimento/
│   │   └── index.html          # Landing page de emagrecimento
│   ├── harmonizacao/
│   │   └── index.html          # Landing page de harmonização
│   ├── css/                    # CSS compartilhado
│   └── images/                 # Imagens compartilhadas
├── vercel.json                 # Configuração de rotas
└── index.html                  # Página principal (root)
```

## Como Criar Nova Landing Page

### 1. Criar o diretório da nova landing page

```bash
mkdir -p /workspace/public/nova-landing
```

### 2. Copiar o template base

```bash
cp /workspace/index.html /workspace/public/nova-landing/index.html
```

### 3. Editar o conteúdo da landing page

Edite o arquivo `/workspace/public/nova-landing/index.html` para:
- Alterar título, descrição e meta tags SEO
- Modificar conteúdo conforme o tema
- Ajustar imagens e cores se necessário

### 4. Adicionar rota no vercel.json

Adicione no array `rewrites` do arquivo `vercel.json`:

```json
{
  "source": "/nova-landing",
  "destination": "/public/nova-landing/index.html"
},
{
  "source": "/nova-landing/",
  "destination": "/public/nova-landing/index.html"
}
```

### 5. Fazer deploy

```bash
git add .
git commit -m "Adiciona landing page: nova-landing"
git push
```

## Testes A/B

Para criar variações A/B de uma landing page:

### Exemplo: Teste A/B para intestino

1. Crie duas versões:
   ```bash
   mkdir -p /workspace/public/intestino-a
   mkdir -p /workspace/public/intestino-b
   ```

2. Copie e edite cada versão com suas variações

3. Adicione rotas separadas:
   ```json
   { "source": "/intestino-a", "destination": "/public/intestino-a/index.html" },
   { "source": "/intestino-b", "destination": "/public/intestino-b/index.html" }
   ```

4. Use ferramentas de analytics para comparar conversões

## Observações Importantes

1. **Paths relativos**: Todos os arquivos HTML devem usar paths relativos para CSS e imagens:
   - ✅ Correto: `<link href="/public/css/output.css">`
   - ✅ Correto: `<img src="/public/images/logo.webp">`
   - ❌ Errado: `<link href="./public/css/output.css">`

2. **SEO**: Cada landing page deve ter suas próprias meta tags:
   - Title único
   - Description específica
   - Canonical URL correta
   - Open Graph tags apropriadas

3. **Imagens**: Imagens específicas de cada landing page podem ser colocadas em subdiretórios próprios ou usar o diretório compartilhado `/public/images/`

4. **Performance**: O Vercel já está configurado com cache agressivo para assets estáticos (CSS, JS, imagens)

## Próximos Passos Sugeridos

1. Criar landing pages específicas para cada serviço
2. Implementar tracking de analytics por landing page
3. Configurar pixels de conversão (Facebook, Google Ads)
4. Criar variações para testes A/B
5. Monitorar métricas de conversão por URL

---

**Documentação criada em**: $(date)
**Versão**: 1.0
