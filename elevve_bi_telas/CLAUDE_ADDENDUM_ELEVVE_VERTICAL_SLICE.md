# CLAUDE ADDENDUM — Elevve BI Vertical Slice

> Este arquivo complementa o `CLAUDE.md` existente.  
> Não substitua o `CLAUDE.md` raiz.

## Contexto

O repo atual já possui `migration-kit` com backend FastAPI, frontend React/Vite/TypeScript, SQL/migrations, contratos e admin TUI.

A missão agora é construir uma **fatia vertical real** do BI Elevve, validando banco, regra de BI e visual.

## Decisão

Não criar Streamlit.

Implementar dentro da arquitetura existente do `migration-kit`.

## Escopo da fatia vertical inicial

Prioridade: **Radar de Longevidade**.

Depois:

1. Histórico/Evolução
2. Relatório/Output
3. Visão Geral
4. Matriz/Centro de Comando, se já existirem no app

## Croquis visuais

Usar os arquivos em:

```text
assets/croquis/
assets/icons/
```

como referência visual para componentes React.

Não tentar pixel-perfect. Priorizar portabilidade e coerência com a arquitetura existente.

## Regra oficial do Radar

Pilares:

- Hormônios
- Intestino
- Recuperação
- Estrutura
- Metabolismo

Escala:

- 0 = Nunca/quase nunca
- 1 = Ocasionalmente
- 2 = Frequentemente
- 3 = Muito frequentemente

Regra:

```text
pontuação_pergunta = resposta_0_3 × peso
score_bruto_eixo = soma das pontuações
cluster ativo = 3 sintomas-chave com resposta >= 2
bônus cluster = +3
score_final_eixo = score_bruto_eixo + bônus
```

Interpretação deve ser configurável por eixo.

Metabolismo e Intestino possuem faixas definidas no material bruto. Para os demais eixos, não inventar faixas; criar configuração pendente ou usar placeholder explicitamente marcado.

## Regra visual

Como a pontuação mede sintomas/desregulação:

```text
score_risco = score_final / score_maximo × 100
score_equilibrio = 100 - score_risco
```

Usar `score_equilibrio` no visual do Radar se o objetivo for mostrar equilíbrio/saúde.

Usar `score_risco` para motor interno de prioridade.

## Fora do escopo

Não implementar:

- Streamlit
- app paralelo
- login novo
- deploy
- WhatsApp
- integração externa
- PDF real
- IA generativa dentro do app
- alteração de contratos existentes sem planejar compatibilidade

## Ordem de implementação

1. Inspecionar features existentes: radar, matrix, command-center.
2. Criar plano de arquivos.
3. Criar migration SQL do Radar.
4. Criar seed mínimo.
5. Criar schemas/DTOs.
6. Criar serviço de cálculo.
7. Criar endpoint.
8. Criar/adaptar tela React do Radar.
9. Criar stubs para Histórico e Relatório.
10. Revisar sem quebrar o migration-kit.

## Critério de pronto da fatia inicial

- migration nova aplicada ou validada;
- seed mínimo disponível;
- endpoint retorna Radar calculado;
- frontend exibe Radar com dados reais seedados/mockados;
- regra de cálculo isolada e testável;
- visual respeita os croquis;
- nenhum módulo crítico existente quebra.
