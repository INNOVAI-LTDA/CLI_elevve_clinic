"""Repository SQLite para o BI Radar de Longevidade.

Leitura e escrita nas tabelas deva_elevveclinic_bi_* definidas em
migration 010. Idempotente. O path do DB vem de SQLITE_DB_PATH
(mesmo padrao do admin_api.py) ou do default
backend-integration/sql/elevve_clinic_deva_db.db.

F1 refactor: clientes do BI sao Users com role='client' em
deva_elevveclinic_users. Nao ha mais tabela bi_patients.

Toda query usa parametros (?) e valida identificadores contra
um pattern estrito (^[A-Za-z_][A-Za-z0-9_]*$). Colunas
sensíveis a injection nao sao interpoladas.
"""
from __future__ import annotations

import os
import re
import sqlite3
from pathlib import Path
from typing import Any


_BACKEND_ROOT = Path(__file__).resolve().parents[2]
_DEFAULT_SQLITE_DB = _BACKEND_ROOT / "sql" / "elevve_clinic_deva_db.db"


def _quote_ident(identifier: str) -> str:
    """Valida e faz quote de um identifier (tabela/coluna)."""
    if not re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", identifier):
        raise ValueError(f"Identificador invalido: {identifier!r}")
    return f'"{identifier}"'


