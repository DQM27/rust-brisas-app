// ==========================================
// src/services/visitante_service.rs
// ==========================================
// Capa de servicio: Lógica de negocio para Visitantes

use crate::db::surrealdb_lista_negra_queries as ln_db;
use crate::db::surrealdb_visitante_queries as db;
use crate::domain::errors::VisitanteError;
use crate::domain::visitante as domain;
use crate::models::visitante::{
    CreateVisitanteInput, VisitanteCreateDTO, VisitanteResponse, VisitanteUpdateDTO,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::{error, info};
use surrealdb::RecordId;

// Helper para mapear errores de SurrealDB a VisitanteError
fn map_db_error(e: SurrealDbError) -> VisitanteError {
    error!("Error de base de datos (SurrealDB): {}", e);
    VisitanteError::Database(e.to_string())
}

/// Helper para parsear ID de visitante (acepta con o sin prefijo)
fn parse_visitante_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("visitante", id_str)
    }
}

/// Helper para parsear ID de empresa (acepta con o sin prefijo)
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

pub async fn create_visitante(
    mut input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
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

    // 5. Preparar DTO
    let dto = VisitanteCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        segundo_nombre: input.segundo_nombre,
        segundo_apellido: input.segundo_apellido,
        empresa: input.empresa_id.map(|id| parse_empresa_id(&id)),
        has_vehicle: input.has_vehicle,
    };

    info!("Creando visitante: {} {}", dto.nombre, dto.apellido);
    let visitante = db::create_visitante(dto).await.map_err(map_db_error)?;

    Ok(VisitanteResponse::from(visitante))
}

pub async fn search_visitantes(term: &str) -> Result<Vec<VisitanteResponse>, VisitanteError> {
    let visitantes = db::search_visitantes(term).await.map_err(map_db_error)?;
    Ok(visitantes.into_iter().map(VisitanteResponse::from).collect())
}

pub async fn get_visitante_by_id(
    id_str: &str,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    let opt = db::find_by_id(&id_thing).await.map_err(map_db_error)?;
    Ok(opt.map(VisitanteResponse::from))
}

pub async fn get_visitante_by_cedula(
    cedula: &str,
) -> Result<Option<VisitanteResponse>, VisitanteError> {
    let cedula_norm = domain::normalizar_cedula(cedula);
    let opt = db::get_visitante_by_cedula(&cedula_norm).await.map_err(map_db_error)?;
    Ok(opt.map(VisitanteResponse::from))
}

pub async fn update_visitante(
    id_str: &str,
    mut input: CreateVisitanteInput,
) -> Result<VisitanteResponse, VisitanteError> {
    let id_thing = parse_visitante_id(id_str);

    // 1. Validar si existe
    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;

    // 2. Normalizar
    input.nombre = domain::normalizar_nombre(&input.nombre);
    input.apellido = domain::normalizar_nombre(&input.apellido);
    // ... (rest of normalization) ...

    let mut dto = VisitanteUpdateDTO::default();
    dto.nombre = Some(input.nombre);
    dto.apellido = Some(input.apellido);
    dto.segundo_nombre = Some(input.segundo_nombre);
    dto.segundo_apellido = Some(input.segundo_apellido);
    dto.empresa = Some(input.empresa_id.map(|id| parse_empresa_id(&id)));
    dto.has_vehicle = Some(input.has_vehicle);
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    let visitante = db::update(&id_thing, dto).await.map_err(map_db_error)?;
    Ok(VisitanteResponse::from(visitante))
}

pub async fn delete_visitante(id_str: &str) -> Result<(), VisitanteError> {
    let id_thing = parse_visitante_id(id_str);
    db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(VisitanteError::NotFound)?;
    db::delete(&id_thing).await.map_err(map_db_error)
}
