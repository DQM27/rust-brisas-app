// ==========================================
// src/services/empresa_service.rs
// ==========================================

// use crate::db::empresa_queries as db;
use crate::db::surrealdb_empresa_queries as db;
use crate::domain::empresa as domain;
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::surrealdb_service::SurrealDbError;
use chrono::Utc;
use log::{error, info};

// Helper para mapear errores de SurrealDB a EmpresaError
fn map_db_error(e: SurrealDbError) -> EmpresaError {
    EmpresaError::Database(e.to_string())
}

// ==========================================
// CONSULTAS DE EMPRESA
// ==========================================

use surrealdb::sql::Thing;

/// Helper para parsear ID de empresa (acepta con o sin prefijo)
fn parse_empresa_id(id_str: &str) -> Thing {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        Thing::from((parts[0], parts[1]))
    } else {
        Thing::from(("empresa", id_str))
    }
}

pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar nombre
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);

    // 3. Verificar que no exista (SurrealDB check)
    let empresas = db::find_all().await.map_err(map_db_error)?;
    if empresas.iter().any(|e| domain::normalizar_nombre(&e.nombre) == nombre_normalizado) {
        return Err(EmpresaError::NameExists);
    }

    // 4. Crear
    info!("Creando empresa '{}'", nombre_normalizado);
    let dto = crate::models::empresa::EmpresaCreateDTO {
        nombre: nombre_normalizado.clone(),
        direccion: input.direccion,
        is_active: true,
    };

    let empresa = db::create(dto).await.map_err(|e| {
        error!("Error al insertar empresa '{}': {}", nombre_normalizado, e);
        map_db_error(e)
    })?;

    info!("Empresa '{}' creada exitosamente con ID {}", nombre_normalizado, empresa.id);

    // 5. Retornar
    Ok(EmpresaResponse::from(empresa))
}

pub async fn get_empresa_by_id(id_str: String) -> Result<EmpresaResponse, EmpresaError> {
    let id_thing = parse_empresa_id(&id_str);
    let empresa_opt = db::find_by_id(&id_thing).await.map_err(map_db_error)?;
    let empresa = empresa_opt.ok_or(EmpresaError::NotFound)?;

    let total_contratistas =
        db::count_contratistas_by_empresa(&id_thing).await.map_err(map_db_error)?;

    let mut response = EmpresaResponse::from(empresa);
    response.total_contratistas = total_contratistas;

    Ok(response)
}

pub async fn get_all_empresas() -> Result<EmpresaListResponse, EmpresaError> {
    let empresas = db::find_all().await.map_err(map_db_error)?;

    let mut responses = Vec::new();
    for empresa in empresas {
        let total_contratistas =
            db::count_contratistas_by_empresa(&empresa.id).await.map_err(map_db_error)?;
        let mut response = EmpresaResponse::from(empresa);
        response.total_contratistas = total_contratistas;
        responses.push(response);
    }

    let total = responses.len();
    let activas = responses.iter().filter(|e| e.is_active).count();

    Ok(EmpresaListResponse { empresas: responses, total, activas })
}

pub async fn get_empresas_activas() -> Result<Vec<EmpresaResponse>, EmpresaError> {
    let empresas = db::get_empresas_activas().await.map_err(map_db_error)?;

    let mut responses = Vec::new();
    for empresa in empresas {
        let total_contratistas =
            db::count_contratistas_by_empresa(&empresa.id).await.map_err(map_db_error)?;
        let mut response = EmpresaResponse::from(empresa);
        response.total_contratistas = total_contratistas;
        responses.push(response);
    }

    Ok(responses)
}

pub async fn update_empresa(
    id_str: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    let id_thing = parse_empresa_id(&id_str);

    // 2. Verificar que existe
    let _ = db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // 3. Normalizar y preparar data
    let mut update_data = serde_json::Map::new();
    if let Some(ref nombre) = input.nombre {
        let normalizado = domain::normalizar_nombre(nombre);
        // Verificar si otra empresa tiene este nombre
        let empresas = db::find_all().await.map_err(map_db_error)?;
        if empresas.iter().any(|e| {
            e.id.to_string() != id_thing.to_string()
                && domain::normalizar_nombre(&e.nombre) == normalizado
        }) {
            return Err(EmpresaError::NameExists);
        }
        update_data.insert("nombre".to_string(), serde_json::json!(normalizado));
    };

    if let Some(ref direccion) = input.direccion {
        update_data.insert("direccion".to_string(), serde_json::json!(direccion));
    }
    if let Some(is_active) = input.is_active {
        update_data.insert("is_active".to_string(), serde_json::json!(is_active));
    }
    update_data.insert("updated_at".to_string(), serde_json::json!(Utc::now()));

    // 4. Actualizar
    info!("Actualizando empresa con ID {}", id_str);
    let empresa =
        db::update(&id_thing, serde_json::Value::Object(update_data)).await.map_err(|e| {
            error!("Error al actualizar empresa {}: {}", id_str, e);
            map_db_error(e)
        })?;

    info!("Empresa {} actualizada exitosamente", id_str);

    // 5. Retornar
    Ok(EmpresaResponse::from(empresa))
}

pub async fn delete_empresa(id_str: String) -> Result<(), EmpresaError> {
    let id_thing = parse_empresa_id(&id_str);

    // 1. Verificar que existe
    let _ = db::find_by_id(&id_thing).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // 2. Verificar que no tenga contratistas
    let count = db::count_contratistas_by_empresa(&id_thing).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(EmpresaError::HasContratistas(count as i64));
    }

    info!("Eliminando empresa con ID {}", id_str);
    // 3. Eliminar
    db::delete(&id_thing).await.map_err(|e| {
        error!("Error al eliminar empresa {}: {}", id_str, e);
        map_db_error(e)
    })?;

    info!("Empresa {} eliminada exitosamente", id_str);
    Ok(())
}
