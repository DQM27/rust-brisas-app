// ==========================================
// src/commands/lista_negra_commands.rs
// ==========================================
// Capa de API: Tauri command handlers (thin wrappers)

use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    NivelStats, UpdateListaNegraInput,
};
use crate::services::lista_negra_service;
use tauri::command;

/// Agrega una persona a la lista negra
#[command]
pub async fn add_to_lista_negra(
    input: AddToListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::add_to_lista_negra(
        input.cedula,
        input.nombre,
        input.apellido,
        input.nivel_severidad,
        input.motivo_bloqueo,
        input.bloqueado_por,
    )
    .await
    .map(|l| l.into())
    .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Obtiene un registro de lista negra por ID
#[command]
pub async fn get_lista_negra_by_id(id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::get_lista_negra_by_id(id)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))?
        .ok_or(ListaNegraError::NotFound)
}

/// Obtiene todos los registros de lista negra
#[command]
pub async fn get_all_lista_negra() -> Result<ListaNegraListResponse, ListaNegraError> {
    let list = lista_negra_service::get_all_lista_negra()
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))?;

    let total = list.len();
    let activos = list.iter().filter(|l| l.is_active).count();

    Ok(ListaNegraListResponse {
        bloqueados: list,
        total,
        activos,
        por_nivel: NivelStats { alto: 0, medio: 0, bajo: 0 },
    })
}

/// Obtiene solo los registros activos de lista negra
#[command]
pub async fn get_lista_negra_activos() -> Result<Vec<ListaNegraResponse>, ListaNegraError> {
    lista_negra_service::get_lista_negra_activos()
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Verifica si una cédula está bloqueada (CRÍTICO para validaciones)
#[command]
pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, ListaNegraError> {
    lista_negra_service::check_is_blocked(cedula)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Obtiene información de bloqueo por cédula
#[command]
pub async fn get_blocked_by_cedula(
    cedula: String,
) -> Result<Option<ListaNegraResponse>, ListaNegraError> {
    lista_negra_service::get_blocked_by_cedula(&cedula)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Desactiva un bloqueo (quita de lista negra)
#[command]
pub async fn remove_from_lista_negra(id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::remove_from_lista_negra(
        id,
        "Desbloqueo manual".to_string(),
        "admin".to_string(),
    )
    .await
    .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))?;

    Err(ListaNegraError::Database(sqlx::Error::Protocol("Not implemented response".to_string())))
}

/// Reactiva un bloqueo
#[command]
pub async fn reactivate_lista_negra(
    id: String,
    _nivel_severidad: String,
    motivo_bloqueo: String,
    bloqueado_por: String,
) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::reactivate_lista_negra(id, motivo_bloqueo, bloqueado_por)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))?;
    Err(ListaNegraError::Database(sqlx::Error::Protocol("Not implemented response".to_string())))
}

/// Actualiza información de un bloqueo
#[command]
pub async fn update_lista_negra(
    id: String,
    input: UpdateListaNegraInput,
) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::update_lista_negra(
        id,
        input.motivo_bloqueo.unwrap_or_default(),
        input.nivel_severidad.unwrap_or_default(),
        "admin".to_string(),
    )
    .await
    .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Elimina permanentemente un registro de lista negra
#[command]
pub async fn delete_lista_negra(id: String) -> Result<(), ListaNegraError> {
    lista_negra_service::delete_lista_negra(id)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}

/// Busca personas para formulario de bloqueo
#[command]
pub async fn search_personas_for_block(
    query: String,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, ListaNegraError> {
    lista_negra_service::search_personas_for_block(&query)
        .await
        .map_err(|e| ListaNegraError::Database(sqlx::Error::Protocol(e)))
}
