"""
Smoke test do Commit 2 (atualizado em F1) - BI Radar de
Longevidade (deva_elevveclinic_bi_*).

F1 refactor:
- Clientes do BI sao User de tipo Client em deva_elevveclinic_users
  (sem tabela bi_patients).
- IDs demo: 9001 (Paciente Demo 01) e 9002 (Paciente Demo 02).
- Coluna user_id INTEGER com FK para users.

Aplica as migrations 010/011 (idempotente), roda 14 checks de
schema/seed/sanidade, valida os 5 runtime-stores JSON, calcula os
scores esperados para os 2 clientes demo e verifica idempotencia
do seed.

Variaveis de ambiente:
  BI_RADAR_DB              caminho do SQLite de referencia
  BI_RADAR_MIGRATIONS      diretorio das migrations
  BI_RADAR_RUNTIME_STORES  diretorio dos runtime-stores JSON
  BI_RADAR_SKIP_APPLY      "1" para pular a aplicacao das migrations

Exit codes:
  0  = todos os checks passaram
  1  = algum check falhou
  2  = erro de setup (DB nao encontrado, etc.)
"""

import os
import sqlite3
import json
import sys


# F1 refactor: IDs altos para nao colidir com usuarios reais.
USER_DEMO_01 = 9001
USER_DEMO_02 = 9002


