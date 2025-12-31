// ==========================================
// src/db/surrealdb_empresa_queries.rs
// ==========================================

use crate::models::empresa::{Empresa, EmpresaCreateDTO, EmpresaUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn create(dto: EmpresaCreateDTO) -> Result<Empresa, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Empresa> =
        db.query("CREATE empresa CONTENT $dto").bind(("dto", dto)).await?.take(0)?;

    result.ok_or(SurrealDbError::Query("No se pudo crear la empresa".to_string()))
}

pub async fn exists_by_name(nombre: &str) -> Result<bool, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query(
            "SELECT id FROM empresa WHERE string::lowercase(nombre) = string::lowercase($nombre)",
        )
        .bind(("nombre", nombre.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct IdOnly {
        id: RecordId,
    }

    let empresas: Vec<IdOnly> = result.take(0)?;
    Ok(!empresas.is_empty())
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Empresa>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Empresa> = db.select(id.clone()).await?;
    Ok(result)
}

pub async fn find_all() -> Result<Vec<Empresa>, SurrealDbError> {
    let db = get_db().await?;
    let result: Vec<Empresa> = db.select("empresa").await?;
    Ok(result)
}

pub async fn get_empresas_activas() -> Result<Vec<Empresa>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db.query("SELECT * FROM empresa WHERE is_active = true").await?;
    let empresas: Vec<Empresa> = result.take(0)?;
    Ok(empresas)
}

pub async fn update(id: &RecordId, dto: EmpresaUpdateDTO) -> Result<Empresa, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Empresa> = db.update(id.clone()).merge(dto).await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar la empresa".to_string()))
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Empresa> = db.delete(id.clone()).await?;
    Ok(())
}

pub async fn count_contratistas_by_empresa(empresa_id: &RecordId) -> Result<usize, SurrealDbError> {
    let db = get_db().await?;

    let mut result = db
        .query("SELECT count() as count FROM contratista WHERE empresa = $empresa_id GROUP ALL")
        .bind(("empresa_id", empresa_id.clone()))
        .await?;

    #[derive(serde::Deserialize)]
    struct CountResult {
        count: usize,
    }

    let count: Option<CountResult> = result.take(0)?;
    Ok(count.map(|c| c.count).unwrap_or(0))
}
