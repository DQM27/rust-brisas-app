// ==========================================
// src/db/surrealdb_ingreso_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{Ingreso, IngresoCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn insert(dto: IngresoCreateDTO) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Ingreso> =
        db.query("CREATE ingreso CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso de proveedor".to_string(),
    ))
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
            AND tipo_ingreso = 'proveedor'
            AND fecha_hora_salida IS NONE
            LIMIT 1
        "#,
        )
        .bind(("cedula", cedula.to_string()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let mut map = serde_json::Map::new();
    map.insert(
        "fecha_hora_salida".to_string(),
        serde_json::json!(surrealdb::Datetime::from(chrono::Utc::now())),
    );
    map.insert("usuario_salida".to_string(), serde_json::json!(usuario_salida_id));
    map.insert("observaciones_salida".to_string(), serde_json::json!(observaciones));
    map.insert(
        "updated_at".to_string(),
        serde_json::json!(surrealdb::Datetime::from(chrono::Utc::now())),
    );

    let result: Option<Ingreso> =
        db.update(ingreso_id.clone()).merge(serde_json::Value::Object(map)).await?;

    result.ok_or(SurrealDbError::TransactionError(
        "Error al registrar salida de proveedor".to_string(),
    ))
}

pub async fn find_activos() -> Result<Vec<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE tipo_ingreso = 'proveedor' AND fecha_hora_salida IS NONE ORDER BY fecha_hora_ingreso DESC")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_historial() -> Result<Vec<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE tipo_ingreso = 'proveedor' AND fecha_hora_salida IS NOT NONE ORDER BY fecha_hora_ingreso DESC LIMIT 100")
        .await?;
    Ok(result.take(0)?)
}
