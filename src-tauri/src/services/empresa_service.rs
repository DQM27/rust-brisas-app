/// Gestión de la Estructura Organizacional (Empresas).
///
/// Este servicio gestiona las entidades empresariales que actúan como "padres" de los
/// contratistas y proveedores. Es el eje que permite organizar a las personas externas
/// por su procedencia, facilitando reportes y controles grupales.
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

/// Mapeo de errores de infraestructura a dominio.
fn map_db_error(e: SurrealDbError) -> EmpresaError {
    EmpresaError::Database(e.to_string())
}

/// Normalización de IDs de empresa para SurrealDB.
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

/// Recupera todas las empresas registradas con estadísticas básicas de actividad.
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

/// Registra una nueva empresa garantizando la unicidad del nombre.
pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    domain::validar_create_input(&input)?;

    // Control de duplicidad: No se permiten dos empresas con el mismo nombre.
    let exists = db::exists_by_name(&input.nombre).await.map_err(map_db_error)?;
    if exists {
        return Err(EmpresaError::NameExists);
    }

    let dto = crate::models::empresa::EmpresaCreateDTO {
        nombre: input.nombre.trim().to_string(),
        direccion: input.direccion.map(|s| s.trim().to_string()),
        is_active: true,
    };

    let saved = db::create(dto).await.map_err(map_db_error)?;

    info!("Nueva empresa registrada: {}", saved.nombre);
    Ok(EmpresaResponse::from(saved))
}

// ==========================================
// ACTUALIZAR
// ==========================================

/// Actualiza los datos de una empresa. Permite la desactivación lógica (is_active).
pub async fn update_empresa(
    id_str: &str,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    let id = parse_empresa_id(id_str);

    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    domain::validar_update_input(&input)?;

    let mut dto = crate::models::empresa::EmpresaUpdateDTO::default();
    if let Some(ref nombre) = input.nombre {
        dto.nombre = Some(nombre.trim().to_string());
    }

    if let Some(ref direccion) = input.direccion {
        dto.direccion = Some(direccion.clone());
    }
    if let Some(is_active) = input.is_active {
        dto.is_active = Some(is_active);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    info!("Actualizando empresa: {}", id_str);
    let updated = db::update(&id, dto).await.map_err(map_db_error)?;

    Ok(EmpresaResponse::from(updated))
}

// ==========================================
// ELIMINAR
// ==========================================

/// Elimina una empresa del sistema.
///
/// Regla de Integridad Referencial:
/// No se puede eliminar una empresa que tenga contratistas asociados.
/// Esto evita dejar registros de personas "huérfanas" en la base de datos.
pub async fn delete_empresa(id_str: &str) -> Result<(), EmpresaError> {
    let id = parse_empresa_id(id_str);

    let count = db::count_contratistas_by_empresa(&id).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(EmpresaError::HasContratistas(count as i64));
    }

    db::delete(&id).await.map_err(map_db_error)?;

    info!("Empresa eliminada físicamente: {}", id_str);
    Ok(())
}
