// ==========================================
// src/services/empresa_service.rs
// ==========================================

use crate::db::surrealdb_empresa_queries as db;
use crate::domain::empresa as domain;
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::info;
use surrealdb::RecordId;

// Helper para mapear errores de SurrealDB a EmpresaError
fn map_db_error(e: SurrealDbError) -> EmpresaError {
    EmpresaError::Database(e.to_string())
}

// Helper para parsear ID de empresa (acepta con o sin prefijo)
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

// ==========================================
// CONSULTAS
// ==========================================

pub async fn get_all_empresas() -> Result<EmpresaListResponse, EmpresaError> {
    let empresas = db::find_all().await.map_err(map_db_error)?;
    let total = empresas.len();
    let activas = empresas.iter().filter(|e| e.is_active).count();

    let responses: Vec<EmpresaResponse> = empresas.into_iter().map(EmpresaResponse::from).collect();

    Ok(EmpresaListResponse { empresas: responses, total, activas })
}

pub async fn get_empresas_activas() -> Result<Vec<EmpresaResponse>, EmpresaError> {
    let empresas = db::get_empresas_activas().await.map_err(map_db_error)?;
    Ok(empresas.into_iter().map(EmpresaResponse::from).collect())
}

pub async fn get_empresa_by_id(id_str: &str) -> Result<EmpresaResponse, EmpresaError> {
    let id = parse_empresa_id(id_str);
    let empresa = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    Ok(EmpresaResponse::from(empresa))
}

// ==========================================
// CREAR
// ==========================================

pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Verificar duplicados (nombre)
    let exists = db::exists_by_name(&input.nombre).await.map_err(map_db_error)?;
    if exists {
        return Err(EmpresaError::NameExists);
    }

    // 3. Crear DTO
    let dto = crate::models::empresa::EmpresaCreateDTO {
        nombre: input.nombre.trim().to_uppercase(),
        direccion: input.direccion.map(|s| s.trim().to_uppercase()),
        is_active: true,
    };

    // 4. Insertar
    let saved = db::create(dto).await.map_err(map_db_error)?;

    info!("Empresa creada: {}", saved.nombre);
    Ok(EmpresaResponse::from(saved))
}

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_empresa(
    id_str: &str,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    let id = parse_empresa_id(id_str);

    // 1. Verificar existencia
    let _existing =
        db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // 2. Validar input
    domain::validar_update_input(&input)?;

    // 3. Preparar update
    let mut update_data = serde_json::Map::new();
    if let Some(ref nombre) = input.nombre {
        update_data.insert("nombre".to_string(), serde_json::json!(nombre.trim().to_uppercase()));
    }

    if let Some(ref direccion) = input.direccion {
        update_data.insert("direccion".to_string(), serde_json::json!(direccion));
    }
    if let Some(is_active) = input.is_active {
        update_data.insert("is_active".to_string(), serde_json::json!(is_active));
    }
    update_data
        .insert("updated_at".to_string(), serde_json::json!(surrealdb::Datetime::from(Utc::now())));

    // 4. Actualizar
    info!("Actualizando empresa con ID {}", id_str);
    let updated =
        db::update(&id, serde_json::Value::Object(update_data)).await.map_err(map_db_error)?;

    Ok(EmpresaResponse::from(updated))
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_empresa(id_str: &str) -> Result<(), EmpresaError> {
    let id = parse_empresa_id(id_str);

    // 1. Verificar si tiene contratistas asociados
    let count = db::count_contratistas_by_empresa(&id).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(EmpresaError::HasContratistas(count as i64));
    }

    // 2. Eliminar
    db::delete(&id).await.map_err(map_db_error)?;

    info!("Empresa eliminada: {}", id_str);
    Ok(())
}
