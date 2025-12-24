// ==========================================
// src/commands/ingreso_proveedor_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::db::DbPool;
use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput, IngresoProveedor, ProveedorSnapshot,
    ValidacionIngresoProveedorResponse,
};
use crate::services::ingreso_proveedor_service;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_proveedor_v2(
    pool_state: State<'_, DbPool>,
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::registrar_ingreso(&pool, input).await
}

#[command]
pub async fn get_ingresos_proveedores_activos(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::get_activos(&pool).await
}

#[command]
pub async fn get_ingresos_proveedores_historial(
    pool_state: State<'_, DbPool>,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::get_historial(&pool).await
}

#[command]
pub async fn registrar_salida_proveedor(
    pool_state: State<'_, DbPool>,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::registrar_salida(
        &pool,
        id,
        usuario_id,
        observaciones,
        devolvio_gafete,
    )
    .await
}

#[command]
pub async fn search_proveedores(
    pool_state: State<'_, DbPool>,
    query: String,
) -> Result<Vec<ProveedorSnapshot>, IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::search_proveedores(&pool, &query).await
}

#[command]
pub async fn validar_ingreso_proveedor(
    pool_state: State<'_, DbPool>,
    proveedor_id: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    let pool = pool_state.0.read().await;
    ingreso_proveedor_service::validar_ingreso(&pool, proveedor_id).await
}
