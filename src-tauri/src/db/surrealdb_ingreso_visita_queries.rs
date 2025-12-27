// ==========================================
// src/db/surrealdb_ingreso_visita_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{Ingreso, IngresoCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::sql::Thing;

pub async fn insert(dto: IngresoCreateDTO) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Ingreso> =
        db.query("CREATE ingreso CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result
        .ok_or(SurrealDbError::TransactionError("Error al insertar ingreso de visita".to_string()))
}

pub async fn find_ingreso_abierto_by_cedula(
    cedula: &str,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(
            r#"
            SELECT * FROM ingreso 
            WHERE cedula = $cedula 
            AND tipo_ingreso = 'visita'
            AND fecha_hora_salida IS NONE
            LIMIT 1
        "#,
        )
        .bind(("cedula", cedula.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &Thing,
    usuario_salida_id: &Thing,
    observaciones: Option<String>,
) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let mut map = serde_json::Map::new();
    map.insert("fecha_hora_salida".to_string(), serde_json::json!(chrono::Utc::now()));
    map.insert("usuario_salida".to_string(), serde_json::json!(usuario_salida_id));
    map.insert("observaciones_salida".to_string(), serde_json::json!(observaciones));
    map.insert("updated_at".to_string(), serde_json::json!(chrono::Utc::now()));

    let result: Option<Ingreso> = db
        .update((ingreso_id.tb.clone(), ingreso_id.id.to_string()))
        .merge(serde_json::Value::Object(map))
        .await?;

    result
        .ok_or(SurrealDbError::TransactionError("Error al registrar salida de visita".to_string()))
}
