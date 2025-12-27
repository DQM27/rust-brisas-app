// ==========================================
// src/commands/vehiculo_commands.rs
// ==========================================

use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{
    CreateVehiculoInput, UpdateVehiculoInput, VehiculoListResponse, VehiculoResponse,
};

/// Crea un nuevo vehículo para un contratista
#[tauri::command]
pub async fn create_vehiculo(
    _input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene un vehículo por ID
#[tauri::command]
pub async fn get_vehiculo_by_id(_id: String) -> Result<VehiculoResponse, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene un vehículo por placa
#[tauri::command]
pub async fn get_vehiculo_by_placa(_placa: String) -> Result<VehiculoResponse, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene todos los vehículos del sistema
#[tauri::command]
pub async fn get_all_vehiculos() -> Result<VehiculoListResponse, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene todos los vehículos activos
#[tauri::command]
pub async fn get_vehiculos_activos() -> Result<Vec<VehiculoResponse>, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Obtiene todos los vehículos de un contratista específico
#[tauri::command]
pub async fn get_vehiculos_by_contratista(
    _contratista_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Actualiza información de un vehículo
#[tauri::command]
pub async fn update_vehiculo(
    _id: String,
    _input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}

/// Elimina un vehículo (eliminación física)
#[tauri::command]
pub async fn delete_vehiculo(_id: String) -> Result<(), VehiculoError> {
    Err(VehiculoError::Database(sqlx::Error::Protocol(
        "No implementado para SurrealDB aún".to_string(),
    )))
}
