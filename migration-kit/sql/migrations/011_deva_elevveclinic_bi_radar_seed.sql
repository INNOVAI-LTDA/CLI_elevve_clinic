-- =========================================================
-- 011 — BI Radar de Longevidade — Seed mínimo
-- Insere 5 pilares, 41 perguntas, 3 clusters (Metabolismo,
-- Hormônios, Recuperação), 9 perguntas-chave, faixas só para
-- Metabolismo e Intestino, 2 pacientes demo e 82 respostas
-- (41 por paciente).
--
-- Idempotente via INSERT OR IGNORE. Pacientes são fictícios
-- ("Paciente Demo 01"/"02"); nenhum dado pessoal real.
-- =========================================================

PRAGMA foreign_keys = ON;

-- -------- Pillars (5) --------
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_pillars
  (id, pillar_code, name, description, score_max, display_order)
VALUES
  ('pi_hormonios',   'hormonios',   'Hormônios',
   'Estabilidade hormonal e neuroendócrina.',        36, 1),
  ('pi_intestino',   'intestino',   'Intestino',
   'Digestão, absorção e tolerância alimentar.',      36, 2),
  ('pi_recuperacao', 'recuperacao', 'Recuperação',
   'Capacidade de recuperação e adaptação ao estresse.', 39, 3),
  ('pi_estrutura',   'estrutura',   'Estrutura',
   'Força, resistência e integridade física.',        39, 4),
  ('pi_metabolismo', 'metabolismo', 'Metabolismo',
   'Produção e estabilidade de energia metabólica.',  33, 5);

-- -------- Questions (41) --------
-- Hormônios (8)
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_questions
  (id, question_code, pillar_code, question_text, short_label, weight, is_active, display_order)
VALUES
  ('q_hormonios_01', 'hormonios_01', 'hormonios', 'Insônia',                                 'Insônia',                2.0, 1, 1),
  ('q_hormonios_02', 'hormonios_02', 'hormonios', 'Acorda cansada',                          'Acorda cansada',         2.0, 1, 2),
  ('q_hormonios_03', 'hormonios_03', 'hormonios', 'Oscilação de humor',                      'Oscilação humor',        1.0, 1, 3),
  ('q_hormonios_04', 'hormonios_04', 'hormonios', 'Irritabilidade',                          'Irritabilidade',         1.0, 1, 4),
  ('q_hormonios_05', 'hormonios_05', 'hormonios', 'Queda de libido',                         'Libido',                 2.0, 1, 5),
  ('q_hormonios_06', 'hormonios_06', 'hormonios', 'Calorão/suor',                            'Calorão/suor',           2.0, 1, 6),
  ('q_hormonios_07', 'hormonios_07', 'hormonios', 'Piora de memória/concentração',           'Memória/concentração',   1.0, 1, 7),
  ('q_hormonios_08', 'hormonios_08', 'hormonios', 'Alterações em cabelo/pele',               'Cabelo/pele',            1.0, 1, 8);

-- Intestino (8)
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_questions
  (id, question_code, pillar_code, question_text, short_label, weight, is_active, display_order)
VALUES
  ('q_intestino_01', 'intestino_01', 'intestino', 'Distensão abdominal',                     'Distensão',              2.0, 1, 1),
  ('q_intestino_02', 'intestino_02', 'intestino', 'Gases',                                   'Gases',                  1.0, 1, 2),
  ('q_intestino_03', 'intestino_03', 'intestino', 'Desconforto nas refeições',               'Desconforto refeições',  1.0, 1, 3),
  ('q_intestino_04', 'intestino_04', 'intestino', 'Piora com alimentos',                     'Piora alimentos',        2.0, 1, 4),
  ('q_intestino_05', 'intestino_05', 'intestino', 'Intestino irregular',                     'Intestino irregular',    2.0, 1, 5),
  ('q_intestino_06', 'intestino_06', 'intestino', 'Fadiga pós-alimentar',                    'Fadiga pós alimentar',   2.0, 1, 6),
  ('q_intestino_07', 'intestino_07', 'intestino', 'Constipação',                             'Constipação',            1.0, 1, 7),
  ('q_intestino_08', 'intestino_08', 'intestino', 'Digestão lenta',                          'Digestão lenta',         1.0, 1, 8);

