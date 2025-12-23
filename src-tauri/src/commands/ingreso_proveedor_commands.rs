// ==========================================
// src/commands/ingreso_proveedor_commands.rs
// ==========================================
// Capa de API: Tauri command handlers

use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::{
    CreateIngresoProveedorInput, IngresoProveedor, ProveedorSnapshot,
    ValidacionIngresoProveedorResponse,
};
use crate::services::ingreso_proveedor_service;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_proveedor_v2(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    ingreso_proveedor_service::registrar_ingreso(&pool, input).await
}

#[command]
pub async fn get_ingresos_proveedores_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    ingreso_proveedor_service::get_activos(&pool).await
}

#[command]
pub async fn get_ingresos_proveedores_historial(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    ingreso_proveedor_service::get_historial(&pool).await
}

#[command]
pub async fn registrar_salida_proveedor(
    pool: State<'_, SqlitePool>,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
    devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
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
    pool: State<'_, SqlitePool>,
    query: String,
) -> Result<Vec<ProveedorSnapshot>, IngresoProveedorError> {
    ingreso_proveedor_service::search_proveedores(&pool, &query).await
}

#[command]
pub async fn validar_ingreso_proveedor(
    pool: State<'_, SqlitePool>,
    proveedor_id: String,
) -> Result<ValidacionIngresoProveedorResponse, IngresoProveedorError> {
    ingreso_proveedor_service::validar_ingreso(&pool, proveedor_id).await
}
