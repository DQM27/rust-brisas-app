use crate::domain::ingreso_visita::{
    CreateIngresoVisitaFullInput, IngresoVisita, IngresoVisitaPopulated,
};
use crate::services::ingreso_visita_service::IngresoVisitaService;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_visita_v2(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoVisitaFullInput,
) -> Result<IngresoVisita, String> {
    let service = IngresoVisitaService::new(pool.inner().clone());
    service.registrar_ingreso_full(input).await
}

#[command]
pub async fn get_ingresos_visitas_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoVisitaPopulated>, String> {
    let service = IngresoVisitaService::new(pool.inner().clone());
    service.get_activos().await
}

#[command]
pub async fn registrar_salida_visita(
    pool: State<'_, SqlitePool>,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
) -> Result<(), String> {
    let service = IngresoVisitaService::new(pool.inner().clone());
    service
        .registrar_salida(id, usuario_id, observaciones)
        .await
}
