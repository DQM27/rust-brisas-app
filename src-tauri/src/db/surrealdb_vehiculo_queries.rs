// ==========================================
// src/db/surrealdb_vehiculo_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::vehiculo::{Vehiculo, VehiculoCreateDTO, VehiculoFetched, VehiculoUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn insert(dto: VehiculoCreateDTO) -> Result<VehiculoFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries
    let created: Option<Vehiculo> =
        db.query("CREATE vehiculo CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    let vehiculo = created
        .ok_or(SurrealDbError::TransactionError("Error al insertar vehículo".to_string()))?;

    // Fetch with all relations
    let mut result = db
        .query("SELECT * FROM $id FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("id", vehiculo.id.clone()))
        .await?;

    let fetched: Option<VehiculoFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::TransactionError(
        "Vehículo creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let res: Option<Vehiculo> = db.select(id.clone()).await?;
    Ok(res)
}

pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM $id FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("id", id.clone()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_placa(placa: &str) -> Result<Option<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM vehiculo WHERE placa = $placa AND is_active = true FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("placa", placa.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all_fetched() -> Result<Vec<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM vehiculo ORDER BY created_at DESC FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_activos_fetched() -> Result<Vec<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM vehiculo WHERE is_active = true FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .await?;
    Ok(result.take(0)?)
}

pub async fn update(
    id: &RecordId,
    dto: VehiculoUpdateDTO,
) -> Result<VehiculoFetched, SurrealDbError> {
    let db = get_db().await?;

    let res: Option<VehiculoFetched> = db
        .query("UPDATE $id MERGE $dto FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("id", id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    res.ok_or(SurrealDbError::TransactionError("Error al actualizar vehículo".to_string()))
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Vehiculo> = db.delete(id.clone()).await?;
    Ok(())
}

pub async fn count_by_placa(placa: &str) -> Result<i64, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT count() FROM vehiculo WHERE placa = $placa AND is_active = true GROUP ALL")
        .bind(("placa", placa.to_string()))
        .await?;

    let count_obj: Option<serde_json::Value> = result.take(0)?;
    let count = count_obj.and_then(|v| v.get("count").and_then(|c| c.as_i64())).unwrap_or(0);
    Ok(count)
}

pub async fn find_by_contratista(
    contratista_id: &RecordId,
) -> Result<Vec<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE contratista = $contratista AND is_active = true FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("contratista", contratista_id.clone()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_proveedor(
    proveedor_id: &RecordId,
) -> Result<Vec<VehiculoFetched>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE proveedor = $proveedor AND is_active = true FETCH contratista, proveedor, contratista.empresa, proveedor.empresa")
        .bind(("proveedor", proveedor_id.clone()))
        .await?;
    Ok(result.take(0)?)
}
