// ==========================================
// src/commands/vehiculo_commands.rs
// ==========================================

use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{
    CreateVehiculoInput, UpdateVehiculoInput, VehiculoListResponse, VehiculoResponse,
};
use crate::services::vehiculo_service as service;

/// Crea un nuevo vehículo para un contratista
#[tauri::command]
pub async fn create_vehiculo(
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    service::create_vehiculo(input).await
}

/// Obtiene un vehículo por ID
#[tauri::command]
pub async fn get_vehiculo_by_id(id: String) -> Result<VehiculoResponse, VehiculoError> {
    service::get_vehiculo_by_id(&id).await
}

/// Obtiene un vehículo por placa
#[tauri::command]
pub async fn get_vehiculo_by_placa(placa: String) -> Result<VehiculoResponse, VehiculoError> {
    service::get_vehiculo_by_placa(placa).await
}

/// Obtiene todos los vehículos del sistema
#[tauri::command]
pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    service::get_all_vehiculos().await
}

/// Obtiene todos los vehículos activos
#[tauri::command]
pub async fn get_vehiculos_activos() -> Result<Vec<VehiculoResponse>, VehiculoError> {
    service::get_vehiculos_activos().await
}

/// Obtiene todos los vehículos de un propietario específico (contratista, proveedor, visitante)
#[tauri::command]
pub async fn get_vehiculos_by_propietario(
    propietario_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    service::get_vehiculos_by_propietario(propietario_id).await
}

/// Actualiza información de un vehículo
#[tauri::command]
pub async fn update_vehiculo(
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    service::update_vehiculo(id, input).await
}

/// Elimina un vehículo (eliminación física)
#[tauri::command]
pub async fn delete_vehiculo(id: String) -> Result<(), VehiculoError> {
    service::delete_vehiculo(id).await
}
