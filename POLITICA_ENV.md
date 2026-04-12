# 📄 Política de Arquivos .env - Landing Page Elevve Clinic

## Visão Geral

Este documento explica a estratégia de gerenciamento de variáveis de ambiente para a landing page, permitindo que os arquivos de configuração sejam **commitados no Git** e revisados em Pull Requests, mantendo a segurança dos dados sensíveis.

---

## 🗂️ Estrutura de Arquivos

```
/frontend/
├── .env.example       # Template base de referência (SEMPRE commitado)
├── .env.local         # Configuração desenvolvimento localhost (COMMITADO)
├── .env.production    # Configuração produção Vercel (COMMITADO)
├── .env               # Arquivo ativo do ambiente (NUNCA commitado - .gitignore)
└── .gitignore         # Regras de exclusão do Git
```

---

## 🔐 Princípios de Segurança

### ✅ O Que PODE Ser Commitado

| Arquivo | Pode Commitar? | Por quê? |
|---------|----------------|----------|
| `.env.example` | ✅ Sim | Template genérico sem dados reais |
| `.env.local` | ✅ Sim | Contém valores de sandbox/teste seguros |
| `.env.production` | ✅ Sim | Template com placeholders (ex: `G-XXXXXXXXXX`) |

### ❌ O Que NUNCA Deve Ser Commitado

| Arquivo | Pode Commitar? | Por quê? |
|---------|----------------|----------|
| `.env` | ❌ Nunca | Contém configurações ativas do ambiente atual (pode ter dados reais) |

---

## 📋 Descrição dos Arquivos

### 1. `.env.example` - Template Base
**Finalidade:** Servir como referência da estrutura de variáveis

**Conteúdo:**
- Todas as variáveis disponíveis
- Valores genéricos ou vazios
- Comentários explicativos

**Uso:**
```bash
# Desenvolvedores consultam este arquivo para saber quais variáveis existem
cat .env.example
```

---

### 2. `.env.local` - Configuração de Desenvolvimento
**Finalidade:** Permitir que qualquer desenvolvedor rode o projeto em localhost imediatamente

**Conteúdo:**
- `MODE=local`
- `BASE_URL=http://localhost:3000`
- `WHATSAPP_NUMBER=5511999999999` (número fictício)
- `GA4_ID=` (vazio - analytics desativado)
- `DEBUG_MODE=true`
- `ANALYTICS_ENABLED=false`

**Por que pode ser commitado?**
- Usa números de WhatsApp fictícios
- Analytics desativado
- URLs apontam para localhost
- Nenhum dado sensível real

**Uso:**
```bash
# Ativar modo local
cp .env.local .env
npm install
npm run dev
# Acessar: http://localhost:3000
```

---

### 3. `.env.production` - Configuração de Produção
**Finalidade:** Template para deploy em produção com estrutura validada em PR

**Conteúdo:**
- `MODE=production`
- `BASE_URL=https://www.elevveclinic.com.br/intestino`
- `WHATSAPP_NUMBER=5511999999999` (**placeholder** - deve ser substituído)
- `GA4_ID=G-XXXXXXXXXX` (**placeholder** - deve ser substituído)
- `GTM_ID=GTM-XXXXXXX` (**placeholder** - deve ser substituído)
- `DEBUG_MODE=false`
- `ANALYTICS_ENABLED=true`

**⚠️ AVISO CRÍTICO:**
Antes de fazer merge do PR em main/production, alguém DEVE:
1. Substituir todos os placeholders pelos valores reais
2. Ou configurar as variáveis no dashboard da Vercel

**Por que pode ser commitado?**
- Contém apenas **placeholders** (ex: `G-XXXXXXXXXX`)
- Serve como checklist do que precisa ser configurado
- Facilita review em PR para verificar se todas variáveis necessárias estão presentes

