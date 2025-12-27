// ==========================================
// src/commands/contratista_commands.rs
// ==========================================
// Capa de comandos Tauri: delega al servicio

use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CambiarEstadoInput, ContratistaListResponse, ContratistaResponse, CreateContratistaInput,
    UpdateContratistaInput,
};
use crate::services::contratista_service;

#[tauri::command]
pub async fn create_contratista(
    _input: CreateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_contratista_by_id(_id: String) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    _cedula: String,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_all_contratistas() -> Result<ContratistaListResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_contratistas_activos() -> Result<Vec<ContratistaResponse>, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn update_contratista(
    _id: String,
    _input: UpdateContratistaInput,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    _id: String,
    _input: CambiarEstadoInput,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn delete_contratista(_id: String) -> Result<(), ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

// ==========================================
// COMANDOS CON AUDITORÍA
// ==========================================

/// Actualiza la fecha PRAIND de un contratista con registro en historial
#[tauri::command]
pub async fn actualizar_praind_con_historial(
    _input: contratista_service::ActualizarPraindInput,
    _usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Cambia el estado de un contratista con registro de motivo en historial
#[tauri::command]
pub async fn cambiar_estado_con_historial(
    _input: contratista_service::CambiarEstadoConHistorialInput,
    _usuario_id: String,
) -> Result<ContratistaResponse, ContratistaError> {
    Err(ContratistaError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
