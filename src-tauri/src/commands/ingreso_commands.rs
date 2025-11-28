// ==========================================
// src/commands/ingreso_commands.rs
// ==========================================

use crate::models::ingreso::{
    AlertaGafeteResponse, CreateIngresoContratistaInput, IngresoListResponse, IngresoResponse,
    RegistrarSalidaInput, ResolverAlertaInput, ValidacionIngresoResponse,
};
use crate::services::ingreso_service;
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// VALIDACIÓN Y CONSULTAS
// ==========================================

#[tauri::command]
pub async fn validar_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    ingreso_service::validar_ingreso_contratista(&pool, contratista_id).await
}

#[tauri::command]
pub async fn get_ingreso_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<IngresoResponse, String> {
    ingreso_service::get_ingreso_by_id(&pool, id).await
}

#[tauri::command]
pub async fn get_all_ingresos(pool: State<'_, SqlitePool>) -> Result<IngresoListResponse, String> {
    ingreso_service::get_all_ingresos(&pool).await
}

#[tauri::command]
pub async fn get_ingresos_abiertos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoResponse>, String> {
    ingreso_service::get_ingresos_abiertos(&pool).await
}

#[tauri::command]
pub async fn get_ingreso_by_gafete(
    pool: State<'_, SqlitePool>,
    gafete_numero: String,
) -> Result<IngresoResponse, String> {
    ingreso_service::get_ingreso_by_gafete(&pool, gafete_numero).await
}

// ==========================================
// CREAR INGRESOS
// ==========================================

#[tauri::command]
pub async fn create_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, String> {
    let usuario_id = input.usuario_ingreso_id.clone();
    ingreso_service::create_ingreso_contratista(&pool, input, usuario_id).await
}

// ==========================================
// REGISTRAR SALIDA
// ==========================================

#[tauri::command]
pub async fn registrar_salida(
    pool: State<'_, SqlitePool>,
    ingreso_id: String,
    mut input: RegistrarSalidaInput,
) -> Result<IngresoResponse, String> {
    input.ingreso_id = ingreso_id;
    let usuario_id = input.usuario_salida_id.clone();
    ingreso_service::registrar_salida(&pool, input, usuario_id).await
}

// ==========================================
// GESTIÓN DE ALERTAS DE GAFETES
// ==========================================

#[tauri::command]
pub async fn get_alertas_pendientes_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    ingreso_service::get_alertas_pendientes_by_cedula(&pool, cedula).await
}

#[tauri::command]
pub async fn get_all_alertas_gafetes(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AlertaGafeteResponse>, String> {
    ingreso_service::get_all_alertas_gafetes(&pool).await
}

#[tauri::command]
pub async fn resolver_alerta_gafete(
    pool: State<'_, SqlitePool>,
    input: ResolverAlertaInput,
) -> Result<AlertaGafeteResponse, String> {
    ingreso_service::resolver_alerta_gafete(&pool, input).await
}