-- Recuperação (9 — inclui recuperacao_extra_01 "acorda cansada", cluster key)
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_questions
  (id, question_code, pillar_code, question_text, short_label, weight, is_active, display_order)
VALUES
  ('q_recuperacao_01',       'recuperacao_01',       'recuperacao', 'Cansaço persistente',              'Cansaço persistente',         2.0, 1, 1),
  ('q_recuperacao_02',       'recuperacao_02',       'recuperacao', 'Energia acaba rápido',             'Energia acaba rápido',        2.0, 1, 2),
  ('q_recuperacao_03',       'recuperacao_03',       'recuperacao', 'Dificuldade de relaxar',            'Dificuldade relaxar',         1.0, 1, 3),
  ('q_recuperacao_04',       'recuperacao_04',       'recuperacao', 'Pequenos problemas esgotam',        'Pequenos problemas esgotam',  2.0, 1, 4),
  ('q_recuperacao_05',       'recuperacao_05',       'recuperacao', 'Piora sob estresse',                'Piora sob estresse',          2.0, 1, 5),
  ('q_recuperacao_06',       'recuperacao_06',       'recuperacao', 'Dependência de cafeína',            'Dependência cafeína',         1.0, 1, 6),
  ('q_recuperacao_07',       'recuperacao_07',       'recuperacao', 'Dores/tensão',                      'Dores/tensão',                1.0, 1, 7),
  ('q_recuperacao_08',       'recuperacao_08',       'recuperacao', 'Recuperação física ruim',           'Recuperação física ruim',     2.0, 1, 8),
  ('q_recuperacao_extra_01', 'recuperacao_extra_01', 'recuperacao', 'Acorda cansada (compartilhada)',    'Acorda cansada',              2.0, 1, 9);

-- Estrutura (8)
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_questions
  (id, question_code, pillar_code, question_text, short_label, weight, is_active, display_order)
VALUES
  ('q_estrutura_01', 'estrutura_01', 'estrutura', 'Perda de força',                          'Perda força',                    2.0, 1, 1),
  ('q_estrutura_02', 'estrutura_02', 'estrutura', 'Flacidez / dificuldade de massa muscular','Flacidez / massa muscular',     1.0, 1, 2),
  ('q_estrutura_03', 'estrutura_03', 'estrutura', 'Dores físicas',                           'Dores físicas',                  1.0, 1, 3),
  ('q_estrutura_04', 'estrutura_04', 'estrutura', 'Retenção/peso nas pernas',                'Retenção/peso pernas',           2.0, 1, 4),
  ('q_estrutura_05', 'estrutura_05', 'estrutura', 'Piora de resistência',                    'Piora resistência',              2.0, 1, 5),
  ('q_estrutura_06', 'estrutura_06', 'estrutura', 'Perda muscular',                          'Perda muscular',                 2.0, 1, 6),
  ('q_estrutura_07', 'estrutura_07', 'estrutura', 'Limitação física',                        'Limitação física',               2.0, 1, 7),
  ('q_estrutura_08', 'estrutura_08', 'estrutura', 'Piora de composição corporal',            'Piora composição corporal',      1.0, 1, 8);

-- Metabolismo (8)
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_questions
  (id, question_code, pillar_code, question_text, short_label, weight, is_active, display_order)
VALUES
  ('q_metabolismo_01', 'metabolismo_01', 'metabolismo', 'Dificuldade emagrecer',                'Dificuldade emagrecer',          2.0, 1, 1),
  ('q_metabolismo_02', 'metabolismo_02', 'metabolismo', 'Fome rápida',                          'Fome rápida',                    1.0, 1, 2),
  ('q_metabolismo_03', 'metabolismo_03', 'metabolismo', 'Vontade de doces',                     'Vontade doces',                  1.0, 1, 3),
  ('q_metabolismo_04', 'metabolismo_04', 'metabolismo', 'Queda energia / sono pós-refeição',    'Queda energia pós-refeição',     1.0, 1, 4),
  ('q_metabolismo_05', 'metabolismo_05', 'metabolismo', 'Sonolência pós-refeição',              'Sonolência pós-refeição',        2.0, 1, 5),
  ('q_metabolismo_06', 'metabolismo_06', 'metabolismo', 'Gordura abdominal',                    'Gordura abdominal',              2.0, 1, 6),
  ('q_metabolismo_07', 'metabolismo_07', 'metabolismo', 'Dificuldade manter peso',              'Dificuldade manter peso',        1.0, 1, 7),
  ('q_metabolismo_08', 'metabolismo_08', 'metabolismo', 'Piora de concentração',                'Piora concentração',             1.0, 1, 8);

