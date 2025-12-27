// ==========================================
// src/commands/gafete_commands.rs
// ==========================================

use crate::domain::errors::GafeteError;
use crate::models::gafete::{
    CreateGafeteInput, CreateGafeteRangeInput, GafeteListResponse, GafeteResponse, StatsGafetes,
    StatsPorTipo, UpdateGafeteInput, UpdateGafeteStatusInput,
};
use crate::services::gafete_service;
use tauri::command;

#[command]
pub async fn create_gafete(input: CreateGafeteInput) -> Result<GafeteResponse, GafeteError> {
    gafete_service::create_gafete(&input.numero, &input.tipo)
        .await
        .map(|g| g.into())
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn create_gafete_range(
    input: CreateGafeteRangeInput,
) -> Result<Vec<String>, GafeteError> {
    gafete_service::create_gafete_range(
        &input.prefix.unwrap_or_default(),
        input.start as i32,
        input.end as i32,
        &input.tipo,
    )
    .await
    .map(|_| vec![])
    .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn get_gafete(numero: String, tipo: String) -> Result<GafeteResponse, GafeteError> {
    gafete_service::get_gafete(&numero, &tipo)
        .await
        .map_err(|e| GafeteError::Validation(e))?
        .ok_or(GafeteError::NotFound)
        .map(|g| g.into())
}

#[command]
pub async fn get_all_gafetes() -> Result<GafeteListResponse, GafeteError> {
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
    numero: String,
    tipo: String,
    input: UpdateGafeteInput,
) -> Result<GafeteResponse, GafeteError> {
    // UpdateGafeteInput solo tiene .tipo como campo
    gafete_service::update_gafete(&numero, &tipo, &input.tipo.unwrap_or_default())
        .await
        .map(|g| g.into())
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn update_gafete_status(
    numero: String,
    tipo: String,
    input: UpdateGafeteStatusInput,
    _usuario_id: Option<String>,
    _motivo: Option<String>,
) -> Result<GafeteResponse, GafeteError> {
    // input.estado es GafeteEstado, convertir a String
    gafete_service::update_gafete_status(&numero, &tipo, input.estado.as_str().to_string())
        .await
        .map(|g| g.into())
        .map_err(|e| GafeteError::Validation(e))
}

#[command]
pub async fn delete_gafete(
    numero: String,
    tipo: String,
    _usuario_id: Option<String>,
) -> Result<(), GafeteError> {
    gafete_service::delete_gafete(&numero, &tipo).await.map_err(|e| GafeteError::Validation(e))
}
