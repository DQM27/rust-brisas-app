// ==========================================
// src/commands/ingreso_visita_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::errors::IngresoVisitaError;
use crate::domain::ingreso_visita::{CreateIngresoVisitaFullInput, IngresoVisitaPopulated};
use crate::models::ingreso::IngresoResponse;
use crate::services::ingreso_visita_service as service;
use crate::services::session::SessionState;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_visita_v2(
    session: State<'_, SessionState>,
    input: CreateIngresoVisitaFullInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoVisitaError> {
    require_perm!(session, "ingresos:create", "Registrando ingreso de visita")?;
    service::registrar_ingreso_full(input, usuario_id).await
}

#[command]
pub async fn validar_ingreso_visita(
    session: State<'_, SessionState>,
    visitante_id: String,
) -> Result<crate::domain::ingreso_visita::ValidacionIngresoVisitaResponse, IngresoVisitaError> {
    require_perm!(session, "ingresos:read", "Validando ingreso")?;
    service::validar_ingreso(&visitante_id).await
}

#[command]
pub async fn get_ingresos_visitas_activos(
    session: State<'_, SessionState>,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    require_perm!(session, "ingresos:read")?;
    service::get_activos().await
}

#[command]
pub async fn registrar_salida_visita(
    session: State<'_, SessionState>,
    id: String,
    usuario_id: String,
    devolvio_gafete: bool,
    observaciones: Option<String>,
) -> Result<IngresoResponse, IngresoVisitaError> {
    require_perm!(session, "ingresos:update", format!("Registrando salida de visita {}", id))?;
    service::registrar_salida(id, usuario_id, devolvio_gafete, observaciones).await
}

#[command]
pub async fn get_ingresos_visitas_historial(
    session: State<'_, SessionState>,
) -> Result<Vec<IngresoVisitaPopulated>, IngresoVisitaError> {
    require_perm!(session, "ingresos:read")?;
    service::get_historial().await
}
