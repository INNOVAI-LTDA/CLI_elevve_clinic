use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, NaiveDate, Utc};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use reqwest::blocking::Client;
use serde::Deserialize;

const ROOT_USERNAME: &str = "root";
const ROOT_PASSWORD: &str = "toor";

fn default_true() -> bool {
    true
}

impl DataHub {
    /// Retorna todos os usuários com role "client" (antigo Student)
    fn list_clients(&self) -> Vec<User> {
        self.users.iter().filter(|u| u.role == "client").cloned().collect()
    }

    // Removido duplicata de list_providers


#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct User {
    id: String,
    full_name: String,
    initials: Option<String>,
    email: Option<String>,
    role: String, // "admin", "provider", "client"
    is_active: Option<bool>,
    organization_id: Option<String>,
    // outros campos conforme necessário
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Organization {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Product {
    id: String,
    organization_id: String,
    name: String,
    slug: String,
    category: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Enrollment {
    id: String,
    student_id: String,
    organization_id: String,
    mentor_id: Option<String>,
    progress_score: f64,
    engagement_score: f64,
    day: i32,
    total_days: i32,
    days_left: i32,
    ltv_cents: i64,
    #[serde(default = "default_true")]
    is_active: bool,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Metric {
    id: String,
    pillar_id: String,
    name: String,
    slug: String,
    unit: Option<String>,
    direction: Option<String>,
    max_score: Option<f64>,
    mcv: Option<f64>,
    is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Measurement {
    id: String,
    enrollment_id: String,
    metric_id: String,
    value_baseline: f64,
    value_current: f64,
    value_projected: Option<f64>,
    improving_trend: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Checkpoint {
    id: String,
    enrollment_id: String,
    week: i32,
    status: String,
    label: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
struct Pillar {
    id: String,
    product_id: String,
    name: String,
    code: Option<String>,
    order_index: Option<i32>,
    axis_sub: Option<String>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
struct TableColumnSchema {
    name: String,
    data_type: String,
    nullable: bool,
    primary_key: bool,
}

#[derive(Debug, Clone, Default)]
struct ForeignKeySchema {
    table: String,
    column: String,
    ref_table: String,
    ref_column: String,
}

#[derive(Debug, Clone, Default)]
struct TableSchema {
    name: String,
    columns: Vec<TableColumnSchema>,
}

#[derive(Debug, Clone, Default)]
struct MappingValidation {
    blocking_errors: Vec<String>,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManageStage {
    Menu,
    SchemaDiagram,
    CsvSelectTable,
    CsvPathInput,
    CsvSelectFile, // NOVO: seleção de arquivo CSV
    CsvMapping,
    ManualSelectTable,
    ManualEditRow,
    SqlDirInput,
    SqlSelectScript,
    SqlConfirm,
    SqlResult,
}
// NOVO: Estrutura para navegação de arquivos
#[derive(Debug, Clone)]
struct FileDialogState {
    current_dir: PathBuf,
    entries: Vec<PathBuf>,
    selected: usize,
}

impl FileDialogState {
    fn new(start_dir: PathBuf) -> io::Result<Self> {
        let mut entries: Vec<_> = std::fs::read_dir(&start_dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        Ok(Self { current_dir: start_dir, entries, selected: 0 })
    }
    fn up(&mut self) {
        if self.selected == 0 {
            self.selected = self.entries.len().saturating_sub(1);
        } else {
            self.selected -= 1;
        }
    }
    fn down(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1) % self.entries.len();
        }
    }
}

#[derive(Debug, Clone)]
struct CommandSummary {
    id: String,
    name: String,
    mentor_id: Option<String>,
    program_name: String,
    urgency: String,
    days_left: i32,
    day: i32,
    total_days: i32,
    engagement: f64,
    progress: f64,
    hormozi_score: i32,
    ltv: i64,
    performance_score: f64,
}

#[derive(Debug, Clone)]
struct MatrixItem {
    id: String,
    name: String,
    quadrant: String,
    progress: f64,
    engagement: f64,
    days_left: i32,
    urgency: String,
    ltv: i64,
}

#[derive(Debug, Clone)]
struct RadarAxis {
    axis_label: String,
    axis_sub: String,
    baseline: f64,
    current: f64,
    projected: f64,
    insight: String,
}

#[derive(Debug, Clone)]
struct RadarPayload {
    student_id: String,
    protocol_name: String,
    avg_baseline: f64,
    avg_current: f64,
    avg_projected: f64,
    axes: Vec<RadarAxis>,
}

#[derive(Debug, Clone)]
struct MetricLine {
    label: String,
    baseline: f64,
    current: f64,
    projected: Option<f64>,
    unit: String,
}

#[derive(Debug, Clone)]
struct TimelineLine {
    week: i32,
    status: String,
    label: String,
    marker: String,
    action: String,
}

#[derive(Debug, Clone)]
struct CommandDetailPayload {
    summary: CommandSummary,
    metrics: Vec<MetricLine>,
    timeline: Vec<TimelineLine>,
    anomaly_count: usize,
    current_week: i32,
    last_week: i32,
}

#[derive(Debug, Clone)]
struct DataHub {
    base_dir: PathBuf,
    sqlite_db_path: PathBuf,
    schema_sql_path: PathBuf,
    api_base_url: String,
    api_token: Option<String>,
    http_client: Client,
    users: Vec<User>,
    products: Vec<Product>,
    enrollments: Vec<Enrollment>,
    metrics: Vec<Metric>,
    measurements: Vec<Measurement>,
    checkpoints: Vec<Checkpoint>,
    pillars: Vec<Pillar>,
    table_schemas: Vec<TableSchema>,
    foreign_keys: Vec<ForeignKeySchema>,
    command_center_cache: Vec<CommandSummary>,
    matrix_cache: HashMap<String, (Vec<MatrixItem>, i64, usize, usize, f64)>,
    command_detail_cache: HashMap<String, CommandDetailPayload>,
    radar_cache: HashMap<String, RadarPayload>,
}

impl DataHub {
    fn from_base_dir(base_dir: PathBuf) -> io::Result<Self> {
        let api_base_url =
            std::env::var("TUI_API_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
        let api_token = std::env::var("TUI_API_TOKEN").ok().filter(|v| !v.trim().is_empty());
        let http_client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Falha client HTTP: {}", err)))?;

        let sql_dir = base_dir.join("sql");
        let sqlite_db_path = std::env::var("TUI_SQLITE_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| sql_dir.join("elevve_clinic_deva_db.db"));
        let schema_sql_path = std::env::var("TUI_SCHEMA_SQL_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| sql_dir.join("sql_create_database.sql"));

        let users = Vec::new();
        let products = Vec::new();
        let enrollments = Vec::new();
        let metrics = Vec::new();
        let measurements = Vec::new();
        let checkpoints = Vec::new();
        let pillars = Vec::new();

        let mut table_schemas = Vec::new();
        let mut foreign_keys = Vec::new();
        if schema_sql_path.exists() {
            if let Ok(tables) = parse_table_schemas_from_sql_file(&schema_sql_path) {
                table_schemas = tables;
            }
            if let Ok(fks) = parse_foreign_keys_from_sql_file(&schema_sql_path) {
                foreign_keys = fks;
            }
        }
        for fk in &foreign_keys {
            if let Some(table) = table_schemas.iter_mut().find(|t| t.name == fk.table) {
                if let Some(column) = table.columns.iter_mut().find(|c| c.name == fk.column) {
                    if !column.data_type.contains("->") {
                        column.data_type = format!("{} -> {}.{}", column.data_type, fk.ref_table, fk.ref_column);
                    }
                }
            }
        }

        let mut hub = Self {
            base_dir,
            sqlite_db_path,
            schema_sql_path,
            api_base_url,
            api_token,
            http_client,
            users,
            products,
            enrollments,
            metrics,
            measurements,
            checkpoints,
            pillars,
            table_schemas,
            foreign_keys,
            command_center_cache: Vec::new(),
            matrix_cache: HashMap::new(),
            command_detail_cache: HashMap::new(),
            radar_cache: HashMap::new(),
        };
        hub.refresh_api_caches();
        Ok(hub)
    }

    fn refresh(&mut self) -> io::Result<()> {
        let reloaded = DataHub::from_base_dir(self.base_dir.clone())?;
        *self = reloaded;
        Ok(())
    }

    fn api_get_json(&self, path: &str, query: &[(&str, String)]) -> io::Result<serde_json::Value> {
        let base = self.api_base_url.trim_end_matches('/');
        let url = format!("{}/{}", base, path.trim_start_matches('/'));
        let mut req = self.http_client.get(url).query(query);
        if let Some(token) = &self.api_token {
            req = req.bearer_auth(token);
        }
        let resp = req
            .send()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("GET {} falhou: {}", path, err)))?;
        let status = resp.status();
        let text = resp
            .text()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Resposta inválida: {}", err)))?;
        if !status.is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("HTTP {} em {}: {}", status, path, text),
            ));
        }
        serde_json::from_str::<serde_json::Value>(&text)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("JSON inválido: {}", err)))
    }

    fn api_post_json(&self, path: &str, body: &serde_json::Value) -> io::Result<serde_json::Value> {
        let base = self.api_base_url.trim_end_matches('/');
        let url = format!("{}/{}", base, path.trim_start_matches('/'));
        let mut req = self.http_client.post(url).json(body);
        if let Some(token) = &self.api_token {
            req = req.bearer_auth(token);
        }
        let resp = req
            .send()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("POST {} falhou: {}", path, err)))?;
        let status = resp.status();
        let text = resp
            .text()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Resposta inválida: {}", err)))?;
        if !status.is_success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("HTTP {} em {}: {}", status, path, text),
            ));
        }
        serde_json::from_str::<serde_json::Value>(&text)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("JSON inválido: {}", err)))
    }

    fn fetch_schema_from_api(&self) -> io::Result<(Vec<TableSchema>, Vec<ForeignKeySchema>)> {
        let payload = self.api_get_json("/admin/db/schema", &[])?;
        let tables_payload = payload
            .get("tables")
            .and_then(|v| v.as_array())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "schema sem campo tables"))?;
        let fks_payload = payload
            .get("foreignKeys")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut tables = Vec::new();
        for table_value in tables_payload {
            let name = table_value
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let cols = table_value
                .get("columns")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let mut columns = Vec::new();
            for col in cols {
                columns.push(TableColumnSchema {
                    name: col
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    data_type: col
                        .get("dataType")
                        .or_else(|| col.get("data_type"))
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    nullable: col
                        .get("nullable")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true),
                    primary_key: col
                        .get("primaryKey")
                        .or_else(|| col.get("primary_key"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                });
            }
            tables.push(TableSchema { name, columns });
        }

        let mut fks = Vec::new();
        for fk in fks_payload {
            fks.push(ForeignKeySchema {
                table: fk
                    .get("table")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                column: fk
                    .get("column")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                ref_table: fk
                    .get("refTable")
                    .or_else(|| fk.get("ref_table"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                ref_column: fk
                    .get("refColumn")
                    .or_else(|| fk.get("ref_column"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
            });
        }

        for fk in &fks {
            if let Some(table) = tables.iter_mut().find(|t| t.name == fk.table) {
                if let Some(column) = table.columns.iter_mut().find(|c| c.name == fk.column) {
                    if !column.data_type.contains("->") {
                        column.data_type = format!("{} -> {}.{}", column.data_type, fk.ref_table, fk.ref_column);
                    }
                }
            }
        }
        Ok((tables, fks))
    }

    fn json_str(value: &serde_json::Value, keys: &[&str]) -> String {
        for key in keys {
            if let Some(v) = value.get(*key) {
                if let Some(s) = v.as_str() {
                    return s.to_string();
                }
                if let Some(n) = v.as_i64() {
                    return n.to_string();
                }
                if let Some(n) = v.as_f64() {
                    return format!("{:.4}", n);
                }
            }
        }
        String::new()
    }

    fn json_i64(value: &serde_json::Value, keys: &[&str]) -> i64 {
        for key in keys {
            if let Some(v) = value.get(*key) {
                if let Some(n) = v.as_i64() {
                    return n;
                }
                if let Some(n) = v.as_f64() {
                    return n.round() as i64;
                }
                if let Some(s) = v.as_str() {
                    if let Ok(parsed) = s.parse::<i64>() {
                        return parsed;
                    }
                }
            }
        }
        0
    }

    fn json_f64(value: &serde_json::Value, keys: &[&str]) -> f64 {
        for key in keys {
            if let Some(v) = value.get(*key) {
                if let Some(n) = v.as_f64() {
                    return n;
                }
                if let Some(n) = v.as_i64() {
                    return n as f64;
                }
                if let Some(s) = v.as_str() {
                    if let Ok(parsed) = s.parse::<f64>() {
                        return parsed;
                    }
                }
            }
        }
        0.0
    }

    fn json_bool(value: &serde_json::Value, keys: &[&str], default_value: bool) -> bool {
        for key in keys {
            if let Some(v) = value.get(*key) {
                if let Some(flag) = v.as_bool() {
                    return flag;
                }
                if let Some(n) = v.as_i64() {
                    return n != 0;
                }
                if let Some(s) = v.as_str() {
                    let lowered = s.trim().to_ascii_lowercase();
                    if lowered == "true" || lowered == "1" {
                        return true;
                    }
                    if lowered == "false" || lowered == "0" {
                        return false;
                    }
                }
            }
        }
        default_value
    }

    fn normalize_score(value: f64) -> f64 {
        if value > 1.0 {
            clamp01(value / 100.0)
        } else {
            clamp01(value)
        }
    }

    fn parse_command_center_collection(&self, payload: &serde_json::Value) -> Vec<CommandSummary> {
        let items = if let Some(arr) = payload.as_array() {
            arr.clone()
        } else if let Some(arr) = payload.get("allItems").and_then(|v| v.as_array()) {
            arr.clone()
        } else if let Some(arr) = payload.get("items").and_then(|v| v.as_array()) {
            arr.clone()
        } else {
            Vec::new()
        };
        let context = payload.get("context").cloned().unwrap_or(serde_json::Value::Null);
        let context_mentor_id = Self::json_str(&context, &["mentorId", "mentor_id"]);
        let mut rows = Vec::new();
        for item in items {
            let progress = Self::normalize_score(Self::json_f64(&item, &["progress"]));
            let engagement = Self::normalize_score(Self::json_f64(&item, &["engagement"]));
            let summary = CommandSummary {
                id: Self::json_str(&item, &["id", "studentId", "student_id"]),
                name: Self::json_str(&item, &["name", "fullName", "full_name"]),
                mentor_id: {
                    let raw = Self::json_str(&item, &["mentorId", "mentor_id"]);
                    let fallback = if raw.is_empty() { context_mentor_id.clone() } else { raw };
                    if fallback.is_empty() {
                        None
                    } else {
                        Some(fallback)
                    }
                },
                program_name: {
                    let name = Self::json_str(&item, &["programName", "program_name", "plan"]);
                    if name.is_empty() {
                        "Programa".to_string()
                    } else {
                        name
                    }
                },
                urgency: {
                    let urg = Self::json_str(&item, &["urgency"]);
                    if urg.is_empty() {
                        "normal".to_string()
                    } else {
                        urg
                    }
                },
                days_left: Self::json_i64(&item, &["daysLeft", "days_left"]) as i32,
                day: Self::json_i64(&item, &["day"]) as i32,
                total_days: Self::json_i64(&item, &["totalDays", "total_days"]) as i32,
                engagement,
                progress,
                hormozi_score: Self::json_i64(&item, &["hormoziScore", "hormozi_score"]) as i32,
                ltv: Self::json_i64(&item, &["ltv", "ltv_cents"]),
                performance_score: 0.0,
            };
            if !summary.id.is_empty() {
                rows.push(summary);
            }
        }
        for row in &mut rows {
            row.performance_score = row.hormozi_score as f64 / 100.0;
        }
        rows.sort_by(|a, b| {
            b.performance_score
                .partial_cmp(&a.performance_score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| b.hormozi_score.cmp(&a.hormozi_score))
                .then_with(|| a.name.cmp(&b.name))
        });
        rows
    }

    fn fetch_command_center_from_api(&self) -> io::Result<Vec<CommandSummary>> {
        let payload = self.api_get_json("/mentor/centro-comando/alunos", &[])?;
        Ok(self.parse_command_center_collection(&payload))
    }

    /// Retorna todos os usuários com role "provider" (mentores)
    fn list_providers(&self) -> Vec<User> {
        self.users.iter().filter(|u| u.role == "provider").cloned().collect()
    }

    fn fetch_matrix_from_api(&self, filter_mode: &str) -> io::Result<(Vec<MatrixItem>, i64, usize, usize, f64)> {
        let payload = self.api_get_json(
            "/mentor/matriz-renovacao",
            &[("filter", filter_mode.to_string())],
        )?;
        let items = payload
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut rows = Vec::new();
        for item in items {
            let progress = Self::normalize_score(Self::json_f64(&item, &["progress"]));
            let engagement = Self::normalize_score(Self::json_f64(&item, &["engagement"]));
            let quadrant = {
                let raw = Self::json_str(&item, &["quadrant"]);
                if raw.is_empty() {
                    classify_quadrant(progress, engagement, 0.7, 0.7)
                } else {
                    raw
                }
            };
            rows.push(MatrixItem {
                id: Self::json_str(&item, &["id", "studentId", "student_id"]),
                name: Self::json_str(&item, &["name", "full_name", "fullName"]),
                quadrant,
                progress,
                engagement,
                days_left: Self::json_i64(&item, &["daysLeft", "days_left"]) as i32,
                urgency: {
                    let value = Self::json_str(&item, &["urgency"]);
                    if value.is_empty() { "normal".to_string() } else { value }
                },
                ltv: Self::json_i64(&item, &["ltv", "ltv_cents"]),
            });
        }

        let kpis = payload.get("kpis").cloned().unwrap_or(serde_json::Value::Null);
        let total_ltv = if kpis.is_object() {
            Self::json_i64(&kpis, &["totalLTV", "total_ltv"])
        } else {
            rows.iter().map(|row| row.ltv).sum()
        };
        let critical = if kpis.is_object() {
            Self::json_i64(&kpis, &["criticalRenewals", "critical_renewals"]) as usize
        } else {
            rows.iter().filter(|row| row.days_left <= 45 && row.quadrant == "topRight").count()
        };
        let rescue = if kpis.is_object() {
            Self::json_i64(&kpis, &["rescueCount", "rescue_count"]) as usize
        } else {
            rows.iter().filter(|row| row.urgency == "rescue").count()
        };
        let avg_eng = if kpis.is_object() {
            let raw = Self::json_f64(&kpis, &["avgEngagement", "avg_engagement"]);
            if raw <= 1.0 { raw * 100.0 } else { raw }
        } else if rows.is_empty() {
            0.0
        } else {
            (rows.iter().map(|row| row.engagement).sum::<f64>() / rows.len() as f64) * 100.0
        };

        Ok((rows, total_ltv, critical, rescue, avg_eng))
    }

    fn fetch_command_detail_from_api(&self, student_id: &str) -> io::Result<CommandDetailPayload> {
        let detail_path = format!("/mentor/centro-comando/alunos/{}", student_id);
        let timeline_path = format!("/mentor/centro-comando/alunos/{}/timeline-anomalias", student_id);
        let detail = self.api_get_json(&detail_path, &[])?;
        let timeline_payload = self.api_get_json(&timeline_path, &[]).ok();

        let progress = Self::normalize_score(Self::json_f64(&detail, &["progress"]));
        let engagement = Self::normalize_score(Self::json_f64(&detail, &["engagement"]));
        let mut summary = CommandSummary {
            id: Self::json_str(&detail, &["id", "studentId", "student_id"]),
            name: Self::json_str(&detail, &["name", "fullName", "full_name"]),
            mentor_id: None,
            program_name: {
                let name = Self::json_str(&detail, &["programName", "program_name", "plan"]);
                if name.is_empty() { "Programa".to_string() } else { name }
            },
            urgency: {
                let urg = Self::json_str(&detail, &["urgency"]);
                if urg.is_empty() { "normal".to_string() } else { urg }
            },
            days_left: Self::json_i64(&detail, &["daysLeft", "days_left"]) as i32,
            day: Self::json_i64(&detail, &["day"]) as i32,
            total_days: Self::json_i64(&detail, &["totalDays", "total_days"]) as i32,
            engagement,
            progress,
            hormozi_score: Self::json_i64(&detail, &["hormoziScore", "hormozi_score"]) as i32,
            ltv: Self::json_i64(&detail, &["ltv", "ltv_cents"]),
            performance_score: 0.0,
        };
        summary.performance_score = summary.hormozi_score as f64 / 100.0;

        let metric_values = detail
            .get("metricValues")
            .or_else(|| detail.get("metric_values"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut metrics = Vec::new();
        for metric in metric_values {
            metrics.push(MetricLine {
                label: Self::json_str(&metric, &["metricLabel", "metric_label", "name"]),
                baseline: Self::normalize_score(Self::json_f64(&metric, &["valueBaseline", "value_baseline"])),
                current: Self::normalize_score(Self::json_f64(&metric, &["valueCurrent", "value_current"])),
                projected: {
                    if metric.get("valueProjected").is_none() && metric.get("value_projected").is_none() {
                        None
                    } else {
                        Some(Self::normalize_score(Self::json_f64(
                            &metric,
                            &["valueProjected", "value_projected"],
                        )))
                    }
                },
                unit: Self::json_str(&metric, &["unit"]),
            });
        }

        let (timeline, anomaly_count, current_week, last_week) = if let Some(payload) = timeline_payload {
            let entries = payload
                .get("timeline")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let mut out = Vec::new();
            for entry in entries {
                let anomaly = entry.get("anomaly").cloned().unwrap_or(serde_json::Value::Null);
                out.push(TimelineLine {
                    week: Self::json_i64(&entry, &["week"]) as i32,
                    status: normalize_checkpoint_status(&Self::json_str(&entry, &["status"])),
                    label: Self::json_str(&entry, &["label"]),
                    marker: Self::json_str(&anomaly, &["marker"]),
                    action: Self::json_str(&anomaly, &["action"]),
                });
            }
            let summary_json = payload.get("summary").cloned().unwrap_or(serde_json::Value::Null);
            let anomaly_count = if summary_json.is_object() {
                Self::json_i64(&summary_json, &["anomalyCount", "anomaly_count"]) as usize
            } else {
                payload
                    .get("anomalies")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.len())
                    .unwrap_or(0)
            };
            let current_week = if summary_json.is_object() {
                Self::json_i64(&summary_json, &["currentWeek", "current_week"]) as i32
            } else {
                i32::max(summary.day / 7, 1)
            };
            let last_week = if summary_json.is_object() {
                Self::json_i64(&summary_json, &["lastWeek", "last_week"]) as i32
            } else {
                out.iter().map(|row| row.week).max().unwrap_or(0)
            };
            (out, anomaly_count, current_week, last_week)
        } else {
            (Vec::new(), 0, i32::max(summary.day / 7, 1), 0)
        };

        Ok(CommandDetailPayload {
            summary,
            metrics,
            timeline,
            anomaly_count,
            current_week,
            last_week,
        })
    }

    fn fetch_radar_from_api(&self, student_id: &str) -> io::Result<RadarPayload> {
        let path = format!("/mentor/radar/alunos/{}", student_id);
        let payload = self.api_get_json(&path, &[])?;
        let protocol_name = Self::json_str(
            payload.get("context").unwrap_or(&serde_json::Value::Null),
            &["protocolName", "protocol_name"],
        );
        let axes_payload = payload
            .get("axisScores")
            .or_else(|| payload.get("axis_scores"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut axes = Vec::new();
        for axis in axes_payload {
            let baseline = Self::normalize_score(Self::json_f64(&axis, &["baseline"]));
            let current = Self::normalize_score(Self::json_f64(&axis, &["current"]));
            let projected = if axis.get("projected").is_none() && axis.get("value_projected").is_none() {
                current
            } else {
                Self::normalize_score(Self::json_f64(&axis, &["projected", "value_projected"]))
            };
            axes.push(RadarAxis {
                axis_label: Self::json_str(&axis, &["axisLabel", "axis_label", "name"]),
                axis_sub: Self::json_str(&axis, &["axisSub", "axis_sub"]),
                baseline,
                current,
                projected,
                insight: Self::json_str(&axis, &["insight"]),
            });
        }
        Ok(RadarPayload {
            student_id: {
                let id = Self::json_str(&payload, &["studentId", "student_id"]);
                if id.is_empty() { student_id.to_string() } else { id }
            },
            protocol_name,
            avg_baseline: Self::normalize_score(Self::json_f64(&payload, &["avgBaseline", "avg_baseline"])),
            avg_current: Self::normalize_score(Self::json_f64(&payload, &["avgCurrent", "avg_current"])),
            avg_projected: Self::normalize_score(Self::json_f64(
                &payload,
                &["avgProjected", "avg_projected"],
            )),
            axes,
        })
    }

    fn refresh_api_caches(&mut self) {
        if let Ok((api_schema, api_fks)) = self.fetch_schema_from_api() {
            self.table_schemas = api_schema;
            self.foreign_keys = api_fks;
        }
        if let Ok(rows) = self.fetch_command_center_from_api() {
            self.command_center_cache = rows;
        }
        if let Ok(matrix) = self.fetch_matrix_from_api("all") {
            self.matrix_cache.insert("all".to_string(), matrix);
        }
    }

    fn ensure_matrix_cache(&mut self, filter_mode: &str) -> io::Result<()> {
        if !self.matrix_cache.contains_key(filter_mode) {
            let payload = self.fetch_matrix_from_api(filter_mode)?;
            self.matrix_cache.insert(filter_mode.to_string(), payload);
        }
        Ok(())
    }

    fn ensure_command_detail_cache(&mut self, student_id: &str) -> io::Result<()> {
        if !self.command_detail_cache.contains_key(student_id) {
            let payload = self.fetch_command_detail_from_api(student_id)?;
            self.command_detail_cache.insert(student_id.to_string(), payload);
        }
        Ok(())
    }

    fn ensure_radar_cache(&mut self, student_id: &str) -> io::Result<()> {
        if !self.radar_cache.contains_key(student_id) {
            let payload = self.fetch_radar_from_api(student_id)?;
            self.radar_cache.insert(student_id.to_string(), payload);
        }
        Ok(())
    }

    fn ensure_sql_scripts(&self, directory: &str) -> io::Result<Vec<String>> {
        let payload = self.api_post_json(
            "/admin/db/sql/list",
            &serde_json::json!({ "directory": directory }),
        )?;
        if let Some(arr) = payload.as_array() {
            let mut rows = Vec::new();
            for item in arr {
                if let Some(path) = item.as_str() {
                    rows.push(path.to_string());
                }
            }
            return Ok(rows);
        }
        let scripts = payload
            .get("scripts")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut rows = Vec::new();
        for item in scripts {
            if let Some(path) = item.as_str() {
                rows.push(path.to_string());
            }
        }
        Ok(rows)
    }

    fn preview_sql_script(&self, script_path: &str) -> io::Result<String> {
        let payload = self.api_post_json(
            "/admin/db/sql/preview",
            &serde_json::json!({ "script_path": script_path }),
        )?;
        let preview = Self::json_str(&payload, &["schema_preview", "schemaPreview", "preview"]);
        if !preview.is_empty() {
            return Ok(preview);
        }
        if let Some(sql) = payload.get("sql").and_then(|v| v.as_str()) {
            return Ok(render_sql_schema_preview(sql));
        }
        Ok(serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "Sem preview".to_string()))
    }

    fn execute_sql_script_via_api(&self, script_path: &str) -> io::Result<String> {
        let payload = self.api_post_json(
            "/admin/db/load-sql",
            &serde_json::json!({ "script_path": script_path }),
        )?;
        let message = Self::json_str(&payload, &["message", "result", "status"]);
        if !message.is_empty() {
            return Ok(message);
        }
        Ok(serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "Execucao concluida".to_string()))
    }

    fn execute_csv_load_via_api(
        &self,
        table: &TableSchema,
        target_columns: &[String],
        mappings: &[Option<usize>],
        headers: &[String],
        csv_path: &Path,
        delimiter: char,
    ) -> io::Result<String> {
        let mapping: Vec<serde_json::Value> = target_columns
            .iter()
            .enumerate()
            .filter_map(|(idx, target)| {
                mappings.get(idx).and_then(|source_idx| {
                    source_idx.and_then(|source| {
                        headers.get(source).map(|source_name| {
                            serde_json::json!({
                                "target_column": target,
                                "source_column": source_name
                            })
                        })
                    })
                })
            })
            .collect();
        let payload = self.api_post_json(
            "/admin/db/load-csv",
            &serde_json::json!({
                "table": table.name,
                "csv_path": csv_path.display().to_string(),
                "mapping": mapping,
                "options": { "delimiter": delimiter.to_string() }
            }),
        )?;
        let message = Self::json_str(&payload, &["message", "result", "status"]);
        if !message.is_empty() {
            return Ok(message);
        }
        Ok(serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "Carga CSV enviada".to_string()))
    }

    fn execute_manual_insert_via_api(
        &self,
        table_name: &str,
        row: &HashMap<String, String>,
    ) -> io::Result<String> {
        let mut payload_row = serde_json::Map::new();
        for (k, v) in row {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                continue;
            }
            payload_row.insert(k.clone(), serde_json::Value::String(trimmed.to_string()));
        }
        if payload_row.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Nenhum campo preenchido para insercao.",
            ));
        }
        let payload = self.api_post_json(
            "/admin/db/insert-row",
            &serde_json::json!({
                "table": table_name,
                "row": payload_row
            }),
        )?;
        let message = Self::json_str(&payload, &["message", "result", "status"]);
        if !message.is_empty() {
            return Ok(message);
        }
        Ok(serde_json::to_string_pretty(&payload).unwrap_or_else(|_| "Insercao enviada".to_string()))
    }

    // Removido: função de compatibilidade list_provider_mentors

    fn list_command_center_students(&self, mentor_id: Option<&str>) -> Vec<CommandSummary> {
        match mentor_id {
            Some(selected) => {
                let filtered: Vec<CommandSummary> = self
                    .command_center_cache
                    .iter()
                    .filter(|row| row.mentor_id.as_deref().map(|id| id == selected).unwrap_or(false))
                    .cloned()
                    .collect();
                if filtered.is_empty() {
                    self.command_center_cache.clone()
                } else {
                    filtered
                }
            }
            None => self.command_center_cache.clone(),
        }
    }

    fn matrix_items(&self, filter_mode: &str) -> (Vec<MatrixItem>, i64, usize, usize, f64) {
        if let Some(payload) = self.matrix_cache.get(filter_mode) {
            return payload.clone();
        }
        if let Some((rows, total_ltv, critical, rescue, avg_eng)) = self.matrix_cache.get("all") {
            let filtered = match filter_mode {
                "topRight" => rows.iter().filter(|r| r.quadrant == "topRight").cloned().collect(),
                "critical" => rows
                    .iter()
                    .filter(|r| r.days_left <= 45 && r.quadrant == "topRight")
                    .cloned()
                    .collect(),
                "rescue" => rows.iter().filter(|r| r.urgency == "rescue").cloned().collect(),
                _ => rows.clone(),
            };
            return (filtered, *total_ltv, *critical, *rescue, *avg_eng);
        }
        (Vec::new(), 0, 0, 0, 0.0)
    }

    fn command_detail(&self, student_id: &str) -> Option<CommandDetailPayload> {
        self.command_detail_cache.get(student_id).cloned()
    }

    fn radar_for_student(&self, student_id: &str, _mentor_id: Option<&str>) -> Option<RadarPayload> {
        self.radar_cache.get(student_id).cloned()
    }
}

