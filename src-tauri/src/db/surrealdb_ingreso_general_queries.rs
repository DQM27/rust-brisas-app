// ==========================================
// src/db/surrealdb_ingreso_general_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{IngresoContratista, IngresoContratistaFetched};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

// NOTE: This "general" query module currently focusing on contractors
// but can be expanded to multi-table SELECT [ingreso_contratista, ingreso_proveedor, ingreso_visita]

pub async fn find_all() -> Result<Vec<IngresoContratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("SELECT * FROM ingreso_contratista ORDER BY created_at DESC LIMIT 500").await?;
    Ok(result.take(0)?)
}

pub async fn find_all_fetched() -> Result<Vec<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso_contratista ORDER BY created_at DESC LIMIT 500 FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_ingresos_abiertos_fetched(
) -> Result<Vec<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso_contratista WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
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

pub async fn find_ingreso_by_gafete_fetched(
    gafete: &str,
) -> Result<Option<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM [ingreso_contratista, ingreso_proveedor, ingreso_visita] WHERE gafete_numero = $gafete AND fecha_hora_salida IS NONE LIMIT 1 FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("gafete", gafete.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_salidas_in_range_fetched(
    start: &str,
    end: &str,
) -> Result<Vec<IngresoContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso_contratista WHERE fecha_hora_salida >= $start AND fecha_hora_salida <= $end ORDER BY fecha_hora_salida DESC FETCH usuario_ingreso, usuario_salida, contratista, contratista.empresa")
        .bind(("start", start.to_string()))
        .bind(("end", end.to_string()))
        .await?;

    Ok(result.take(0)?)
}
