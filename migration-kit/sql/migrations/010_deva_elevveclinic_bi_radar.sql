-- =========================================================
-- 010 — BI Radar de Longevidade (deva_elevveclinic_bi_*)
-- Schema SQLite. Compatível com DB Browser for SQLite.
-- Fonte de verdade: spec docs/BI_RULE_RADAR_CANONICAL.md
-- e docs/DB_VERTICAL_SLICE_SPEC.md.
--
-- F1 refactor: clientes do BI sao Users com role='client' em
-- deva_elevveclinic_users (sem tabela dedicada bi_patients).
-- A coluna user_id e' INTEGER com FK para users.id.
--
-- IMPORTANTE: o F1 e' destrutivo para as tabelas que mudaram
-- (bi_patients, bi_radar_responses, bi_radar_score_snapshots).
-- Como estamos em dev (branch deva), a migration 010 faz DROP
-- explícito dessas 3 tabelas antes de recria-las. Tabelas
-- inalteradas (pillars, questions, clusters, cluster_questions,
-- interpretation_ranges) usam CREATE TABLE IF NOT EXISTS e
-- preservam dados existentes.
-- =========================================================

-- F1 refactor: drop das tabelas que mudaram de schema
DROP TABLE IF EXISTS deva_elevveclinic_bi_patients;
DROP TABLE IF EXISTS deva_elevveclinic_bi_radar_responses;
DROP TABLE IF EXISTS deva_elevveclinic_bi_radar_score_snapshots;

-- F1 refactor: normaliza nomes de colunas (versoes antigas do F1
-- tinham 'pillar_code' com i acentuado; canonical passa a ser 'pillar_code'
-- ASCII para casar com o codigo Python e o smoke test).
ALTER TABLE deva_elevveclinic_bi_radar_pillars          RENAME COLUMN pillar_code TO pillar_code;
ALTER TABLE deva_elevveclinic_bi_radar_questions       RENAME COLUMN pillar_code TO pillar_code;
ALTER TABLE deva_elevveclinic_bi_radar_clusters        RENAME COLUMN pillar_code TO pillar_code;
ALTER TABLE deva_elevveclinic_bi_radar_interpretation_ranges RENAME COLUMN pillar_code TO pillar_code;
-- (cluster_questions e responses ja foram droppados)

PRAGMA foreign_keys = ON;

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

-- Responses (respostas 0..3 por user por pergunta)
-- F1 refactor: user_id INTEGER, FK para deva_elevveclinic_users(id)
-- (em vez de TEXT + tabela bi_patients separada).
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_responses (
  id TEXT PRIMARY KEY,
  user_id INTEGER NOT NULL,
  question_code TEXT NOT NULL,
  response_value INTEGER NOT NULL CHECK (response_value BETWEEN 0 AND 3),
  response_date TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (user_id)
    REFERENCES deva_elevveclinic_users (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (question_code)
    REFERENCES deva_elevveclinic_bi_radar_questions (question_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_responses_user
  ON deva_elevveclinic_bi_radar_responses (user_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_responses_question
  ON deva_elevveclinic_bi_radar_responses (question_code);

-- Interpretation Ranges (faixas de classificação por pilar)
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

-- Score Snapshots (auditoria do cálculo por user por pilar)
-- F1 refactor: user_id INTEGER, FK para users.
CREATE TABLE IF NOT EXISTS deva_elevveclinic_bi_radar_score_snapshots (
  id TEXT PRIMARY KEY,
  user_id INTEGER NOT NULL,
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
  FOREIGN KEY (user_id)
    REFERENCES deva_elevveclinic_users (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (pillar_code)
    REFERENCES deva_elevveclinic_bi_radar_pillars (pillar_code)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_user
  ON deva_elevveclinic_bi_radar_score_snapshots (user_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_pillar
  ON deva_elevveclinic_bi_radar_score_snapshots (pillar_code);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_bi_radar_score_snapshots_user_calc
  ON deva_elevveclinic_bi_radar_score_snapshots (user_id, calculated_at);
