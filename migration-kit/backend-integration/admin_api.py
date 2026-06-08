from __future__ import annotations

import csv
import os
import re
import sqlite3
from pathlib import Path
from typing import Any

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SQLITE_DB = ROOT / "sql" / "elevve_clinic_deva_db.db"


class SqlListRequest(BaseModel):
    directory: str


class SqlPreviewRequest(BaseModel):
    script_path: str


class LoadSqlRequest(BaseModel):
    script_path: str


class CsvMappingItem(BaseModel):
    target_column: str = Field(alias="target_column")
    source_column: str = Field(alias="source_column")


class LoadCsvRequest(BaseModel):
    table: str
    csv_path: str
    mapping: list[CsvMappingItem]
    options: dict[str, Any] | None = None


class InsertRowRequest(BaseModel):
    table: str
    row: dict[str, Any]


app = FastAPI(title="Admin DB API", version="1.0.0")


def get_db_provider() -> str:
    return os.getenv("DB_PROVIDER", "sqlite").strip().lower() or "sqlite"


def get_sqlite_db_path() -> Path:
    configured = os.getenv("SQLITE_DB_PATH", "").strip()
    if configured:
        return Path(configured).expanduser().resolve()
    return DEFAULT_SQLITE_DB.resolve()


def open_sqlite() -> sqlite3.Connection:
    db_path = get_sqlite_db_path()
    if not db_path.exists():
        raise HTTPException(status_code=500, detail=f"SQLite nao encontrado em: {db_path}")
    conn = sqlite3.connect(str(db_path))
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA foreign_keys = ON")
    return conn


def require_sqlite() -> None:
    provider = get_db_provider()
    if provider != "sqlite":
        raise HTTPException(
            status_code=501,
            detail=f"DB_PROVIDER='{provider}' ainda nao implementado neste adapter admin.",
        )


def quote_ident(identifier: str) -> str:
    if not re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", identifier):
        raise HTTPException(status_code=400, detail=f"Identificador invalido: {identifier}")
    return f'"{identifier}"'


def table_columns(conn: sqlite3.Connection, table: str) -> list[dict[str, Any]]:
    rows = conn.execute(f"PRAGMA table_info({quote_ident(table)})").fetchall()
    out = []
    for row in rows:
        out.append(
            {
                "name": row["name"],
                "dataType": row["type"] or "TEXT",
                "nullable": row["notnull"] == 0,
                "primaryKey": row["pk"] == 1,
            }
        )
    return out


def normalize_table_name(raw: str) -> str:
    value = raw.strip()
    if not value:
        raise HTTPException(status_code=400, detail="Tabela vazia.")
    return value


@app.get("/health")
def health() -> dict[str, Any]:
    return {
        "ok": True,
        "provider": get_db_provider(),
        "sqlite_db_path": str(get_sqlite_db_path()),
    }


@app.get("/admin/db/schema")
def admin_db_schema() -> dict[str, Any]:
    require_sqlite()
    with open_sqlite() as conn:
        table_rows = conn.execute(
            """
            SELECT name
            FROM sqlite_master
            WHERE type = 'table'
              AND name NOT LIKE 'sqlite_%'
            ORDER BY name
            """
        ).fetchall()

        tables = []
        foreign_keys = []

        for row in table_rows:
            table = row["name"]
            cols = table_columns(conn, table)
            tables.append({"name": table, "columns": cols})

            fk_rows = conn.execute(f"PRAGMA foreign_key_list({quote_ident(table)})").fetchall()
            for fk in fk_rows:
                foreign_keys.append(
                    {
                        "table": table,
                        "column": fk["from"],
                        "refTable": fk["table"],
                        "refColumn": fk["to"],
                    }
                )

    return {"tables": tables, "foreignKeys": foreign_keys}


@app.post("/admin/db/sql/list")
def admin_db_sql_list(payload: SqlListRequest) -> dict[str, Any]:
    directory = Path(payload.directory).expanduser().resolve()
    if not directory.exists() or not directory.is_dir():
        raise HTTPException(status_code=400, detail=f"Diretorio invalido: {directory}")
    scripts = sorted(str(path) for path in directory.glob("*.sql") if path.is_file())
    return {"scripts": scripts}


def _render_schema_preview(sql_text: str) -> str:
    create_blocks = re.findall(
        r"CREATE\s+TABLE\s+(?:IF\s+NOT\s+EXISTS\s+)?([A-Za-z0-9_]+)\s*\((.*?)\);",
        sql_text,
        flags=re.IGNORECASE | re.DOTALL,
    )
    if not create_blocks:
        return "Nenhum CREATE TABLE detectado no script."

    lines: list[str] = []
    for table_name, body in create_blocks:
        lines.append(f"- tabela: {table_name}")
        for raw in body.splitlines():
            line = raw.strip().rstrip(",")
            if not line:
                continue
            lines.append(f"  - {line}")
        lines.append("")
    return "\n".join(lines).strip()


@app.post("/admin/db/sql/preview")
def admin_db_sql_preview(payload: SqlPreviewRequest) -> dict[str, Any]:
    script_path = Path(payload.script_path).expanduser().resolve()
    if not script_path.exists() or not script_path.is_file():
        raise HTTPException(status_code=400, detail=f"Script invalido: {script_path}")
    sql_text = script_path.read_text(encoding="utf-8")
    return {
        "script": str(script_path),
        "schema_preview": _render_schema_preview(sql_text),
    }


