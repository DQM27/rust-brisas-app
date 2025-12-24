use crate::db::DbPool;
use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::errors::CitaError;
use crate::models::visitante::{CreateVisitanteInput, Visitante};
use crate::services::cita_service::CitaService;
use log::debug;
use tauri::{command, State};

#[command]
pub async fn create_cita(
    pool_state: State<'_, DbPool>,
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
) -> Result<Cita, CitaError> {
    debug!("Creating Cita: {:?}", cita);
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.agendar_cita(cita, visitante).await
}

#[command]
pub async fn get_citas_hoy(pool_state: State<'_, DbPool>) -> Result<Vec<CitaPopulated>, CitaError> {
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.get_citas_hoy().await
}

#[command]
pub async fn get_citas_pendientes(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<CitaPopulated>, CitaError> {
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.get_citas_pendientes().await
}

#[command]
pub async fn procesar_ingreso_cita(
    pool_state: State<'_, DbPool>,
    cita_id: String,
    gafete: String,
    usuario_id: String,
) -> Result<String, CitaError> {
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.procesar_ingreso_cita(cita_id, gafete, usuario_id).await
}

#[command]
pub async fn get_visitante_by_cedula(
    pool_state: State<'_, DbPool>,
    cedula: String,
) -> Result<Option<Visitante>, CitaError> {
    let pool = pool_state.0.read().await;
    crate::db::visitante_queries::get_visitante_by_cedula(&pool, &cedula)
        .await
        .map_err(CitaError::Database)
}

#[command]
pub async fn update_cita(
    pool_state: State<'_, DbPool>,
    id: String,
    fecha_cita: String,
    anfitrion: String,
    area_visitada: String,
    motivo: Option<String>,
) -> Result<(), CitaError> {
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.update_cita(id, fecha_cita, anfitrion, area_visitada, motivo).await
}

#[command]
pub async fn cancelar_cita(pool_state: State<'_, DbPool>, id: String) -> Result<(), CitaError> {
    let pool = pool_state.0.read().await;
    let service = CitaService::new(pool.clone());
    service.cancelar_cita(id).await
}
