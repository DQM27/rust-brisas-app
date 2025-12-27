// ==========================================
// src/services/visitante_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Visitantes

use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_visitante_queries as db;
use crate::domain::errors::VisitanteError;
use crate::domain::visitante as domain;
use crate::models::visitante::{CreateVisitanteInput, Visitante};
use crate::services::surrealdb_service::SurrealDbError;
use log::error;

// Helper para mapear errores de SurrealDB a VisitanteError
fn map_db_error(e: SurrealDbError) -> VisitanteError {
    error!("Error de base de datos (SurrealDB): {}", e);
    VisitanteError::Database(e.to_string())
}

pub async fn create_visitante(
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
    let block_status =
        ln_db::check_if_blocked_by_cedula(&input.cedula).await.map_err(map_db_error)?;

    if block_status.is_blocked {
        let nivel = block_status.nivel_severidad.unwrap_or_else(|| "BAJO".to_string());
        return Err(VisitanteError::Validation(format!(
            "No se puede registrar. La persona con cédula {} está en lista negra. Nivel: {}",
            input.cedula, nivel
        )));
    }

    // 4. Validar si ya existe cédula
    if db::get_visitante_by_cedula(&input.cedula).await.map_err(map_db_error)?.is_some() {
        return Err(VisitanteError::CedulaExists);
    }

    db::create_visitante(input).await.map_err(map_db_error)
}

pub async fn search_visitantes(term: &str) -> Result<Vec<Visitante>, VisitanteError> {
    db::search_visitantes(term).await.map_err(map_db_error)
}

pub async fn get_visitante_by_id(id: &str) -> Result<Option<Visitante>, VisitanteError> {
    db::get_visitante_by_id(id).await.map_err(map_db_error)
}

pub async fn get_visitante_by_cedula(cedula: &str) -> Result<Option<Visitante>, VisitanteError> {
    db::get_visitante_by_cedula(cedula).await.map_err(map_db_error)
}
