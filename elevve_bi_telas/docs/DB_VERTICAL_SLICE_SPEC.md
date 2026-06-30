# Especificação Banco de Dados — Vertical Slice Radar

## Objetivo

Criar o modelo mínimo de banco para validar o Radar de Longevidade ponta a ponta dentro do `migration-kit`.

## Prefixo obrigatório

Todos os objetos devem seguir o padrão do repo:

```text
deva_elevveclinic_
```

## Migrations sugeridas

```text
010_deva_elevveclinic_bi_radar.sql
011_deva_elevveclinic_bi_radar_seed.sql
```

## Tabelas mínimas

### deva_elevveclinic_bi_patients

```sql
id TEXT PRIMARY KEY,
patient_code TEXT UNIQUE NOT NULL,
name TEXT NOT NULL,
sex TEXT,
birth_date TEXT,
chronological_age INTEGER,
program_name TEXT,
program_start_date TEXT,
program_end_date TEXT,
status TEXT NOT NULL DEFAULT 'active',
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL
```

### deva_elevveclinic_bi_radar_pillars

```sql
id TEXT PRIMARY KEY,
pillar_code TEXT UNIQUE NOT NULL,
name TEXT NOT NULL,
description TEXT,
score_max INTEGER,
display_order INTEGER NOT NULL
```

### deva_elevveclinic_bi_radar_questions

```sql
id TEXT PRIMARY KEY,
question_code TEXT UNIQUE NOT NULL,
pillar_code TEXT NOT NULL,
question_text TEXT NOT NULL,
short_label TEXT NOT NULL,
weight REAL NOT NULL,
is_active INTEGER NOT NULL DEFAULT 1,
display_order INTEGER NOT NULL,
FOREIGN KEY (pillar_code) REFERENCES deva_elevveclinic_bi_radar_pillars(pillar_code)
```

### deva_elevveclinic_bi_radar_clusters

```sql
id TEXT PRIMARY KEY,
cluster_code TEXT UNIQUE NOT NULL,
pillar_code TEXT NOT NULL,
cluster_name TEXT NOT NULL,
activation_threshold INTEGER NOT NULL DEFAULT 2,
minimum_key_symptoms INTEGER NOT NULL DEFAULT 3,
bonus_points REAL NOT NULL DEFAULT 3,
is_active INTEGER NOT NULL DEFAULT 1,
FOREIGN KEY (pillar_code) REFERENCES deva_elevveclinic_bi_radar_pillars(pillar_code)
```

### deva_elevveclinic_bi_radar_cluster_questions

```sql
id TEXT PRIMARY KEY,
cluster_code TEXT NOT NULL,
question_code TEXT NOT NULL,
FOREIGN KEY (cluster_code) REFERENCES deva_elevveclinic_bi_radar_clusters(cluster_code),
FOREIGN KEY (question_code) REFERENCES deva_elevveclinic_bi_radar_questions(question_code)
```

### deva_elevveclinic_bi_radar_responses

```sql
id TEXT PRIMARY KEY,
patient_id TEXT NOT NULL,
question_code TEXT NOT NULL,
response_value INTEGER NOT NULL CHECK (response_value BETWEEN 0 AND 3),
response_date TEXT NOT NULL,
created_at TEXT NOT NULL,
FOREIGN KEY (patient_id) REFERENCES deva_elevveclinic_bi_patients(id),
FOREIGN KEY (question_code) REFERENCES deva_elevveclinic_bi_radar_questions(question_code)
```

### deva_elevveclinic_bi_radar_interpretation_ranges

```sql
id TEXT PRIMARY KEY,
pillar_code TEXT NOT NULL,
label TEXT NOT NULL,
min_score REAL NOT NULL,
max_score REAL,
priority_level INTEGER NOT NULL,
FOREIGN KEY (pillar_code) REFERENCES deva_elevveclinic_bi_radar_pillars(pillar_code)
```

### deva_elevveclinic_bi_radar_score_snapshots

```sql
id TEXT PRIMARY KEY,
patient_id TEXT NOT NULL,
calculated_at TEXT NOT NULL,
pillar_code TEXT NOT NULL,
raw_score REAL NOT NULL,
cluster_bonus REAL NOT NULL,
final_score REAL NOT NULL,
max_score REAL NOT NULL,
risk_score REAL NOT NULL,
balance_score REAL NOT NULL,
classification TEXT,
priority_index REAL NOT NULL,
is_priority_axis INTEGER NOT NULL DEFAULT 0,
details_json TEXT,
FOREIGN KEY (patient_id) REFERENCES deva_elevveclinic_bi_patients(id),
FOREIGN KEY (pillar_code) REFERENCES deva_elevveclinic_bi_radar_pillars(pillar_code)
```

## Observações

- Interpretação deve vir de tabela, não hardcoded.
- Se faixa estiver ausente, backend deve retornar `classification = "Pendente de configuração"`.
- Score visual deve ser derivado, não salvo como única verdade.
- Score bruto/final deve ser preservado.
