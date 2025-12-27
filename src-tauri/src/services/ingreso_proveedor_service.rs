// src/services/ingreso_proveedor_service.rs
use crate::domain::errors::IngresoProveedorError;
use crate::domain::ingreso_proveedor::{CreateIngresoProveedorInput, IngresoProveedor};
use crate::services::gafete_service;
use sqlx::SqlitePool;

pub async fn registrar_ingreso(
    _pool: &SqlitePool,
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, IngresoProveedorError> {
    if let Some(ref g) = input.gafete {
        let disp = gafete_service::is_gafete_disponible(g, "proveedor")
            .await
            .map_err(|e| IngresoProveedorError::Validation(e))?;
        if !disp {
            return Err(IngresoProveedorError::Validation("Gafete no disponible".to_string()));
        }
    }
    Err(IngresoProveedorError::Database(sqlx::Error::Protocol("Stubbed".to_string())))
}

pub async fn registrar_salida(
    _pool: &SqlitePool,
    _id: String,
    _usuario_id: String,
    _observaciones: Option<String>,
    _devolvio_gafete: bool,
) -> Result<(), IngresoProveedorError> {
    Err(IngresoProveedorError::Database(sqlx::Error::Protocol("Stubbed".to_string())))
}

pub async fn get_activos(
    _pool: &SqlitePool,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}
pub async fn get_historial(
    _pool: &SqlitePool,
) -> Result<Vec<IngresoProveedor>, IngresoProveedorError> {
    Ok(vec![])
}
pub async fn search_proveedores(
    _pool: &SqlitePool,
    _q: &str,
) -> Result<Vec<serde_json::Value>, IngresoProveedorError> {
    Ok(vec![])
}
pub async fn validar_ingreso(
    _pool: &SqlitePool,
    _proveedor_id: String,
) -> Result<serde_json::Value, IngresoProveedorError> {
    Ok(serde_json::json!({"puedeIngresar":true}))
}
