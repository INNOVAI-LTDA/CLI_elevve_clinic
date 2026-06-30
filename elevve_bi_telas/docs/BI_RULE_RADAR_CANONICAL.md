# Regra BI Oficial — Radar de Longevidade Elevve

## Pilares

1. Hormônios
2. Intestino
3. Recuperação
4. Estrutura
5. Metabolismo

## Escala de resposta

| Frequência | Pontos |
|---|---:|
| Nunca/quase nunca | 0 |
| Ocasionalmente | 1 |
| Frequentemente | 2 |
| Muito frequentemente | 3 |

## Fórmula por pergunta

```text
pontuacao_pergunta = resposta_0_3 × peso
```

## Fórmula por eixo

```text
score_bruto_eixo = soma(pontuacao_pergunta)
```

## Regra de cluster

Se o paciente tiver 3 sintomas-chave daquele eixo com pontuação maior ou igual a 2:

```text
cluster_ativo = true
bonus_cluster = +3
```

Caso contrário:

```text
cluster_ativo = false
bonus_cluster = 0
```

## Score final do eixo

```text
score_final_eixo = score_bruto_eixo + bonus_cluster
```

## Normalização visual

Como o questionário mede sintomas/desregulação:

```text
score_risco = score_final_eixo / score_maximo_eixo × 100
score_equilibrio = 100 - score_risco
```

Use:

- `score_risco` para prioridade interna;
- `score_equilibrio` para visual premium do Radar.

---

# Eixo Metabolismo

## Capacidade central

Produção e estabilidade de energia metabólica.

## Perguntas e pesos

| Código | Pergunta curta | Peso | Cluster key |
|---|---|---:|---|
| metabolismo_01 | dificuldade emagrecer | 2 | Sim |
| metabolismo_02 | fome rápida | 1 | Não |
| metabolismo_03 | vontade doces | 1 | Não |
| metabolismo_04 | queda energia / sono pós-refeição | 1 ou 2 | Sim |
| metabolismo_05 | sonolência pós-refeição | 2 | Sim |
| metabolismo_06 | gordura abdominal | 2 | Sim |
| metabolismo_07 | dificuldade manter peso | 1 | Não |
| metabolismo_08 | piora concentração | 1 | Não |

## Score máximo informado

```text
33 pontos
```

## Interpretação

| Faixa | Interpretação |
|---|---|
| 0–7 | Estável |
| 8–14 | Atenção funcional |
| 15–22 | Desregulação importante |
| >22 | Eixo prioritário |

## Cluster Metabólico

Sintomas-chave:

- dificuldade emagrecer;
- gordura abdominal;
- sonolência pós-refeição.

Regra:

```text
se os 3 sintomas-chave tiverem resposta >= 2:
    +3 pontos metabólicos
```

---

# Eixo Intestino

## Capacidade central

Digestão, absorção e tolerância alimentar adequadas.

## Perguntas e pesos

| Código | Pergunta curta | Peso |
|---|---|---:|
| intestino_01 | distensão | 2 |
| intestino_02 | gases | 1 |
| intestino_03 | desconforto refeições | 1 |
| intestino_04 | piora alimentos | 2 |
| intestino_05 | intestino irregular | 2 |
| intestino_06 | fadiga pós alimentar | 2 |
| intestino_07 | constipação | 1 |
| intestino_08 | digestão lenta | 1 |

## Score máximo informado

```text
36 pontos
```

## Interpretação

| Faixa | Interpretação |
|---|---|
| 0–8 | Estável |
| 9–16 | Atenção funcional |
| 17–24 | Desregulação importante |
| >24 | Eixo prioritário |

## Cluster

Cluster não explicitado no material bruto. Não inventar. Deixar configurável.

---

# Eixo Hormônios

## Capacidade central

Estabilidade hormonal e neuroendócrina.

## Perguntas e pesos