**Uso:**
```bash
# Opção 1: Copiar e editar manualmente
cp .env.production .env
# Editar .env com valores reais antes de build

# Opção 2 (RECOMENDADO): Usar environment variables da Vercel
# Configurar no dashboard da Vercel as variáveis reais
# O .env.production serve apenas como referência
```

---

### 4. `.env` - Arquivo Ativo
**Finalidade:** Armazenar a configuração ativa do ambiente atual

**Status:** 🔴 **NUNCA COMMITAR**

**Por que é ignorado pelo Git?**
- Pode conter dados reais (IDs do Google Analytics, números de WhatsApp reais)
- É específico de cada ambiente (desenvolvedor, CI, produção)
- Muda frequentemente durante o desenvolvimento

**Como funciona:**
```bash
# O arquivo .env é criado dinamicamente copiando um dos templates
cp .env.local .env        # Para desenvolvimento
cp .env.production .env   # Para produção (antes de build)

# OU a Vercel injeta as variáveis diretamente no build
```

---

## 🔄 Fluxo de Trabalho com Pull Requests

### Cenário 1: Adicionar Nova Variável de Ambiente

1. **Desenvolvedor** adiciona nova variável nos 3 arquivos:
   ```bash
   # .env.example
   NEW_FEATURE_KEY=
   
   # .env.local
   NEW_FEATURE_KEY=false
   
   # .env.production
   NEW_FEATURE_KEY=true
   ```

2. **Commit** os 3 arquivos no branch da feature:
   ```bash
   git add .env.example .env.local .env.production
   git commit -m "feat: add NEW_FEATURE_KEY variable"
   ```

3. **Pull Request** mostra as mudanças nos 3 arquivos

4. **Reviewers** verificam se:
   - A variável faz sentido
   - Os valores default são seguros
   - A documentação está clara

5. **Merge** na branch principal

---

### Cenário 2: Atualizar Configuração de Produção

1. **Desenvolvedor** identifica necessidade de mudar configuração:
   ```bash
   # Exemplo: Ativar newsletter
   # Mudar em .env.production:
   ENABLE_NEWSLETTER=false → ENABLE_NEWSLETTER=true
   ```

2. **Commit** apenas `.env.production`:
   ```bash
   git add .env.production
   git commit -m "config: enable newsletter in production"
   ```

3. **Pull Request** mostra a mudança claramente

4. **Reviewers** aprovam a configuração

5. **Antes do Deploy:**
   - Se usar Vercel Dashboard: configurar a variável lá
   - Se usar arquivo: substituir placeholders antes do build

---

## 🚀 Deploy na Vercel

### Opção 1: Environment Variables no Dashboard (RECOMENDADO)

