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
use log::{error, info};

// Helper para mapear errores de SurrealDB a EmpresaError
fn map_db_error(e: SurrealDbError) -> EmpresaError {
    EmpresaError::Database(e.to_string())
}

// ==========================================
// CREAR EMPRESA
// ==========================================

pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validar input
    domain::validar_create_input(&input)?;

    // 2. Normalizar nombre
    let nombre_normalizado = domain::normalizar_nombre(&input.nombre);

    // 3. Verificar que no exista (SurrealDB check)
    // Nota: El índice único en SurrealDB también lo atraparía,
    // pero mantenemos la lógica de validación de negocio.
    let empresas = db::find_all().await.map_err(map_db_error)?;
    if empresas.iter().any(|e| domain::normalizar_nombre(&e.nombre) == nombre_normalizado) {
        return Err(EmpresaError::NameExists);
    }

    // 4. Crear
    info!("Creando empresa '{}'", nombre_normalizado);
    let mut create_input = input;
    create_input.nombre = nombre_normalizado.clone();

    let empresa = db::create(create_input).await.map_err(|e| {
        error!("Error al insertar empresa '{}': {}", nombre_normalizado, e);
        map_db_error(e)
    })?;

    info!("Empresa '{}' creada exitosamente con ID {}", nombre_normalizado, empresa.id);

    // 5. Retornar
    Ok(EmpresaResponse::from(empresa))
}

// ==========================================
// OBTENER EMPRESA
// ==========================================

pub async fn get_empresa_by_id(id: String) -> Result<EmpresaResponse, EmpresaError> {
    let empresa_opt = db::find_by_id(&id).await.map_err(map_db_error)?;
    let empresa = empresa_opt.ok_or(EmpresaError::NotFound)?;

    // Contar contratistas (placeholder por ahora en el query module)
    let total_contratistas = db::count_contratistas_by_empresa(&id).await.map_err(map_db_error)?;

    let mut response = EmpresaResponse::from(empresa);
    response.total_contratistas = total_contratistas;

    Ok(response)
}

// ==========================================
// OBTENER TODAS
// ==========================================

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

// ==========================================
// OBTENER ACTIVAS
// ==========================================

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

// ==========================================
// ACTUALIZAR
// ==========================================

pub async fn update_empresa(
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validar input
    domain::validar_update_input(&input)?;

    // 2. Verificar que existe
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // 3. Normalizar y verificar nombre si viene
    if let Some(ref nombre) = input.nombre {
        let normalizado = domain::normalizar_nombre(nombre);

        // Verificar si otra empresa tiene este nombre
        let empresas = db::find_all().await.map_err(map_db_error)?;
        if empresas
            .iter()
            .any(|e| e.id != id && domain::normalizar_nombre(&e.nombre) == normalizado)
        {
            return Err(EmpresaError::NameExists);
        }
    };

    // 4. Actualizar
    info!("Actualizando empresa con ID {}", id);
    let empresa = db::update(&id, input).await.map_err(|e| {
        error!("Error al actualizar empresa {}: {}", id, e);
        map_db_error(e)
    })?;

    info!("Empresa {} actualizada exitosamente", id);

    // 5. Retornar
    Ok(EmpresaResponse::from(empresa))
}

// ==========================================
// ELIMINAR
// ==========================================

pub async fn delete_empresa(id: String) -> Result<(), EmpresaError> {
    // 1. Verificar que existe
    let _ = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // 2. Verificar que no tenga contratistas
    let count = db::count_contratistas_by_empresa(&id).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(EmpresaError::HasContratistas(count as i64));
    }

    info!("Eliminando empresa con ID {}", id);
    // 3. Eliminar
    db::delete(&id).await.map_err(|e| {
        error!("Error al eliminar empresa {}: {}", id, e);
        map_db_error(e)
    })?;

    info!("Empresa {} eliminada exitosamente", id);
    Ok(())
}
