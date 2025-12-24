// ==========================================
// src/commands/vehiculo_commands.rs
// ==========================================

use crate::db::DbPool;
use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{
    CreateVehiculoInput, UpdateVehiculoInput, VehiculoListResponse, VehiculoResponse,
};
use crate::services::vehiculo_service;
use tauri::State;

/// Crea un nuevo vehículo para un contratista
#[tauri::command]
pub async fn create_vehiculo(
    pool_state: State<'_, DbPool>,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::create_vehiculo(&pool, input).await
}

/// Obtiene un vehículo por ID
#[tauri::command]
pub async fn get_vehiculo_by_id(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<VehiculoResponse, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::get_vehiculo_by_id(&pool, &id).await
}

/// Obtiene un vehículo por placa
#[tauri::command]
pub async fn get_vehiculo_by_placa(
    pool_state: State<'_, DbPool>,
    placa: String,
) -> Result<VehiculoResponse, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::get_vehiculo_by_placa(&pool, placa).await
}

/// Obtiene todos los vehículos del sistema
#[tauri::command]
pub async fn get_all_vehiculos(
    pool_state: State<'_, DbPool>,
) -> Result<VehiculoListResponse, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::get_all_vehiculos(&pool).await
}

/// Obtiene todos los vehículos activos
#[tauri::command]
pub async fn get_vehiculos_activos(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::get_vehiculos_activos(&pool).await
}

/// Obtiene todos los vehículos de un contratista específico
#[tauri::command]
pub async fn get_vehiculos_by_contratista(
    pool_state: State<'_, DbPool>,
    contratista_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::get_vehiculos_by_contratista(&pool, contratista_id).await
}

/// Actualiza información de un vehículo
#[tauri::command]
pub async fn update_vehiculo(
    pool_state: State<'_, DbPool>,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::update_vehiculo(&pool, id, input).await
}

/// Elimina un vehículo (eliminación física)
#[tauri::command]
pub async fn delete_vehiculo(
    pool_state: State<'_, DbPool>,
    id: String,
) -> Result<(), VehiculoError> {
    let pool = pool_state.0.read().await;
    vehiculo_service::delete_vehiculo(&pool, id).await
}
