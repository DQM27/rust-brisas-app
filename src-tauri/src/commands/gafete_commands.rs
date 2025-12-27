// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse,
    UpdateGafeteInput, UpdateGafeteStatusInput,
};

#[tauri::command]
pub async fn create_gafete(_input: CreateGafeteInput) -> Result<GafeteResponse, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn create_gafete_range(
    _input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_gafete(_numero: String, _tipo: String) -> Result<GafeteResponse, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_all_gafetes() -> Result<GafeteListResponse, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn get_gafetes_disponibles(_tipo: String) -> Result<Vec<GafeteResponse>, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn is_gafete_disponible(_numero: String, _tipo: String) -> Result<bool, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn update_gafete(
    _numero: String,
    _tipo: String,
    _input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn update_gafete_status(
    _numero: String,
    _tipo: String,
    _input: UpdateGafeteStatusInput,
    _usuario_id: Option<String>,
    _motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

#[tauri::command]
pub async fn delete_gafete(
    _numero: String,
    _tipo: String,
    _usuario_id: Option<String>,
) -> Result<(), GafeteError> {
    Err(GafeteError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
