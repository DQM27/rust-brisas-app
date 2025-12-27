// ==========================================
// src/db/surrealdb_vehiculo_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::vehiculo::{Vehiculo, VehiculoCreateDTO, VehiculoUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn insert(dto: VehiculoCreateDTO) -> Result<Vehiculo, SurrealDbError> {
    let db = get_db().await?;

    let res: Option<Vehiculo> =
        db.query("CREATE vehiculo CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    res.ok_or(SurrealDbError::TransactionError("Error al insertar vehículo".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let res: Option<Vehiculo> = db.select(id.clone()).await?;
    Ok(res)
}

pub async fn find_by_placa(placa: &str) -> Result<Option<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM vehiculo WHERE placa = $placa AND is_active = true")
        .bind(("placa", placa.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let res: Vec<Vehiculo> = db.select("vehiculo").await?;
    Ok(res)
}

pub async fn find_activos() -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM vehiculo WHERE is_active = true").await?;
    Ok(result.take(0)?)
}

pub async fn update(id: &RecordId, dto: VehiculoUpdateDTO) -> Result<Vehiculo, SurrealDbError> {
    let db = get_db().await?;

    let res: Option<Vehiculo> = db.update(id.clone()).merge(dto).await?;
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
) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE contratista = $contratista AND is_active = true")
        .bind(("contratista", contratista_id.clone()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_by_proveedor(proveedor_id: &RecordId) -> Result<Vec<Vehiculo>, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT * FROM vehiculo WHERE proveedor = $proveedor AND is_active = true")
        .bind(("proveedor", proveedor_id.clone()))
        .await?;
    Ok(result.take(0)?)
}
