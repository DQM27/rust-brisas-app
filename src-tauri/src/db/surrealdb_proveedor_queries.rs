// ==========================================
// src/db/surrealdb_proveedor_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::proveedor::{
    Proveedor, ProveedorCreateDTO, ProveedorFetched, ProveedorUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn create(dto: ProveedorCreateDTO) -> Result<ProveedorFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries
    let created: Option<Proveedor> =
        db.query("CREATE proveedor CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    let proveedor = created.ok_or(SurrealDbError::Query("Error creando proveedor".to_string()))?;

    // Fetch with empresa populated
    let mut result =
        db.query("SELECT * FROM $id FETCH empresa").bind(("id", proveedor.id.clone())).await?;

    let fetched: Option<ProveedorFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::Query(
        "Proveedor creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Proveedor> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_id_fetched(id: &RecordId) -> Result<Option<ProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<ProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM proveedor WHERE cedula = $cedula AND deleted_at IS NONE LIMIT 1 FETCH empresa")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all() -> Result<Vec<Proveedor>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM proveedor WHERE deleted_at IS NONE ORDER BY created_at DESC")
        .await?;
    Ok(result.take(0)?)
}

pub async fn find_all_fetched() -> Result<Vec<ProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("SELECT * FROM proveedor WHERE deleted_at IS NONE ORDER BY created_at DESC FETCH empresa").await?;
    Ok(result.take(0)?)
}

pub async fn search(query: &str, limit: usize) -> Result<Vec<ProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let query_upper = query.to_uppercase();

    let mut result = db
        .query(
            r#"
            SELECT * FROM proveedor 
            WHERE 
                (string::uppercase(nombre) CONTAINS $q OR 
                string::uppercase(apellido) CONTAINS $q OR 
                cedula CONTAINS $q)
                AND deleted_at IS NONE
            ORDER BY created_at DESC 
            LIMIT $limit
            FETCH empresa
        "#,
        )
        .bind(("q", query_upper))
        .bind(("limit", limit))
        .await?;

    Ok(result.take(0)?)
}

pub async fn update(
    id: &RecordId,
    dto: ProveedorUpdateDTO,
) -> Result<ProveedorFetched, SurrealDbError> {
    let db = get_db().await?;

    // 1. Update using native SDK (consistent with User module)
    let _: Option<Proveedor> = db.update(id.clone()).merge(dto).await?;

    // 2. Fetch with empresa populated in a separate atomic query
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;

    let fetched: Option<ProveedorFetched> = result.take(0)?;
    fetched
        .ok_or(SurrealDbError::Query("Proveedor no encontrado o error al actualizar".to_string()))
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    // Soft delete
    let db = get_db().await?;
    let _: Option<Proveedor> = db
        .query("UPDATE $id SET deleted_at = time::now()")
        .bind(("id", id.clone()))
        .await?
        .take(0)?;
    Ok(())
}

pub async fn restore(id: &RecordId) -> Result<ProveedorFetched, SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Proveedor> =
        db.query("UPDATE $id SET deleted_at = NONE").bind(("id", id.clone())).await?.take(0)?;

    // Fetch updated
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;
    let fetched: Option<ProveedorFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::Query("Error restaurando proveedor".to_string()))
}

pub async fn find_archived() -> Result<Vec<ProveedorFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("SELECT * FROM proveedor WHERE deleted_at IS NOT NONE ORDER BY deleted_at DESC FETCH empresa").await?;
    Ok(result.take(0)?)
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
