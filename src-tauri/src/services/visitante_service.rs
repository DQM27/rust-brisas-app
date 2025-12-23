// ==========================================
// src/services/visitante_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Visitantes

use crate::db::{lista_negra_queries, visitante_queries};
use crate::domain::errors::VisitanteError;
use crate::domain::visitante as domain;
use crate::models::visitante::{CreateVisitanteInput, Visitante};
use sqlx::SqlitePool;

pub async fn create_visitante(
    pool: &SqlitePool,
    mut input: CreateVisitanteInput,
) -> Result<Visitante, VisitanteError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar
    input.cedula = domain::normalizar_cedula(&input.cedula);
    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);
    if let Some(s) = input.segundo_nombre.as_ref() {
        input.segundo_nombre = Some(domain::normalizar_nombre(s));
    }
    if let Some(s) = input.segundo_apellido.as_ref() {
        input.segundo_apellido = Some(domain::normalizar_nombre(s));
    }
    if let Some(s) = input.empresa.as_ref() {
        input.empresa = Some(s.trim().to_uppercase());
    }

    // 3. Verificar que NO esté en lista negra
    let block_status = lista_negra_queries::check_if_blocked_by_cedula(pool, &input.cedula)
        .await
        .map_err(|e| VisitanteError::Validation(e.to_string()))?;

    if block_status.blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        return Err(VisitanteError::Validation(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Nivel: {}",
            input.cedula, nivel
        )));
    }

    // 4. Validar si ya existe cédula
    if visitante_queries::get_visitante_by_cedula(pool, &input.cedula).await?.is_some() {
        return Err(VisitanteError::CedulaExists);
    }

    visitante_queries::create_visitante(pool, input).await.map_err(VisitanteError::Database)
}

pub async fn search_visitantes(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<Visitante>, VisitanteError> {
    visitante_queries::search_visitantes(pool, term).await.map_err(VisitanteError::Database)
}

pub async fn get_visitante_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<Visitante>, VisitanteError> {
    visitante_queries::get_visitante_by_id(pool, id).await.map_err(VisitanteError::Database)
}

pub async fn get_visitante_by_cedula(
    pool: &SqlitePool,
    cedula: &str,
) -> Result<Option<Visitante>, VisitanteError> {
    visitante_queries::get_visitante_by_cedula(pool, cedula).await.map_err(VisitanteError::Database)
}