class BiRadarRepository:
    """Leitura/escrita isolada nas tabelas deva_elevveclinic_bi_*."""

    def __init__(self, db_path: Path | str | None = None) -> None:
        if db_path is None:
            configured = os.getenv("SQLITE_DB_PATH", "").strip()
            if configured:
                self._db_path = Path(configured).expanduser().resolve()
            else:
                self._db_path = _DEFAULT_SQLITE_DB
        else:
            self._db_path = Path(db_path).expanduser().resolve()

    # ---------- internal ----------

    def _connect(self) -> sqlite3.Connection:
        if not self._db_path.exists():
            raise FileNotFoundError(
                f"DB nao encontrado em {self._db_path}. "
                "Aplique as migrations 010/011 antes de subir a API."
            )
        conn = sqlite3.connect(str(self._db_path))
        conn.row_factory = sqlite3.Row
        conn.execute("PRAGMA foreign_keys = ON")
        return conn

    # ---------- leitura ----------

    def list_pillars(self) -> list[dict[str, Any]]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT pillar_code, name, description, score_max, display_order "
                "FROM deva_elevveclinic_bi_radar_pillars "
                "ORDER BY display_order"
            ).fetchall()
        return [dict(r) for r in rows]

    def list_questions(self) -> list[dict[str, Any]]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT question_code, pillar_code, weight, is_active "
                "FROM deva_elevveclinic_bi_radar_questions "
                "ORDER BY pillar_code, display_order"
            ).fetchall()
        return [dict(r) for r in rows]

    def list_clusters(self) -> list[dict[str, Any]]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT cluster_code, pillar_code, activation_threshold, "
                "       minimum_key_symptoms, bonus_points "
                "FROM deva_elevveclinic_bi_radar_clusters "
                "WHERE is_active = 1"
            ).fetchall()
        return [dict(r) for r in rows]

    def list_cluster_questions(self) -> list[dict[str, Any]]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT cluster_code, question_code "
                "FROM deva_elevveclinic_bi_radar_cluster_questions"
            ).fetchall()
        return [dict(r) for r in rows]

    def list_interpretation_ranges(self) -> list[dict[str, Any]]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT pillar_code, label, min_score, max_score, priority_level "
                "FROM deva_elevveclinic_bi_radar_interpretation_ranges "
                "ORDER BY pillar_code, priority_level"
            ).fetchall()
        return [dict(r) for r in rows]

    # ---------- Clients (Users com role='client') ----------

    def list_clients(self) -> list[dict[str, Any]]:
        """Lista os clientes (Users com role='client') do BI.

        F1 refactor: substitui list_patients(). A fonte de dados
        agora e' deva_elevveclinic_users (modelo unificado).
        """
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT id, email, full_name, organization_id "
                "FROM deva_elevveclinic_users "
                "WHERE role = 'client' AND is_active = 1 "
                "ORDER BY id"
            ).fetchall()
        return [dict(r) for r in rows]

    def get_client(self, user_id: int) -> dict[str, Any] | None:
        """Retorna o cliente (User de tipo Client) pelo id.

        Retorna None se nao existir ou se nao tiver role='client'.
        """
        with self._connect() as conn:
            row = conn.execute(
                "SELECT id, email, full_name, role, organization_id, is_active "
                "FROM deva_elevveclinic_users "
                "WHERE id = ? AND role = 'client'",
                (user_id,),
            ).fetchone()
        return dict(row) if row else None

    def list_responses(self, user_id: int) -> list[dict[str, Any]]:
        """Lista as respostas de um cliente (User de tipo Client)."""
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT question_code, response_value "
                "FROM deva_elevveclinic_bi_radar_responses "
                "WHERE user_id = ?",
                (user_id,),
            ).fetchall()
        return [dict(r) for r in rows]

    def get_snapshots(
        self,
        user_id: int,
        pillar_code: str | None = None,
    ) -> list[dict[str, Any]]:
        with self._connect() as conn:
            if pillar_code is not None:
                rows = conn.execute(
                    "SELECT id, user_id, calculated_at, pillar_code, "
                    "       raw_score, cluster_bonus, final_score, max_score, "
                    "       risk_score, balance_score, classification, "
                    "       priority_index, is_priority_axis, details_json "
                    "FROM deva_elevveclinic_bi_radar_score_snapshots "
                    "WHERE user_id = ? AND pillar_code = ? "
                    "ORDER BY calculated_at DESC",
                    (user_id, pillar_code),
                ).fetchall()
            else:
                rows = conn.execute(
                    "SELECT id, user_id, calculated_at, pillar_code, "
                    "       raw_score, cluster_bonus, final_score, max_score, "
                    "       risk_score, balance_score, classification, "
                    "       priority_index, is_priority_axis, details_json "
                    "FROM deva_elevveclinic_bi_radar_score_snapshots "
                    "WHERE user_id = ? "
                    "ORDER BY calculated_at DESC, pillar_code",
                    (user_id,),
                ).fetchall()
        return [dict(r) for r in rows]

    # ---------- escrita ----------

    def insert_snapshot(self, row: dict[str, Any]) -> None:
        """Insere uma linha em deva_elevveclinic_bi_radar_score_snapshots.

        Espera chaves: id, user_id, calculated_at, pillar_code,
        raw_score, cluster_bonus, final_score, max_score, risk_score,
        balance_score, classification, priority_index, is_priority_axis,
        details_json.
        """
        required = {
            "id", "user_id", "calculated_at", "pillar_code",
            "raw_score", "cluster_bonus", "final_score", "max_score",
            "risk_score", "balance_score", "classification",
            "priority_index", "is_priority_axis",
        }
        missing = required - set(row.keys())
        if missing:
            raise ValueError(f"Snapshot row missing keys: {missing}")

        with self._connect() as conn:
            conn.execute(
                "INSERT INTO deva_elevveclinic_bi_radar_score_snapshots "
                "(id, user_id, calculated_at, pillar_code, raw_score, "
                " cluster_bonus, final_score, max_score, risk_score, balance_score, "
                " classification, priority_index, is_priority_axis, details_json) "
                "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                (
                    row["id"],
                    row["user_id"],
                    row["calculated_at"],
                    row["pillar_code"],
                    row["raw_score"],
                    row["cluster_bonus"],
                    row["final_score"],
                    row["max_score"],
                    row["risk_score"],
                    row["balance_score"],
                    row["classification"],
                    row["priority_index"],
                    row["is_priority_axis"],
                    row.get("details_json"),
                ),
            )
            conn.commit()