1. Acesse [Vercel Dashboard](https://vercel.com/dashboard)
2. Selecione o projeto
3. Vá em **Settings** → **Environment Variables**
4. Adicione as variáveis reais:
   ```
   WHATSAPP_NUMBER = 5511987654321
   GA4_ID = G-ABC123XYZ
   GTM_ID = GTM-ABC123
   ```
5. Faça redeploy

**Vantagens:**
- ✅ Dados sensíveis não estão no Git
- ✅ Fácil rotação de credenciais
- ✅ Diferentes valores por preview/deployment
- ✅ Audit trail no dashboard

---

### Opção 2: Usar Arquivo .env.production Editado

1. **Edite** `.env.production` localmente com valores reais:
   ```bash
   cp .env.production .env.production.real
   # Edite .env.production.real com valores reais
   ```

2. **NÃO commite** o arquivo com valores reais!

3. **Build** local:
   ```bash
   cp .env.production.real .env
   npm run build
   vercel --prod
   ```

**Desvantagens:**
- ❌ Risco de commitar acidentalmente
- ❌ Difícil gerenciar múltiplos ambientes
- ❌ Sem audit trail

---

## 📝 Checklist de Review para PRs

Quando um PR modificar arquivos `.env.*`, verificar:

### Para `.env.local`:
- [ ] Todos os valores são seguros para desenvolvimento?
- [ ] Analytics está desativado (`ANALYTICS_ENABLED=false`)?
- [ ] URLs apontam para localhost?
- [ ] Números de WhatsApp são fictícios?
- [ ] Debug mode está ativado?

### Para `.env.production`:
- [ ] Placeholders estão claros (ex: `G-XXXXXXXXXX`)?
- [ ] Comentários indicam o que precisa ser substituído?
- [ ] Todas variáveis necessárias estão presentes?
- [ ] Valores default são seguros caso esqueçam de substituir?

### Para `.env.example`:
- [ ] Documentação está clara?
- [ ] Todas variáveis dos outros arquivos estão listadas?
- [ ] Exemplos de uso estão corretos?

---

## 🔍 Como Verificar Se Algo Sensível Foi Commitado

### Antes de Commitar:
```bash
# Verificar o que será commitado
git diff --cached .env.local
git diff --cached .env.production

# Buscar por padrões suspeitos
grep -r "G-[A-Z0-9]\{10\}" .env.production  # IDs do Google reais
grep -r "55[0-9]\{10\}" .env.production      # Números de telefone reais
```

### Depois de Commitar (mas antes de push):
```bash
# Verificar último commit
git show HEAD -- .env.local .env.production

# Se achar algo sensível, faça amend ou revert
git commit --amend
# ou
git revert HEAD
```

### Ferramentas Recomendadas:
- [GitGuardian](https://www.gitguardian.com/) - Scan de secrets no Git
- [TruffleHog](https://github.com/trufflesecurity/trufflehog) - Detectar credenciais
- [pre-commit hooks](https://pre-commit.com/) - Validar antes de commit

---

## 🛡️ Boas Práticas

1. **Sempre use placeholders** em `.env.production`
   ```bash
   # ✅ Bom
   GA4_ID=G-XXXXXXXXXX
   
   # ❌ Ruim
   GA4_ID=G-ABC123XYZ789
   ```

2. **Documente o que precisa ser substituído**
   ```bash
   # WhatsApp - SUBSTITUIR PELO NÚMERO REAL DA CLÍNICA
   WHATSAPP_NUMBER=5511999999999
   ```

3. **Use Environment Variables da Vercel** para dados sensíveis
   - Mais seguro
   - Mais flexível
   - Melhor auditabilidade

4. **Revise PRs cuidadosamente**
   - Diff de arquivos .env deve ser uma prática padrão no review

5. **Mantenha .env no .gitignore**
   - Nunca remova esta regra
   - É sua última linha de defesa

---

## 📞 Dúvidas Comuns

### "Posso commitar .env.production com dados reais?"
**NÃO!** Use placeholders e configure os dados reais no dashboard da Vercel.

### "E se eu precisar de valores diferentes por desenvolvedor?"
Use `.env.local` para isso. Cada dev pode ter seu próprio `.env` (não commitado) baseado no template `.env.local`.

### "Como faço rollback se commitar dados sensíveis?"
1. Reverta o commit imediatamente
2. Rotacione as credenciais expostas
3. Use ferramentas como [BFG Repo-Cleaner](https://rtyley.github.io/bfg-repo-cleaner/) para remover do histórico

### "Posso ter múltiplos arquivos .env.production?"
Sim! Exemplo:
- `.env.production.template` - Template genérico commitado
- `.env.production.real` - Com dados reais (no .gitignore)

---

## 📚 Referências

- [Vercel Environment Variables](https://vercel.com/docs/concepts/projects/environment-variables)
- [12 Factor App - Config](https://12factor.net/config)
- [GitGuardian Best Practices](https://blog.gitguardian.com/best-practices-for-managing-environment-variables/)

---

**Última atualização:** Abril 2025  
**Responsável:** Equipe de Engenharia - Elevve Clinic
