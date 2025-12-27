// ==========================================
// src/db/surrealdb_audit_queries.rs
// ==========================================

use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use chrono::Utc;

pub async fn insert_praind_historial(
    contratista_id: &str,
    fecha_anterior: Option<&str>,
    fecha_nueva: &str,
    usuario_id: &str,
    motivo: Option<&str>,
) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = Utc::now().to_rfc3339();
    let c_id = contratista_id.strip_prefix("contratistas:").unwrap_or(contratista_id);
    let u_id = usuario_id.strip_prefix("usuarios:").unwrap_or(usuario_id);

    let _: Option<serde_json::Value> = client
        .create("audit_praind")
        .content(serde_json::json!({
            "contratista": format!("contratistas:{}", c_id),
            "fecha_anterior": fecha_anterior,
            "fecha_nueva": fecha_nueva,
            "usuario": format!("usuarios:{}", u_id),
            "motivo": motivo,
            "created_at": now
        }))
        .await?;

    Ok(())
}

pub async fn insert_historial_estado(
    contratista_id: &str,
    estado_anterior: &str,
    estado_nuevo: &str,
    usuario_id: Option<&str>,
    motivo: &str,
) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = Utc::now().to_rfc3339();
    let c_id = contratista_id.strip_prefix("contratistas:").unwrap_or(contratista_id);
    let u_id = usuario_id.map(|id| id.strip_prefix("usuarios:").unwrap_or(id));

    let _: Option<serde_json::Value> = client
        .create("audit_estado")
        .content(serde_json::json!({
            "contratista": format!("contratistas:{}", c_id),
            "estado_anterior": estado_anterior,
            "estado_nuevo": estado_nuevo,
            "usuario": u_id.map(|id| format!("usuarios:{}", id)),
            "motivo": motivo,
            "created_at": now
        }))
        .await?;

    Ok(())
}
