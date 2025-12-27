// ==========================================
// src/commands/empresa_commands.rs
// ==========================================

use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service as service;

#[tauri::command]
pub async fn create_empresa(input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    service::create_empresa(input).await
}

#[tauri::command]
pub async fn get_empresa_by_id(id: String) -> Result<EmpresaResponse, EmpresaError> {
    service::get_empresa_by_id(&id).await
}

#[tauri::command]
pub async fn get_all_empresas() -> Result<EmpresaListResponse, EmpresaError> {
    service::get_all_empresas().await
}

#[tauri::command]
pub async fn get_empresas_activas() -> Result<Vec<EmpresaResponse>, EmpresaError> {
    service::get_empresas_activas().await
}

#[tauri::command]
pub async fn update_empresa(
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    service::update_empresa(&id, input).await
}

#[tauri::command]
pub async fn delete_empresa(id: String) -> Result<(), EmpresaError> {
    service::delete_empresa(&id).await
}
