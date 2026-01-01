/// Puertos de Entrada: Gestión de Restricciones y Lista Negra (Security Barrier Bridge).
///
/// Este módulo es el núcleo de las políticas de denegación de acceso. Permite
/// identificar, registrar y consultar a sujetos que han incurrido en faltas graves,
/// asegurando que el sistema bloquee proactivamente su entrada en portería.
use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{
    AddToListaNegraInput, BlockCheckResponse, ListaNegraListResponse, ListaNegraResponse,
    NivelStats,
};
use crate::services::lista_negra_service;
use tauri::command;

/// Registra una nueva restricción de acceso para un individuo.
#[command]
pub async fn add_to_lista_negra(input: AddToListaNegraInput) -> Result<(), ListaNegraError> {
    lista_negra_service::add_to_lista_negra(
        input.cedula,
        input.nombre,
        input.apellido,
        input.nivel_severidad,
        input.motivo_bloqueo,
        input.bloqueado_por,
    )
    .await
    .map_err(|e| ListaNegraError::Database(e))
}

#[command]
pub async fn get_lista_negra_by_id(id: String) -> Result<ListaNegraResponse, ListaNegraError> {
    lista_negra_service::get_lista_negra_by_id(id)
        .await
        .map_err(|e| ListaNegraError::Database(e))?
        .ok_or(ListaNegraError::NotFound)
}

/// Auditoría de Seguridad: Obtiene la relación completa de personas con restricciones vigentes.
#[command]
pub async fn get_all_lista_negra() -> Result<ListaNegraListResponse, ListaNegraError> {
    let list = lista_negra_service::get_all_lista_negra()
        .await
        .map_err(|e| ListaNegraError::Database(e))?;

    let total = list.len();
    let activos = list.iter().filter(|l| l.is_active).count();

    Ok(ListaNegraListResponse {
        bloqueados: list,
        total,
        activos,
        por_nivel: NivelStats { alto: 0, medio: 0, bajo: 0 },
    })
}

/// Motor de Validación: Comprueba en tiempo real si una cédula tiene prohibido el acceso (Punto Crítico).
#[command]
pub async fn check_is_blocked(cedula: String) -> Result<BlockCheckResponse, ListaNegraError> {
    lista_negra_service::check_is_blocked(cedula).await.map_err(|e| ListaNegraError::Database(e))
}

/// Asistente de Registro: Localiza perfiles existentes para facilitar su inclusión en lista negra.
#[command]
pub async fn search_personas_for_block(
    query: String,
) -> Result<Vec<crate::models::lista_negra::PersonaSearchResult>, ListaNegraError> {
    lista_negra_service::search_personas_for_block(&query)
        .await
        .map_err(|e| ListaNegraError::Database(e))
}
