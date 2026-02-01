// ==========================================
// src/db/surrealdb_audit_queries.rs
// ==========================================

use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn insert_praind_historial(
    contratista_id: &str,
    fecha_anterior: Option<&str>,
    fecha_nueva: &str,
    usuario_id: &str,
    motivo: Option<&str>,
) -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Convert all to owned types for bind() which requires 'static
    let c_id = format!(
        "contratista:{}",
        contratista_id.strip_prefix("contratista:").unwrap_or(contratista_id)
    );
    let u_id = format!("user:{}", usuario_id.strip_prefix("user:").unwrap_or(usuario_id));
    let fecha_anterior_owned = fecha_anterior.map(std::string::ToString::to_string);
    let fecha_nueva_owned = fecha_nueva.to_string();
    let motivo_owned = motivo.map(std::string::ToString::to_string);

    let _: Option<serde_json::Value> = db
        .query(
            r"
            CREATE audit_praind CONTENT {
                contratista_id: $contratista_id,
                fecha_anterior: $fecha_anterior,
                fecha_nueva: $fecha_nueva,
                usuario_id: $usuario_id,
                motivo: $motivo,
                created_at: time::now()
            }
        ",
        )
        .bind(("contratista_id", c_id))
        .bind(("fecha_anterior", fecha_anterior_owned))
        .bind(("fecha_nueva", fecha_nueva_owned))
        .bind(("usuario_id", u_id))
        .bind(("motivo", motivo_owned))
        .await?
        .take(0)?;

    Ok(())
}

pub async fn insert_historial_estado(
    contratista_id: &str,
    estado_anterior: &str,
    estado_nuevo: &str,
    usuario_id: Option<&str>,
    motivo: &str,
) -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    // Convert all to owned types for bind() which requires 'static
    let c_id = format!(
        "contratista:{}",
        contratista_id.strip_prefix("contratista:").unwrap_or(contratista_id)
    );
    let u_id = usuario_id.map(|id| format!("user:{}", id.strip_prefix("user:").unwrap_or(id)));
    let estado_anterior_owned = estado_anterior.to_string();
    let estado_nuevo_owned = estado_nuevo.to_string();
    let motivo_owned = motivo.to_string();

    let _: Option<serde_json::Value> = db
        .query(
            r"
            CREATE audit_estado CONTENT {
                contratista_id: $contratista_id,
                estado_anterior: $estado_anterior,
                estado_nuevo: $estado_nuevo,
                usuario_id: $usuario_id,
                motivo: $motivo,
                created_at: time::now()
            }
        ",
        )
        .bind(("contratista_id", c_id))
        .bind(("estado_anterior", estado_anterior_owned))
        .bind(("estado_nuevo", estado_nuevo_owned))
        .bind(("usuario_id", u_id))
        .bind(("motivo", motivo_owned))
        .await?
        .take(0)?;

    Ok(())
}

// ==========================================
// SYSTEM AUDIT LOGS
// ==========================================

pub async fn insert_sys_log(
    terminal_id: String,
    terminal_name: String,
    user_name: String,
    event_type: String, // LOGIN, LOGOUT, TIMEOUT
    duration: Option<String>,
    ip_address: Option<String>,
    details: Option<String>,
) -> Result<(), SurrealDbError> {
    let db = get_db().await?;

    let duration_val = duration.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null);
    let ip_val = ip_address.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null);
    let details_val = details.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null);

    let _: Option<serde_json::Value> = db
        .query(
            r"
            CREATE sys_audit_log CONTENT {
                terminal_id: $terminal_id,
                terminal_name: $terminal_name,
                user_name: $user_name,
                event_type: $event_type,
                duration: $duration,
                ip_address: $ip_address,
                details: $details,
                access_date: time::now()
            }
        ",
        )
        .bind(("terminal_id", terminal_id))
        .bind(("terminal_name", terminal_name))
        .bind(("user_name", user_name))
        .bind(("event_type", event_type))
        .bind(("duration", duration_val))
        .bind(("ip_address", ip_val))
        .bind(("details", details_val))
        .await?
        .take(0)?;

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SysLogEntry {
    pub id: surrealdb::sql::Thing,
    pub terminal_id: String,
    pub terminal_name: String,
    pub user_name: String,
    pub event_type: String,
    pub duration: Option<String>,
    pub ip_address: Option<String>,
    pub details: Option<String>,
    pub access_date: surrealdb::sql::Datetime,
}

pub async fn get_sys_logs(
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<Vec<SysLogEntry>, SurrealDbError> {
    let db = get_db().await?;
    let limit_val = limit.unwrap_or(50);
    let offset_val = offset.unwrap_or(0);

    let logs: Vec<SysLogEntry> = db
        .query("SELECT * FROM sys_audit_log ORDER BY access_date DESC LIMIT $limit START $offset")
        .bind(("limit", limit_val))
        .bind(("offset", offset_val))
        .await?
        .take(0)?;

    Ok(logs)
}
