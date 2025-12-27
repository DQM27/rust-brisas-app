// ==========================================
// src/commands/empresa_commands.rs
// ==========================================

use crate::domain::errors::EmpresaError;
use crate::models::empresa::{
    CreateEmpresaInput, EmpresaListResponse, EmpresaResponse, UpdateEmpresaInput,
};

#[tauri::command]
pub async fn create_empresa(_input: CreateEmpresaInput) -> Result<EmpresaResponse, EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_empresa_by_id(_id: String) -> Result<EmpresaResponse, EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_all_empresas() -> Result<EmpresaListResponse, EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_empresas_activas() -> Result<Vec<EmpresaResponse>, EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn update_empresa(
    _id: String,
    _input: UpdateEmpresaInput,
) -> Result<EmpresaResponse, EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn delete_empresa(_id: String) -> Result<(), EmpresaError> {
    Err(EmpresaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
