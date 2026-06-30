# Mapeamento UI — Croquis Elevve para Frontend React

## Assets

Croquis disponíveis em:

```text
assets/croquis/
```

Ícones disponíveis em:

```text
assets/icons/
```

## Telas identificadas

### Sidebar

Arquivo esperado:

```text
eixo_sidebar.png
```

Objetivo:

- navegação principal;
- identidade visual;
- atalhos para telas do BI.

Componentes React sugeridos:

```text
SidebarNav
SidebarNavItem
SidebarIcon
```

### Visão Geral

Arquivo esperado:

```text
Visao_geral.png
```

Objetivo:

- resumo do paciente ou carteira;
- principais cards;
- eixo prioritário;
- chamadas para Radar, Histórico e Relatório.

Componentes:

```text
OverviewPage
SummaryCard
PatientHeader
PriorityAxisCard
```

### Visão Radar

Arquivo esperado:

```text
Visao_radar.png
```

Objetivo:

- mostrar o Radar de Longevidade;
- barras/scores dos pilares;
- eixo prioritário;
- interpretação.

Componentes:

```text
RadarPage
RadarChart
RadarPillarScoreList
RadarAxisCard
RadarTechnicalScoreTable
```

### Histórico / Evolução

Arquivo esperado:

```text
Historico_evolucao.png
```

Objetivo:

- mostrar evolução temporal;
- snapshots do Radar;
- comparação entre ciclos ou respostas.

Componentes:

```text
HistoryEvolutionPage
RadarSnapshotTimeline
PillarEvolutionChart
```

### Relatório / Output

Arquivo esperado:

```text
Relatorio_output.png
```

Objetivo:

- saída interpretativa;
- síntese para paciente/equipe;
- recomendações;
- possível base futura para proposta.

Componentes:

```text
ReportOutputPage
RadarSummaryNarrative
RecommendationCard
NextCycleCallout
```

## Regras de implementação visual

1. Não tentar pixel-perfect.
2. Extrair layout, hierarquia e componentes.
3. Usar design system existente do frontend quando possível.
4. Manter CSS modular por feature.
5. Evitar dependências visuais pesadas.
6. Não embutir imagens como UI final, salvo ícones se fizer sentido.
7. Implementar primeiro Radar; Histórico e Relatório podem ficar como stubs navegáveis.

## Navegação mínima

```text
Visão Geral
Radar
Histórico
Relatório
```

## Prioridade de implementação

1. Sidebar/navegação
2. Radar
3. Visão Geral
4. Histórico stub
5. Relatório stub
