PRAGMA foreign_keys = ON;

-- =========================================================
-- SQLite schema for elevve_clinic_deva_db.db
-- Compatible with DB Browser for SQLite
-- =========================================================

-- Organizations
CREATE TABLE IF NOT EXISTS deva_elevveclinic_organizations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  brand_name TEXT NOT NULL,
  slug TEXT NOT NULL,
  cnpj TEXT,
  timezone TEXT NOT NULL DEFAULT 'America/Sao_Paulo',
  currency TEXT NOT NULL DEFAULT 'BRL',
  status TEXT NOT NULL DEFAULT 'active',
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  notes TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_organizations_slug_idx
  ON deva_elevveclinic_organizations (slug);

-- Users (admin / provider / client)
CREATE TABLE IF NOT EXISTS deva_elevveclinic_users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT 'client' CHECK (role IN ('admin', 'provider', 'client')),
  full_name TEXT NOT NULL,
  phone TEXT CHECK (
    phone IS NULL OR (
      phone GLOB '+[0-9]*'
      AND length(phone) BETWEEN 12 AND 15
    )
  ),
  birth_date TEXT CHECK (
    birth_date IS NULL OR birth_date GLOB '[0-3][0-9]/[0-1][0-9]/[1-2][0-9][0-9][0-9]'
  ),
  city TEXT,
  labels TEXT,
  cpf TEXT CHECK (
    cpf IS NULL OR (
      length(cpf) = 11
      AND cpf GLOB '[0-9]*'
    )
  ),
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  organization_id INTEGER,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (organization_id)
    REFERENCES deva_elevveclinic_organizations (id)
    ON UPDATE RESTRICT
    ON DELETE SET NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_users_email_lower_idx
  ON deva_elevveclinic_users (LOWER(email));

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_users_organization_id
  ON deva_elevveclinic_users (organization_id);

-- Products
CREATE TABLE IF NOT EXISTS deva_elevveclinic_products (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  organization_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  category TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'active',
  description TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (organization_id)
    REFERENCES deva_elevveclinic_organizations (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_products_slug_idx
  ON deva_elevveclinic_products (slug);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_products_organization_id
  ON deva_elevveclinic_products (organization_id);

-- Enrollments
CREATE TABLE IF NOT EXISTS deva_elevveclinic_enrollments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  provider_user_id INTEGER NOT NULL,
  client_user_id INTEGER NOT NULL,
  product_id INTEGER NOT NULL,
  start_day TEXT,
  days_left INTEGER CHECK (days_left IS NULL OR days_left >= 0),
  investment REAL,
  decision_matrix_status TEXT,
  status TEXT NOT NULL DEFAULT 'active' CHECK (
    status IN (
      'active',
      'finished',
      'suspended',
      'canceled by client',
      'canceled by organization'
    )
  ),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  CHECK (provider_user_id <> client_user_id),
  FOREIGN KEY (provider_user_id)
    REFERENCES deva_elevveclinic_users (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT,
  FOREIGN KEY (client_user_id)
    REFERENCES deva_elevveclinic_users (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT,
  FOREIGN KEY (product_id)
    REFERENCES deva_elevveclinic_products (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_enrollments_triplet_idx
  ON deva_elevveclinic_enrollments (provider_user_id, client_user_id, product_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_enrollments_provider_user_id
  ON deva_elevveclinic_enrollments (provider_user_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_enrollments_client_user_id
  ON deva_elevveclinic_enrollments (client_user_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_enrollments_product_id
  ON deva_elevveclinic_enrollments (product_id);

-- Product Pillars
CREATE TABLE IF NOT EXISTS deva_elevveclinic_product_pillars (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  product_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  order_index INTEGER NOT NULL,
  metadata TEXT,
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  FOREIGN KEY (product_id)
    REFERENCES deva_elevveclinic_products (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_product_pillars_product_slug_idx
  ON deva_elevveclinic_product_pillars (product_id, slug);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_product_pillars_product_id
  ON deva_elevveclinic_product_pillars (product_id);

-- Product Metrics
CREATE TABLE IF NOT EXISTS deva_elevveclinic_product_metrics (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pillar_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  slug TEXT NOT NULL,
  direction TEXT NOT NULL,
  unit TEXT,
  scoring_rules TEXT NOT NULL,
  score_type TEXT NOT NULL,
  min_score REAL,
  max_score REAL,
  max_score_basis TEXT,
  mcv REAL,
  is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (pillar_id)
    REFERENCES deva_elevveclinic_product_pillars (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_product_metrics_pillar_slug_idx
  ON deva_elevveclinic_product_metrics (pillar_id, slug);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_product_metrics_pillar_id
  ON deva_elevveclinic_product_metrics (pillar_id);

-- Runtime Measurements
CREATE TABLE IF NOT EXISTS deva_elevveclinic_measurements (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  enrollment_id INTEGER NOT NULL,
  metric_id INTEGER NOT NULL,
  value_baseline REAL,
  value_current REAL NOT NULL DEFAULT 0,
  value_projected REAL,
  improving_trend INTEGER CHECK (improving_trend IN (0, 1) OR improving_trend IS NULL),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (enrollment_id)
    REFERENCES deva_elevveclinic_enrollments (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE,
  FOREIGN KEY (metric_id)
    REFERENCES deva_elevveclinic_product_metrics (id)
    ON UPDATE RESTRICT
    ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_measurements_enrollment_id
  ON deva_elevveclinic_measurements (enrollment_id);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_measurements_metric_id
  ON deva_elevveclinic_measurements (metric_id);

-- Runtime Checkpoints
CREATE TABLE IF NOT EXISTS deva_elevveclinic_checkpoints (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  enrollment_id INTEGER NOT NULL,
  week INTEGER NOT NULL CHECK (week >= 0),
  status TEXT NOT NULL CHECK (status IN ('green', 'yellow', 'red')),
  label TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  FOREIGN KEY (enrollment_id)
    REFERENCES deva_elevveclinic_enrollments (id)
    ON UPDATE RESTRICT
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_deva_elevveclinic_checkpoints_enrollment_week_idx
  ON deva_elevveclinic_checkpoints (enrollment_id, week);

CREATE INDEX IF NOT EXISTS ix_deva_elevveclinic_checkpoints_enrollment_id
  ON deva_elevveclinic_checkpoints (enrollment_id);

-- =========================================================
-- BI Elevve — Radar de Longevidade
-- Schema versionado em migrations/010_deva_elevveclinic_bi_radar.sql.
-- Bloco replicado aqui para coerência com o snapshot SQLite.
-- =========================================================

PRAGMA foreign_keys = ON;

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
