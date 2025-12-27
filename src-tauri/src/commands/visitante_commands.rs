// ==========================================
// src/commands/visitante_commands.rs
// ==========================================

use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use crate::services::visitante_service;
use tauri::command;

#[command]
pub async fn create_visitante(input: CreateVisitanteInput) -> Result<VisitanteResponse, String> {
    visitante_service::create_visitante(input).await.map_err(|e| e.to_string())
}

#[command]
pub async fn search_visitantes_catalog(query: String) -> Result<Vec<VisitanteResponse>, String> {
    visitante_service::search_visitantes(&query).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_visitante_by_cedula(cedula: String) -> Result<Option<VisitanteResponse>, String> {
    visitante_service::get_visitante_by_cedula(&cedula).await.map_err(|e| e.to_string())
}

#[command]
pub async fn update_visitante(
    id: String,
    input: CreateVisitanteInput,
) -> Result<VisitanteResponse, String> {
    visitante_service::update_visitante(&id, input).await.map_err(|e| e.to_string())
}

#[command]
pub async fn delete_visitante(id: String) -> Result<(), String> {
    visitante_service::delete_visitante(&id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn restore_visitante(id: String) -> Result<VisitanteResponse, String> {
    visitante_service::restore_visitante(&id).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_archived_visitantes() -> Result<Vec<VisitanteResponse>, String> {
    visitante_service::get_archived_visitantes().await.map_err(|e| e.to_string())
}
