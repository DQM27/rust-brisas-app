// ==========================================
// src/db/surrealdb_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::proveedor::{Proveedor, ProveedorCreateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn create(dto: ProveedorCreateDTO) -> Result<Proveedor, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Proveedor> =
        db.query("CREATE proveedor CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result.ok_or(SurrealDbError::Query("Error creando proveedor".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Proveedor> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM proveedor WHERE cedula = $cedula LIMIT 1")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM proveedor ORDER BY created_at DESC").await?;
    Ok(result.take(0)?)
}

pub async fn search(query: &str, limit: usize) -> Result<Vec<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let query_upper = query.to_uppercase();

    let mut result = db
        .query(
            r#"
            SELECT * FROM proveedor 
            WHERE 
                nombre CONTAINS $q OR 
                apellido CONTAINS $q OR 
                cedula CONTAINS $q
            ORDER BY created_at DESC 
            LIMIT $limit
        "#,
        )
        .bind(("q", query_upper))
        .bind(("limit", limit))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update(id: &RecordId, data: serde_json::Value) -> Result<Proveedor, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Proveedor> = db.update(id.clone()).merge(data).await?;

    result.ok_or(SurrealDbError::Query("Proveedor no encontrado o error al actualizar".to_string()))
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Proveedor> = db.delete(id.clone()).await?;
    Ok(())
}

pub async fn get_empresa_nombre(empresa_id: &RecordId) -> Result<String, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db.query("SELECT nombre FROM $id").bind(("id", empresa_id.clone())).await?;

    #[derive(serde::Deserialize)]
    struct NombreResult {
        nombre: String,
    }

    let res: Option<NombreResult> = result.take(0)?;
    Ok(res.map(|r| r.nombre).unwrap_or_else(|| "Empresa desconocida".to_string()))
}
