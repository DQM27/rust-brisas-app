// ==========================================
// src/commands/ingreso_visita_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::services::ingreso_visita_service;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_visita_v2(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, String> {
    ingreso_visita_service::registrar_ingreso_full(&pool, input).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_ingresos_visitas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoVisitaPopulated>, String> {
    ingreso_visita_service::get_activos(&pool).await.map_err(|e| e.to_string())
}

#[command]
pub async fn registrar_salida_visita(
    pool: State<'_, SqlitePool>,
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<(), String> {
    ingreso_visita_service::registrar_salida(&pool, id, usuario_id, devolvio_gafete, observaciones)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_ingresos_visitas_historial(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoVisitaPopulated>, String> {
    ingreso_visita_service::get_historial(&pool).await.map_err(|e| e.to_string())
}
