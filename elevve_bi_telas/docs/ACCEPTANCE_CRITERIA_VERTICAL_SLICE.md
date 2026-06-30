# Critérios de Aceite — Vertical Slice BI Elevve

## Banco

- Nova migration segue numeração e prefixo existentes.
- Tabelas do Radar existem.
- Seed mínimo existe.
- Respostas aceitam apenas 0–3.
- Pesos e faixas são configuráveis.

## Backend

- Regra de cálculo isolada em service.
- Endpoint retorna Radar por paciente.
- Error envelope existente não é quebrado.
- DTOs seguem padrão do repo.
- Regra não depende do frontend.

## Frontend

- Tela Radar renderiza dados da API ou mock service compatível.
- Visual usa croquis como referência.
- Sidebar/navegação contempla Visão Geral, Radar, Histórico, Relatório.
- Histórico e Relatório podem ser stubs.
- Não quebra build existente.

## BI

- Calcula score por pergunta.
- Soma por eixo.
- Aplica cluster +3.
- Interpreta faixas quando configuradas.
- Marca pendente quando faixa não existe.
- Calcula score risco.
- Calcula score equilíbrio.
- Determina eixo prioritário.

## Fora do escopo validado

- Não implementar Streamlit.
- Não implementar PDF real.
- Não implementar autenticação nova.
- Não implementar integração externa.
