// ==========================================
// src/commands/vehiculo_commands.rs
// ==========================================

use crate::models::vehiculo::{
    CreateVehiculoInput, UpdateVehiculoInput, VehiculoListResponse, VehiculoResponse,
};
use crate::services::vehiculo_service;
use sqlx::SqlitePool;
use tauri::State;

/// Crea un nuevo vehículo para un contratista
#[tauri::command]
pub async fn create_vehiculo(
    pool: State<'_, SqlitePool>,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    vehiculo_service::create_vehiculo(&pool, input).await.map_err(|e| e.to_string())
}

/// Obtiene un vehículo por ID
#[tauri::command]
pub async fn get_vehiculo_by_id(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<VehiculoResponse, String> {
    vehiculo_service::get_vehiculo_by_id(&pool, &id).await.map_err(|e| e.to_string())
}

/// Obtiene un vehículo por placa
#[tauri::command]
pub async fn get_vehiculo_by_placa(
    pool: State<'_, SqlitePool>,
    placa: String,
) -> Result<VehiculoResponse, String> {
    vehiculo_service::get_vehiculo_by_placa(&pool, placa).await.map_err(|e| e.to_string())
}

/// Obtiene todos los vehículos del sistema
#[tauri::command]
pub async fn get_all_vehiculos(
    pool: State<'_, SqlitePool>,
) -> Result<VehiculoListResponse, String> {
    vehiculo_service::get_all_vehiculos(&pool).await.map_err(|e| e.to_string())
}

/// Obtiene todos los vehículos activos
#[tauri::command]
pub async fn get_vehiculos_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<VehiculoResponse>, String> {
    vehiculo_service::get_vehiculos_activos(&pool).await.map_err(|e| e.to_string())
}

/// Obtiene todos los vehículos de un contratista específico
#[tauri::command]
pub async fn get_vehiculos_by_contratista(
    pool: State<'_, SqlitePool>,
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, String> {
    vehiculo_service::get_vehiculos_by_contratista(&pool, contratista_id)
        .await
        .map_err(|e| e.to_string())
}

/// Actualiza información de un vehículo
#[tauri::command]
pub async fn update_vehiculo(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, String> {
    vehiculo_service::update_vehiculo(&pool, id, input).await.map_err(|e| e.to_string())
}

/// Elimina un vehículo (eliminación física)
#[tauri::command]
pub async fn delete_vehiculo(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    vehiculo_service::delete_vehiculo(&pool, id).await.map_err(|e| e.to_string())
}
