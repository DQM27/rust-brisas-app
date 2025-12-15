use crate::domain::cita::{Cita, CitaPopulated, CreateCitaInput};
use crate::domain::visitante::CreateVisitanteInput;
use crate::services::cita_service::CitaService;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn create_cita(
    pool: State<'_, SqlitePool>,
    cita: CreateCitaInput,
    visitante: Option<CreateVisitanteInput>,
) -> Result<Cita, String> {
    println!("📅 Creating Cita: {:?}", cita);
    let service = CitaService::new(pool.inner().clone());
    service.agendar_cita(cita, visitante).await
}

#[command]
pub async fn get_citas_hoy(pool: State<'_, SqlitePool>) -> Result<Vec<CitaPopulated>, String> {
    let service = CitaService::new(pool.inner().clone());
    service.get_citas_hoy().await
}

#[command]
pub async fn get_citas_pendientes(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<CitaPopulated>, String> {
    let service = CitaService::new(pool.inner().clone());
    service.get_citas_pendientes().await
}

#[command]
pub async fn procesar_ingreso_cita(
    pool: State<'_, SqlitePool>,
    cita_id: String,
    gafete: String,
    usuario_id: String,
) -> Result<String, String> {
    let service = CitaService::new(pool.inner().clone());
    service
        .procesar_ingreso_cita(cita_id, gafete, usuario_id)
        .await
    // .await removed
}

#[command]
pub async fn get_visitante_by_cedula(
    pool: State<'_, SqlitePool>,
    cedula: String,
) -> Result<Option<crate::domain::visitante::Visitante>, String> {
    crate::db::visitante_queries::get_visitante_by_cedula(pool.inner(), &cedula)
        .await
        .map_err(|e| e.to_string())
}
