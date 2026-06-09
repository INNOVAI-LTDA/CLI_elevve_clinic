"""Smoke test do BI Radar de Longevidade via FastAPI TestClient.

F1 refactor: testes usam user_id 9001 e 9002 (clientes demo do
seed) ao inves dos antigos 'bi_pat_demo_01' / 'bi_pat_demo_02'.

Valida:
- Os 4 endpoints /bi/* retornam 200
- 404 vem no envelope canonico { "error": { ... } }
- O calculo do service esta sendo executado end-to-end (priority axis bate)
- Snapshots sao persistidos em deva_elevveclinic_bi_radar_score_snapshots
  (FK para deva_elevveclinic_users)
- camelCase no JSON (mirror do contract TypeScript)
- Endpoints legados (ex.: /health) seguem intactos

Requer: migrations 010/011 aplicadas e DB populado (rode
test_bi_radar.ps1 antes se o DB estiver vazio).
"""
from __future__ import annotations

import sqlite3
import unicodedata
from pathlib import Path

import pytest
from fastapi.testclient import TestClient


_BACKEND_ROOT = Path(__file__).resolve().parents[2]
_REPO_ROOT = _BACKEND_ROOT.parent
_SQLITE_DB = _REPO_ROOT / "sql" / "elevve_clinic_deva_db.db"

# F1 refactor: clientes demo do seed (IDs altos para nao colidir com
# usuarios reais do Acelerador Medico).
USER_DEMO_01 = 9001
USER_DEMO_02 = 9002


def _nfc(s: str) -> str:
    """Normaliza string para NFC para comparacao robusta."""
    return unicodedata.normalize("NFC", s)


def _has_seed() -> bool:
    """Confere que o seed minimo esta presente (demo clients em users)."""
    if not _SQLITE_DB.exists():
        return False
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        # 8 tabelas BI presentes?
        n = conn.execute(
            "SELECT COUNT(*) FROM sqlite_master "
            "WHERE type='table' AND name='deva_elevveclinic_bi_radar_pillars'"
        ).fetchone()[0]
        if n == 0:
            return False
        # Os 2 demo clients estao em users com role='client'?
        n_users = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_users "
            "WHERE id IN (?, ?) AND role='client'",
            (USER_DEMO_01, USER_DEMO_02),
        ).fetchone()[0]
        if n_users < 2:
            return False
        # 82 respostas (41 por user)
        n_resp = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_responses"
        ).fetchone()[0]
        return n_resp >= 82
    finally:
        conn.close()


pytestmark = pytest.mark.skipif(
    not _has_seed(),
    reason=(
        "DB nao populado ou refactor F1 nao aplicado "
        "(rode migration-kit/sql/test_bi_radar.ps1 antes)"
    ),
)


@pytest.fixture(scope="module")
def client():
    from admin_api import app
    return TestClient(app)


@pytest.fixture(scope="module")
def expected_labels():
    """Le os labels reais do banco (e o literal 'Pendente de
    configuracao' do service) para evitar dependencia de strings
    acentuadas hard-coded."""
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        rows = conn.execute(
            "SELECT pillar_code, label FROM deva_elevveclinic_bi_radar_interpretation_ranges"
        ).fetchall()
        by_pillar: dict[str, set[str]] = {}
        for pillar, label in rows:
            by_pillar.setdefault(pillar, set()).add(_nfc(label))
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


# ---------- clients ----------

def test_list_clients_returns_2(client):
    """F1 refactor: substitui /bi/radar/patients por /bi/radar/clients."""
    r = client.get("/bi/radar/clients")
    assert r.status_code == 200
    clients = r.json()
    assert len(clients) == 2
    # Ordenado por id, primeiro e' 9001
    assert clients[0]["userId"] == USER_DEMO_01
    assert clients[0]["email"] == "bi.demo.01@elevve.local"
    assert clients[0]["fullName"] == "Paciente Demo 01"
    assert clients[0]["role"] == "client"
    assert clients[1]["userId"] == USER_DEMO_02


# ---------- client radar 9001 ----------

