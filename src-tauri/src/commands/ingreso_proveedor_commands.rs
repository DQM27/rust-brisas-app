use crate::domain::ingreso_proveedor::{CreateIngresoProveedorInput, IngresoProveedor};
use crate::services::ingreso_proveedor_service::IngresoProveedorService;
use sqlx::SqlitePool;
use tauri::{command, State};

#[command]
pub async fn crear_ingreso_proveedor_v2(
    pool: State<'_, SqlitePool>,
    input: CreateIngresoProveedorInput,
) -> Result<IngresoProveedor, String> {
    let service = IngresoProveedorService::new(pool.inner().clone());
    service.registrar_ingreso(input).await
}

#[command]
pub async fn get_ingresos_proveedores_activos(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<IngresoProveedor>, String> {
    let service = IngresoProveedorService::new(pool.inner().clone());
    service.get_activos().await
}

#[command]
pub async fn registrar_salida_proveedor(
    pool: State<'_, SqlitePool>,
    id: String,
    usuario_id: String,
    observaciones: Option<String>,
) -> Result<(), String> {
    let service = IngresoProveedorService::new(pool.inner().clone());
    service
        .registrar_salida(id, usuario_id, observaciones)
        .await
}