@app.post("/admin/db/load-sql")
def admin_db_load_sql(payload: LoadSqlRequest) -> dict[str, Any]:
    require_sqlite()
    script_path = Path(payload.script_path).expanduser().resolve()
    if not script_path.exists() or not script_path.is_file():
        raise HTTPException(status_code=400, detail=f"Script invalido: {script_path}")
    sql_text = script_path.read_text(encoding="utf-8")

    with open_sqlite() as conn:
        try:
            conn.executescript(sql_text)
            conn.commit()
        except sqlite3.DatabaseError as exc:
            raise HTTPException(status_code=400, detail=f"Falha ao executar SQL: {exc}") from exc

    return {"message": f"Script executado com sucesso: {script_path.name}"}


@app.post("/admin/db/insert-row")
def admin_db_insert_row(payload: InsertRowRequest) -> dict[str, Any]:
    require_sqlite()
    table = normalize_table_name(payload.table)

    with open_sqlite() as conn:
        cols_meta = table_columns(conn, table)
        if not cols_meta:
            raise HTTPException(status_code=404, detail=f"Tabela nao encontrada: {table}")

        known_cols = {col["name"] for col in cols_meta}
        row = {k: v for k, v in payload.row.items() if k in known_cols}
        if not row:
            raise HTTPException(status_code=400, detail="Nenhuma coluna valida para insercao.")

        columns = list(row.keys())
        placeholders = ", ".join("?" for _ in columns)
        sql = f"INSERT INTO {quote_ident(table)} ({', '.join(quote_ident(c) for c in columns)}) VALUES ({placeholders})"
        values = [None if str(row[c]).strip() == "" else row[c] for c in columns]

        try:
            cur = conn.execute(sql, values)
            conn.commit()
        except sqlite3.DatabaseError as exc:
            raise HTTPException(status_code=400, detail=f"Falha na insercao: {exc}") from exc

    return {"message": f"Insercao concluida em {table}. rowid={cur.lastrowid}"}


@app.post("/admin/db/load-csv")
def admin_db_load_csv(payload: LoadCsvRequest) -> dict[str, Any]:
    require_sqlite()
    table = normalize_table_name(payload.table)
    csv_path = Path(payload.csv_path).expanduser().resolve()
    if not csv_path.exists() or not csv_path.is_file():
        raise HTTPException(status_code=400, detail=f"CSV invalido: {csv_path}")

    delimiter = ";"
    if payload.options and isinstance(payload.options.get("delimiter"), str) and payload.options.get("delimiter"):
        delimiter = str(payload.options["delimiter"])
    if delimiter == "\\t":
        delimiter = "\t"

    mapping = payload.mapping
    if not mapping:
        raise HTTPException(status_code=400, detail="Mapeamento CSV vazio.")

    with open_sqlite() as conn:
        cols_meta = table_columns(conn, table)
        if not cols_meta:
            raise HTTPException(status_code=404, detail=f"Tabela nao encontrada: {table}")
        known_cols = {col["name"] for col in cols_meta}

        target_cols: list[str] = []
        source_cols: list[str] = []
        for item in mapping:
            if item.target_column not in known_cols:
                raise HTTPException(status_code=400, detail=f"Coluna alvo inexistente: {item.target_column}")
            target_cols.append(item.target_column)
            source_cols.append(item.source_column)

        inserted = 0
        skipped = 0
        with csv_path.open("r", encoding="utf-8-sig", newline="") as fp:
            reader = csv.DictReader(fp, delimiter=delimiter)
            if not reader.fieldnames:
                raise HTTPException(status_code=400, detail="CSV sem cabecalho.")

            missing_sources = [src for src in source_cols if src not in reader.fieldnames]
            if missing_sources:
                raise HTTPException(
                    status_code=400,
                    detail=f"Cabecalho CSV nao contem colunas mapeadas: {', '.join(missing_sources)}",
                )

            sql = (
                f"INSERT INTO {quote_ident(table)} "
                f"({', '.join(quote_ident(c) for c in target_cols)}) VALUES ({', '.join('?' for _ in target_cols)})"
            )

            for row in reader:
                values = []
                for source in source_cols:
                    raw = row.get(source, "")
                    value = raw.strip() if isinstance(raw, str) else raw
                    values.append(None if value == "" else value)

                if all(v is None for v in values):
                    skipped += 1
                    continue

                try:
                    conn.execute(sql, values)
                    inserted += 1
                except sqlite3.DatabaseError as exc:
                    raise HTTPException(status_code=400, detail=f"Erro na linha {inserted + skipped + 2}: {exc}") from exc

        conn.commit()

    return {
        "message": f"Carga CSV concluida em {table}. inseridos={inserted}, ignorados={skipped}",
        "inserted": inserted,
        "skipped": skipped,
    }


# -----------------------------------------------------------
# BI Radar de Longevidade (Commit 4) - novo router /bi/...
# Nao altera nenhum dos endpoints acima. Erros 4xx sao
# devolvidos via JSONResponse no formato canonico dentro do
# proprio router (sem mexer em handlers globais).
# -----------------------------------------------------------
from bi_radar_api import router as bi_radar_router
app.include_router(bi_radar_router)

