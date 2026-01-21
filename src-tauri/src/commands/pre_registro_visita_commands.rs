// ==========================================
// src/commands/pre_registro_visita_commands.rs
// ==========================================

use crate::db::surrealdb_pre_registro_visita_queries as db;
use crate::models::ingreso::{
    CreatePreRegistroInput, PreRegistroEstado, PreRegistroVisitaCreateDTO, PreRegistroVisitaFetched,
};
use crate::services::session::SessionState;
use log::info;
use surrealdb::RecordId;
use tauri::State;

#[tauri::command]
pub async fn create_pre_registro_visita(
    session: State<'_, SessionState>,
    input: CreatePreRegistroInput,
) -> Result<PreRegistroVisitaFetched, String> {
    let user = session.get_user().ok_or("Usuario no autenticado para esta acción".to_string())?;

    let user_id_str = user.id.clone();

    // Create DTO
    let dto = PreRegistroVisitaCreateDTO {
        cedula: input.cedula,
        nombre: input.nombre,
        apellido: input.apellido,
        empresa_nombre: input.empresa_nombre,
        fecha_esperada: input.fecha_esperada,
        anfitrion: input.anfitrion,
        area_visitada: input.area_visitada,
        motivo: input.motivo,
        modo_ingreso: input.modo_ingreso,
        observaciones: input.observaciones,
        estado: PreRegistroEstado::Pendiente.to_string(), // Default state
        visitante: None, // Logic to link existing visitor could be added here or in UI
        registrado_por: RecordId::from(("user", user_id_str)),
    };

    let result = db::create(dto).await.map_err(|e| e.to_string())?;
    info!("Pre-registro creado: {}", result.id);
    Ok(result)
}

#[tauri::command]
pub async fn get_pre_registros_pendientes() -> Result<Vec<PreRegistroVisitaFetched>, String> {
    db::find_pending().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_pre_registro_by_cedula(
    cedula: String,
) -> Result<Option<PreRegistroVisitaFetched>, String> {
    db::find_by_cedula_pending(&cedula).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_pre_registro_visita(id: String) -> Result<PreRegistroVisitaFetched, String> {
    let rid = RecordId::from(("pre_registro_visita", id));
    db::update_status(&rid, PreRegistroEstado::Cancelado).await.map_err(|e| e.to_string())
}
