// ==========================================
// src/db/surrealdb_ingreso_contratista_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{Ingreso, IngresoCreateDTO, IngresoUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn insert(dto: IngresoCreateDTO) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Ingreso> =
        db.query("CREATE ingreso CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result.ok_or(SurrealDbError::TransactionError("Error al insertar ingreso".to_string()))
}

pub async fn find_ingreso_abierto_by_contratista(
    contratista_id: &RecordId,
) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM ingreso WHERE contratista = $contratista AND fecha_hora_salida IS NONE LIMIT 1")
        .bind(("contratista", contratista_id.clone()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<Ingreso, SurrealDbError> {
    let db = get_db().await?;

    let mut dto = IngresoUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones_salida = observaciones;
    dto.updated_at = Some(surrealdb::Datetime::from(chrono::Utc::now()));

    let result: Option<Ingreso> = db.update(ingreso_id.clone()).merge(dto).await?;

    result.ok_or(SurrealDbError::TransactionError("Error al registrar salida".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Ingreso> = db.select(id.clone()).await?;
    Ok(result)
}
