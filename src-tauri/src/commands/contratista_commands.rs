// commands/contratista_commands.rs
use sqlx::SqlitePool;
use tauri::State;
use crate::models::contratista::*;
use crate::services::contratista_service::ContratistaService;

#[tauri::command]
pub async fn create_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateContratistaInput,
) -> Result<ContratistaResponse, String> {
    ContratistaService::crear(&pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratista_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ContratistaResponse, String> {
    ContratistaService::obtener_por_id(&pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratista_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ContratistaResponse, String> {
    ContratistaService::obtener_por_cedula(&pool, cedula)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_contratistas(
    pool: State<'_, SqlitePool>,
) -> Result<ContratistaListResponse, String> {
    ContratistaService::listar_todos(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_contratistas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ContratistaResponse>, String> {
    ContratistaService::listar_activos(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateContratistaInput,
) -> Result<ContratistaResponse, String> {
    ContratistaService::actualizar(&pool, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cambiar_estado_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
    input: CambiarEstadoInput,
) -> Result<ContratistaResponse, String> {
    ContratistaService::cambiar_estado(&pool, id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_contratista(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    ContratistaService::eliminar(&pool, id)
        .await
        .map_err(|e| e.to_string())
}