// ==========================================
// src/db/surrealdb_contratista_queries.rs
// ==========================================

use crate::models::contratista::{Contratista, ContratistaCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::sql::Thing;

pub async fn create(dto: ContratistaCreateDTO) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Contratista> =
        db.query("CREATE contratista CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result.ok_or(SurrealDbError::Query("No se pudo crear el contratista".to_string()))
}

pub async fn find_by_id(id: &Thing) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Contratista> = db.select((id.tb.clone(), id.id.to_string())).await?;
    Ok(result)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE cedula = $cedula")
        .bind(("cedula", cedula.to_string()))
        .await?;
    let contratista: Option<Contratista> = result.take(0)?;
    Ok(contratista)
}

pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Contratista> = db.select("contratista").await?;
    Ok(result)
}

pub async fn find_by_empresa(empresa_id: &Thing) -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE empresa = $empresa")
        .bind(("empresa", empresa_id.clone()))
        .await?;
    let contratistas: Vec<Contratista> = result.take(0)?;
    Ok(contratistas)
}

pub async fn update(id: &Thing, data: serde_json::Value) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Contratista> =
        db.update((id.tb.clone(), id.id.to_string())).merge(data).await?;

    result
        .ok_or(SurrealDbError::Query("Contratista no encontrado o error al actualizar".to_string()))
}

pub async fn update_status(id: &Thing, estado: &str) -> Result<Contratista, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Contratista> = db
        .update((id.tb.clone(), id.id.to_string()))
        .merge(serde_json::json!({ "estado": estado }))
        .await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar el estado".to_string()))
}

pub async fn delete(id: &Thing) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Contratista> = db.delete((id.tb.clone(), id.id.to_string())).await?;
    Ok(())
}

pub async fn get_empresa_nombre(empresa_id: &Thing) -> Result<String, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db.query("SELECT nombre FROM $id").bind(("id", empresa_id.clone())).await?;

    #[derive(serde::Deserialize)]
    struct NombreResult {
        nombre: String,
    }

    let res: Option<NombreResult> = result.take(0)?;
    Ok(res.map(|r| r.nombre).unwrap_or_else(|| "Empresa desconocida".to_string()))
}
