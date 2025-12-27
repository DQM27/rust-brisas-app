// ==========================================
// src/db/surrealdb_contratista_queries.rs
// ==========================================

use crate::models::contratista::{Contratista, CreateContratistaInput, UpdateContratistaInput};
use crate::services::surrealdb_service::{get_surrealdb, SurrealDbError};
use chrono::Utc;

pub async fn create(input: CreateContratistaInput) -> Result<Contratista, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let now = Utc::now().to_rfc3339();
    let empresa_id = input.empresa_id.strip_prefix("empresas:").unwrap_or(&input.empresa_id);

    let result: Option<Contratista> = client
        .create("contratistas")
        .content(serde_json::json!({
            "cedula": input.cedula,
            "nombre": input.nombre,
            "segundo_nombre": input.segundo_nombre,
            "apellido": input.apellido,
            "segundo_apellido": input.segundo_apellido,
            "empresa": format!("empresas:{}", empresa_id),
            "fecha_vencimiento_praind": input.fecha_vencimiento_praind,
            "estado": "activo",
            "created_at": now,
            "updated_at": now
        }))
        .await?;

    result.ok_or(SurrealDbError::Query("No se pudo crear el contratista".to_string()))
}

pub async fn find_by_id(id: &str) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("contratistas:").unwrap_or(id);
    let result: Option<Contratista> = client.select(("contratistas", id_only)).await?;
    Ok(result)
}

pub async fn find_by_cedula(cedula: &str) -> Result<Option<Contratista>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let mut result = client
        .query("SELECT * FROM contratistas WHERE cedula = $cedula")
        .bind(("cedula", cedula.to_string()))
        .await?;

    let contratista: Option<Contratista> = result.take(0)?;
    Ok(contratista)
}

pub async fn find_all() -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let result: Vec<Contratista> = client.select("contratistas").await?;
    Ok(result)
}

pub async fn find_by_empresa(empresa_id: &str) -> Result<Vec<Contratista>, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = empresa_id.strip_prefix("empresas:").unwrap_or(empresa_id);

    let mut result = client
        .query("SELECT * FROM contratistas WHERE empresa = type::thing('empresas', $empresa_id)")
        .bind(("empresa_id", id_only.to_string()))
        .await?;

    let contratistas: Vec<Contratista> = result.take(0)?;
    Ok(contratistas)
}

pub async fn update(
    id: &str,
    input: UpdateContratistaInput,
) -> Result<Contratista, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("contratistas:").unwrap_or(id);
    let now = Utc::now().to_rfc3339();

    let mut update_data = serde_json::Map::new();
    if let Some(v) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.segundo_nombre {
        update_data.insert("segundo_nombre".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.apellido {
        update_data.insert("apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.segundo_apellido {
        update_data.insert("segundo_apellido".to_string(), serde_json::json!(v));
    }
    if let Some(v) = input.empresa_id {
        let emp_id = v.strip_prefix("empresas:").unwrap_or(&v);
        update_data
            .insert("empresa".to_string(), serde_json::json!(format!("empresas:{}", emp_id)));
    }
    if let Some(v) = input.fecha_vencimiento_praind {
        update_data.insert("fecha_vencimiento_praind".to_string(), serde_json::json!(v));
    }

    update_data.insert("updated_at".to_string(), serde_json::json!(now));

    let result: Option<Contratista> = client
        .update(("contratistas", id_only))
        .merge(serde_json::Value::Object(update_data))
        .await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar el contratista".to_string()))
}

pub async fn update_status(id: &str, estado: &str) -> Result<Contratista, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("contratistas:").unwrap_or(id);
    let now = Utc::now().to_rfc3339();

    let result: Option<Contratista> = client
        .update(("contratistas", id_only))
        .merge(serde_json::json!({
            "estado": estado,
            "updated_at": now
        }))
        .await?;

    result.ok_or(SurrealDbError::Query("No se pudo actualizar el estado".to_string()))
}

pub async fn delete(id: &str) -> Result<(), SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = id.strip_prefix("contratistas:").unwrap_or(id);
    let _: Option<Contratista> = client.delete(("contratistas", id_only)).await?;
    Ok(())
}

pub async fn get_empresa_nombre(empresa_id: &str) -> Result<String, SurrealDbError> {
    let db = get_surrealdb().ok_or(SurrealDbError::NotConnected)?;
    let client = db.get_client().await?;

    let id_only = empresa_id.strip_prefix("empresas:").unwrap_or(empresa_id);

    let mut result = client
        .query("SELECT nombre FROM type::thing('empresas', $id)")
        .bind(("id", id_only.to_string()))
        .await?;

    #[derive(serde::Deserialize)]
    struct NombreResult {
        nombre: String,
    }
    let res: Option<NombreResult> = result.take(0)?;
    Ok(res.map(|r| r.nombre).unwrap_or_else(|| "Empresa desconocida".to_string()))
}