-- -------- Clusters (3, apenas onde a spec define) --------
-- Intestino e Estrutura NÃO têm cluster (spec: "Cluster não explicitado
-- no material bruto. Não inventar. Deixar configurável.").
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_clusters
  (id, cluster_code, pillar_code, cluster_name, activation_threshold, minimum_key_symptoms, bonus_points, is_active)
VALUES
  ('cl_metabolismo', 'cl_metabolismo', 'metabolismo', 'Cluster Metabólico',  2, 3, 3.0, 1),
  ('cl_hormonios',   'cl_hormonios',   'hormonios',   'Cluster Hormonal',    2, 3, 3.0, 1),
  ('cl_recuperacao', 'cl_recuperacao', 'recuperacao', 'Cluster Recuperação', 2, 3, 3.0, 1);

-- -------- Cluster <-> Questions (9) --------
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_cluster_questions
  (id, cluster_code, question_code)
VALUES
  ('cq_mb_01', 'cl_metabolismo', 'metabolismo_01'),
  ('cq_mb_05', 'cl_metabolismo', 'metabolismo_05'),
  ('cq_mb_06', 'cl_metabolismo', 'metabolismo_06'),
  ('cq_hr_05', 'cl_hormonios',   'hormonios_05'),
  ('cq_hr_06', 'cl_hormonios',   'hormonios_06'),
  ('cq_hr_08', 'cl_hormonios',   'hormonios_08'),
  ('cq_rc_02', 'cl_recuperacao', 'recuperacao_02'),
  ('cq_rc_06', 'cl_recuperacao', 'recuperacao_06'),
  ('cq_rc_x1', 'cl_recuperacao', 'recuperacao_extra_01');

-- -------- Interpretation Ranges (apenas Metabolismo e Intestino) --------
-- Hormônios, Recuperação e Estrutura ficam SEM ranges para acionar
-- "Pendente de configuração" no service.
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_interpretation_ranges
  (id, pillar_code, label, min_score, max_score, priority_level)
VALUES
  -- Metabolismo (spec: 0–7 / 8–14 / 15–22 / > 22)
  ('ir_metabolismo_01', 'metabolismo', 'Estável',                 0.0,  7.0, 1),
  ('ir_metabolismo_02', 'metabolismo', 'Atenção funcional',       8.0, 14.0, 2),
  ('ir_metabolismo_03', 'metabolismo', 'Desregulação importante', 15.0, 22.0, 3),
  ('ir_metabolismo_04', 'metabolismo', 'Eixo prioritário',        23.0, NULL, 4),
  -- Intestino (spec: 0–8 / 9–16 / 17–24 / > 24)
  ('ir_intestino_01',   'intestino',   'Estável',                 0.0,  8.0, 1),
  ('ir_intestino_02',   'intestino',   'Atenção funcional',       9.0, 16.0, 2),
  ('ir_intestino_03',   'intestino',   'Desregulação importante', 17.0, 24.0, 3),
  ('ir_intestino_04',   'intestino',   'Eixo prioritário',        25.0, NULL, 4);

-- -------- Patients (2, fictícios) --------
INSERT OR IGNORE INTO deva_elevveclinic_bi_patients
  (id, patient_code, name, sex, birth_date, chronological_age, program_name, program_start_date, program_end_date, status)
VALUES
  ('bi_pat_demo_01', 'BI-DEMO-01', 'Paciente Demo 01', 'F', '1980-05-12', 46, 'Mentoria Acelerador Medico', '2025-09-01', NULL, 'active'),
  ('bi_pat_demo_02', 'BI-DEMO-02', 'Paciente Demo 02', 'F', '1975-09-23', 50, 'Mentoria Acelerador Medico', '2025-08-15', NULL, 'active');

-- -------- Responses (82 = 41 × 2) --------
-- Paciente 01: Metabolismo cluster ATIVO, Intestino em faixa alta,
-- Recuperação sem cluster, Hormônios/Estrutura com classificação
-- "Pendente de configuração".
-- Paciente 02: Recuperação cluster ATIVO, Hormônios cluster ATIVO
-- (mas sem range → "Pendente de configuração"), Metabolismo/Intestino
-- em faixa intermediária, Estrutura sem range.

-- Paciente Demo 01
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_responses
  (id, patient_id, question_code, response_value, response_date)
VALUES
  -- Metabolismo (3 sintomas-chave em resposta 3 → cluster_ativo=true)
  ('r_01_metabolismo_01', 'bi_pat_demo_01', 'metabolismo_01', 3, '2026-05-10'),
  ('r_01_metabolismo_02', 'bi_pat_demo_01', 'metabolismo_02', 1, '2026-05-10'),
  ('r_01_metabolismo_03', 'bi_pat_demo_01', 'metabolismo_03', 2, '2026-05-10'),
  ('r_01_metabolismo_04', 'bi_pat_demo_01', 'metabolismo_04', 2, '2026-05-10'),
  ('r_01_metabolismo_05', 'bi_pat_demo_01', 'metabolismo_05', 3, '2026-05-10'),
  ('r_01_metabolismo_06', 'bi_pat_demo_01', 'metabolismo_06', 3, '2026-05-10'),
  ('r_01_metabolismo_07', 'bi_pat_demo_01', 'metabolismo_07', 2, '2026-05-10'),
  ('r_01_metabolismo_08', 'bi_pat_demo_01', 'metabolismo_08', 1, '2026-05-10'),
  -- Intestino (sem cluster; raw 19 → "Desregulação importante")
  ('r_01_intestino_01', 'bi_pat_demo_01', 'intestino_01', 2, '2026-05-10'),
  ('r_01_intestino_02', 'bi_pat_demo_01', 'intestino_02', 2, '2026-05-10'),
  ('r_01_intestino_03', 'bi_pat_demo_01', 'intestino_03', 1, '2026-05-10'),
  ('r_01_intestino_04', 'bi_pat_demo_01', 'intestino_04', 2, '2026-05-10'),
  ('r_01_intestino_05', 'bi_pat_demo_01', 'intestino_05', 2, '2026-05-10'),
  ('r_01_intestino_06', 'bi_pat_demo_01', 'intestino_06', 1, '2026-05-10'),
  ('r_01_intestino_07', 'bi_pat_demo_01', 'intestino_07', 1, '2026-05-10'),
  ('r_01_intestino_08', 'bi_pat_demo_01', 'intestino_08', 1, '2026-05-10'),
  -- Hormônios (apenas 2 chaves ≥ 2 → cluster_ativo=false; "Pendente de configuração")
  ('r_01_hormonios_01', 'bi_pat_demo_01', 'hormonios_01', 2, '2026-05-10'),
  ('r_01_hormonios_02', 'bi_pat_demo_01', 'hormonios_02', 2, '2026-05-10'),
  ('r_01_hormonios_03', 'bi_pat_demo_01', 'hormonios_03', 2, '2026-05-10'),
  ('r_01_hormonios_04', 'bi_pat_demo_01', 'hormonios_04', 1, '2026-05-10'),
  ('r_01_hormonios_05', 'bi_pat_demo_01', 'hormonios_05', 1, '2026-05-10'),
  ('r_01_hormonios_06', 'bi_pat_demo_01', 'hormonios_06', 2, '2026-05-10'),
  ('r_01_hormonios_07', 'bi_pat_demo_01', 'hormonios_07', 1, '2026-05-10'),
  ('r_01_hormonios_08', 'bi_pat_demo_01', 'hormonios_08', 2, '2026-05-10'),
  -- Recuperação (apenas 2 chaves ≥ 2 → cluster_ativo=false; "Pendente de configuração")
  ('r_01_recuperacao_01',       'bi_pat_demo_01', 'recuperacao_01',       1, '2026-05-10'),
  ('r_01_recuperacao_02',       'bi_pat_demo_01', 'recuperacao_02',       2, '2026-05-10'),
  ('r_01_recuperacao_03',       'bi_pat_demo_01', 'recuperacao_03',       1, '2026-05-10'),
  ('r_01_recuperacao_04',       'bi_pat_demo_01', 'recuperacao_04',       1, '2026-05-10'),
  ('r_01_recuperacao_05',       'bi_pat_demo_01', 'recuperacao_05',       2, '2026-05-10'),
  ('r_01_recuperacao_06',       'bi_pat_demo_01', 'recuperacao_06',       2, '2026-05-10'),
  ('r_01_recuperacao_07',       'bi_pat_demo_01', 'recuperacao_07',       1, '2026-05-10'),
  ('r_01_recuperacao_08',       'bi_pat_demo_01', 'recuperacao_08',       1, '2026-05-10'),
  ('r_01_recuperacao_extra_01', 'bi_pat_demo_01', 'recuperacao_extra_01', 1, '2026-05-10'),
  -- Estrutura (sem cluster; "Pendente de configuração")
  ('r_01_estrutura_01', 'bi_pat_demo_01', 'estrutura_01', 1, '2026-05-10'),
  ('r_01_estrutura_02', 'bi_pat_demo_01', 'estrutura_02', 1, '2026-05-10'),
  ('r_01_estrutura_03', 'bi_pat_demo_01', 'estrutura_03', 2, '2026-05-10'),
  ('r_01_estrutura_04', 'bi_pat_demo_01', 'estrutura_04', 1, '2026-05-10'),
  ('r_01_estrutura_05', 'bi_pat_demo_01', 'estrutura_05', 1, '2026-05-10'),
  ('r_01_estrutura_06', 'bi_pat_demo_01', 'estrutura_06', 1, '2026-05-10'),
  ('r_01_estrutura_07', 'bi_pat_demo_01', 'estrutura_07', 1, '2026-05-10'),
  ('r_01_estrutura_08', 'bi_pat_demo_01', 'estrutura_08', 2, '2026-05-10');

-- Paciente Demo 02
INSERT OR IGNORE INTO deva_elevveclinic_bi_radar_responses
  (id, patient_id, question_code, response_value, response_date)
VALUES
  -- Metabolismo (todas 1 → cluster_ativo=false; raw 11 → "Atenção funcional")
  ('r_02_metabolismo_01', 'bi_pat_demo_02', 'metabolismo_01', 1, '2026-05-10'),
  ('r_02_metabolismo_02', 'bi_pat_demo_02', 'metabolismo_02', 1, '2026-05-10'),
  ('r_02_metabolismo_03', 'bi_pat_demo_02', 'metabolismo_03', 1, '2026-05-10'),
  ('r_02_metabolismo_04', 'bi_pat_demo_02', 'metabolismo_04', 1, '2026-05-10'),
  ('r_02_metabolismo_05', 'bi_pat_demo_02', 'metabolismo_05', 1, '2026-05-10'),
  ('r_02_metabolismo_06', 'bi_pat_demo_02', 'metabolismo_06', 1, '2026-05-10'),
  ('r_02_metabolismo_07', 'bi_pat_demo_02', 'metabolismo_07', 1, '2026-05-10'),
  ('r_02_metabolismo_08', 'bi_pat_demo_02', 'metabolismo_08', 1, '2026-05-10'),
  -- Intestino (todas 1 → cluster_ativo=false; raw 12 → "Atenção funcional")
  ('r_02_intestino_01', 'bi_pat_demo_02', 'intestino_01', 1, '2026-05-10'),
  ('r_02_intestino_02', 'bi_pat_demo_02', 'intestino_02', 1, '2026-05-10'),
  ('r_02_intestino_03', 'bi_pat_demo_02', 'intestino_03', 1, '2026-05-10'),
  ('r_02_intestino_04', 'bi_pat_demo_02', 'intestino_04', 1, '2026-05-10'),
  ('r_02_intestino_05', 'bi_pat_demo_02', 'intestino_05', 1, '2026-05-10'),
  ('r_02_intestino_06', 'bi_pat_demo_02', 'intestino_06', 1, '2026-05-10'),
  ('r_02_intestino_07', 'bi_pat_demo_02', 'intestino_07', 1, '2026-05-10'),
  ('r_02_intestino_08', 'bi_pat_demo_02', 'intestino_08', 1, '2026-05-10'),
  -- Hormônios (3 chaves em 2 → cluster_ativo=true; "Pendente de configuração" porque sem range)
  ('r_02_hormonios_01', 'bi_pat_demo_02', 'hormonios_01', 1, '2026-05-10'),
  ('r_02_hormonios_02', 'bi_pat_demo_02', 'hormonios_02', 2, '2026-05-10'),
  ('r_02_hormonios_03', 'bi_pat_demo_02', 'hormonios_03', 1, '2026-05-10'),
  ('r_02_hormonios_04', 'bi_pat_demo_02', 'hormonios_04', 1, '2026-05-10'),
  ('r_02_hormonios_05', 'bi_pat_demo_02', 'hormonios_05', 2, '2026-05-10'),
  ('r_02_hormonios_06', 'bi_pat_demo_02', 'hormonios_06', 2, '2026-05-10'),
  ('r_02_hormonios_07', 'bi_pat_demo_02', 'hormonios_07', 1, '2026-05-10'),
  ('r_02_hormonios_08', 'bi_pat_demo_02', 'hormonios_08', 2, '2026-05-10'),
  -- Recuperação (3 chaves em ≥2 → cluster_ativo=true; "Pendente de configuração")
  ('r_02_recuperacao_01',       'bi_pat_demo_02', 'recuperacao_01',       1, '2026-05-10'),
  ('r_02_recuperacao_02',       'bi_pat_demo_02', 'recuperacao_02',       3, '2026-05-10'),
  ('r_02_recuperacao_03',       'bi_pat_demo_02', 'recuperacao_03',       1, '2026-05-10'),
  ('r_02_recuperacao_04',       'bi_pat_demo_02', 'recuperacao_04',       1, '2026-05-10'),
  ('r_02_recuperacao_05',       'bi_pat_demo_02', 'recuperacao_05',       1, '2026-05-10'),
  ('r_02_recuperacao_06',       'bi_pat_demo_02', 'recuperacao_06',       2, '2026-05-10'),
  ('r_02_recuperacao_07',       'bi_pat_demo_02', 'recuperacao_07',       1, '2026-05-10'),
  ('r_02_recuperacao_08',       'bi_pat_demo_02', 'recuperacao_08',       1, '2026-05-10'),
  ('r_02_recuperacao_extra_01', 'bi_pat_demo_02', 'recuperacao_extra_01', 2, '2026-05-10'),
  -- Estrutura (sem cluster; "Pendente de configuração")
  ('r_02_estrutura_01', 'bi_pat_demo_02', 'estrutura_01', 1, '2026-05-10'),
  ('r_02_estrutura_02', 'bi_pat_demo_02', 'estrutura_02', 1, '2026-05-10'),
  ('r_02_estrutura_03', 'bi_pat_demo_02', 'estrutura_03', 1, '2026-05-10'),
  ('r_02_estrutura_04', 'bi_pat_demo_02', 'estrutura_04', 1, '2026-05-10'),
  ('r_02_estrutura_05', 'bi_pat_demo_02', 'estrutura_05', 1, '2026-05-10'),
  ('r_02_estrutura_06', 'bi_pat_demo_02', 'estrutura_06', 1, '2026-05-10'),
  ('r_02_estrutura_07', 'bi_pat_demo_02', 'estrutura_07', 1, '2026-05-10'),
  ('r_02_estrutura_08', 'bi_pat_demo_02', 'estrutura_08', 1, '2026-05-10');

-- Sanidade pos-seed (queries de leitura, não alteram estado)
SELECT
  pillar_code,
  (SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_questions q WHERE q.pillar_code = p.pillar_code AND q.is_active = 1) AS questions,
  (SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_clusters c WHERE c.pillar_code = p.pillar_code AND c.is_active = 1) AS clusters,
  (SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_interpretation_ranges r WHERE r.pillar_code = p.pillar_code) AS ranges
FROM deva_elevveclinic_bi_radar_pillars p
ORDER BY p.display_order;

SELECT patient_id, COUNT(*) AS total_responses
FROM deva_elevveclinic_bi_radar_responses
GROUP BY patient_id;