| Código | Pergunta curta | Peso | Cluster key |
|---|---|---:|---|
| hormonios_01 | insônia | 2 | Não |
| hormonios_02 | acorda cansada | 2 | Não |
| hormonios_03 | oscilação humor | 1 | Não |
| hormonios_04 | irritabilidade | 1 | Não |
| hormonios_05 | libido | 2 | Sim |
| hormonios_06 | calorão/suor | 2 | Sim |
| hormonios_07 | memória/concentração | 1 | Não |
| hormonios_08 | cabelo/pele | 1 | Sim |

## Score máximo informado

```text
36 pontos
```

## Cluster Hormonal

Sintomas-chave:

- calorão/suor;
- libido;
- cabelo/pele.

Regra:

```text
se os 3 sintomas-chave tiverem resposta >= 2:
    +3 pontos hormonais
```

## Interpretação

Faixas não definidas no material bruto. Não inventar. Criar configuração pendente.

---

# Eixo Recuperação Funcional

## Capacidade central

Capacidade de recuperação e adaptação ao estresse físico e mental.

## Perguntas e pesos

| Código | Pergunta curta | Peso | Cluster key |
|---|---|---:|---|
| recuperacao_01 | cansaço persistente | 2 | Não |
| recuperacao_02 | energia acaba rápido | 2 | Sim |
| recuperacao_03 | dificuldade relaxar | 1 | Não |
| recuperacao_04 | pequenos problemas esgotam | 2 | Não |
| recuperacao_05 | piora sob estresse | 2 | Não |
| recuperacao_06 | dependência cafeína | 1 | Sim |
| recuperacao_07 | dores/tensão | 1 | Não |
| recuperacao_08 | recuperação física ruim | 2 | Não |
| recuperacao_extra_01 | acorda cansada | 2 | Sim |

## Score máximo informado

```text
39 pontos
```

## Cluster Recuperação

Sintomas-chave:

- acorda cansada;
- dependência de cafeína;
- energia acaba rápido.

Regra:

```text
se os 3 sintomas-chave tiverem resposta >= 2:
    +3 pontos recuperação
```

## Observação

O item "acorda cansada" também aparece em Hormônios. Para implementação, permitir perguntas compartilhadas entre clusters ou duplicar como regra de cluster referenciando uma pergunta de outro eixo.

## Interpretação

Faixas não definidas no material bruto. Não inventar. Criar configuração pendente.

---

# Eixo Estrutura

## Capacidade central

Manutenção da força, resistência e integridade física ao longo do tempo.

## Perguntas e pesos

| Código | Pergunta curta | Peso |
|---|---|---:|
| estrutura_01 | perda força | 2 |
| estrutura_02 | flacidez / dificuldade massa muscular | 1 |
| estrutura_03 | dores físicas | 1 |
| estrutura_04 | retenção/peso pernas | 2 |
| estrutura_05 | piora resistência | 2 |
| estrutura_06 | perda muscular | 2 |
| estrutura_07 | limitação física | 2 |
| estrutura_08 | piora composição corporal | 1 |

## Score máximo informado

```text
39 pontos
```

## Cluster

Cluster não explicitado no material bruto. Não inventar. Deixar configurável.

## Interpretação

Faixas não definidas no material bruto. Não inventar. Criar configuração pendente.

---

# Eixo prioritário

O eixo prioritário não é apenas a maior pontuação.

A definição considera:

- score;
- cluster;
- impacto;
- tempo de evolução;
- efeito sistêmico.

Para a versão inicial:

```text
indice_prioridade = score_final_eixo + bonus_cluster_ativo
```

ou:

```text
indice_prioridade = score_risco + 10 se cluster_ativo
```

Permitir evolução posterior com impacto, tempo e efeito sistêmico.

---

# Pendências a validar

1. Faixas de interpretação de Hormônios.
2. Faixas de interpretação de Recuperação.
3. Faixas de interpretação de Estrutura.
4. Cluster de Intestino.
5. Cluster de Estrutura.
6. Consistência dos scores máximos informados versus soma de pesos.
7. Como tratar pergunta "acorda cansada" compartilhada entre Hormônios e Recuperação.
8. Se o visual do Radar deve mostrar risco ou equilíbrio.
