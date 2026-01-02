//! # Comandos: Gestión de Vehículos (Tauri Bridge)
//!
//! Este módulo expone las funcionalidades de control vehicular al frontend,
//! regulando el acceso mediante permisos (RBAC) y vinculando activos
//! móviles con sus responsables legales.

/// Puertos de Entrada: Gestión de Activos Móviles y Vehículos (Vehicle Bridge).
///
/// Este módulo controla el registro y trazabilidad de los vehículos que ingresan
/// a la planta, vinculándolos con sus respectivos propietarios (Contratistas,
/// Proveedores o Visitantes) para garantizar la seguridad patrimonial.
use crate::domain::errors::VehiculoError;
use crate::models::vehiculo::{
    CreateVehiculoInput, UpdateVehiculoInput, VehiculoListResponse, VehiculoResponse,
};
use crate::require_perm;
use crate::services::session::SessionState;
use crate::services::vehiculo_service as service;
use tauri::State;

/// Registra una nueva unidad móvil vinculada a un sujeto autorizado.
#[tauri::command]
pub async fn create_vehiculo(
    session: State<'_, SessionState>,
    input: CreateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    require_perm!(session, "vehiculos:create", "Registrando nuevo activo móvil")?;
    service::create_vehiculo(input).await
}

#[tauri::command]
pub async fn get_vehiculo_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<VehiculoResponse, VehiculoError> {
    require_perm!(session, "vehiculos:read")?;
    service::get_vehiculo_by_id(&id).await
}

/// Identificación Rápida: Localiza un registro de vehículo mediante su placa/matrícula.
#[tauri::command]
pub async fn get_vehiculo_by_placa(
    session: State<'_, SessionState>,
    placa: String,
) -> Result<VehiculoResponse, VehiculoError> {
    require_perm!(session, "vehiculos:read")?;
    service::get_vehiculo_by_placa(placa).await
}

/// Auditoría Global: Lista todo el parque automotor registrado en el sistema.
#[tauri::command]
pub async fn get_all_vehiculos(
    session: State<'_, SessionState>,
) -> Result<VehiculoListResponse, VehiculoError> {
    require_perm!(session, "vehiculos:read")?;
    service::get_all_vehiculos().await
}

/// Monitor de Planta: Obtiene los vehículos que se encuentran actualmente en movimiento o parqueados.
#[tauri::command]
pub async fn get_vehiculos_activos(
    session: State<'_, SessionState>,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    require_perm!(session, "vehiculos:read")?;
    service::get_vehiculos_activos().await
}

/// Trazabilidad por Propietario: Filtra vehículos asociados a un ID específico (Persona o Empresa).
#[tauri::command]
pub async fn get_vehiculos_by_propietario(
    session: State<'_, SessionState>,
    propietario_id: String,
) -> Result<Vec<VehiculoResponse>, VehiculoError> {
    require_perm!(session, "vehiculos:read")?;
    service::get_vehiculos_by_propietario(propietario_id).await
}

/// Actualiza los detalles de un vehículo registrado (Color, Marca o Estado).
#[tauri::command]
pub async fn update_vehiculo(
    session: State<'_, SessionState>,
    id: String,
    input: UpdateVehiculoInput,
) -> Result<VehiculoResponse, VehiculoError> {
    require_perm!(session, "vehiculos:update", format!("Actualizando datos del vehículo {}", id))?;
    service::update_vehiculo(id, input).await
}

/// Baja definitiva del vehículo del sistema de control.
#[tauri::command]
pub async fn delete_vehiculo(
    session: State<'_, SessionState>,
    id: String,
) -> Result<(), VehiculoError> {
    require_perm!(session, "vehiculos:delete", format!("Dando de baja al vehículo {}", id))?;
    service::delete_vehiculo(id).await
}
