// ==========================================
// src/db/surrealdb_visitante_queries.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::visitante::{CreateVisitanteInput, Visitante};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use serde_json::json;

pub async fn create_visitante(input: CreateVisitanteInput) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;
    let empresa_link = input.empresa_id.as_ref().map(|id| {
        let id_only = id.strip_prefix("empresa:").unwrap_or(id);
        format!("empresa:{}", id_only)
    });

    let res: Option<Visitante> = db
        .query(
            r#"
            CREATE visitante CONTENT {
                cedula: $cedula,
                nombre: $nombre,
                apellido: $apellido,
                segundo_nombre: $segundo_nombre,
                segundo_apellido: $segundo_apellido,
                empresa: $empresa,
                empresa_id: $empresa_id,
                has_vehicle: $has_vehicle,
                created_at: time::now(),
                updated_at: time::now()
            }
        "#,
        )
        .bind(("cedula", input.cedula))
        .bind(("nombre", input.nombre))
        .bind(("apellido", input.apellido))
        .bind(("segundo_nombre", input.segundo_nombre))
        .bind(("segundo_apellido", input.segundo_apellido))
        .bind(("empresa", input.empresa))
        .bind(("empresa_id", empresa_link))
        .bind(("has_vehicle", input.has_vehicle))
        .await?
        .take(0)?;

    res.ok_or(SurrealDbError::TransactionError("Error al crear visitante".to_string()))
}

pub async fn get_visitante_by_id(id: &str) -> Result<Option<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("visitante:").unwrap_or(id).to_string();
    let res: Option<Visitante> = db.select(("visitante", id_only)).await?;
    Ok(res)
}

pub async fn get_visitante_by_cedula(cedula: &str) -> Result<Option<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM visitante WHERE cedula = $cedula")
        .bind(("cedula", cedula.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn search_visitantes(term: &str) -> Result<Vec<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    let mut result = db
        .query("SELECT * FROM visitante WHERE cedula CONTAINS $term OR nombre CONTAINS $term OR apellido CONTAINS $term")
        .bind(("term", term.to_string()))
        .await?;
    Ok(result.take(0)?)
}

pub async fn update_visitante(
    id: &str,
    input: CreateVisitanteInput,
) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("visitante:").unwrap_or(id).to_string();
    let empresa_link = input.empresa_id.as_ref().map(|id| {
        let id_only = id.strip_prefix("empresa:").unwrap_or(id);
        format!("empresa:{}", id_only)
    });

    let res: Option<Visitante> = db
        .update(("visitante", id_only))
        .merge(json!({
            "nombre": input.nombre,
            "apellido": input.apellido,
            "segundo_nombre": input.segundo_nombre,
            "segundo_apellido": input.segundo_apellido,
            "empresa": input.empresa,
            "empresa_id": empresa_link,
            "has_vehicle": input.has_vehicle
        }))
        .await?;

    res.ok_or(SurrealDbError::TransactionError("Error al actualizar visitante".to_string()))
}

pub async fn delete_visitante(id: &str) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let id_only = id.strip_prefix("visitante:").unwrap_or(id).to_string();
    let _: Option<Visitante> = db.delete(("visitante", id_only)).await?;
    Ok(())
}