def test_client_9001_radar_priority_is_metabolismo(client, expected_labels):
    r = client.get(f"/bi/radar/clients/{USER_DEMO_01}")
    assert r.status_code == 200
    body = r.json()
    # F1 refactor: userId no body, nao patientId
    assert body["userId"] == USER_DEMO_01
    assert len(body["axes"]) == 5
    assert body["priorityAxisKey"] == "metabolismo"
    assert body["summary"]["risk"] > 0

    metabolismo = next(a for a in body["axes"] if a["axisKey"] == "metabolismo")
    assert metabolismo["clusterActive"] is True
    assert metabolismo["clusterBonus"] == 3
    assert _nfc(metabolismo["classification"]) in expected_labels["metabolismo"]
    assert metabolismo["isPriorityAxis"] is True

    hormonios = next(a for a in body["axes"] if a["axisKey"] == "hormonios")
    assert _nfc(hormonios["classification"]) == expected_labels["pending"]


def test_client_9001_pending_configuration_flag_is_true(client):
    r = client.get(f"/bi/radar/clients/{USER_DEMO_01}")
    assert r.status_code == 200
    assert r.json()["summary"]["pendingConfiguration"] is True


# ---------- client radar 9002 ----------

def test_client_9002_radar_priority_is_recuperacao(client):
    r = client.get(f"/bi/radar/clients/{USER_DEMO_02}")
    assert r.status_code == 200
    body = r.json()
    assert body["priorityAxisKey"] == "recuperacao"
    rec = next(a for a in body["axes"] if a["axisKey"] == "recuperacao")
    assert rec["clusterActive"] is True
    assert rec["isPriorityAxis"] is True


# ---------- 404 com envelope canonico ----------

def test_client_not_found_uses_canonical_envelope(client):
    r = client.get("/bi/radar/clients/99999999")
    assert r.status_code == 404
    err = r.json()
    # Estrutura: { "error": { "status", "code", "message", "details" } }
    assert "error" in err
    assert err["error"]["status"] == 404
    # F1 refactor: code nao e' mais PATIENT_NOT_FOUND, e' CLIENT_NOT_FOUND
    assert err["error"]["code"] == "CLIENT_NOT_FOUND"
    assert "message" in err["error"]
    assert err["error"]["details"] == {"user_id": 99999999}


# ---------- snapshot persistido (FK para users) ----------

def test_client_radar_persists_snapshot_rows(client):
    """Apos chamar /bi/radar/clients/{id}, espera-se 5 linhas novas
    em deva_elevveclinic_bi_radar_score_snapshots (1 por pilar)."""
    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        before = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_score_snapshots"
        ).fetchone()[0]
    finally:
        conn.close()

    r = client.get(f"/bi/radar/clients/{USER_DEMO_01}")
    assert r.status_code == 200

    conn = sqlite3.connect(str(_SQLITE_DB))
    try:
        after = conn.execute(
            "SELECT COUNT(*) FROM deva_elevveclinic_bi_radar_score_snapshots"
        ).fetchone()[0]
        # Espera 5 snapshots novos (um por pilar)
        assert after - before == 5, f"expected 5 new snapshots, got {after - before}"

        # Verifica que tem 1 snapshot por pilar para este user
        rows = conn.execute(
            "SELECT pillar_code, raw_score, final_score, classification, "
            "       priority_index, is_priority_axis "
            "FROM deva_elevveclinic_bi_radar_score_snapshots "
            "WHERE user_id = ? "
            "ORDER BY calculated_at DESC, pillar_code "
            "LIMIT 5",
            (USER_DEMO_01,),
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
    assert "provider" in body
    assert "sqlite_db_path" in body


def test_legacy_404_still_uses_simple_detail_envelope(client):
    """Confirma que um 404 do admin_api.py continua no formato
    antigo { "detail": "..." } (NAO no envelope canonico)."""
    r = client.get("/admin/db/schema/inexistente")
    # admin_api nao tem essa rota; FastAPI devolve 404 com { "detail": "Not Found" }
    assert r.status_code == 404
    body = r.json()
    # O envelope canonico NAO deve estar aqui
    assert "error" not in body or not isinstance(body.get("error"), dict)
    assert "detail" in body
