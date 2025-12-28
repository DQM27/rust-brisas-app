// ==========================================
// src/db/surrealdb_ingreso_contratista_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{
    IngresoContratista, IngresoContratistaCreateDTO, IngresoContratistaFetched,
    IngresoContratistaUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

/// Tabla en SurrealDB
const TABLE: &str = "ingreso_contratista";

pub async fn insert(
    dto: IngresoContratistaCreateDTO,
) -> Result<IngresoContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries
    let created: Option<IngresoContratista> =
        db.query(format!("CREATE {} CONTENT $dto", TABLE)).bind(("dto", dto)).await?.take(0)?;

    let ingreso = created.ok_or(SurrealDbError::TransactionError(
        "Error al insertar ingreso_contratista".to_string(),
    ))?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", ingreso.id.clone()))
        .await?;

    let fetched: Option<IngresoContratistaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Ingreso creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_ingreso_abierto_by_contratista(
    contratista_id: &RecordId,
) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query(format!(
            "SELECT * FROM {} WHERE contratista = $contratista AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa",
            TABLE
        ))
        .bind(("contratista", contratista_id.clone()))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update_salida(
    ingreso_id: &RecordId,
    usuario_salida_id: &RecordId,
    observaciones: Option<String>,
) -> Result<IngresoContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    let mut dto = IngresoContratistaUpdateDTO::default();
    dto.fecha_hora_salida = Some(surrealdb::Datetime::from(chrono::Utc::now()));
    dto.usuario_salida = Some(usuario_salida_id.clone());
    dto.observaciones = observaciones;

    // UPDATE doesn't support FETCH, so we need two queries
    let _: Option<IngresoContratista> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", ingreso_id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", ingreso_id.clone()))
        .await?;

    let fetched: Option<IngresoContratistaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError("Error al registrar salida".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<IngresoContratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<IngresoContratista> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_id_fetched(
    id: &RecordId,
) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}
