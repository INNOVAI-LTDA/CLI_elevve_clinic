# Prompt — Criar migration e seed do Radar

Com base em `docs/DB_VERTICAL_SLICE_SPEC.md` e após inspecionar a numeração das migrations existentes:

1. Crie a próxima migration SQL para as tabelas do Radar.
2. Use prefixo `deva_elevveclinic_`.
3. Crie seed mínimo com:
   - 2 pacientes;
   - 5 pilares;
   - perguntas/pesos do Radar;
   - clusters conhecidos;
   - faixas de Metabolismo e Intestino;
   - respostas de exemplo.

Regras:

- Não inventar faixas para Hormônios, Recuperação e Estrutura.
- Para faixas ausentes, deixar sem configuração ou marcar pendente.
- Não quebrar migrations anteriores.
- Seguir padrão do repo.

Antes de escrever, mostre o nome da migration planejada.
