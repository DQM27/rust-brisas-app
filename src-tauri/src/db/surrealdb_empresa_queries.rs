// ==========================================
// src/db/surrealdb_empresa_queries.rs
// ==========================================

use crate::models::empresa::{CreateEmpresaInput, Empresa, UpdateEmpresaInput};
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use chrono::Utc;

pub async fn create(input: CreateEmpresaInput) -> Result<Empresa, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = Utc::now().to_rfc3339();

    // Validar nombre unico o algo? Por ahora insert directo.

    let result: Option<Empresa> = client
        .create("empresas")
        .content(serde_json::json!({
            "nombre": input.nombre,
            "is_active": true,
            "created_at": now,
            "updated_at": now
        }))
        .await?;

    result.ok_or(SurrealDbError::Query("No se pudo crear la empresa".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Empresa>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("empresas:").unwrap_or(id);

    let result: Option<Empresa> = client.select(("empresas", id_only)).await?;
    Ok(result)
}

pub async fn find_all() -> Result<Vec<Empresa>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let result: Vec<Empresa> = client.select("empresas").await?;
    Ok(result)
}

pub async fn get_empresas_activas() -> Result<Vec<Empresa>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let mut result = client.query("SELECT * FROM empresas WHERE is_active = true").await?;
    let empresas: Vec<Empresa> = result.take(0)?;
    Ok(empresas)
}

pub async fn update(id: &str, input: UpdateEmpresaInput) -> Result<Empresa, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("empresas:").unwrap_or(id);
    let now = Utc::now().to_rfc3339();

    let mut update_data = serde_json::Map::new();
    if let Some(nombre) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::Value::String(nombre));
    }
    if let Some(is_active) = input.is_active {
        update_data.insert("is_active".to_string(), serde_json::Value::Bool(is_active));
    }
    update_data.insert("updated_at".to_string(), serde_json::Value::String(now));

    let result: Option<Empresa> =
        client.update(("empresas", id_only)).merge(serde_json::Value::Object(update_data)).await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar la empresa".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("empresas:").unwrap_or(id);

    let _: Option<Empresa> = client.delete(("empresas", id_only)).await?;
    Ok(())
}

pub async fn count_contratistas_by_empresa(_empresa_id: &str) -> Result<usize, SurrealDbError> {
    Ok(0)
}
