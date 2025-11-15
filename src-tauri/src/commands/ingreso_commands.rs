// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================

use sqlx::SqlitePool;
use tauri::State;
use crate::models::ingreso::*;
use crate::services::ingreso_service::IngresoService;

#[tauri::command]
pub async fn validar_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<ValidacionIngresoResponse, String> {
    IngresoService::validar_ingreso_contratista(&pool, cedula)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, String> {
    IngresoService::crear_ingreso_contratista(&pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_ingreso_temporal(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoTemporalInput,
) -> Result<IngresoResponse, String> {
    IngresoService::crear_ingreso_temporal(&pool, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ingreso_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<IngresoResponse, String> {
    IngresoService::obtener_por_id(&pool, id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ingreso_by_gafete(
    pool: State<'_, SqlitePool>,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    IngresoService::obtener_por_gafete(&pool, gafete_numero)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_ingresos(
    pool: State<'_, SqlitePool>,
) -> Result<IngresoListResponse, String> {
    IngresoService::listar_todos(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ingresos_abiertos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoResponse>, String> {
    IngresoService::listar_abiertos(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn registrar_salida(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    input: RegistrarSalidaInput,
) -> Result<IngresoResponse, String> {
    IngresoService::registrar_salida(&pool, ingreso_id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn registrar_salida_con_gafete_perdido(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    input: RegistrarSalidaConGafetePerdidoInput,
) -> Result<IngresoResponse, String> {
    IngresoService::registrar_salida_con_gafete_perdido(&pool, ingreso_id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cerrar_ingreso_anterior(
    pool: State<'_, SqlitePool>,
    cedula: String,
    usuario_salida_id: String,
) -> Result<(), String> {
    IngresoService::cerrar_ingreso_anterior(&pool, cedula, usuario_salida_id)
        .await
        .map_err(|e| e.to_string())
}