#[derive(Debug, Clone)]
enum Route {
    Login,
    Main,
    ManageDb,
    Matrix,
    CommandCenter,
    CommandDetail { student_id: String },
    RadarProviderMentor,
    RadarProviderStudent { mentor_id: String, mentor_name: String },
    RadarProviderView {
        mentor_id: String,
        mentor_name: String,
        student_id: String,
        student_name: String,
    },
    RadarClient,
    RadarClientView { student_id: String, student_name: String },
}

#[derive(Debug)]
struct App {
    route: Route,
    should_quit: bool,
    login_username: String,
    login_password: String,
    login_focus_password: bool,
    login_error: String,
    main_idx: usize,
    manage_stage: ManageStage,
    manage_menu_idx: usize,
    schema_table_idx: usize,
    csv_table_idx: usize,
    csv_mapping_idx: usize,
    csv_path_input: String,
    csv_delimiter: char,
    csv_headers: Vec<String>,
    csv_rows_preview: Vec<Vec<String>>,
    csv_target_columns: Vec<String>,
    csv_mappings: Vec<Option<usize>>,
    manual_table_idx: usize,
    manual_field_idx: usize,
    manual_table_name: String,
    manual_field_names: Vec<String>,
    manual_values: HashMap<String, String>,
    manual_editing: bool,
    manual_edit_buffer: String,
    sql_dir_input: String,
    sql_scripts: Vec<String>,
    sql_script_idx: usize,
    sql_confirm_script: Option<String>,
    sql_confirm_preview: String,
    sql_last_result: String,
    matrix_filter_idx: usize,
    cc_idx: usize,
    provider_mentor_idx: usize,
    provider_student_idx: usize,
    client_student_idx: usize,
    message: String,
    message_popup_open: bool,
    message_popup_title: String,
    csv_files: Vec<std::path::PathBuf>, // NOVO: lista de arquivos CSV
    csv_file_idx: usize,                // NOVO: índice do arquivo selecionado
}

impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Login,
            should_quit: false,
            login_username: String::new(),
            login_password: String::new(),
            login_focus_password: false,
            login_error: String::new(),
            main_idx: 0,
            manage_stage: ManageStage::Menu,
            manage_menu_idx: 0,
            schema_table_idx: 0,
            csv_table_idx: 0,
            csv_mapping_idx: 0,
            csv_path_input: String::new(),
            csv_delimiter: ',',
            csv_headers: Vec::new(),
            csv_rows_preview: Vec::new(),
            csv_target_columns: Vec::new(),
            csv_mappings: Vec::new(),
            manual_table_idx: 0,
            manual_field_idx: 0,
            manual_table_name: String::new(),
            manual_field_names: Vec::new(),
            manual_values: HashMap::new(),
            manual_editing: false,
            manual_edit_buffer: String::new(),
            sql_dir_input: String::new(),
            sql_scripts: Vec::new(),
            sql_script_idx: 0,
            sql_confirm_script: None,
            sql_confirm_preview: String::new(),
            sql_last_result: String::new(),
            matrix_filter_idx: 0,
            cc_idx: 0,
            provider_mentor_idx: 0,
            provider_student_idx: 0,
            client_student_idx: 0,
            message: String::new(),
            message_popup_open: false,
            message_popup_title: String::new(),
            csv_files: Vec::new(),
            csv_file_idx: 0,
        }
    }
}

