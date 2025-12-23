// ==========================================
// src/commands/empresa_commands.rs
// ==========================================

use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};
use crate::services::empresa_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_empresa(
    pool: State<'_, SqlitePool>,
    input: CreateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    empresa_service::create_empresa(&pool, input).await
}

#[tauri::command]
pub async fn get_empresa_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<EmpresaResponse, EmpresaError> {
    empresa_service::get_empresa_by_id(&pool, id).await
}

#[tauri::command]
pub async fn get_all_empresas(
    pool: State<'_, SqlitePool>,
) -> Result<EmpresaListResponse, EmpresaError> {
    empresa_service::get_all_empresas(&pool).await
}

#[tauri::command]
pub async fn get_empresas_activas(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<EmpresaResponse>, EmpresaError> {
    empresa_service::get_empresas_activas(&pool).await
}

#[tauri::command]
pub async fn update_empresa(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    empresa_service::update_empresa(&pool, id, input).await
}

#[tauri::command]
pub async fn delete_empresa(pool: State<'_, SqlitePool>, id: String) -> Result<(), EmpresaError> {
    empresa_service::delete_empresa(&pool, id).await
}
