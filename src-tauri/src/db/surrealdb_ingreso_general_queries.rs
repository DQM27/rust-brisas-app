// ==========================================
// src/db/surrealdb_ingreso_general_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::ingreso::{Ingreso, IngresoFetched};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

// IngresoDetails and find_details_for_ingreso are deprecated in favor of FETCH joins

pub async fn find_all() -> Result<Vec<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM ingreso ORDER BY created_at DESC LIMIT 500").await?;
    Ok(result.take(0)?)
}

pub async fn find_all_fetched() -> Result<Vec<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso ORDER BY created_at DESC LIMIT 500 FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_ingresos_abiertos_fetched() -> Result<Vec<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE fecha_hora_salida IS NONE ORDER BY created_at DESC FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Ingreso>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Ingreso> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM $id FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}

// find_details_for_ingreso and find_all_with_details are removed in favor of find_all_fetched

// find_ingresos_abiertos_with_details is removed in favor of find_ingresos_abiertos_fetched

pub async fn find_ingreso_by_gafete_fetched(
    gafete: &str,
) -> Result<Option<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE gafete_numero = $gafete AND fecha_hora_salida IS NONE FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa LIMIT 1")
        .bind(("gafete", gafete.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_salidas_in_range_fetched(
    start: &str,
    end: &str,
) -> Result<Vec<IngresoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM ingreso WHERE fecha_hora_salida >= $start AND fecha_hora_salida <= $end ORDER BY fecha_hora_salida DESC FETCH usuario_ingreso, usuario_salida, vehiculo, contratista, contratista.empresa")
        .bind(("start", start.to_string()))
        .bind(("end", end.to_string()))
        .await?;

    Ok(result.take(0)?)
}