impl App {
    fn handle_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match &self.route {
            Route::Login => self.handle_login_key(key),
            Route::Main => self.handle_main_key(key, data),
            Route::ManageDb => self.handle_manage_key(key, data),
            Route::Matrix => self.handle_matrix_key(key, data),
            Route::CommandCenter => self.handle_command_center_key(key, data),
            Route::CommandDetail { .. } => self.handle_detail_key(key),
            Route::RadarProviderMentor => self.handle_provider_mentor_key(key, data),
            Route::RadarProviderStudent { mentor_id, mentor_name } => {
                self.handle_provider_student_key(key, data, mentor_id.clone(), mentor_name.clone())
            }
            Route::RadarProviderView { .. } => self.handle_detail_key(key),
            Route::RadarClient => self.handle_radar_client_key(key, data),
            Route::RadarClientView { .. } => self.handle_detail_key(key),
        }
    }

    fn handle_login_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.login_focus_password = !self.login_focus_password,
            KeyCode::Backspace => {
                if self.login_focus_password {
                    self.login_password.pop();
                } else {
                    self.login_username.pop();
                }
            }
            KeyCode::Enter => {
                if self.login_username == ROOT_USERNAME && self.login_password == ROOT_PASSWORD {
                    self.route = Route::Main;
                    self.login_error.clear();
                    self.login_password.clear();
                } else {
                    self.login_error = "Credenciais invalidas".to_string();
                }
            }
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Char(c) => {
                if self.login_focus_password {
                    self.login_password.push(c);
                } else {
                    self.login_username.push(c);
                }
            }
            _ => {}
        }
    }

    fn handle_main_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        let max = 5usize;
        match key.code {
            KeyCode::Up => {
                if self.main_idx == 0 {
                    self.main_idx = max - 1;
                } else {
                    self.main_idx -= 1;
                }
            }
            KeyCode::Down => {
                self.main_idx = (self.main_idx + 1) % max;
            }
            KeyCode::Enter => match self.main_idx {
                0 => {
                    self.route = Route::ManageDb;
                    self.manage_stage = ManageStage::Menu;
                    self.manage_menu_idx = 0;
                }
                1 => {
                    if let Err(err) = data.ensure_matrix_cache("all") {
                        self.message = format!("Falha Matrix API: {}", err);
                    }
                    self.route = Route::Matrix;
                }
                2 => {
                    if self.command_center_needs_bootstrap(data) {
                        self.message = "Sem cache de Command Center. Use R para recarregar API.".to_string();
                    }
                    self.route = Route::CommandCenter;
                }
                3 => {
                    if data.list_providers().is_empty() {
                        self.message = "Mentores indisponiveis via API.".to_string();
                    }
                    self.route = Route::RadarProviderMentor;
                }
                4 => {
                    if self.command_center_needs_bootstrap(data) {
                        self.message = "Sem cache de alunos. Use R para recarregar API.".to_string();
                    }
                    self.route = Route::RadarClient;
                }
                _ => {}
            },
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_manage_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        match self.manage_stage {
            ManageStage::Menu => match key.code {
                KeyCode::Up => {
                    if self.manage_menu_idx == 0 {
                        self.manage_menu_idx = 3;
                    } else {
                        self.manage_menu_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    self.manage_menu_idx = (self.manage_menu_idx + 1) % 4;
                }
                KeyCode::Enter => match self.manage_menu_idx {
                    0 => self.manage_stage = ManageStage::SchemaDiagram,
                    1 => self.manage_stage = ManageStage::CsvSelectTable,
                    2 => self.manage_stage = ManageStage::ManualSelectTable,
                    3 => self.manage_stage = ManageStage::SqlDirInput,
                    _ => {}
                },
                KeyCode::Esc | KeyCode::Char('q') => self.route = Route::Main,
                _ => {}
            },
            ManageStage::SchemaDiagram => match key.code {
                KeyCode::Up => {
                    if self.schema_table_idx == 0 {
                        self.schema_table_idx = data.table_schemas.len().saturating_sub(1);
                    } else {
                        self.schema_table_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if !data.table_schemas.is_empty() {
                        self.schema_table_idx = (self.schema_table_idx + 1) % data.table_schemas.len();
                    }
                }
                KeyCode::Char('b') => match backup_stores(data) {
                    Ok(path) => self.message = format!("Backup criado em {}", path.display()),
                    Err(err) => self.message = format!("Falha no backup: {}", err),
                },
                KeyCode::Char('r') => {
                    if let Err(err) = data.refresh() {
                        self.message = format!("Falha no reload: {}", err);
                    } else {
                        self.message = "Schemas e caches API recarregados".to_string();
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => self.manage_stage = ManageStage::Menu,
                _ => {}
            },
            ManageStage::CsvSelectTable => match key.code {
                KeyCode::Up => {
                    if self.csv_table_idx == 0 {
                        self.csv_table_idx = data.table_schemas.len().saturating_sub(1);
                    } else {
                        self.csv_table_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if !data.table_schemas.is_empty() {
                        self.csv_table_idx = (self.csv_table_idx + 1) % data.table_schemas.len();
                    }
                }
                KeyCode::Enter => {
                    self.csv_path_input.clear();
                    self.csv_delimiter = ',';
                    self.csv_headers.clear();
                    self.csv_rows_preview.clear();
                    self.csv_target_columns.clear();
                    self.csv_mappings.clear();
                    self.csv_mapping_idx = 0;
                    self.manage_stage = ManageStage::CsvPathInput;
                }
                KeyCode::Esc | KeyCode::Char('q') => self.manage_stage = ManageStage::Menu,
                _ => {}
            },
            ManageStage::CsvPathInput => match key.code {
                KeyCode::Esc => self.manage_stage = ManageStage::CsvSelectTable,
                KeyCode::Char('f') | KeyCode::Char('F') => {
                    // NOVO: listar arquivos .csv do diretório CSV_IMPORT_DIR ou padrão
                    let dir = std::env::var("CSV_IMPORT_DIR").unwrap_or_else(|_| "migration-kit/sql".to_string());
                    let entries = std::fs::read_dir(&dir)
                        .map(|rd| {
                            let mut v: Vec<_> = rd.filter_map(|e| e.ok().map(|e| e.path()))
                                .filter(|p| p.is_file() && p.extension().map(|e| e == "csv").unwrap_or(false))
                                .collect();
                            v.sort();
                            v
                        })
                        .unwrap_or_default();
                    if entries.is_empty() {
                        self.message = format!("Nenhum arquivo .csv encontrado em {}", dir);
                    } else {
                        self.csv_files = entries;
                        self.csv_file_idx = 0;
                        self.manage_stage = ManageStage::CsvSelectFile;
                    }
                }
                KeyCode::Backspace => {
                    self.csv_path_input.pop();
                }
                KeyCode::Char(c) => {
                    if c == 'q' {
                        self.manage_stage = ManageStage::CsvSelectTable;
                    } else {
                        self.csv_path_input.push(c);
                    }
                }
                KeyCode::Enter => {
                    let selected = data.table_schemas.get(self.csv_table_idx);
                    if let Some(table) = selected {
                        match load_csv_preview(Path::new(&self.csv_path_input), 6) {
                            Ok((headers, rows, delimiter)) => {
                                self.csv_headers = headers;
                                self.csv_rows_preview = rows;
                                self.csv_delimiter = delimiter;
                                self.csv_target_columns = table.columns.iter().map(|c| c.name.clone()).collect();
                                self.csv_mappings =
                                    self.csv_target_columns.iter().map(|_| None).collect();
                                for (i, col) in self.csv_target_columns.iter().enumerate() {
                                    if let Some(pos) = self
                                        .csv_headers
                                        .iter()
                                        .position(|h| h.eq_ignore_ascii_case(col))
                                    {
                                        self.csv_mappings[i] = Some(pos);
                                    }
                                }
                                self.manage_stage = ManageStage::CsvMapping;
                                self.message = format!(
                                    "CSV lido (delimitador '{}'). Revise CSV x Tabela, ajuste o arquivo se necessario e valide antes de carregar.",
                                    delimiter
                                );
                            }
                            Err(err) => {
                                self.message = format!("Falha ao ler CSV: {}", err);
                            }
                        }
                    }
                }
                _ => {}
            },

            // NOVO: seleção de arquivo CSV
            ManageStage::CsvSelectFile => match key.code {
                KeyCode::Esc => {
                    self.manage_stage = ManageStage::CsvPathInput;
                }
                KeyCode::Up => {
                    if self.csv_file_idx == 0 {
                        self.csv_file_idx = self.csv_files.len().saturating_sub(1);
                    } else {
                        self.csv_file_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if !self.csv_files.is_empty() {
                        self.csv_file_idx = (self.csv_file_idx + 1) % self.csv_files.len();
                    }
                }
                KeyCode::Enter => {
                    if let Some(path) = self.csv_files.get(self.csv_file_idx) {
                        self.csv_path_input = path.display().to_string();
                        self.manage_stage = ManageStage::CsvPathInput;
                    }
                }
                _ => {}
            },
            // Adicione ao struct principal:

            // Exemplo: struct App {
            //     ...
            //     file_dialog: Option<FileDialogState>,
            // }

            // Procure a struct principal (App, Model, etc) e adicione:
            // file_dialog: Option<FileDialogState>,
            // Adicione um match para renderização:
            // Exemplo:
            // match app.manage_stage {
            //     ...
            // (removido: CsvFileDialog)
                KeyCode::Up => {
                    if self.message_popup_open {
                        return;
                    }
                    if self.csv_mapping_idx == 0 {
                        self.csv_mapping_idx = self.csv_target_columns.len().saturating_sub(1);
                    } else {
                        self.csv_mapping_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.message_popup_open {
                        return;
                    }
                    if !self.csv_target_columns.is_empty() {
                        self.csv_mapping_idx = (self.csv_mapping_idx + 1) % self.csv_target_columns.len();
                    }
                }
                KeyCode::Left => {
                    if self.message_popup_open {
                        return;
                    }
                    if let Some(mapping) = self.csv_mappings.get_mut(self.csv_mapping_idx) {
                        *mapping = shift_mapping(*mapping, self.csv_headers.len(), -1);
                    }
                }
                KeyCode::Right => {
                    if self.message_popup_open {
                        return;
                    }
                    if let Some(mapping) = self.csv_mappings.get_mut(self.csv_mapping_idx) {
                        *mapping = shift_mapping(*mapping, self.csv_headers.len(), 1);
                    }
                }
                KeyCode::Char('v') | KeyCode::Char('V') => {
                    if self.message_popup_open {
                        return;
                    }
                    if let Some(table) = data.table_schemas.get(self.csv_table_idx) {
                        let validation = validate_csv_mapping(
                            table,
                            &self.csv_target_columns,
                            &self.csv_mappings,
                            &self.csv_headers,
                        );
                        if validation.blocking_errors.is_empty() && validation.warnings.is_empty() {
                            self.message = "Validacao OK".to_string();
                        } else if validation.blocking_errors.is_empty() {
                            self.message = format!(
                                "Validacao com alertas nao bloqueantes: {}",
                                validation.warnings.join(" | ")
                            );
                        } else {
                            self.message =
                                format!("Validacao bloqueante: {}", validation.blocking_errors.join(" | "));
                        }
                        self.open_message_popup("Resultado da Validacao");
                    }
                }
                KeyCode::Char('l') | KeyCode::Char('L') => {
                    if self.message_popup_open {
                        return;
                    }
                    if let Some(table) = data.table_schemas.get(self.csv_table_idx) {
                        let validation = validate_csv_mapping(
                            table,
                            &self.csv_target_columns,
                            &self.csv_mappings,
                            &self.csv_headers,
                        );
                        if !validation.blocking_errors.is_empty() {
                            self.message = format!(
                                "Carga bloqueada. Corrija mapeamento: {}",
                                validation.blocking_errors.join(" | ")
                            );
                        } else {
                            match data.execute_csv_load_via_api(
                                table,
                                &self.csv_target_columns,
                                &self.csv_mappings,
                                &self.csv_headers,
                                Path::new(&self.csv_path_input),
                                self.csv_delimiter,
                            ) {
                                Ok(output) => self.message = output,
                                Err(err) => self.message = format!("Falha carga CSV: {}", err),
                            }
                            if !validation.warnings.is_empty() {
                                self.message = format!(
                                    "{}\n\nAlertas nao bloqueantes: {}",
                                    self.message,
                                    validation.warnings.join(" | ")
                                );
                            }
                        }
                        self.open_message_popup("Resultado da Carga CSV");
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => self.manage_stage = ManageStage::CsvSelectTable,
                _ => {}
            },
            ManageStage::ManualSelectTable => match key.code {
                KeyCode::Up => {
                    if self.manual_table_idx == 0 {
                        self.manual_table_idx = data.table_schemas.len().saturating_sub(1);
                    } else {
                        self.manual_table_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if !data.table_schemas.is_empty() {
                        self.manual_table_idx = (self.manual_table_idx + 1) % data.table_schemas.len();
                    }
                }
                KeyCode::Enter => {
                    if let Some(table) = data.table_schemas.get(self.manual_table_idx) {
                        self.begin_manual_insert_for_table(table);
                        self.manage_stage = ManageStage::ManualEditRow;
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => self.manage_stage = ManageStage::Menu,
                _ => {}
            },
            ManageStage::ManualEditRow => match key.code {
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q')
                    if self.message_popup_open =>
                {
                    self.message_popup_open = false;
                }
                KeyCode::Esc if self.manual_editing => {
                    self.manual_editing = false;
                    self.manual_edit_buffer.clear();
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') if !self.manual_editing => {
                    self.manual_editing = false;
                    self.manual_edit_buffer.clear();
                    self.manage_stage = ManageStage::ManualSelectTable;
                }
                KeyCode::Up => {
                    if self.manual_editing || self.message_popup_open {
                        return;
                    }
                    if self.manual_field_idx == 0 {
                        self.manual_field_idx = self.manual_field_names.len().saturating_sub(1);
                    } else {
                        self.manual_field_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.manual_editing || self.message_popup_open {
                        return;
                    }
                    if !self.manual_field_names.is_empty() {
                        self.manual_field_idx = (self.manual_field_idx + 1) % self.manual_field_names.len();
                    }
                }
                KeyCode::Enter => {
                    if self.manual_field_names.is_empty() || self.message_popup_open {
                        return;
                    }
                    let field = self.manual_field_names[self.manual_field_idx].clone();
                    if self.manual_editing {
                        self.manual_values.insert(field, self.manual_edit_buffer.clone());
                        self.manual_editing = false;
                        self.manual_edit_buffer.clear();
                    } else {
                        self.manual_edit_buffer = self.manual_values.get(&field).cloned().unwrap_or_default();
                        self.manual_editing = true;
                    }
                }
                KeyCode::Backspace => {
                    if self.manual_editing {
                        self.manual_edit_buffer.pop();
                    }
                }
                KeyCode::Char('c') | KeyCode::Char('C')
                    if !self.manual_editing && !self.message_popup_open =>
                {
                    if let Some(field) = self.manual_field_names.get(self.manual_field_idx).cloned() {
                        self.manual_values.insert(field, String::new());
                    }
                }
                KeyCode::Char('v') | KeyCode::Char('V')
                    if !self.manual_editing && !self.message_popup_open =>
                {
                    if let Some(table) = data
                        .table_schemas
                        .iter()
                        .find(|t| t.name.eq_ignore_ascii_case(&self.manual_table_name))
                    {
                        let validation = self.manual_insert_validation(table, &data.foreign_keys);
                        if validation.blocking_errors.is_empty() && validation.warnings.is_empty() {
                            self.message = "Validacao manual OK".to_string();
                        } else if validation.blocking_errors.is_empty() {
                            self.message = format!("Validacao com alertas: {}", validation.warnings.join(" | "));
                        } else {
                            self.message =
                                format!("Validacao bloqueante: {}", validation.blocking_errors.join(" | "));
                        }
                        self.open_message_popup("Validacao Manual");
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S')
                    if !self.manual_editing && !self.message_popup_open =>
                {
                    if let Some(table) = data
                        .table_schemas
                        .iter()
                        .find(|t| t.name.eq_ignore_ascii_case(&self.manual_table_name))
                    {
                        let validation = self.manual_insert_validation(table, &data.foreign_keys);
                        if !validation.blocking_errors.is_empty() {
                            self.message =
                                format!("Insercao bloqueada: {}", validation.blocking_errors.join(" | "));
                        } else {
                            match data.execute_manual_insert_via_api(&self.manual_table_name, &self.manual_values) {
                                Ok(msg) => {
                                    if validation.warnings.is_empty() {
                                        self.message = msg;
                                    } else {
                                        self.message = format!(
                                            "{}\n\nAlertas de relacionamento: {}",
                                            msg,
                                            validation.warnings.join(" | ")
                                        );
                                    }
                                }
                                Err(err) => self.message = format!("Falha insercao manual: {}", err),
                            }
                        }
                        self.open_message_popup("Resultado Insercao Manual");
                    }
                }
                KeyCode::Char(c) => {
                    if self.manual_editing && !self.message_popup_open {
                        self.manual_edit_buffer.push(c);
                    }
                }
                _ => {}
            },
            ManageStage::SqlDirInput => match key.code {
                KeyCode::Esc => self.manage_stage = ManageStage::Menu,
                KeyCode::Backspace => {
                    self.sql_dir_input.pop();
                }
                KeyCode::Char(c) => {
                    if c == 'q' {
                        self.manage_stage = ManageStage::Menu;
                    } else {
                        self.sql_dir_input.push(c);
                    }
                }
                KeyCode::Enter => {
                    match data.ensure_sql_scripts(&self.sql_dir_input) {
                        Ok(list) if !list.is_empty() => {
                            self.sql_scripts = list;
                            self.sql_script_idx = 0;
                            self.manage_stage = ManageStage::SqlSelectScript;
                        }
                        Ok(_) => self.message = "Nenhum .sql encontrado no diretorio".to_string(),
                        Err(err) => self.message = format!("Falha ao listar scripts: {}", err),
                    }
                }
                _ => {}
            },
            ManageStage::SqlSelectScript => match key.code {
                KeyCode::Up => {
                    if self.sql_script_idx == 0 {
                        self.sql_script_idx = self.sql_scripts.len().saturating_sub(1);
                    } else {
                        self.sql_script_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if !self.sql_scripts.is_empty() {
                        self.sql_script_idx = (self.sql_script_idx + 1) % self.sql_scripts.len();
                    }
                }
                KeyCode::Enter => {
                    self.sql_confirm_script = self.sql_scripts.get(self.sql_script_idx).cloned();
                    if let Some(script) = self.sql_confirm_script.as_ref() {
                        match data.preview_sql_script(script) {
                            Ok(preview) => {
                                self.sql_confirm_preview = preview;
                                self.manage_stage = ManageStage::SqlConfirm;
                            }
                            Err(err) => {
                                self.message = format!("Falha preview SQL: {}", err);
                            }
                        }
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => self.manage_stage = ManageStage::SqlDirInput,
                _ => {}
            },
            ManageStage::SqlConfirm => match key.code {
                KeyCode::Char('y') => {
                    if let Some(script) = self.sql_confirm_script.as_ref() {
                        match data.execute_sql_script_via_api(script) {
                            Ok(result) => {
                                self.sql_last_result = result;
                                self.manage_stage = ManageStage::SqlResult;
                            }
                            Err(err) => {
                                self.sql_last_result = format!("Erro: {}", err);
                                self.manage_stage = ManageStage::SqlResult;
                            }
                        }
                    }
                }
                KeyCode::Char('n') | KeyCode::Esc | KeyCode::Char('q') => {
                    self.manage_stage = ManageStage::SqlSelectScript;
                }
                _ => {}
            },
            ManageStage::SqlResult => match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => {
                    self.manage_stage = ManageStage::SqlSelectScript;
                }
                _ => {}
            },
        }
    }

    fn handle_matrix_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        let filters = ["all", "topRight", "critical", "rescue"];
        match key.code {
            KeyCode::Left => {
                if self.matrix_filter_idx == 0 {
                    self.matrix_filter_idx = filters.len() - 1;
                } else {
                    self.matrix_filter_idx -= 1;
                }
                let filter = filters[self.matrix_filter_idx % filters.len()];
                if let Err(err) = data.ensure_matrix_cache(filter) {
                    self.message = format!("Falha Matrix API: {}", err);
                }
            }
            KeyCode::Right => {
                self.matrix_filter_idx = (self.matrix_filter_idx + 1) % filters.len();
                let filter = filters[self.matrix_filter_idx % filters.len()];
                if let Err(err) = data.ensure_matrix_cache(filter) {
                    self.message = format!("Falha Matrix API: {}", err);
                }
            }
            KeyCode::Char('r') => {
                if let Err(err) = data.refresh() {
                    self.message = format!("Falha no reload: {}", err);
                } else {
                    self.message = "Caches API recarregados".to_string();
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.route = Route::Main,
            _ => {}
        }
    }

    fn handle_command_center_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        let students = data.list_command_center_students(None);
        match key.code {
            KeyCode::Up => {
                if self.cc_idx == 0 {
                    self.cc_idx = students.len().saturating_sub(1);
                } else {
                    self.cc_idx -= 1;
                }
            }
            KeyCode::Down => {
                if !students.is_empty() {
                    self.cc_idx = (self.cc_idx + 1) % students.len();
                }
            }
            KeyCode::Enter => {
                if let Some(row) = students.get(self.cc_idx) {
                    if let Err(err) = data.ensure_command_detail_cache(&row.id) {
                        self.message = format!("Falha detalhe API: {}", err);
                        return;
                    }
                    self.route = Route::CommandDetail {
                        student_id: row.id.clone(),
                    };
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.route = Route::Main,
            _ => {}
        }
    }

    fn handle_provider_mentor_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        let providers = data.list_providers();
        let mentors: Vec<&User> = providers.iter().filter(|u| u.is_active.unwrap_or(true)).collect();
        match key.code {
            KeyCode::Up => {
                if self.provider_mentor_idx == 0 {
                    self.provider_mentor_idx = mentors.len().saturating_sub(1);
                } else {
                    self.provider_mentor_idx -= 1;
                }
            }
            KeyCode::Down => {
                if !mentors.is_empty() {
                    self.provider_mentor_idx = (self.provider_mentor_idx + 1) % mentors.len();
                }
            }
            KeyCode::Enter => {
                if let Some(mentor) = mentors.get(self.provider_mentor_idx) {
                    self.provider_student_idx = 0;
                    self.route = Route::RadarProviderStudent {
                        mentor_id: mentor.id.clone(),
                        mentor_name: mentor.full_name.clone(),
                    };
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.route = Route::Main,
            _ => {}
        }
    }

    fn handle_provider_student_key(
        &mut self,
        key: KeyEvent,
        data: &mut DataHub,
        mentor_id: String,
        mentor_name: String,
    ) {
        let students = data.list_command_center_students(Some(&mentor_id));
        match key.code {
            KeyCode::Up => {
                if self.provider_student_idx == 0 {
                    self.provider_student_idx = students.len().saturating_sub(1);
                } else {
                    self.provider_student_idx -= 1;
                }
            }
            KeyCode::Down => {
                if !students.is_empty() {
                    self.provider_student_idx = (self.provider_student_idx + 1) % students.len();
                }
            }
            KeyCode::Enter => {
                if let Some(student) = students.get(self.provider_student_idx) {
                    if let Err(err) = data.ensure_radar_cache(&student.id) {
                        self.message = format!("Falha radar API: {}", err);
                        return;
                    }
                    self.route = Route::RadarProviderView {
                        mentor_id,
                        mentor_name,
                        student_id: student.id.clone(),
                        student_name: student.name.clone(),
                    };
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.route = Route::RadarProviderMentor,
            _ => {}
        }
    }

    fn handle_radar_client_key(&mut self, key: KeyEvent, data: &mut DataHub) {
        let students = data.list_command_center_students(None);
        match key.code {
            KeyCode::Up => {
                if self.client_student_idx == 0 {
                    self.client_student_idx = students.len().saturating_sub(1);
                } else {
                    self.client_student_idx -= 1;
                }
            }
            KeyCode::Down => {
                if !students.is_empty() {
                    self.client_student_idx = (self.client_student_idx + 1) % students.len();
                }
            }
            KeyCode::Enter => {
                if let Some(student) = students.get(self.client_student_idx) {
                    if let Err(err) = data.ensure_radar_cache(&student.id) {
                        self.message = format!("Falha radar API: {}", err);
                        return;
                    }
                    self.route = Route::RadarClientView {
                        student_id: student.id.clone(),
                        student_name: student.name.clone(),
                    };
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => self.route = Route::Main,
            _ => {}
        }
    }

    fn handle_detail_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.route = match self.route {
                    Route::CommandDetail { .. } => Route::CommandCenter,
                    Route::RadarProviderView { .. } => Route::RadarProviderMentor,
                    Route::RadarClientView { .. } => Route::RadarClient,
                    _ => Route::Main,
                };
            }
            _ => {}
        }
    }

    fn command_center_needs_bootstrap(&self, data: &DataHub) -> bool {
        data.list_command_center_students(None).is_empty()
    }

    fn open_message_popup(&mut self, title: &str) {
        self.message_popup_title = title.to_string();
        self.message_popup_open = true;
    }

    fn begin_manual_insert_for_table(&mut self, table: &TableSchema) {
        self.manual_table_name = table.name.clone();
        self.manual_field_names = table.columns.iter().map(|c| c.name.clone()).collect();
        self.manual_values.clear();
        for field in &self.manual_field_names {
            self.manual_values.insert(field.clone(), String::new());
        }
        self.manual_field_idx = 0;
        self.manual_editing = false;
        self.manual_edit_buffer.clear();
    }

    fn manual_insert_validation(&self, table: &TableSchema, fks: &[ForeignKeySchema]) -> MappingValidation {
        let mut blocking_errors = Vec::new();
        let mut warnings = Vec::new();
        for col in &table.columns {
            let value = self
                .manual_values
                .get(&col.name)
                .map(|v| v.trim().to_string())
                .unwrap_or_default();
            let has_value = !value.is_empty();
            if !col.nullable && !col.primary_key && !has_implicit_default_column(&col.name) && !has_value {
                blocking_errors.push(format!("campo obrigatorio vazio: {}", col.name));
            }
            if has_value && col.name.eq_ignore_ascii_case("cpf") {
                if value.len() != 11 || !value.chars().all(|c| c.is_ascii_digit()) {
                    blocking_errors.push("cpf deve ter 11 digitos numericos".to_string());
                }
            }
        }

        for fk in fks.iter().filter(|fk| fk.table.eq_ignore_ascii_case(&table.name)) {
            let raw = self
                .manual_values
                .get(&fk.column)
                .map(|v| v.trim().to_string())
                .unwrap_or_default();
            if raw.is_empty() {
                warnings.push(format!("FK nao informada: {} -> {}.{}", fk.column, fk.ref_table, fk.ref_column));
            } else if raw.parse::<i64>().is_err() {
                blocking_errors.push(format!("FK {} deve ser numerica (id da tabela {})", fk.column, fk.ref_table));
            }
        }

        MappingValidation { blocking_errors, warnings }
    }
}


#[derive(Debug, Clone)]
struct SeedUser {
    id: String,
    email: String,
    role: String,
    full_name: String,
    is_active: bool,
}

fn extract_insert_values_block(sql: &str, marker: &str) -> Option<String> {
    let lower = sql.to_lowercase();
    let marker_lower = marker.to_lowercase();
    let start = lower.find(&marker_lower)?;
    let after_marker = &sql[start..];
    let after_marker_lower = &lower[start..];
    let values_pos = after_marker_lower.find("values")?;
    let conflict_pos = after_marker_lower.find("on conflict").unwrap_or(after_marker.len());
    let raw_block = &after_marker[(values_pos + "values".len())..conflict_pos];
    Some(raw_block.trim().trim_end_matches(';').trim().to_string())
}

fn extract_tuples(values_block: &str) -> Vec<String> {
    let mut tuples = Vec::new();
    let mut depth = 0_i32;
    let mut in_string = false;
    let mut current = String::new();
    let chars: Vec<char> = values_block.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            if in_string && i + 1 < chars.len() && chars[i + 1] == '\'' {
                current.push(c);
                current.push(chars[i + 1]);
                i += 2;
                continue;
            }
            in_string = !in_string;
            current.push(c);
            i += 1;
            continue;
        }
        if !in_string {
            if c == '(' {
                depth += 1;
                if depth == 1 {
                    current.clear();
                    i += 1;
                    continue;
                }
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    tuples.push(current.trim().to_string());
                    current.clear();
                    i += 1;
                    continue;
                }
            }
        }
        if depth >= 1 {
            current.push(c);
        }
        i += 1;
    }
    tuples
}

fn split_sql_fields(tuple_raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut paren_depth = 0_i32;
    let mut bracket_depth = 0_i32;
    let mut brace_depth = 0_i32;
    let chars: Vec<char> = tuple_raw.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            if in_string && i + 1 < chars.len() && chars[i + 1] == '\'' {
                current.push(c);
                current.push(chars[i + 1]);
                i += 2;
                continue;
            }
            in_string = !in_string;
            current.push(c);
            i += 1;
            continue;
        }
        if !in_string {
            match c {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                    out.push(current.trim().to_string());
                    current.clear();
                    i += 1;
                    continue;
                }
                _ => {}
            }
        }
        current.push(c);
        i += 1;
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

fn strip_sql_literal(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.eq_ignore_ascii_case("null") {
        return String::new();
    }
    if let Some(first_quote) = trimmed.find('\'') {
        let remainder = &trimmed[first_quote + 1..];
        if let Some(last_quote) = remainder.rfind('\'') {
            return remainder[..last_quote].replace("''", "'");
        }
    }
    trimmed
        .trim_end_matches("::timestamptz")
        .trim_end_matches("::jsonb")
        .trim()
        .to_string()
}

fn parse_bool_sql(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("true")
}

fn parse_i64_sql(value: &str, fallback: i64) -> i64 {
    strip_sql_literal(value).parse::<i64>().unwrap_or(fallback)
}

fn parse_f64_sql(value: &str, fallback: f64) -> f64 {
    strip_sql_literal(value).parse::<f64>().unwrap_or(fallback)
}

fn parse_users_seed(sql: &str) -> Vec<SeedUser> {
    let Some(values_block) = extract_insert_values_block(sql, "INSERT INTO deva_accmed_users") else {
        return Vec::new();
    };
    extract_tuples(&values_block)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let fields = split_sql_fields(&raw);
            if fields.len() < 6 {
                return None;
            }
            Some(SeedUser {
                id: (idx + 1).to_string(),
                email: strip_sql_literal(&fields[0]),
                role: strip_sql_literal(&fields[1]),
                full_name: strip_sql_literal(&fields[2]),
                is_active: parse_bool_sql(&fields[3]),
            })
        })
        .collect()
}

/// Filtra usuários por role: retorna (clients, providers)
fn split_users_by_role(users: &[User]) -> (Vec<User>, Vec<User>) {
    let clients = users.iter().filter(|u| u.role == "client").cloned().collect();
    let providers = users.iter().filter(|u| u.role == "provider").cloned().collect();
    (clients, providers)
}

fn parse_organizations_seed(sql: &str) -> Vec<Organization> {
    let Some(values_block) = extract_insert_values_block(sql, "INSERT INTO deva_accmed_organizations") else {
        return Vec::new();
    };
    extract_tuples(&values_block)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let fields = split_sql_fields(&raw);
            if fields.len() < 2 {
                return None;
            }
            Some(Organization {
                id: (idx + 1).to_string(),
                name: strip_sql_literal(&fields[0]),
            })
        })
        .collect()
}

fn parse_products_seed(sql: &str, organizations: &[Organization]) -> Vec<Product> {
    let Some(values_block) = extract_insert_values_block(sql, "WITH source_rows") else {
        return Vec::new();
    };
    extract_tuples(&values_block)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let fields = split_sql_fields(&raw);
            if fields.len() < 8 {
                return None;
            }
            let legacy_org = strip_sql_literal(&fields[0]);
            let org_id = if legacy_org == "1" || legacy_org == "cli_1" {
                organizations
                    .iter()
                    .find(|o| o.name.to_lowercase().contains("acelerador"))
                    .map(|o| o.id.clone())
                    .unwrap_or_else(|| "1".to_string())
            } else {
                organizations
                    .iter()
                    .find(|o| o.name.to_lowercase().contains("innovai"))
                    .map(|o| o.id.clone())
                    .unwrap_or_else(|| "2".to_string())
            };
            Some(Product {
                id: (idx + 1).to_string(),
                organization_id: org_id,
                name: strip_sql_literal(&fields[1]),
                slug: strip_sql_literal(&fields[2]),
                category: strip_sql_literal(&fields[3]),
                status: strip_sql_literal(&fields[4]),
            })
        })
        .collect()
}

fn parse_pillars_seed(sql: &str) -> Vec<Pillar> {
    let Some(values_block) = extract_insert_values_block(sql, "INSERT INTO deva_accmed_product_pillars") else {
        return Vec::new();
    };
    extract_tuples(&values_block)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let fields = split_sql_fields(&raw);
            if fields.len() < 6 {
                return None;
            }
            let metadata_raw = strip_sql_literal(&fields[4]);
            let metadata_value = serde_json::from_str::<serde_json::Value>(&metadata_raw).ok();
            Some(Pillar {
                id: (idx + 1).to_string(),
                product_id: strip_sql_literal(&fields[0]),
                name: strip_sql_literal(&fields[1]),
                code: Some(strip_sql_literal(&fields[2])),
                order_index: Some(parse_i64_sql(&fields[3], (idx + 1) as i64) as i32),
                axis_sub: metadata_value
                    .as_ref()
                    .and_then(axis_sub_from_metadata),
                metadata: metadata_value,
            })
        })
        .collect()
}

fn parse_metrics_seed(sql: &str) -> Vec<Metric> {
    let Some(values_block) = extract_insert_values_block(sql, "WITH source_rows") else {
        return Vec::new();
    };
    extract_tuples(&values_block)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let fields = split_sql_fields(&raw);
            if fields.len() < 12 {
                return None;
            }
            Some(Metric {
                id: (idx + 1).to_string(),
                pillar_id: strip_sql_literal(&fields[0]),
                name: strip_sql_literal(&fields[1]),
                slug: strip_sql_literal(&fields[2]),
                direction: Some(strip_sql_literal(&fields[3])),
                unit: Some(strip_sql_literal(&fields[4])),
                max_score: Some(parse_f64_sql(&fields[8], 0.0)),
                mcv: Some(parse_f64_sql(&fields[10], 0.0)),
                is_active: Some(parse_bool_sql(&fields[11])),
            })
        })
        .collect()
}

fn build_enrollments_from_seed(
    users: &[SeedUser],
    products: &[Product],
    enrollments_sql: &str,
) -> Vec<Enrollment> {
    let provider_id = if enrollments_sql.contains("WHERE p.id = 2") {
        "2".to_string()
    } else {
        users
            .iter()
            .find(|u| u.role.eq_ignore_ascii_case("provider"))
            .map(|u| u.id.clone())
            .unwrap_or_else(|| "2".to_string())
    };
    let product_id = if enrollments_sql.contains("pr.slug = 'mnt_accmed'") {
        products
            .iter()
            .find(|p| p.slug == "mnt_accmed")
            .map(|p| p.id.clone())
            .unwrap_or_else(|| "1".to_string())
    } else {
        products.first().map(|p| p.id.clone()).unwrap_or_else(|| "1".to_string())
    };
    users.iter()
        .filter(|u| u.role.eq_ignore_ascii_case("client") && u.is_active)
        .enumerate()
        .map(|(idx, user)| Enrollment {
            id: (idx + 1).to_string(),
            student_id: user.id.clone(),
            organization_id: product_id.clone(),
            mentor_id: Some(provider_id.clone()),
            progress_score: 0.35,
            engagement_score: 0.55,
            day: 42,
            total_days: 180,
            days_left: 138,
            ltv_cents: 0,
            is_active: true,
            updated_at: Some("2026-05-07T19:19:54.252230Z".to_string()),
        })
        .collect()
}

fn parse_table_schemas_from_sql_file(path: &Path) -> io::Result<Vec<TableSchema>> {
    let sql = fs::read_to_string(path)?;
    let mut tables: Vec<TableSchema> = Vec::new();
    for (table_name, block) in extract_create_table_blocks(&sql) {
        let columns = parse_columns_from_create_block(&block);
        if columns.is_empty() {
            continue;
        }
        if let Some(existing) = tables.iter_mut().find(|t| t.name == table_name) {
            for col in columns {
                if !existing.columns.iter().any(|c| c.name.eq_ignore_ascii_case(&col.name)) {
                    existing.columns.push(col);
                }
            }
        } else {
            tables.push(TableSchema {
                name: table_name,
                columns,
            });
        }
    }
    tables.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(tables)
}

fn parse_foreign_keys_from_sql_file(path: &Path) -> io::Result<Vec<ForeignKeySchema>> {
    let sql = fs::read_to_string(path)?;
    let mut fks = Vec::new();
    for (table_name, block) in extract_create_table_blocks(&sql) {
        for raw in split_sql_fields(&block) {
            let line = raw.trim();
            if line.is_empty() {
                continue;
            }
            let lower = line.to_ascii_lowercase();
            if !lower.contains("foreign key") || !lower.contains("references") {
                continue;
            }
            let Some(fk_pos) = lower.find("foreign key") else {
                continue;
            };
            let fk_slice = &line[fk_pos..];
            let Some(col) = between(fk_slice, "(", ")") else {
                continue;
            };
            let Some(ref_pos) = lower.find("references") else {
                continue;
            };
            let reference_part = line[ref_pos + "references".len()..].trim();
            let ref_table = reference_part
                .split_whitespace()
                .next()
                .map(normalize_table_name)
                .unwrap_or_default();
            let Some(ref_col) = between(reference_part, "(", ")") else {
                continue;
            };
            if ref_table.is_empty() {
                continue;
            }
            fks.push(ForeignKeySchema {
                table: table_name.clone(),
                column: col.trim().trim_matches('"').to_string(),
                ref_table,
                ref_column: ref_col.trim().trim_matches('"').to_string(),
            });
        }
    }
    Ok(fks)
}

fn extract_create_table_blocks(sql: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let lower = sql.to_lowercase();
    let needle = "create table if not exists";
    let mut start_idx = 0usize;
    while let Some(pos) = lower[start_idx..].find(needle) {
        let absolute = start_idx + pos;
        let rest = &sql[absolute..];
        let open_paren = match rest.find('(') {
            Some(v) => v,
            None => break,
        };
        let table_part = rest[needle.len()..open_paren].trim();
        let table_name = table_part
            .trim_matches('"')
            .trim()
            .trim_end_matches('(')
            .trim()
            .to_string();
        let block_start = absolute + open_paren + 1;
        let mut depth = 1_i32;
        let bytes = sql.as_bytes();
        let mut idx = block_start;
        while idx < bytes.len() {
            let ch = bytes[idx] as char;
            if ch == '(' {
                depth += 1;
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    let block = sql[block_start..idx].to_string();
                    out.push((normalize_table_name(&table_name), block));
                    start_idx = idx + 1;
                    break;
                }
            }
            idx += 1;
        }
        if idx >= bytes.len() {
            break;
        }
    }
    out
}

fn normalize_table_name(name: &str) -> String {
    name.trim()
        .trim_matches('"')
        .split_whitespace()
        .next()
        .unwrap_or(name)
        .to_string()
}

fn parse_columns_from_create_block(block: &str) -> Vec<TableColumnSchema> {
    let mut columns = Vec::new();
    for line in split_sql_fields(block) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let lower = trimmed.to_lowercase();
        if lower.starts_with("constraint ") || lower.starts_with("check ") || lower.starts_with("foreign key") {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(name_raw) = parts.next() else {
            continue;
        };
        let name = name_raw.trim_matches('"').to_string();
        let tokens: Vec<&str> = parts.collect();
        if tokens.is_empty() {
            continue;
        }
        let mut dtype_tokens = Vec::new();
        for token in &tokens {
            let upper = token.to_ascii_uppercase();
            if matches!(
                upper.as_str(),
                "NOT" | "NULL" | "DEFAULT" | "PRIMARY" | "CONSTRAINT" | "CHECK" | "REFERENCES"
            ) {
                break;
            }
            dtype_tokens.push(*token);
        }
        let data_type = dtype_tokens.join(" ");
        let nullable = !lower.contains("not null");
        let primary_key = lower.contains("primary key");
        columns.push(TableColumnSchema {
            name,
            data_type,
            nullable,
            primary_key,
        });
    }
    columns
}

fn between<'a>(value: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let s = value.find(start)?;
    let rest = &value[s + start.len()..];
    let e = rest.find(end)?;
    Some(&rest[..e])
}

fn render_class_diagram_for_table(table: &TableSchema, fks: &[ForeignKeySchema]) -> String {
    let mut lines = Vec::new();
    lines.push(format!("[{}]", table.name));
    for col in &table.columns {
        let mut markers = Vec::new();
        if col.primary_key {
            markers.push("PK");
        }
        if !col.nullable {
            markers.push("NOT NULL");
        }
        let marker = if markers.is_empty() {
            "".to_string()
        } else {
            format!(" <<{}>>", markers.join(", "))
        };
        lines.push(format!("+ {} : {}{}", col.name, col.data_type, marker));
    }
    lines.push(String::new());
    lines.push("Relations:".to_string());
    let mut has_rel = false;
    for fk in fks.iter().filter(|fk| fk.table == table.name) {
        has_rel = true;
        lines.push(format!(
            "- {}.{} -> {}.{}",
            fk.table, fk.column, fk.ref_table, fk.ref_column
        ));
    }
    if !has_rel {
        lines.push("- (none)".to_string());
    }
    lines.join("\n")
}

fn load_csv_preview(path: &Path, max_rows: usize) -> io::Result<(Vec<String>, Vec<Vec<String>>, char)> {
    let raw = fs::read_to_string(path)?;
    let mut lines = raw.lines();
    let headers_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "CSV vazio"))?;
    let delimiter = detect_csv_delimiter(headers_line);
    let headers: Vec<String> = parse_delimited_line(headers_line, delimiter)
        .into_iter()
        .map(|s| s.trim().to_string())
        .collect();
    let mut preview = Vec::new();
    for line in lines.take(max_rows) {
        if line.trim().is_empty() {
            continue;
        }
        preview.push(
            parse_delimited_line(line, delimiter)
                .into_iter()
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }
    Ok((headers, preview, delimiter))
}

fn detect_csv_delimiter(header_line: &str) -> char {
    let candidates = [(';', header_line.matches(';').count()), (',', header_line.matches(',').count()), ('\t', header_line.matches('\t').count()), ('|', header_line.matches('|').count())];
    let best = candidates
        .iter()
        .max_by_key(|(_, count)| *count)
        .copied()
        .unwrap_or((',', 0));
    if best.1 == 0 { ',' } else { best.0 }
}

fn parse_delimited_line(line: &str, delimiter: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_quotes && matches!(chars.peek(), Some('"')) {
                current.push('"');
                let _ = chars.next();
            } else {
                in_quotes = !in_quotes;
            }
            continue;
        }
        if ch == delimiter && !in_quotes {
            out.push(current.trim().to_string());
            current.clear();
            continue;
        }
        current.push(ch);
    }
    out.push(current.trim().to_string());
    out
}

