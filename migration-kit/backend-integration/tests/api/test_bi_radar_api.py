"""Smoke test do BI Radar de Longevidade (Commit 4) via FastAPI TestClient.

Valida:
- Os 4 endpoints /bi/* retornam 200
- 404 vem no envelope canonico { "error": { ... } }
- O calculo do service esta sendo executado end-to-end (priority axis bate)
- Snapshots sao persistidos em deva_elevveclinic_bi_radar_score_snapshots
- camelCase no JSON (mirror do contract TypeScript)
- Endpoints legados (ex.: /admin/db/schema) seguem intactos

Requer: migrations 010/011 aplicadas e DB populado (rode
test_bi_radar.ps1 antes se o DB estiver vazio).
"""
from __future__ import annotations

import sqlite3
import unicodedata
from pathlib import Path

import pytest
from fastapi.testclient import TestClient


def _nfc(s: str) -> str:
    """Normaliza string para NFC para comparacao robusta contra
    diferencas de encoding do test source (NFD vs NFC)."""
    return unicodedata.normalize("NFC", s)


_BACKEND_ROOT = Path(__file__).resolve().parents[2]
_REPO_ROOT = _BACKEND_ROOT.parent
_SQLITE_DB = _REPO_ROOT / "sql" / "elevve_clinic_deva_db.db"


def _has_seed() -> bool:
    """Confere que as 8 tabelas BI estao presentes e populadas."""
    if not _SQLITE_DB.exists():
        return False
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        n = conn.execute(
            "SELECT COUNT(*) FROM sqlite_master "
            "WHERE type='table' AND name='deva_elevveclinic_bi_radar_pillars'"
        ).fetchone()[0]
        if n == 0:
            return False
        n_pat = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_patients"
        ).fetchone()[0]
        return n_pat >= 1
    finally:
        conn.close()


pytestmark = pytest.mark.skipif(
    not _has_seed(),
    reason="DB nao populado (rode migration-kit/sql/test_bi_radar.ps1 antes)",
)


@pytest.fixture(scope="module")
def client():
    from admin_api import app
    return TestClient(app)


@pytest.fixture(scope="module")
def expected_labels():
    """Lê os labels reais do banco (e o literal 'Pendente de configuração'
    do codigo) para evitar dependencia de strings acentuadas hard-coded."""
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        rows = conn.execute(
            "SELECT pillar_code, label FROM deva_elevveclinic_bi_radar_interpretation_ranges"
        ).fetchall()
        by_pillar: dict[str, set[str]] = {}
        for pillar, label in rows:
            by_pillar.setdefault(pillar, set()).add(_nfc(label))
        # 'Pendente de configuracao' e literal do service; usamos a mesma fonte
        from services.bi_radar_service import PENDING_CONFIG_LABEL
        by_pillar["pending"] = _nfc(PENDING_CONFIG_LABEL)
        return by_pillar
    finally:
        conn.close()


# ---------- health ----------

def test_health_returns_ok(client):
    r = client.get("/bi/health")
    assert r.status_code == 200
    assert r.json() == {"ok": True, "scope": "bi-radar"}


# ---------- pillars ----------

def test_list_pillars_returns_5(client):
    r = client.get("/bi/radar/pillars")
    assert r.status_code == 200
    pillars = r.json()
    assert len(pillars) == 5
    assert pillars[0]["pillarCode"] == "hormonios"
    # camelCase fields
    assert "scoreMax" in pillars[0]
    assert "displayOrder" in pillars[0]


# ---------- patients ----------

def test_list_patients_returns_2(client):
    r = client.get("/bi/radar/patients")
    assert r.status_code == 200
    patients = r.json()
    assert len(patients) == 2
    assert patients[0]["patientId"] == "bi_pat_demo_01"
    assert patients[0]["patientCode"] == "BI-DEMO-01"
    assert patients[0]["name"] == "Paciente Demo 01"


# ---------- patient radar 01 ----------

def test_patient_01_radar_priority_is_metabolismo(client, expected_labels):
    r = client.get("/bi/radar/patients/bi_pat_demo_01")
    assert r.status_code == 200
    body = r.json()
    assert body["patientId"] == "bi_pat_demo_01"
    assert len(body["axes"]) == 5
    assert body["priorityAxisKey"] == "metabolismo"
    assert body["summary"]["risk"] > 0

    metabolismo = next(a for a in body["axes"] if a["axisKey"] == "metabolismo")
    assert metabolismo["clusterActive"] is True
    assert metabolismo["clusterBonus"] == 3.0
    # Classification: le direto do banco para evitar dependencia de
    # string literal com acento (problemas de encoding pytest/console).
    assert _nfc(metabolismo["classification"]) in expected_labels["metabolismo"]
    assert metabolismo["isPriorityAxis"] is True

    hormonios = next(a for a in body["axes"] if a["axisKey"] == "hormonios")
    assert _nfc(hormonios["classification"]) == expected_labels["pending"]


