// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse, StatsGafetes,
    StatsPorTipo, UpdateGafeteInput, UpdateGafeteStatusInput,
};
use crate::services::gafete_service;
use crate::services::session::SessionState;
use tauri::{command, State};

#[command]
pub async fn create_gafete(
    session: State<'_, SessionState>,
    input: CreateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    require_perm!(session, "gafetes:create", "Creando gafete")?;
    gafete_service::create_gafete(input).await.map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn create_gafete_range(
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    gafete_service::create_gafete_range(input)
        .await
        .map(|_| vec![])
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn get_gafete(id: String) -> Result<GafeteResponse, GafeteError> {
    gafete_service::get_gafete_by_id(&id)
        .await
        .map_err(|e| GafeteError::Validation(e))?
        .ok_or(GafeteError::NotFound)
}

#[command]
pub async fn get_all_gafetes(
    session: State<'_, SessionState>,
) -> Result<GafeteListResponse, GafeteError> {
    require_perm!(session, "gafetes:read")?;
    let list = gafete_service::get_all_gafetes().await.map_err(|e| GafeteError::Validation(e))?;
    let responses: Vec<GafeteResponse> = list.into_iter().map(GafeteResponse::from).collect();
    let total = responses.len();

    Ok(GafeteListResponse {
        gafetes: responses,
        total,
        stats: StatsGafetes {
            total,
            disponibles: 0,
            en_uso: 0,
            danados: 0,
            extraviados: 0,
            por_tipo: StatsPorTipo { contratistas: 0, proveedores: 0, visitas: 0, otros: 0 },
        },
    })
}

#[command]
pub async fn get_gafetes_disponibles(tipo: String) -> Result<Vec<GafeteResponse>, GafeteError> {
    let list = gafete_service::get_gafetes_disponibles(&tipo)
        .await
        .map_err(|e| GafeteError::Validation(e))?;
    Ok(list.into_iter().map(GafeteResponse::from).collect())
}

#[command]
pub async fn is_gafete_disponible(numero: String, tipo: String) -> Result<bool, GafeteError> {
    gafete_service::is_gafete_disponible(&numero, &tipo)
        .await
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn update_gafete(
    id: String,
    _input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    // Note: service update logic might need to be implemented or adjusted
    gafete_service::get_gafete_by_id(&id)
        .await
        .map_err(|e| GafeteError::Validation(e))?
        .ok_or(GafeteError::NotFound)
}

#[command]
pub async fn update_gafete_status(
    id: String,
    input: UpdateGafeteStatusInput,
    _usuario_id: Option<String>,
    _motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    gafete_service::update_gafete_status(&id, input.estado)
        .await
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn delete_gafete(id: String, _usuario_id: Option<String>) -> Result<(), GafeteError> {
    gafete_service::delete_gafete(&id).await.map_err(|e| GafeteError::Validation(e))
}