fn csv_vs_table_columns(table: &TableSchema, headers: &[String]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let table_cols_lower: Vec<String> = table.columns.iter().map(|c| c.name.to_ascii_lowercase()).collect();
    let header_lower: Vec<String> = headers.iter().map(|h| h.to_ascii_lowercase()).collect();

    let matched: Vec<String> = headers
        .iter()
        .filter(|h| table_cols_lower.iter().any(|t| t == &h.to_ascii_lowercase()))
        .cloned()
        .collect();

    let csv_only: Vec<String> = headers
        .iter()
        .filter(|h| !table_cols_lower.iter().any(|t| t == &h.to_ascii_lowercase()))
        .cloned()
        .collect();

    let table_only: Vec<String> = table
        .columns
        .iter()
        .map(|c| c.name.clone())
        .filter(|t| !header_lower.iter().any(|h| h == &t.to_ascii_lowercase()))
        .collect();

    (matched, csv_only, table_only)
}

fn shift_mapping(current: Option<usize>, source_len: usize, delta: i32) -> Option<usize> {
    let total = source_len + 1;
    if total == 0 {
        return None;
    }
    let current_pos = current.map(|v| v + 1).unwrap_or(0) as i32;
    let mut next = current_pos + delta;
    if next < 0 {
        next = total as i32 - 1;
    } else if next >= total as i32 {
        next = 0;
    }
    if next == 0 {
        None
    } else {
        Some((next - 1) as usize)
    }
}

