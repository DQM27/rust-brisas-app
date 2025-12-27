use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::errors::CitaError;
use crate::models::visitante::{CreateVisitanteInput, VisitanteResponse};
use log::debug;
use tauri::command;

#[command]
pub async fn create_cita(
    cita: CreateCitaInput,
    _visitante: Option<CreateVisitanteInput>,
) -> Result<Cita, CitaError> {
    debug!("Creating Cita: {:?}", cita);
    Err(CitaError::Database("No implementado para SurrealDB aún".to_string()))
}

#[command]
pub async fn get_citas_hoy() -> Result<Vec<CitaPopulated>, CitaError> {
    Ok(vec![])
}

#[command]
pub async fn get_citas_pendientes() -> Result<Vec<CitaPopulated>, CitaError> {
    Ok(vec![])
}

#[command]
pub async fn procesar_ingreso_cita(
    _cita_id: String,
    _gafete: String,
    _usuario_id: String,
) -> Result<String, CitaError> {
    Err(CitaError::Database("No implementado para SurrealDB aún".to_string()))
}

#[command]
pub async fn get_visitante_by_cedula(
    cedula: String,
) -> Result<Option<VisitanteResponse>, CitaError> {
    crate::services::visitante_service::get_visitante_by_cedula(&cedula)
        .await
        .map_err(|e| CitaError::Database(e.to_string()))
}

#[command]
pub async fn update_cita(
    _id: String,
    _fecha_cita: String,
    _anfitrion: String,
    _area_visitada: String,
    _motivo: Option<String>,
) -> Result<(), CitaError> {
    Err(CitaError::Database("No implementado para SurrealDB aún".to_string()))
}

#[command]
pub async fn cancelar_cita(_id: String) -> Result<(), CitaError> {
    Err(CitaError::Database("No implementado para SurrealDB aún".to_string()))
}
