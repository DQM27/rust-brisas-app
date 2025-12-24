// ==========================================
// src/commands/empresa_commands.rs
// ==========================================

use crate::db::DbPool;
use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service;
use tauri::State;

#[tauri::command]
pub async fn create_empresa(
    pool_state: State<'_, DbPool>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::create_empresa(&pool, input).await
}

#[tauri::command]
pub async fn get_empresa_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<EmpresaResponse, EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::get_empresa_by_id(&pool, id).await
}

#[tauri::command]
pub async fn get_all_empresas(
    pool_state: State<'_, DbPool>,
) -> Result<EmpresaListResponse, EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::get_all_empresas(&pool).await
}

#[tauri::command]
pub async fn get_empresas_activas(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<EmpresaResponse>, EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::get_empresas_activas(&pool).await
}

#[tauri::command]
pub async fn update_empresa(
    pool_state: State<'_, DbPool>,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::update_empresa(&pool, id, input).await
}

#[tauri::command]
pub async fn delete_empresa(pool_state: State<'_, DbPool>, id: String) -> Result<(), EmpresaError> {
    let pool = pool_state.0.read().await;
    empresa_service::delete_empresa(&pool, id).await
}
