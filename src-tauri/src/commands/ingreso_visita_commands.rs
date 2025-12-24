// ==========================================
// src/commands/ingreso_visita_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::db::DbPool;
use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::services::ingreso_visita_service;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_visita_v2(
    pool_state: State<'_, DbPool>,
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, IngresoVisitaError> {
    let pool = pool_state.0.read().await;
    ingreso_visita_service::registrar_ingreso_full(&pool, input).await
}

#[command]
pub async fn validar_ingreso_visita(
    pool_state: State<'_, DbPool>,
    visitante_id: String,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    let pool = pool_state.0.read().await;
    ingreso_visita_service::validar_ingreso(&pool, &visitante_id).await
}

#[command]
pub async fn get_ingresos_visitas_activos(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    let pool = pool_state.0.read().await;
    ingreso_visita_service::get_activos(&pool).await
}

#[command]
pub async fn registrar_salida_visita(
    pool_state: State<'_, DbPool>,
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<(), IngresoVisitaError> {
    let pool = pool_state.0.read().await;
    ingreso_visita_service::registrar_salida(&pool, id, usuario_id, devolvio_gafete, observaciones)
        .await
}

#[command]
pub async fn get_ingresos_visitas_historial(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    let pool = pool_state.0.read().await;
    ingreso_visita_service::get_historial(&pool).await
}
