// ==========================================
// src/services/visitante_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Visitantes

use crate::db::visitante_queries;
use crate::domain::errors::VisitanteError;
use crate::domain::visitante::{CreateVisitanteInput, Visitante};
use sqlx::SqlitePool;

pub async fn create_visitante(
    pool: &SqlitePool,
    input: CreateVisitanteInput,
) -> Result<Visitante, VisitanteError> {
    // Validar si ya existe cédula
    if visitante_queries::get_visitante_by_cedula(pool, &input.cedula)
        .await?
        .is_some()
    {
        return Err(VisitanteError::AlreadyExists);
    }

    visitante_queries::create_visitante(pool, input)
        .await
        .map_err(VisitanteError::Database)
}

pub async fn search_visitantes(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<Visitante>, VisitanteError> {
    visitante_queries::search_visitantes(pool, term)
        .await
        .map_err(VisitanteError::Database)
}

pub async fn get_visitante_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<Visitante>, VisitanteError> {
    visitante_queries::get_visitante_by_id(pool, id)
        .await
        .map_err(VisitanteError::Database)
}

pub async fn get_visitante_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<Visitante>, VisitanteError> {
    visitante_queries::get_visitante_by_cedula(pool, cedula)
        .await
        .map_err(VisitanteError::Database)
}
