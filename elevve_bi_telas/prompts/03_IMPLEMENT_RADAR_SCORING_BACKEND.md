# Prompt — Implementar service de cálculo do Radar

Implemente o cálculo do Radar no backend.

Regras:

- Pontuação = resposta × peso.
- Score bruto = soma por eixo.
- Cluster ativo = 3 sintomas-chave com resposta >= 2.
- Bônus cluster = +3.
- Score final = score bruto + bônus.
- Score risco = score final / score máximo × 100.
- Score equilíbrio = 100 - score risco.
- Classificação vem de tabela de faixas.
- Se faixa não existir, retornar `Pendente de configuração`.
- Eixo prioritário usa score risco + bônus se cluster ativo.

Requisitos:

- Criar service isolado.
- Não acoplar cálculo a rota.
- Criar testes simples se o padrão do repo permitir.
- Preservar error envelope existente.
