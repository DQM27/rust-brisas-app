// ==========================================
// src/db/surrealdb_visitante_queries.rs
// ==========================================

use crate::models::visitante::{Visitante, VisitanteCreateDTO, VisitanteUpdateDTO};
use crate::services::surrealdb_service::{get_db, SurrealDbError};
use surrealdb::RecordId;

pub async fn create_visitante(dto: VisitanteCreateDTO) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;

    let res: Option<Visitante> = db
        .query(
            r#"
            CREATE visitante CONTENT $dto
        "#,
        )
        .bind(("dto", dto))
        .await?
        .take(0)?;

    res.ok_or(SurrealDbError::TransactionError("Error al crear visitante".to_string()))
}

pub async fn find_by_id(id: &RecordId) -> Result<Option<Visitante>, SurrealDbError> {
    let db = get_db().await?;
    let result: Option<Visitante> = db.select(id.clone()).await?;
    Ok(result)
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

pub async fn update(id: &RecordId, dto: VisitanteUpdateDTO) -> Result<Visitante, SurrealDbError> {
    let db = get_db().await?;

    let result: Option<Visitante> = db.update(id.clone()).merge(dto).await?;

    result.ok_or(SurrealDbError::Query("Visitante no encontrado o error al actualizar".to_string()))
}

pub async fn delete(id: &RecordId) -> Result<(), SurrealDbError> {
    let db = get_db().await?;
    let _: Option<Visitante> = db.delete(id.clone()).await?;
    Ok(())
}