fn validate_csv_mapping(
    table: &TableSchema,
    target_columns: &[String],
    mappings: &[Option<usize>],
    headers: &[String],
) -> MappingValidation {
    let mut blocking_errors = Vec::new();
    let mut warnings = Vec::new();
    let mut used_sources: HashMap<usize, usize> = HashMap::new();

    for (idx, target) in target_columns.iter().enumerate() {
        let required = table
            .columns
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(target))
            .map(|c| !c.nullable && !c.primary_key)
            .unwrap_or(false);
        let has_mapping = mappings.get(idx).and_then(|m| *m).is_some();
        if required && !has_mapping && !has_implicit_default_column(target) {
            blocking_errors.push(format!("coluna obrigatoria sem mapeamento: {}", target));
        }
        if let Some(Some(source_idx)) = mappings.get(idx) {
            *used_sources.entry(*source_idx).or_insert(0) += 1;
        }
    }
    for (source_idx, use_count) in used_sources {
        if use_count > 1 {
            let source_name = headers.get(source_idx).cloned().unwrap_or_else(|| format!("col#{}", source_idx));
            blocking_errors.push(format!("coluna CSV mapeada mais de uma vez: {}", source_name));
        }
    }

    let (_, csv_only, table_only) = csv_vs_table_columns(table, headers);
    if !csv_only.is_empty() {
        warnings.push(format!(
            "colunas presentes no CSV e ausentes na tabela: {}",
            csv_only.join(", ")
        ));
    }
    if !table_only.is_empty() {
        warnings.push(format!(
            "colunas da tabela sem correspondente no CSV: {}",
            table_only.join(", ")
        ));
    }
    MappingValidation { blocking_errors, warnings }
}

fn has_implicit_default_column(column_name: &str) -> bool {
    matches!(
        column_name.to_ascii_lowercase().as_str(),
        "role" | "is_active" | "status" | "created_at" | "updated_at"
    )
}

fn render_csv_preview(table: &TableSchema, delimiter: char, headers: &[String], rows: &[Vec<String>]) -> String {
    let mut out = Vec::new();
    let (matched, csv_only, table_only) = csv_vs_table_columns(table, headers);
    out.push(format!("Delimitador detectado: '{}'", delimiter));
    out.push(format!("Tabela alvo: {}", table.name));
    out.push(format!("Colunas CSV ({}): {}", headers.len(), headers.join(" | ")));
    out.push(format!(
        "Colunas tabela ({}): {}",
        table.columns.len(),
        table.columns.iter().map(|c| c.name.clone()).collect::<Vec<String>>().join(" | ")
    ));
    out.push(format!(
        "Match por nome ({}): {}",
        matched.len(),
        if matched.is_empty() { "-".to_string() } else { matched.join(", ") }
    ));
    out.push(format!(
        "Somente CSV ({}): {}",
        csv_only.len(),
        if csv_only.is_empty() { "-".to_string() } else { csv_only.join(", ") }
    ));
    out.push(format!(
        "Somente Tabela ({}): {}",
        table_only.len(),
        if table_only.is_empty() { "-".to_string() } else { table_only.join(", ") }
    ));
    out.push(String::new());
    out.push("Rows preview:".to_string());
    if rows.is_empty() {
        out.push("- sem linhas".to_string());
    } else {
        for row in rows.iter().take(5) {
            out.push(format!("- {}", row.join(" | ")));
        }
    }
    out.push("ALERTA: se houver divergencias, edite o CSV fora do TUI e recarregue (ENTER no caminho).".to_string());
    out.join("\n")
}

