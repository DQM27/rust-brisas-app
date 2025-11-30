// ==========================================
// src/commands/entrada_commands.rs
// ==========================================
// Comandos Tauri para la fase de ENTRADA

use crate::models::ingreso::{CreateIngresoContratistaInput, IngresoResponse, ValidacionIngresoResponse};
use crate::services::entrada_service;
use sqlx::SqlitePool;
use tauri::State;

// ==========================================
// VALIDACIÓN PRE-INGRESO
// ==========================================

/// Valida si un contratista puede ingresar
#[tauri::command]
pub async fn validar_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, String> {
    entrada_service::validar_ingreso_contratista(&pool, contratista_id).await
}

// ==========================================
// CREAR INGRESO
// ==========================================

/// Crea un nuevo registro de ingreso para un contratista
#[tauri::command]
pub async fn crear_ingreso_contratista(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, String> {
    let usuario_id = input.usuario_ingreso_id.clone();
    entrada_service::crear_ingreso_contratista(&pool, input, usuario_id).await
}