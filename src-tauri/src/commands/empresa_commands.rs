// ==========================================
// src/commands/empresa_commands.rs
// ==========================================

use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service as service;
use crate::services::session::SessionState;
use tauri::State;

#[tauri::command]
pub async fn create_empresa(
    session: State<'_, SessionState>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:create", "Creando empresa")?;
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

#[tauri::command]
pub async fn get_all_empresas(
    session: State<'_, SessionState>,
) -> Result<EmpresaListResponse, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_all_empresas().await
}

#[tauri::command]
pub async fn get_empresas_activas(
    session: State<'_, SessionState>,
) -> Result<Vec<EmpresaResponse>, EmpresaError> {
    require_perm!(session, "empresas:read")?;
    service::get_empresas_activas().await
}

#[tauri::command]
pub async fn update_empresa(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    require_perm!(session, "empresas:update", format!("Actualizando empresa {}", id))?;
    service::update_empresa(&id, input).await
}

#[tauri::command]
pub async fn delete_empresa(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), EmpresaError> {
    require_perm!(session, "empresas:delete", format!("Eliminando empresa {}", id))?;
    service::delete_empresa(&id).await
}
