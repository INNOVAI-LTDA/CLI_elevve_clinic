-- =========================================================
-- 010 — BI Radar de Longevidade (deva_elevveclinic_bi_*)
-- Schema SQLite. Compatível com DB Browser for SQLite.
-- Fonte de verdade: spec docs/BI_RULE_RADAR_CANONICAL.md
-- e docs/DB_VERTICAL_SLICE_SPEC.md.
-- =========================================================

PRAGMA foreign_keys = ON;

-- Patients (BI Elevve)
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_patients (
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
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_patients_status
  ON deva_elevveclinic_bi_patients (status);

-- Pillars (5 eixos: Hormônios, Intestino, Recuperação, Estrutura, Metabolismo)
-- score_max é o valor informado na spec; o service do commit 3 recalcula
-- max = sum(3*peso) a partir de deva_elevveclinic_bi_radar_questions e
-- usa a soma real como denominador, registrando divergência em log.
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_pillars (
  id TEXT PRIMARY KEY,
  pillar_code TEXT UNIQUE NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  score_max INTEGER,
  display_order INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_pillars_display_order
  ON deva_elevveclinic_bi_radar_pillars (display_order);

-- Questions (perguntas do questionário, 0..3)
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_questions (
  id TEXT PRIMARY KEY,
  question_code TEXT UNIQUE NOT NULL,
  pillar_code TEXT NOT NULL,
  question_text TEXT NOT NULL,
  short_label TEXT NOT NULL,
  weight REAL NOT NULL,
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  display_order INTEGER NOT NULL,
  FOREIGN KEY (pillar_code)
    REFERENCES deva_elevveclinic_bi_radar_pillars (pillar_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_questions_pillar
  ON deva_elevveclinic_bi_radar_questions (pillar_code);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_questions_active
  ON deva_elevveclinic_bi_radar_questions (is_active);

-- Clusters (gatilhos de bônus por pilar)
-- activation_threshold: resposta mínima para considerar o sintoma-chave como "ativo" (default 2)
-- minimum_key_symptoms: quantos sintomas-chave precisam estar ativos para disparar o cluster (default 3)
-- bonus_points: pontos a somar no score_final_eixo se cluster_ativo (default 3)
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_clusters (
  id TEXT PRIMARY KEY,
  cluster_code TEXT UNIQUE NOT NULL,
  pillar_code TEXT NOT NULL,
  cluster_name TEXT NOT NULL,
  activation_threshold INTEGER NOT NULL DEFAULT 2,
  minimum_key_symptoms INTEGER NOT NULL DEFAULT 3,
  bonus_points REAL NOT NULL DEFAULT 3,
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  FOREIGN KEY (pillar_code)
    REFERENCES deva_elevveclinic_bi_radar_pillars (pillar_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_clusters_pillar
  ON deva_elevveclinic_bi_radar_clusters (pillar_code);

-- Cluster <-> Questions (N:N por question_code).
-- UNIQUE em (cluster_code, question_code): mesma pergunta não pode
-- aparecer 2x no mesmo cluster; mas pode aparecer em clusters
-- diferentes (compatível com a observação da spec sobre "acorda cansada"
-- compartilhada entre Hormônios e Recuperação).
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_cluster_questions (
  id TEXT PRIMARY KEY,
  cluster_code TEXT NOT NULL,
  question_code TEXT NOT NULL,
  FOREIGN KEY (cluster_code)
    REFERENCES deva_elevveclinic_bi_radar_clusters (cluster_code)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (question_code)
    REFERENCES deva_elevveclinic_bi_radar_questions (question_code)
    ON UPDATE RESTRICT
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_bi_radar_cluster_questions_pair
  ON deva_elevveclinic_bi_radar_cluster_questions (cluster_code, question_code);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_cluster_questions_question
  ON deva_elevveclinic_bi_radar_cluster_questions (question_code);

-- Responses (respostas 0..3 por paciente por pergunta)
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_responses (
  id TEXT PRIMARY KEY,
  patient_id TEXT NOT NULL,
  question_code TEXT NOT NULL,
  response_value INTEGER NOT NULL CHECK (response_value BETWEEN 0 AND 3),
  response_date TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (patient_id)
    REFERENCES deva_elevveclinic_bi_patients (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (question_code)
    REFERENCES deva_elevveclinic_bi_radar_questions (question_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_responses_patient
  ON deva_elevveclinic_bi_radar_responses (patient_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_responses_question
  ON deva_elevveclinic_bi_radar_responses (question_code);

-- Interpretation Ranges (faixas de classificação por pilar)
-- max_score NULL = "faixa aberta" (ex.: > 22 no Metabolismo).
-- Se um pilar não tem nenhuma linha, classification = "Pendente de configuração".
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_interpretation_ranges (
  id TEXT PRIMARY KEY,
  pillar_code TEXT NOT NULL,
  label TEXT NOT NULL,
  min_score REAL NOT NULL,
  max_score REAL,
  priority_level INTEGER NOT NULL,
  FOREIGN KEY (pillar_code)
    REFERENCES deva_elevveclinic_bi_radar_pillars (pillar_code)
    ON UPDATE RESTRICT
    ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_interpretation_ranges_pillar
  ON deva_elevveclinic_bi_radar_interpretation_ranges (pillar_code);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_interpretation_ranges_pillar_level
  ON deva_elevveclinic_bi_radar_interpretation_ranges (pillar_code, priority_level);

-- Score Snapshots (auditoria do cálculo por paciente por pilar)
-- details_json TEXT: json serializado com pesos, respostas,
-- perguntas-chave batidas, max calculado, etc.
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_score_snapshots (
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
  is_priority_axis INTEGER NOT NULL DEFAULT 0 CHECK (is_priority_axis IN (0, 1)),
  details_json TEXT,
  FOREIGN KEY (patient_id)
    REFERENCES deva_elevveclinic_bi_patients (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (pillar_code)
    REFERENCES deva_elevveclinic_bi_radar_pillars (pillar_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_patient
  ON deva_elevveclinic_bi_radar_score_snapshots (patient_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_pillar
  ON deva_elevveclinic_bi_radar_score_snapshots (pillar_code);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_patient_calc
  ON deva_elevveclinic_bi_radar_score_snapshots (patient_id, calculated_at);
