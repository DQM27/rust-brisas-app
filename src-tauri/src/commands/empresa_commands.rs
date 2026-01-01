/// Puertos de Entrada: Gestión de Entidades Corporativas (Corporate Bridge).
///
/// Este módulo orquesta la administración de las empresas (contratistas o proveedores)
/// registradas en el sistema, sirviendo como "paraguas" bajo el cual se agrupan
/// los perfiles individuales de trabajadores.
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service as service;
use crate::services::session::SessionState;
use tauri::State;

/// Crea un perfil corporativo nuevo en el directorio central de la planta.
#[tauri::command]
pub async fn create_empresa(
    session: State<'_, SessionState>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:create", "Registrando nueva entidad corporativa")?;
    service::create_empresa(input).await
}

#[tauri::command]
pub async fn get_empresa_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_empresa_by_id(&id).await
}

/// Auditoría Central: Lista la totalidad de empresas vinculadas operativamente a la planta.
#[tauri::command]
pub async fn get_all_empresas(
    session: State<'_, SessionState>,
) -> Result<EmpresaListResponse, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_all_empresas().await
}

/// Filtro Operativo: Recupera exclusivamente empresas con estatus activo (habilitadas para laborar).
#[tauri::command]
pub async fn get_empresas_activas(
    session: State<'_, SessionState>,
) -> Result<Vec<EmpresaResponse>, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_empresas_activas().await
}

/// Actualiza los datos fiscales, de contacto o administrativos de una empresa.
#[tauri::command]
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

/// Baja Administrativa: Elimina (o archiva) una empresa del catálogo vigente.
#[tauri::command]
pub async fn delete_empresa(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), EmpresaError> {
    require_perm!(session, "empresas:delete", format!("Dando de baja entidad corporativa {}", id))?;
    service::delete_empresa(&id).await
}
