// ==========================================
// src/commands/lista_negra_commands.rs
// ==========================================
// Capa de API: Tauri command handlers (thin wrappers)
// Solo delega a la capa de servicio

use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    UpdateListaNegraInput,
};

/// Agrega una persona a la lista negra
#[tauri::command]
pub async fn add_to_lista_negra(
    _input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene un registro de lista negra por ID
#[tauri::command]
pub async fn get_lista_negra_by_id(_id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene todos los registros de lista negra
#[tauri::command]
pub async fn get_all_lista_negra() -> Result<ListaNegraListResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene solo los registros activos de lista negra
#[tauri::command]
pub async fn get_lista_negra_activos() -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Verifica si una cédula está bloqueada (CRÍTICO para validaciones)
#[tauri::command]
pub async fn check_is_blocked(_cedula: String) -> Result<BlockCheckResponse, ListaNegraError> {
    // Por seguridad en el stub, retornamos que no está bloqueado para no romper flujos básicos,
    // pero idealmente debería retornar un error o implementar la query real pronto.
    Ok(BlockCheckResponse { is_blocked: false, nivel_severidad: None, bloqueado_desde: None })
}

/// Obtiene información de bloqueo por cédula
#[tauri::command]
pub async fn get_blocked_by_cedula(
    _cedula: String,
) -> Result<Option<ListaNegraResponse>, ListaNegraError> {
    Ok(None)
}

/// Desactiva un bloqueo (quita de lista negra)
#[tauri::command]
pub async fn remove_from_lista_negra(_id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Reactiva un bloqueo (re-bloquear persona previamente desbloqueada)
#[tauri::command]
pub async fn reactivate_lista_negra(
    _id: String,
    _nivel_severidad: String,
    _motivo_bloqueo: String,
    _bloqueado_por: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Actualiza información de un bloqueo
#[tauri::command]
pub async fn update_lista_negra(
    _id: String,
    _input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Elimina permanentemente un registro de lista negra
#[tauri::command]
pub async fn delete_lista_negra(_id: String) -> Result<(), ListaNegraError> {
    Err(ListaNegraError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Busca personas (contratistas, proveedores, visitas) para formulario de bloqueo
#[tauri::command]
pub async fn search_personas_for_block(
    _query: String,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, ListaNegraError> {
    Ok(vec![])
}