fn render_sql_schema_preview(sql: &str) -> String {
    let tables = extract_create_table_blocks(sql);
    if tables.is_empty() {
        return "Nenhuma CREATE TABLE detectada no script.".to_string();
    }
    let mut lines = Vec::new();
    for (name, block) in tables {
        lines.push(format!("[{}]", name));
        for col in parse_columns_from_create_block(&block).into_iter().take(10) {
            lines.push(format!(
                "  - {}: {}{}",
                col.name,
                col.data_type,
                if col.nullable { "" } else { " NOT NULL" }
            ));
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

fn parse_datetime(raw: Option<&str>) -> i64 {
    let Some(value) = raw else { return i64::MIN };
    let normalized = value.replace('Z', "+00:00");
    match DateTime::parse_from_rfc3339(&normalized) {
        Ok(parsed) => parsed.timestamp(),
        Err(_) => i64::MIN,
    }
}

fn parse_date(raw: Option<&str>) -> Option<NaiveDate> {
    let value = raw?.trim();
    if value.len() < 10 {
        return None;
    }
    NaiveDate::parse_from_str(&value[..10], "%Y-%m-%d").ok()
}



fn clamp01(value: f64) -> f64 {
    value.max(0.0).min(1.0)
}

fn derive_progress(day: i32, total_days: i32, fallback_progress: f64) -> f64 {
    if total_days > 0 {
        clamp01(day as f64 / total_days as f64)
    } else {
        clamp01(fallback_progress)
    }
}

fn derive_urgency(engagement: f64, days_left: i32) -> String {
    let d45 = days_left <= 45;
    if d45 && engagement <= 0.1 {
        "rescue".to_string()
    } else if engagement <= 0.2 {
        "critical".to_string()
    } else if d45 || engagement < 0.6 {
        "watch".to_string()
    } else {
        "normal".to_string()
    }
}

fn derive_hormozi_score(progress: f64, engagement: f64) -> i32 {
    (((progress * 0.4 + engagement * 0.6) * 100.0).round() as i32).clamp(0, 100)
}

fn classify_quadrant(progress: f64, engagement: f64, prd_thr: f64, eng_thr: f64) -> String {
    if progress >= prd_thr && engagement >= eng_thr {
        "topRight".to_string()
    } else if progress < 0.3 && engagement < 0.3 {
        "bottomLeft".to_string()
    } else if progress < prd_thr && engagement >= eng_thr {
        "topLeft".to_string()
    } else if progress >= prd_thr && engagement < eng_thr {
        "bottomRight".to_string()
    } else {
        "bottomLeft".to_string()
    }
}

fn normalize_checkpoint_status(raw: &str) -> String {
    match raw.to_ascii_lowercase().as_str() {
        "green" | "yellow" | "red" => raw.to_ascii_lowercase(),
        _ => "yellow".to_string(),
    }
}

fn build_anomaly_texts(direction: &str) -> (&'static str, &'static str) {
    match direction {
        "lower_better" => (
            "Aumento acima do baseline no indicador de risco.",
            "Priorizar acao corretiva na rotina e reforcar acompanhamento com o mentor.",
        ),
        "target_range" => (
            "Oscilacao fora da faixa esperada para o eixo.",
            "Ajustar plano de execucao e revisar checkpoints de curto prazo.",
        ),
        _ => (
            "Queda de consistencia no indicador de execucao.",
            "Reforcar rotina semanal e revisar bloqueios com o mentor.",
        ),
    }
}

fn is_anomaly(direction: &str, baseline: f64, current: f64, improving_trend: Option<bool>) -> bool {
    if improving_trend == Some(false) {
        return true;
    }
    match direction {
        "lower_better" => current > baseline,
        "target_range" => (current - baseline).abs() >= 5.0,
        _ => current < baseline,
    }
}

fn build_axis_insight(axis_label: &str, baseline: f64, current: f64, projected: f64) -> String {
    let delta_current = (current - baseline).round();
    let delta_projection = (projected - current).round();
    if delta_current < 0.0 {
        format!("{axis_label}: abaixo do baseline. Reforcar execucao semanal e remover bloqueios.")
    } else if delta_projection <= 0.0 {
        format!("{axis_label}: manter consistencia para preservar o resultado atual.")
    } else if delta_projection >= 8.0 {
        format!("{axis_label}: alta alavanca para o proximo ciclo com ganho projetado relevante.")
    } else {
        format!("{axis_label}: evolucao positiva e espaco de consolidacao no proximo ciclo.")
    }
}

fn avg(values: Vec<f64>) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn axis_sub_from_metadata(metadata: &serde_json::Value) -> Option<String> {
    let obj = metadata.as_object()?;
    obj.get("axis_sub")?.as_str().map(ToOwned::to_owned)
}

fn backup_stores(data: &DataHub) -> io::Result<PathBuf> {
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
    let destination = data.base_dir.join("sql").join("backups").join(timestamp);
    fs::create_dir_all(&destination)?;
    let mut copied = 0usize;
    if data.sqlite_db_path.exists() {
        let file_name = data.sqlite_db_path.file_name().unwrap_or_default();
        fs::copy(&data.sqlite_db_path, destination.join(file_name))?;
        copied += 1;
    }
    if data.schema_sql_path.exists() {
        let file_name = data.schema_sql_path.file_name().unwrap_or_default();
        fs::copy(&data.schema_sql_path, destination.join(file_name))?;
        copied += 1;
    }
    if copied == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Nenhum artefato encontrado para backup (db/sql).",
        ));
    }
    Ok(destination)
}

fn draw(app: &mut App, frame: &mut Frame, data: &DataHub) {
    match &app.route {
        Route::Login => draw_login(frame, app),
        Route::Main => draw_main(frame, app),
        Route::ManageDb => draw_manage_db(frame, app, data),
        Route::Matrix => draw_matrix(frame, app, data),
        Route::CommandCenter => draw_command_center(frame, app, data),
        Route::CommandDetail { student_id } => draw_command_detail(frame, app, data, student_id),
        Route::RadarProviderMentor => draw_provider_mentor(frame, app, data),
        Route::RadarProviderStudent {
            mentor_id,
            mentor_name,
        } => draw_provider_students(frame, app, data, mentor_id, mentor_name),
        Route::RadarProviderView {
            mentor_name,
            student_id,
            mentor_id,
            student_name,
        } => draw_provider_radar(
            frame,
            app,
            data,
            mentor_name,
            mentor_id,
            student_id,
            student_name,
        ),
        Route::RadarClient => draw_client_picker(frame, app, data),
        Route::RadarClientView {
            student_id,
            student_name,
        } => draw_client_radar(frame, app, data, student_id, student_name),
    }
}

fn draw_login(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 50, frame.area());
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Admin TUI - Login (root/toor)");
    frame.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(1),
        ])
        .split(area);

    let user_style = if !app.login_focus_password {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let pass_style = if app.login_focus_password {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Username: ", user_style),
            Span::raw(app.login_username.as_str()),
        ])),
        inner[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Password: ", pass_style),
            Span::raw("*".repeat(app.login_password.len())),
        ])),
        inner[1],
    );
    frame.render_widget(
        Paragraph::new("TAB alterna campo | ENTER login | ESC sair")
            .style(Style::default().fg(Color::DarkGray)),
        inner[2],
    );
    if !app.login_error.is_empty() {
        frame.render_widget(
            Paragraph::new(app.login_error.clone()).style(Style::default().fg(Color::Red)),
            inner[3],
        );
    }
}

fn draw_main(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    frame.render_widget(
        Paragraph::new("Painel Principal")
            .block(Block::default().borders(Borders::ALL).title("Admin TUI"))
            .style(Style::default().add_modifier(Modifier::BOLD)),
        chunks[0],
    );

    let items = vec![
        "1 - Manage DB",
        "2 - View Matrix Decision",
        "3 - View Command Center",
        "4 - View Radar as Provider",
        "5 - View Radar as Client",
    ];
    let list_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();
    let mut state = ListState::default().with_selected(Some(app.main_idx));
    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Opcoes"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
        .highlight_symbol(">> ");
    frame.render_stateful_widget(list, chunks[1], &mut state);
    frame.render_widget(
        Paragraph::new("UP/DOWN navega | ENTER seleciona | Q sai"),
        chunks[2],
    );
}

