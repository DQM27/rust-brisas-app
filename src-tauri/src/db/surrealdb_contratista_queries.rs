// ==========================================
// src/db/surrealdb_contratista_queries.rs
// ==========================================

use crate::models::contratista::{
    Contratista, ContratistaCreateDTO, ContratistaFetched, ContratistaUpdateDTO,
};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn create(dto: ContratistaCreateDTO) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // CREATE doesn't support FETCH, so we need two queries:
    // 1. Create the record and get the raw result
    let created: Option<Contratista> =
        db.query("CREATE contratista CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    let contratista =
        created.ok_or(SurrealDbError::Query("No se pudo crear el contratista".to_string()))?;

    // 2. Fetch the created record with empresa populated
    let mut result =
        db.query("SELECT * FROM $id FETCH empresa").bind(("id", contratista.id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::Query(
        "Contratista creado pero no se pudo obtener con FETCH".to_string(),
    ))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Contratista> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_by_id_fetched(
    id: &RecordId,
) -> Result<Option<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;
    Ok(result.take(0)?)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            "SELECT * FROM contratista WHERE cedula = $cedula AND deleted_at IS NONE FETCH empresa",
        )
        .bind(("cedula", cedula.to_string()))
        .await?;
    let contratista: Option<ContratistaFetched> = result.take(0)?;
    Ok(contratista)
}

pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    // Only return non-deleted records
    let result: Vec<Contratista> =
        db.query("SELECT * FROM contratista WHERE deleted_at IS NONE").await?.take(0)?;
    Ok(result)
}

pub async fn find_all_fetched() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result =
        db.query("SELECT * FROM contratista WHERE deleted_at IS NONE FETCH empresa").await?;
    Ok(result.take(0)?)
}

pub async fn find_by_empresa(empresa_id: &RecordId) -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE empresa = $empresa AND deleted_at IS NONE")
        .bind(("empresa", empresa_id.clone()))
        .await?;
    let contratistas: Vec<Contratista> = result.take(0)?;
    Ok(contratistas)
}

pub async fn update(
    id: &RecordId,
    dto: ContratistaUpdateDTO,
) -> Result<ContratistaFetched, SurrealDbError> {
    println!(">>> DEBUG: Updating contratista {} with DTO: {:?}", id, dto);
    let db = get_db().await?;

    // UPDATE doesn't support FETCH, so we need two queries
    let _: Option<Contratista> = db
        .query("UPDATE $id MERGE $dto")
        .bind(("id", id.clone()))
        .bind(("dto", dto))
        .await?
        .take(0)?;

    // Fetch with empresa populated
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    println!(">>> DEBUG: Update result: {:?}", fetched);
    fetched
        .ok_or(SurrealDbError::Query("Contratista no encontrado o error al actualizar".to_string()))
}

pub async fn update_status(
    id: &RecordId,
    estado: &str,
) -> Result<ContratistaFetched, SurrealDbError> {
    let db = get_db().await?;

    // UPDATE doesn't support FETCH
    let _: Option<Contratista> = db
        .query("UPDATE $id SET estado = $estado")
        .bind(("id", id.clone()))
        .bind(("estado", estado.to_string()))
        .await?
        .take(0)?;

    // Fetch with empresa populated
    let mut result = db.query("SELECT * FROM $id FETCH empresa").bind(("id", id.clone())).await?;

    let fetched: Option<ContratistaFetched> = result.take(0)?;
    fetched.ok_or(SurrealDbError::Query("No se pudo actualizar el estado".to_string()))
}

// Soft delete implementation
pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Contratista> = db
        .query("UPDATE $id SET deleted_at = time::now()")
        .bind(("id", id.clone()))
        .await?
        .take(0)?;
    Ok(())
}

pub async fn restore(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Contratista> =
        db.query("UPDATE $id SET deleted_at = NONE").bind(("id", id.clone())).await?.take(0)?;
    Ok(())
}

pub async fn find_archived() -> Result<Vec<ContratistaFetched>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM contratista WHERE deleted_at IS NOT NONE ORDER BY deleted_at DESC FETCH empresa")
        .await?;
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
