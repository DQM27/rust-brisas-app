/// Servicio: Gestión de la Estructura Organizacional (Empresas).
///
/// Este servicio orquesta la administración de las entidades empresariales que actúan
/// como "padres" de los contratistas y proveedores.
///
/// Responsabilidades:
/// - Registro y actualización de perfiles corporativos.
/// - Gestión de la vigencia operativa (activación/desactivación).
/// - Control de integridad referencial para eliminaciones.
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

// --------------------------------------------------------------------------
// CONSULTAS DE EMPRESA
// --------------------------------------------------------------------------

/// Recupera todas las empresas registradas con estadísticas básicas de actividad.
///
/// # Retorno
/// Retorna `EmpresaListResponse` con la lista de empresas y conteos totales/activos.
///
/// # Errores
/// - `EmpresaError::Database`: Fallo en la comunicación con `SurrealDB`.
pub async fn get_all_empresas() -> Result<EmpresaListResponse, EmpresaError> {
    let empresas = db::find_all().await.map_err(map_db_error)?;
    let total = empresas.len();
    let activas = empresas.iter().filter(|e| e.is_active).count();

    let responses: Vec<EmpresaResponse> = empresas.into_iter().map(EmpresaResponse::from).collect();

    Ok(EmpresaListResponse { empresas: responses, total, activas })
}

/// Recupera exclusivamente las empresas con estatus activo.
///
/// # Retorno
/// Vector de `EmpresaResponse`.
///
/// # Errores
/// - `EmpresaError::Database`: Error de acceso a datos.
pub async fn get_empresas_activas() -> Result<Vec<EmpresaResponse>, EmpresaError> {
    let empresas = db::get_empresas_activas().await.map_err(map_db_error)?;
    Ok(empresas.into_iter().map(EmpresaResponse::from).collect())
}

/// Obtiene el detalle completo de una empresa específica por su ID.
///
/// # Argumentos
/// * `id_str` - ID de la empresa (formato "empresa:id" o "id").
///
/// # Retorno
/// Detalle de la empresa o error si no existe.
///
/// # Errores
/// - `EmpresaError::NotFound`: La empresa no existe.
/// - `EmpresaError::Database`: Error técnico de base de datos.
pub async fn get_empresa_by_id(id_str: &str) -> Result<EmpresaResponse, EmpresaError> {
    let id = parse_empresa_id(id_str);
    let empresa = db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    Ok(EmpresaResponse::from(empresa))
}

// --------------------------------------------------------------------------
// OPERACIONES DE GESTIÓN (MUTACIONES)
// --------------------------------------------------------------------------

/// Registra una nueva empresa garantizando la unicidad del nombre.
///
/// # Argumentos
/// * `input` - Datos de creación (nombre, dirección).
///
/// # Retorno
/// La empresa recién creada.
///
/// # Errores
/// - `EmpresaError::Validation`: Datos inválidos según reglas de dominio.
/// - `EmpresaError::NameExists`: Ya existe una empresa con ese nombre.
/// - `EmpresaError::Database`: Error de persistencia.
pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    // 1. Validación de Dominio (Capa Pura)
    domain::validar_create_input(&input)?;

    // 2. Control de duplicidad
    let exists = db::exists_by_name(&input.nombre).await.map_err(map_db_error)?;
    if exists {
        return Err(EmpresaError::NameExists);
    }

    let dto = crate::models::empresa::EmpresaCreateDTO {
        nombre: domain::normalizar_nombre(&input.nombre),
        direccion: input.direccion.map(|s| s.trim().to_string()),
        is_active: true,
    };

    let saved = db::create(dto).await.map_err(map_db_error)?;

    info!("Nueva empresa registrada: {}", saved.nombre);
    Ok(EmpresaResponse::from(saved))
}

/// Actualiza los datos de una empresa existente.
///
/// # Argumentos
/// * `id_str` - ID de la empresa.
/// * `input` - Campos a actualizar.
///
/// # Retorno
/// La empresa actualizada.
///
/// # Errores
/// - `EmpresaError::NotFound`: La empresa no existe.
/// - `EmpresaError::Validation`: Reglas de dominio fallidas.
/// - `EmpresaError::Database`: Error de `SurrealDB`.
pub async fn update_empresa(
    id_str: &str,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    let id = parse_empresa_id(id_str);

    // Verificar existencia previa
    db::find_by_id(&id).await.map_err(map_db_error)?.ok_or(EmpresaError::NotFound)?;

    // Validación de dominio
    domain::validar_update_input(&input)?;

    let mut dto = crate::models::empresa::EmpresaUpdateDTO::default();
    if let Some(ref nombre) = input.nombre {
        dto.nombre = Some(domain::normalizar_nombre(nombre));
    }

    if let Some(ref direccion) = input.direccion {
        dto.direccion = Some(direccion.trim().to_string());
    }
    if let Some(is_active) = input.is_active {
        dto.is_active = Some(is_active);
    }
    dto.updated_at = Some(surrealdb::Datetime::from(Utc::now()));

    info!("Actualizando empresa: {id_str}");
    let updated = db::update(&id, dto).await.map_err(map_db_error)?;

    Ok(EmpresaResponse::from(updated))
}

/// Elimina una empresa del sistema verificando vínculos previos.
///
/// Regla de Integridad: No se puede eliminar si tiene contratistas asociados.
///
/// # Argumentos
/// * `id_str` - ID de la empresa a borrar.
///
/// # Retorno
/// Ok(()) si la eliminación física fue exitosa.
///
/// # Errores
/// - `EmpresaError::HasContratistas`: Violación de integridad referencial.
/// - `EmpresaError::Database`: Error en la operación de borrado.
pub async fn delete_empresa(id_str: &str) -> Result<(), EmpresaError> {
    let id = parse_empresa_id(id_str);

    // Verificación de integridad referencial
    let count = db::count_contratistas_by_empresa(&id).await.map_err(map_db_error)?;
    if count > 0 {
        return Err(EmpresaError::HasContratistas(count as i64));
    }

    db::delete(&id).await.map_err(map_db_error)?;

    info!("Empresa eliminada físicamente: {id_str}");
    Ok(())
}

// --------------------------------------------------------------------------
// HELPERS INTERNOS
// --------------------------------------------------------------------------

/// Mapeo de errores de infraestructura a dominio.
fn map_db_error(e: SurrealDbError) -> EmpresaError {
    EmpresaError::Database(e.to_string())
}

/// Normalización de IDs de empresa para `SurrealDB`.
fn parse_empresa_id(id_str: &str) -> RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        RecordId::from_table_key(parts[0], parts[1])
    } else {
        RecordId::from_table_key("empresa", id_str)
    }
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS (Lógica Pura)
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empresa_id() {
        let id_with_table = parse_empresa_id("empresa:abc");
        assert_eq!(id_with_table.to_string().replace("⟨", "").replace("⟩", ""), "empresa:abc");

        let id_simple = parse_empresa_id("xyz");
        assert_eq!(id_simple.to_string().replace("⟨", "").replace("⟩", ""), "empresa:xyz");

        let id_other = parse_empresa_id("proveedor:123");
        assert_eq!(id_other.to_string().replace("⟨", "").replace("⟩", ""), "proveedor:123");
    }
}
