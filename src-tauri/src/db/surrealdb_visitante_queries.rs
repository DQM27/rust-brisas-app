// ==========================================
// src/db/surrealdb_visitante_queries.rs
// ==========================================
use crate::models::visitante::{CreateVisitanteInput, Visitante};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use chrono::Utc;
use serde_json::json;

pub async fn create_visitante(input: CreateVisitanteInput) -> Result<Visitante, SurrealDbError> {
    let client = get_db().await?;
    let now = Utc::now().to_rfc3339();
    let empresa_link = input.empresa_id.as_ref().map(|id| format!("empresas:{}", id));

    let res: Option<Visitante> = client
        .create("visitantes")
        .content(json!({
            "cedula": input.cedula,
            "nombre": input.nombre,
            "apellido": input.apellido,
            "segundo_nombre": input.segundo_nombre,
            "segundo_apellido": input.segundo_apellido,
            "empresa": input.empresa,
            "empresa_id": empresa_link,
            "has_vehicle": input.has_vehicle,
            "created_at": now,
            "updated_at": now
        }))
        .await?;

    res.ok_or(SurrealDbError::TransactionError("Error al crear visitante".to_string()))
}

pub async fn get_visitante_by_id(id: &str) -> Result<Option<Visitante>, SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("visitantes:").unwrap_or(id);
    let res: Option<Visitante> = client.select(("visitantes", id_only.to_string())).await?;
    Ok(res)
}

pub async fn get_visitante_by_cedula(cedula: &str) -> Result<Option<Visitante>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT * FROM visitantes WHERE cedula = $cedula")
        .bind(("cedula", cedula.to_string()))
        .await?;
    let visitante: Option<Visitante> = result.take(0)?;
    Ok(visitante)
}

pub async fn search_visitantes(term: &str) -> Result<Vec<Visitante>, SurrealDbError> {
    let client = get_db().await?;
    let mut result = client
        .query("SELECT * FROM visitantes WHERE cedula CONTAINS $term OR nombre CONTAINS $term OR apellido CONTAINS $term")
        .bind(("term", term.to_string()))
        .await?;
    let visitantes: Vec<Visitante> = result.take(0)?;
    Ok(visitantes)
}

pub async fn update_visitante(id: &str, input: CreateVisitanteInput) -> Result<Visitante, SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("visitantes:").unwrap_or(id);
    let now = Utc::now().to_rfc3339();
    let empresa_link = input.empresa_id.as_ref().map(|id| format!("empresas:{}", id));

    let res: Option<Visitante> = client
        .update(("visitantes", id_only.to_string()))
        .merge(json!({
            "nombre": input.nombre,
            "apellido": input.apellido,
            "segundo_nombre": input.segundo_nombre,
            "segundo_apellido": input.segundo_apellido,
            "empresa": input.empresa,
            "empresa_id": empresa_link,
            "has_vehicle": input.has_vehicle,
            "updated_at": now
        }))
        .await?;

    res.ok_or(SurrealDbError::TransactionError("Error al actualizar visitante".to_string()))
}

pub async fn delete_visitante(id: &str) -> Result<(), SurrealDbError> {
    let client = get_db().await?;
    let id_only = id.strip_prefix("visitantes:").unwrap_or(id);
    let _: Option<Visitante> = client.delete(("visitantes", id_only.to_string())).await?;
    Ok(())
}
