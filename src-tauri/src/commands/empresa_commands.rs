/// Puertos de Entrada: Gestión de Entidades Corporativas (Corporate Bridge).
///
/// Este módulo orquesta la administración de las empresas (contratistas o proveedores)
/// registradas en el sistema.
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service as service;
use crate::services::session::SessionState;
use tauri::{command, State};

// --------------------------------------------------------------------------
// CONSULTAS CORPORATIVAS
// --------------------------------------------------------------------------

/// [Comando Tauri] Auditoría Central: Lista la totalidad de empresas vinculadas.
///
/// # Argumentos
/// * `session` - Estado de la sesión actual para validación de permisos.
///
/// # Retorno
/// Lista completa de empresas con estadísticas de actividad o error de permisos.
#[command]
pub async fn get_all_empresas(
    session: State<'_, SessionState>,
) -> Result<EmpresaListResponse, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_all_empresas().await
}

/// [Comando Tauri] Filtro Operativo: Recupera exclusivamente empresas activas.
///
/// # Argumentos
/// * `session` - Estado de la sesión actual.
///
/// # Retorno
/// Vector de empresas habilitadas para operar en planta.
#[command]
pub async fn get_empresas_activas(
    session: State<'_, SessionState>,
) -> Result<Vec<EmpresaResponse>, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_empresas_activas().await
}

/// [Comando Tauri] Obtiene el perfil detallado de una empresa por ID.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `id` - Identificador de la empresa.
///
/// # Retorno
/// Datos de la empresa o error si no existe.
#[command]
pub async fn get_empresa_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_empresa_by_id(&id).await
}

// --------------------------------------------------------------------------
// OPERACIONES DE GESTIÓN (MUTACIONES)
// --------------------------------------------------------------------------

/// [Comando Tauri] Crea un perfil corporativo nuevo en el sistema.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `input` - Datos de la nueva empresa.
///
/// # Retorno
/// La empresa creada o error de validación/duplicidad.
#[command]
pub async fn create_empresa(
    session: State<'_, SessionState>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:create", "Registrando nueva entidad corporativa")?;
    service::create_empresa(input).await
}

/// [Comando Tauri] Actualiza los datos administrativos de una empresa.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `id` - ID de la empresa a modificar.
/// * `input` - Campos a actualizar.
///
/// # Retorno
/// Perfil corporativo actualizado.
#[command]
pub async fn update_empresa(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(
        session,
        "empresas:update",
        format!("Actualizando perfil de empresa ID: {}", id)
    )?;
    service::update_empresa(&id, input).await
}

/// [Comando Tauri] Baja Administrativa: Elimina una empresa del catálogo.
///
/// # Argumentos
/// * `session` - Estado de la sesión.
/// * `id` - ID de la empresa a eliminar.
///
/// # Retorno
/// Ok(()) o error si tiene dependencias activas (contratistas).
#[command]
pub async fn delete_empresa(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), EmpresaError> {
    require_perm!(session, "empresas:delete", format!("Dando de baja entidad corporativa {}", id))?;
    service::delete_empresa(&id).await
}