def main():
    db = os.environ['BI_RADAR_DB']
    migrations = os.environ['BI_RADAR_MIGRATIONS']
    runtime = os.environ['BI_RADAR_RUNTIME_STORES']
    skip_apply = os.environ.get('BI_RADAR_SKIP_APPLY', '0') == '1'

    failures = []

    def header(label):
        print()
        print('[' + label + ']')

    def check(label, cond, detail=''):
        status = 'OK  ' if cond else 'FAIL'
        line = '  [' + status + '] ' + label
        if detail:
            line += ' (' + detail + ')'
        print(line)
        if not cond:
            failures.append(label)

    def banner(s):
        print()
        print('=' * 70)
        print(' ' + s)
        print('=' * 70)

    banner('BI Radar de Longevidade - Smoke test (F1 refactor)')

    print('  DB:                ' + db)
    print('  Migrations dir:    ' + migrations)
    print('  Runtime-stores:    ' + runtime)
    print('  Apply migrations:  ' + ('no' if skip_apply else 'yes'))
    print('  Demo users:        ' + str(USER_DEMO_01) + ', ' + str(USER_DEMO_02))

    if not os.path.exists(db):
        print()
        print('ERRO: DB nao encontrado em ' + db, file=sys.stderr)
        sys.exit(2)

    conn = sqlite3.connect(db)
    conn.execute('PRAGMA foreign_keys = ON')
    cur = conn.cursor()

    # ---------- 1) Apply migrations ----------
    if not skip_apply:
        header('1) Applying 010 + 011')
        with open(os.path.join(migrations, '010_deva_elevveclinic_bi_radar.sql'), encoding='utf-8') as f:
            conn.executescript(f.read())
        with open(os.path.join(migrations, '011_deva_elevveclinic_bi_radar_seed.sql'), encoding='utf-8') as f:
            conn.executescript(f.read())
        conn.commit()
        print('  applied')

    # ---------- 2) Schema checks ----------
    header('2) Schema')

    bi_tables = [r[0] for r in cur.execute(
        "SELECT name FROM sqlite_master "
        "WHERE type='table' AND name LIKE 'deva_elevveclinic_bi_%' "
        "ORDER BY name"
    ).fetchall()]
    # F1: 7 tabelas (sem bi_patients). Antes era 8.
    check('7 BI tables exist', len(bi_tables) == 7, 'got ' + str(len(bi_tables)))
    for t in bi_tables:
        print('       - ' + t)

    fk = cur.execute('PRAGMA foreign_keys').fetchone()[0]
    check('PRAGMA foreign_keys = ON', fk == 1, 'got ' + str(fk))

    # F1: nao deve existir mais a tabela bi_patients
    bi_patients_exists = cur.execute(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='deva_elevveclinic_bi_patients'"
    ).fetchone()[0]
    check('bi_patients table dropped (F1)', bi_patients_exists == 0,
          'still present' if bi_patients_exists else '')

    # ---------- 3) Seed counts ----------
    header('3) Seed counts')

    n_pillars   = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_pillars').fetchone()[0]
    n_questions = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_questions').fetchone()[0]
    n_clusters  = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_clusters').fetchone()[0]
    n_cq        = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_cluster_questions').fetchone()[0]
    n_ranges    = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_interpretation_ranges').fetchone()[0]
    # F1: clientes do BI vivem em deva_elevveclinic_users com role='client'
    n_clients   = cur.execute(
        "SELECT COUNT(*) FROM deva_elevveclinic_users "
        "WHERE id IN (?, ?) AND role='client'", (USER_DEMO_01, USER_DEMO_02)
    ).fetchone()[0]
    n_responses = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_responses').fetchone()[0]

    check('5 pillars',            n_pillars   == 5,  'got ' + str(n_pillars))
    check('41 questions',         n_questions == 41, 'got ' + str(n_questions))
    check('3 clusters',           n_clusters  == 3,  'got ' + str(n_clusters))
    check('9 cluster_questions',  n_cq        == 9,  'got ' + str(n_cq))
    check('8 ranges',             n_ranges    == 8,  'got ' + str(n_ranges))
    check('2 demo clients (Users with role=client)',
          n_clients == 2, 'got ' + str(n_clients))
    check('82 responses',         n_responses == 82, 'got ' + str(n_responses))

    print('       questions per pillar:')
    for r in cur.execute(
        'SELECT pillar_code, COUNT(*) FROM deva_elevveclinic_bi_radar_questions '
        "WHERE is_active=1 GROUP BY pillar_code ORDER BY pillar_code"
    ):
        print('         - ' + r[0] + ': ' + str(r[1]))

    ranges_by_pillar = [r[0] for r in cur.execute(
        'SELECT DISTINCT pillar_code FROM deva_elevveclinic_bi_radar_interpretation_ranges '
        'ORDER BY pillar_code'
    ).fetchall()]
    check('ranges so em Metabolismo e Intestino',
          ranges_by_pillar == ['intestino', 'metabolismo'],
          'got ' + str(ranges_by_pillar))

    viol = cur.execute(
        'SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_responses '
        'WHERE response_value < 0 OR response_value > 3'
    ).fetchone()[0]
    check('CHECK response_value 0..3', viol == 0, 'violations=' + str(viol))

    # FK responses.user_id -> users.id funcionando
    fk_viol = cur.execute(
        "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_responses r "
        "LEFT JOIN deva_elevveclinic_users u ON u.id = r.user_id "
        "WHERE u.id IS NULL"
    ).fetchone()[0]
    check('FK responses.user_id -> users.id', fk_viol == 0,
          'orphan responses=' + str(fk_viol))

    # ---------- 4) JSON files ----------
    header('4) Runtime-stores JSON')

    for n in ['pillars', 'questions', 'clusters', 'responses', 'interpretation_ranges']:
        path = os.path.join(runtime, 'bi_radar_' + n + '.json')
        try:
            with open(path, encoding='utf-8') as f:
                d = json.load(f)
            items = d.get('items') or d.get('patients', [])
            check('json bi_radar_' + n + '.json',
                  d.get('version') == 1 and len(items) > 0,
                  'version=' + str(d.get('version')) + ' items=' + str(len(items)))
        except Exception as e:
            check('json bi_radar_' + n + '.json', False, str(e))

    # ---------- 5) Expected scores ----------
    header('5) Expected scores (BI rule)')

    weights = {q: w for q, w in cur.execute(
        'SELECT question_code, weight FROM deva_elevveclinic_bi_radar_questions'
    )}
    ranges = list(cur.execute(
        'SELECT pillar_code, min_score, max_score, label '
        'FROM deva_elevveclinic_bi_radar_interpretation_ranges '
        'ORDER BY pillar_code, priority_level'
    ))
    cluster_keys = {}
    for c, q in cur.execute(
        'SELECT cluster_code, question_code FROM deva_elevveclinic_bi_radar_cluster_questions'
    ):
        cluster_keys.setdefault(c, []).append(q)
    cluster_thresh = {c: (t, m) for c, t, m in cur.execute(
        'SELECT cluster_code, activation_threshold, minimum_key_symptoms '
        'FROM deva_elevveclinic_bi_radar_clusters'
    )}

    def classify(raw, p):
        for pp, mn, mx, lbl in ranges:
            if pp == p and raw >= mn and (mx is None or raw <= mx):
                return lbl
        return 'Pendente de configuracao'

    # F1: iterar por user_id INTEGER (em vez de patient_id TEXT)
    for uid in (USER_DEMO_01, USER_DEMO_02):
        resp = dict(cur.execute(
            'SELECT question_code, response_value FROM deva_elevveclinic_bi_radar_responses '
            'WHERE user_id = ?', (uid,)
        ).fetchall())
        print('  user ' + str(uid) + ':')

        # 1o pass: calcula tudo
        rows = []
        for p in ['metabolismo', 'intestino', 'hormonios', 'recuperacao', 'estrutura']:
            qs = [q for q in weights if q.startswith(p + '_')]
            raw = sum(resp.get(q, 0) * weights[q] for q in qs)
            mx = sum(3 * weights[q] for q in qs)
            cl = 'cl_' + p
            thr, mn = cluster_thresh.get(cl, (0, 99))
            active = sum(1 for k in cluster_keys.get(cl, []) if resp.get(k, 0) >= thr)
            bonus = 3.0 if active >= mn else 0.0
            final = raw + bonus
            risk = final / mx * 100 if mx else 0
            priority = final + bonus
            cls = classify(raw, p)
            rows.append((p, raw, mx, active >= mn, bonus, final, risk, cls, priority))

        # desempate: priority_index, depois risk, depois ordem de exibicao
        best_idx = max(range(len(rows)), key=lambda i: (rows[i][8], rows[i][6]))

        # 2o pass: imprime com marca no vencedor
        for i, r in enumerate(rows):
            p, raw, mx, cluster, bonus, final, risk, cls, _priority = r
            mark = ' <- priority' if i == best_idx else ''
            print('    {:12s} raw={:5.1f} max={:5.1f} cluster={:<5} bonus={} final={:5.1f} risk={:5.2f}% class={}{}'.format(
                p, raw, mx, str(cluster), bonus, final, risk, cls, mark
            ))
        winner = rows[best_idx]
        print('    -> priority: {} (priority_index={:.1f}, risk={:.2f}%)'.format(
            winner[0], winner[8], winner[6]
        ))

    # ---------- 6) Idempotency ----------
    header('6) Idempotency (rerun 011)')
    n_before = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_questions').fetchone()[0]
    with open(os.path.join(migrations, '011_deva_elevveclinic_bi_radar_seed.sql'), encoding='utf-8') as f:
        conn.executescript(f.read())
    conn.commit()
    n_after = cur.execute('SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_questions').fetchone()[0]
    check('idempotency: rerun 011 keeps question count',
          n_before == n_after,
          str(n_before) + ' -> ' + str(n_after))

    conn.close()

    # ---------- final ----------
    banner('Result')
    if failures:
        print('FAILED: ' + str(len(failures)) + ' check(s) failed:')
        for f in failures:
            print('  - ' + f)
        sys.exit(1)
    else:
        print('ALL CHECKS PASSED')
        sys.exit(0)


if __name__ == '__main__':
    main()

