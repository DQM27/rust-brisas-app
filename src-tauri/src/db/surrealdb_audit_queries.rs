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