fn draw_manage_db(frame: &mut Frame, app: &App, data: &DataHub) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
        .split(area);
    frame.render_widget(
        Paragraph::new("Manage DB")
            .block(Block::default().borders(Borders::ALL).title("Admin TUI")),
        chunks[0],
    );

    match app.manage_stage {
        ManageStage::Menu => {
            let items = vec![
                ListItem::new("0 - Schema Diagram"),
                ListItem::new("1.1 - Load from CSV"),
                ListItem::new("1.3 - Manual Insert"),
                ListItem::new("1.2 - Load from SQL"),
            ];
            let mut state = ListState::default().with_selected(Some(app.manage_menu_idx.min(3)));
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Subopcoes"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                chunks[1],
                &mut state,
            );
            frame.render_widget(
                Paragraph::new("UP/DOWN navega | ENTER abre | Q/ESC volta"),
                chunks[2],
            );
        }
        ManageStage::SchemaDiagram => {
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
                .split(chunks[1]);
            let table_items: Vec<ListItem> = data
                .table_schemas
                .iter()
                .map(|t| ListItem::new(t.name.clone()))
                .collect();
            let mut state = ListState::default()
                .with_selected(Some(app.schema_table_idx.min(table_items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(table_items)
                    .block(Block::default().borders(Borders::ALL).title("Tabelas"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                body[0],
                &mut state,
            );

            let schema_text = data
                .table_schemas
                .get(app.schema_table_idx)
                .map(|table| render_class_diagram_for_table(table, &data.foreign_keys))
                .unwrap_or_else(|| "Sem schema".to_string());
            frame.render_widget(
                Paragraph::new(schema_text)
                    .block(Block::default().borders(Borders::ALL).title("Class Diagram"))
                    .wrap(Wrap { trim: false }),
                body[1],
            );
            frame.render_widget(
                Paragraph::new(format!(
                    "UP/DOWN tabela | B backup sqlite+schema | R reload schema | Q/ESC voltar | {}",
                    app.message
                )),
                chunks[2],
            );
        }
        ManageStage::CsvSelectTable => {
            let items: Vec<ListItem> = data
                .table_schemas
                .iter()
                .map(|t| ListItem::new(format!("{} ({} colunas)", t.name, t.columns.len())))
                .collect();
            let mut state = ListState::default()
                .with_selected(Some(app.csv_table_idx.min(items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("1.1 Select Table"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                chunks[1],
                &mut state,
            );
            frame.render_widget(
                Paragraph::new("UP/DOWN tabela | ENTER continuar | Q/ESC voltar"),
                chunks[2],
            );
        }
        ManageStage::CsvFileDialog => {
            // Renderização simples do file dialog
            let area = frame.area();
            let block = Block::default().borders(Borders::ALL).title("Selecione um arquivo CSV");
            frame.render_widget(block, area);
            if let Some(dialog) = &app.file_dialog {
                let entries: Vec<ListItem> = dialog.entries.iter().enumerate().map(|(i, path)| {
                    let name = if path.is_dir() {
                        format!("[DIR] {}", path.file_name().unwrap_or_default().to_string_lossy())
                    } else {
                        path.file_name().unwrap_or_default().to_string_lossy().to_string()
                    };
                    ListItem::new(name)
                }).collect();
                let mut state = ListState::default().with_selected(Some(dialog.selected));
                frame.render_stateful_widget(
                    List::new(entries)
                        .block(Block::default().borders(Borders::ALL).title(dialog.current_dir.display().to_string()))
                        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                        .highlight_symbol(">> "),
                    area,
                    &mut state,
                );
            }
            // Instruções
            let bottom = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(2)])
                .split(area);
            frame.render_widget(
                Paragraph::new("UP/DOWN navega | ENTER entra/seleciona | ESC volta").style(Style::default().fg(Color::DarkGray)),
                bottom[1],
            );
        }
        ManageStage::CsvPathInput => {
            frame.render_widget(
                Paragraph::new(format!(
                    "Digite o caminho do CSV e ENTER (suporta ; , TAB |):\n{}",
                    app.csv_path_input
                ))
                .block(Block::default().borders(Borders::ALL).title("1.1 CSV Path")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!(
                    "ENTER recarrega header e comparacao CSV x Tabela | Q/ESC voltar | {}",
                    app.message
                )),
                chunks[2],
            );
        }
        ManageStage::CsvMapping => {
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let mapping_items: Vec<ListItem> = app
                .csv_target_columns
                .iter()
                .enumerate()
                .map(|(idx, target)| {
                    let source = app
                        .csv_mappings
                        .get(idx)
                        .and_then(|m| *m)
                        .and_then(|pos| app.csv_headers.get(pos).cloned())
                        .unwrap_or_else(|| "<sem mapeamento>".to_string());
                    ListItem::new(format!("{} <- {}", target, source))
                })
                .collect();
            let mut state = ListState::default()
                .with_selected(Some(app.csv_mapping_idx.min(mapping_items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(mapping_items)
                    .block(Block::default().borders(Borders::ALL).title("De-Para Manual"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                body[0],
                &mut state,
            );

            let preview = data
                .table_schemas
                .get(app.csv_table_idx)
                .map(|table| {
                    render_csv_preview(table, app.csv_delimiter, &app.csv_headers, &app.csv_rows_preview)
                })
                .unwrap_or_else(|| "Tabela nao encontrada".to_string());
            frame.render_widget(
                Paragraph::new(preview)
                    .block(Block::default().borders(Borders::ALL).title("CSV Preview"))
                    .wrap(Wrap { trim: false }),
                body[1],
            );
            frame.render_widget(
                Paragraph::new(format!(
                    "UP/DOWN coluna alvo | LEFT/RIGHT mapeia origem | V validar | L carregar via API | Q/ESC voltar | {}",
                    app.message
                )),
                chunks[2],
            );

            if app.message_popup_open {
                let popup = centered_rect(75, 70, frame.area());
                frame.render_widget(Clear, popup);
                frame.render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(if app.message_popup_title.is_empty() {
                            "Resultado"
                        } else {
                            app.message_popup_title.as_str()
                        })
                        .style(Style::default().bg(Color::Black)),
                    popup,
                );
                let inner = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(popup);
                frame.render_widget(
                    Paragraph::new(app.message.clone()).wrap(Wrap { trim: false }),
                    inner[0],
                );
                frame.render_widget(
                    Paragraph::new("ENTER/ESC/Q fecha").style(Style::default().fg(Color::DarkGray)),
                    inner[1],
                );
            }
        }
        ManageStage::ManualSelectTable => {
            let items: Vec<ListItem> = data
                .table_schemas
                .iter()
                .map(|t| ListItem::new(format!("{} ({} colunas)", t.name, t.columns.len())))
                .collect();
            let mut state =
                ListState::default().with_selected(Some(app.manual_table_idx.min(items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("1.3 Manual Insert - Select Table"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                chunks[1],
                &mut state,
            );
            frame.render_widget(
                Paragraph::new("UP/DOWN tabela | ENTER editar registro | Q/ESC voltar"),
                chunks[2],
            );
        }
        ManageStage::ManualEditRow => {
            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let value_items: Vec<ListItem> = app
                .manual_field_names
                .iter()
                .enumerate()
                .map(|(idx, name)| {
                    let mut val = app.manual_values.get(name).cloned().unwrap_or_default();
                    if app.manual_editing && idx == app.manual_field_idx {
                        val = format!("{}_", app.manual_edit_buffer);
                    } else if val.trim().is_empty() {
                        val = "<vazio>".to_string();
                    }
                    ListItem::new(format!("{} = {}", name, val))
                })
                .collect();
            let mut state =
                ListState::default().with_selected(Some(app.manual_field_idx.min(value_items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(value_items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(format!("1.3 Manual Insert - {}", app.manual_table_name)),
                    )
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                body[0],
                &mut state,
            );

            let mut guidance = Vec::new();
            if let Some(table) = data
                .table_schemas
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(&app.manual_table_name))
            {
                guidance.push(format!("Tabela: {}", table.name));
                guidance.push("Relacionamentos (FK):".to_string());
                let mut has_fk = false;
                for fk in data.foreign_keys.iter().filter(|fk| fk.table.eq_ignore_ascii_case(&table.name)) {
                    has_fk = true;
                    guidance.push(format!("- {} -> {}.{}", fk.column, fk.ref_table, fk.ref_column));
                }
                if !has_fk {
                    guidance.push("- (nenhum)".to_string());
                }
                guidance.push(String::new());
                guidance.push("Regras:".to_string());
                guidance.push("- campos NOT NULL devem ser preenchidos".to_string());
                guidance.push("- campos FK devem receber id numerico".to_string());
            }
            frame.render_widget(
                Paragraph::new(guidance.join("\n"))
                    .block(Block::default().borders(Borders::ALL).title("Guidance"))
                    .wrap(Wrap { trim: false }),
                body[1],
            );

            frame.render_widget(
                Paragraph::new(
                    "UP/DOWN campo | ENTER editar/salvar | C limpar | V validar | S inserir via API | Q/ESC voltar",
                ),
                chunks[2],
            );

            if app.message_popup_open {
                let popup = centered_rect(75, 70, frame.area());
                frame.render_widget(Clear, popup);
                frame.render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(if app.message_popup_title.is_empty() {
                            "Resultado"
                        } else {
                            app.message_popup_title.as_str()
                        })
                        .style(Style::default().bg(Color::Black)),
                    popup,
                );
                let inner = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Min(1), Constraint::Length(1)])
                    .split(popup);
                frame.render_widget(
                    Paragraph::new(app.message.clone()).wrap(Wrap { trim: false }),
                    inner[0],
                );
                frame.render_widget(
                    Paragraph::new("ENTER/ESC/Q fecha").style(Style::default().fg(Color::DarkGray)),
                    inner[1],
                );
            }
        }
        ManageStage::SqlDirInput => {
            frame.render_widget(
                Paragraph::new(format!(
                    "Digite diretorio com scripts SQL e ENTER:\n{}",
                    app.sql_dir_input
                ))
                .block(Block::default().borders(Borders::ALL).title("1.2 SQL Directory")),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(format!("ENTER listar .sql | Q/ESC voltar | {}", app.message)),
                chunks[2],
            );
        }
        ManageStage::SqlSelectScript => {
            let items: Vec<ListItem> = app
                .sql_scripts
                .iter()
                .map(|p| {
                    let name = Path::new(p)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(p);
                    ListItem::new(name.to_string())
                })
                .collect();
            let mut state = ListState::default()
                .with_selected(Some(app.sql_script_idx.min(items.len().saturating_sub(1))));
            frame.render_stateful_widget(
                List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("1.2 Select Script"))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                    .highlight_symbol(">> "),
                chunks[1],
                &mut state,
            );
            frame.render_widget(
                Paragraph::new("UP/DOWN script | ENTER schema+confirm | Q/ESC voltar"),
                chunks[2],
            );
        }
        ManageStage::SqlConfirm => {
            let script = app
                .sql_confirm_script
                .as_ref()
                .cloned()
                .unwrap_or_else(|| "-".to_string());
            let schema_preview = if app.sql_confirm_preview.is_empty() {
                "Sem preview".to_string()
            } else {
                app.sql_confirm_preview.clone()
            };
            frame.render_widget(
                Paragraph::new(format!(
                    "Script: {}\n\nSchema detectado:\n{}\n\nConfirmar execucao? (Y/N)",
                    script, schema_preview
                ))
                .block(Block::default().borders(Borders::ALL).title("1.2 Confirm SQL Load"))
                .wrap(Wrap { trim: false }),
                chunks[1],
            );
            frame.render_widget(Paragraph::new("Y executa | N/ESC cancela"), chunks[2]);
        }
        ManageStage::SqlResult => {
            frame.render_widget(
                Paragraph::new(app.sql_last_result.clone())
                    .block(Block::default().borders(Borders::ALL).title("1.2 SQL Result"))
                    .wrap(Wrap { trim: false }),
                chunks[1],
            );
            frame.render_widget(Paragraph::new("ENTER/Q/ESC voltar"), chunks[2]);
        }
    }
}

fn draw_matrix(frame: &mut Frame, app: &App, data: &DataHub) {
    let filters = ["all", "topRight", "critical", "rescue"];
    let filter = filters[app.matrix_filter_idx % filters.len()];
    let (rows, total_ltv, critical, rescue, avg_eng) = data.matrix_items(filter);

    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1), Constraint::Length(2)])
        .split(area);

    let header = format!(
        "View Matrix Decision | filtro={} | LTV={} | critical={} | rescue={} | avgEng={:.2}%",
        filter, total_ltv, critical, rescue, avg_eng
    );
    frame.render_widget(
        Paragraph::new(header).block(Block::default().borders(Borders::ALL).title("Matrix")),
        chunks[0],
    );

    let lines: Vec<ListItem> = rows
        .iter()
        .take(30)
        .map(|r| {
            ListItem::new(format!(
                "{} | {} | q={} | p={:.2} e={:.2} | dLeft={} | {}",
                r.id, r.name, r.quadrant, r.progress, r.engagement, r.days_left, r.urgency
            ))
        })
        .collect();
    frame.render_widget(
        List::new(lines).block(Block::default().borders(Borders::ALL).title("Items (top 30)")),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new("LEFT/RIGHT altera filtro | R reload | Q/ESC voltar"),
        chunks[2],
    );
}

fn draw_command_center(frame: &mut Frame, app: &App, data: &DataHub) {
    let rows = data.list_command_center_students(None);
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    frame.render_widget(
        Paragraph::new(format!("View Command Center | alunos={}", rows.len()))
            .block(Block::default().borders(Borders::ALL).title("Command Center")),
        chunks[0],
    );
    let list_items: Vec<ListItem> = rows
        .iter()
        .map(|r| {
            ListItem::new(format!(
                "{} | {} | {} | dLeft={} | urg={} | p={:.2} e={:.2} | hs={}",
                r.id, r.name, r.program_name, r.days_left, r.urgency, r.progress, r.engagement, r.hormozi_score
            ))
        })
        .collect();
    let mut state = ListState::default().with_selected(Some(app.cc_idx.min(list_items.len().saturating_sub(1))));
    frame.render_stateful_widget(
        List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Alunos"))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .highlight_symbol(">> "),
        chunks[1],
        &mut state,
    );
    frame.render_widget(
        Paragraph::new("UP/DOWN seleciona | ENTER detalhe | Q/ESC voltar"),
        chunks[2],
    );
}

fn draw_command_detail(frame: &mut Frame, _app: &App, data: &DataHub, student_id: &str) {
    let area = frame.area();
    let Some(payload) = data.command_detail(student_id) else {
        frame.render_widget(
            Paragraph::new("Aluno sem detalhe").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
            Constraint::Length(2),
        ])
        .split(area);

    let top = format!(
        "{} | {} | ciclo {}/{} dLeft={} urg={} | p={:.2} e={:.2} hs={} | anomalies={} cw={} lw={}",
        payload.summary.id,
        payload.summary.name,
        payload.summary.day,
        payload.summary.total_days,
        payload.summary.days_left,
        payload.summary.urgency,
        payload.summary.progress,
        payload.summary.engagement,
        payload.summary.hormozi_score,
        payload.anomaly_count,
        payload.current_week,
        payload.last_week
    );
    frame.render_widget(
        Paragraph::new(top).block(Block::default().borders(Borders::ALL).title("Command Detail")),
        chunks[0],
    );

    let metric_lines: Vec<ListItem> = payload
        .metrics
        .iter()
        .take(18)
        .map(|m| {
            ListItem::new(format!(
                "{} | base={:.2} atual={:.2} proj={} {}",
                m.label,
                m.baseline,
                m.current,
                m.projected
                    .map(|v| format!("{v:.2}"))
                    .unwrap_or_else(|| "-".to_string()),
                m.unit
            ))
        })
        .collect();
    frame.render_widget(
        List::new(metric_lines).block(Block::default().borders(Borders::ALL).title("Metricas")),
        chunks[1],
    );

    let timeline_lines: Vec<ListItem> = payload
        .timeline
        .iter()
        .take(18)
        .map(|t| {
            ListItem::new(format!(
                "w{} | {} | {} | marker={} | action={}",
                t.week, t.status, t.label, t.marker, t.action
            ))
        })
        .collect();
    frame.render_widget(
        List::new(timeline_lines).block(Block::default().borders(Borders::ALL).title("Timeline")),
        chunks[2],
    );
    frame.render_widget(Paragraph::new("Q/ESC voltar"), chunks[3]);
}

fn draw_provider_mentor(frame: &mut Frame, app: &App, data: &DataHub) {
    let providers = data.list_providers();
    let mentors: Vec<&User> = providers.iter().filter(|u| u.is_active.unwrap_or(true)).collect();
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    frame.render_widget(
        Paragraph::new("View Radar as Provider - selecione mentor")
            .block(Block::default().borders(Borders::ALL).title("Radar Provider")),
        chunks[0],
    );
    let items: Vec<ListItem> = mentors
        .iter()
        .map(|u| ListItem::new(format!("{} | {}", u.id, u.full_name)))
        .collect();
    let mut state =
        ListState::default().with_selected(Some(app.provider_mentor_idx.min(items.len().saturating_sub(1))));
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Mentores"))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .highlight_symbol(">> "),
        chunks[1],
        &mut state,
    );
    frame.render_widget(
        Paragraph::new("UP/DOWN seleciona | ENTER escolher mentor | Q/ESC voltar"),
        chunks[2],
    );
}

fn draw_provider_students(
    frame: &mut Frame,
    app: &App,
    data: &DataHub,
    mentor_id: &str,
    mentor_name: &str,
) {
    let rows = data.list_command_center_students(Some(mentor_id));
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    frame.render_widget(
        Paragraph::new(format!(
            "View Radar as Provider | mentor={} ({}) | alunos={}",
            mentor_name,
            mentor_id,
            rows.len()
        ))
        .block(Block::default().borders(Borders::ALL).title("Radar Provider")),
        chunks[0],
    );
    let items: Vec<ListItem> = rows
        .iter()
        .map(|r| ListItem::new(format!("{} | {} | {} | urg={}", r.id, r.name, r.program_name, r.urgency)))
        .collect();
    let mut state =
        ListState::default().with_selected(Some(app.provider_student_idx.min(items.len().saturating_sub(1))));
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Alunos do mentor"))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .highlight_symbol(">> "),
        chunks[1],
        &mut state,
    );
    frame.render_widget(
        Paragraph::new("UP/DOWN seleciona | ENTER abrir radar | Q/ESC voltar"),
        chunks[2],
    );
}

fn draw_provider_radar(
    frame: &mut Frame,
    _app: &App,
    data: &DataHub,
    mentor_name: &str,
    mentor_id: &str,
    student_id: &str,
    student_name: &str,
) {
    let area = frame.area();
    let Some(radar) = data.radar_for_student(student_id, Some(mentor_id)) else {
        frame.render_widget(
            Paragraph::new("Radar nao encontrado").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    };
    draw_radar_payload(frame, area, "Radar as Provider", Some(mentor_name), student_name, &radar);
}

fn draw_client_picker(frame: &mut Frame, app: &App, data: &DataHub) {
    let rows = data.list_command_center_students(None);
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    frame.render_widget(
        Paragraph::new(format!("View Radar as Client | alunos={}", rows.len()))
            .block(Block::default().borders(Borders::ALL).title("Radar Client")),
        chunks[0],
    );
    let items: Vec<ListItem> = rows
        .iter()
        .map(|r| ListItem::new(format!("{} | {} | {}", r.id, r.name, r.program_name)))
        .collect();
    let mut state =
        ListState::default().with_selected(Some(app.client_student_idx.min(items.len().saturating_sub(1))));
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Alunos"))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .highlight_symbol(">> "),
        chunks[1],
        &mut state,
    );
    frame.render_widget(
        Paragraph::new("UP/DOWN seleciona | ENTER abrir radar | Q/ESC voltar"),
        chunks[2],
    );
}

fn draw_client_radar(
    frame: &mut Frame,
    _app: &App,
    data: &DataHub,
    student_id: &str,
    student_name: &str,
) {
    let area = frame.area();
    let Some(radar) = data.radar_for_student(student_id, None) else {
        frame.render_widget(
            Paragraph::new("Radar nao encontrado").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    };
    draw_radar_payload(frame, area, "Radar as Client", None, student_name, &radar);
}

fn draw_radar_payload(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    mentor_name: Option<&str>,
    student_name: &str,
    radar: &RadarPayload,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    let top = format!(
        "{} | mentor={} | aluno={} ({}) | protocol={} | avg base={:.2} real={:.2} meta={:.2}",
        title,
        mentor_name.unwrap_or("-"),
        student_name,
        radar.student_id,
        if radar.protocol_name.is_empty() { "-" } else { &radar.protocol_name },
        radar.avg_baseline,
        radar.avg_current,
        radar.avg_projected
    );
    frame.render_widget(
        Paragraph::new(top).block(Block::default().borders(Borders::ALL).title("Radar")),
        chunks[0],
    );
    let items: Vec<ListItem> = radar
        .axes
        .iter()
        .map(|axis| {
            ListItem::new(format!(
                "{} | base={:.2} real={:.2} meta={:.2} | {} {}",
                axis.axis_label,
                axis.baseline,
                axis.current,
                axis.projected,
                axis.axis_sub,
                axis.insight
            ))
        })
        .collect();
    frame.render_widget(
        List::new(items).block(Block::default().borders(Borders::ALL).title("Eixos")),
        chunks[1],
    );
    frame.render_widget(Paragraph::new("Q/ESC voltar"), chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn main() -> io::Result<()> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();
    let mut data = DataHub::from_base_dir(base_dir)?;
    let mut app = App::default();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| draw(&mut app, f, &data))?;
        if app.should_quit {
            break;
        }
        if event::poll(Duration::from_millis(120))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key, &mut data);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