def test_patient_01_pending_configuration_flag_is_true(client):
    r = client.get("/bi/radar/patients/bi_pat_demo_01")
    assert r.status_code == 200
    assert r.json()["summary"]["pendingConfiguration"] is True


# ---------- patient radar 02 ----------

def test_patient_02_radar_priority_is_recuperacao(client):
    r = client.get("/bi/radar/patients/bi_pat_demo_02")
    assert r.status_code == 200
    body = r.json()
    assert body["priorityAxisKey"] == "recuperacao"
    rec = next(a for a in body["axes"] if a["axisKey"] == "recuperacao")
    assert rec["clusterActive"] is True
    assert rec["isPriorityAxis"] is True


# ---------- 404 com envelope canonico ----------

def test_patient_not_found_uses_canonical_envelope(client):
    r = client.get("/bi/radar/patients/inexistente")
    assert r.status_code == 404
    err = r.json()
    # Estrutura: { "error": { "status", "code", "message", "details" } }
    assert "error" in err
    assert err["error"]["status"] == 404
    assert err["error"]["code"] == "PATIENT_NOT_FOUND"
    assert "message" in err["error"]
    assert err["error"]["details"] == {"patient_id": "inexistente"}


# ---------- snapshot persistido ----------

def test_patient_radar_persists_snapshot_rows(client):
    """Apos chamar /bi/radar/patients/{id}, espera-se que existam
    5 linhas novas em deva_elevveclinic_bi_radar_score_snapshots
    (1 por pilar)."""
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        before = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_score_snapshots"
        ).fetchone()[0]
    finally:
        conn.close()

    r = client.get("/bi/radar/patients/bi_pat_demo_01")
    assert r.status_code == 200

    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        after = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_score_snapshots"
        ).fetchone()[0]
        # Espera 5 snapshots novos (um por pilar)
        assert after - before == 5, f"expected 5 new snapshots, got {after - before}"

        # Verifica que tem 1 snapshot por pilar para este paciente
        rows = conn.execute(
            "SELECT pillar_code, raw_score, final_score, classification, "
            "       priority_index, is_priority_axis "
            "FROM deva_elevveclinic_bi_radar_score_snapshots "
            "WHERE patient_id = 'bi_pat_demo_01' "
            "ORDER BY calculated_at DESC, pillar_code "
            "LIMIT 5"
        ).fetchall()
        pillar_codes = sorted(r[0] for r in rows)
        assert pillar_codes == [
            "estrutura", "hormonios", "intestino", "metabolismo", "recuperacao",
        ]
        # Confere que o is_priority_axis == 1 so' para metabolismo
        priorities = [r[5] for r in rows]
        assert sum(priorities) == 1, f"expected exactly 1 priority axis, got {sum(priorities)}"
        priority_pillar = next(r for r in rows if r[5] == 1)
        assert priority_pillar[0] == "metabolismo"
    finally:
        conn.close()


# ---------- endpoints legados intactos ----------

def test_legacy_health_still_works(client):
    """Confirma que o endpoint /health do admin_api.py NAO foi afetado."""
    r = client.get("/health")
    assert r.status_code == 200
    body = r.json()
    assert body.get("ok") is True
    # admin_api /health retorna "provider" e "sqlite_db_path" alem de "ok"
    assert "provider" in body
    assert "sqlite_db_path" in body


def test_legacy_404_still_uses_simple_detail_envelope(client):
    """Confirma que um 404 do admin_api.py continua no formato
    antigo { "detail": "..." } (NAO no envelope canonico)."""
    r = client.get("/admin/db/schema/inexistente")
    # admin_api nao tem essa rota; FastAPI devolve 404 com { "detail": "Not Found" }
    # Este teste documenta que a montanha NAO vazou.
    assert r.status_code == 404
    body = r.json()
    # O envelope canonico NAO deve estar aqui
    assert "error" not in body or not isinstance(body.get("error"), dict)
    assert "detail" in body
