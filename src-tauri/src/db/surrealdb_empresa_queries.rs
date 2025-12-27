// ==========================================
// src/db/surrealdb_empresa_queries.rs
// ==========================================

use crate::models::empresa::{CreateEmpresaInput, Empresa, UpdateEmpresaInput};
use crate::services::surrealdb_service::{get_db, SurrealDbError};

pub async fn create(input: CreateEmpresaInput) -> Result<Empresa, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Empresa> = db
        .query(
            r#"
            CREATE empresa CONTENT {
                nombre: $nombre,
                is_active: true,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("nombre", input.nombre))
        .await?
        .take(0)?;

    result.ok_or(SurrealDbError::Query("No se pudo crear la empresa".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Empresa>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("empresa:").unwrap_or(id);
    let result: Option<Empresa> = db.select(("empresa", id_only)).await?;
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

pub async fn update(id: &str, input: UpdateEmpresaInput) -> Result<Empresa, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("empresa:").unwrap_or(id);

    let mut update_data = serde_json::Map::new();
    if let Some(nombre) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::Value::String(nombre));
    }
    if let Some(is_active) = input.is_active {
        update_data.insert("is_active".to_string(), serde_json::Value::Bool(is_active));
    }

    let result: Option<Empresa> =
        db.update(("empresa", id_only)).merge(serde_json::Value::Object(update_data)).await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar la empresa".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("empresa:").unwrap_or(id);
    let _: Option<Empresa> = db.delete(("empresa", id_only)).await?;
    Ok(())
}

pub async fn count_contratistas_by_empresa(empresa_id: &str) -> Result<usize, SurrealDbError> {
    let db = get_db().await?;
    let id_only = empresa_id.strip_prefix("empresa:").unwrap_or(empresa_id);

    let mut result = db
        .query("SELECT count() as count FROM contratista WHERE empresa_id = $empresa_id GROUP ALL")
        .bind(("empresa_id", format!("empresa:{}", id_only)))
        .await?;

    #[derive(serde::Deserialize)]
    struct CountResult {
        count: usize,
    }

    let count: Option<CountResult> = result.take(0)?;
    Ok(count.map(|c| c.count).unwrap_or(0))
}
