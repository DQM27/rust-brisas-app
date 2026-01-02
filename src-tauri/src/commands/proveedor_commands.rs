//! # Comandos: Gestión de Proveedores (Tauri Bridge)
//!
//! Este módulo expone las funciones del sistema al frontend de Tauri,
//! integrando el control de acceso basado en roles (RBAC) y la
//! estandarización de errores de dominio.

use crate::domain::errors::ProveedorError;
use crate::models::proveedor::{CreateProveedorInput, ProveedorResponse, UpdateProveedorInput};
use crate::require_perm;
use crate::services::proveedor_service;
use crate::services::search_service::SearchService;
use crate::services::session::SessionState;
use std::sync::Arc;
use tauri::{command, State};

/// Registra un nuevo proveedor en el sistema (Protegido por RBAC).
#[command]
pub async fn create_proveedor(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    input: CreateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    require_perm!(session, "proveedores:create", "Registrando nuevo socio comercial (Proveedor)")?;
    proveedor_service::create_proveedor(&search_service, input).await
}

/// Motor de Búsqueda: Localiza proveedores en el catálogo general para agilizar gestiones.
#[command]
pub async fn search_proveedores_catalog(
    session: State<'_, SessionState>,
    query: String,
) -> Result<Vec<ProveedorResponse>, ProveedorError> {
    require_perm!(session, "proveedores:read")?;
    proveedor_service::search_proveedores(&query).await
}

/// Motores de búsqueda: Recupera un perfil de proveedor mediante su cédula o RUC.
#[command]
pub async fn get_proveedor_by_cedula(
    session: State<'_, SessionState>,
    cedula: String,
) -> Result<Option<ProveedorResponse>, ProveedorError> {
    require_perm!(session, "proveedores:read")?;
    proveedor_service::get_proveedor_by_cedula(&cedula).await
}

/// Gestión de Estatus: Permite habilitar o deshabilitar comercialmente a un proveedor.
#[command]
pub async fn change_proveedor_status(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    new_status: String,
) -> Result<ProveedorResponse, ProveedorError> {
    require_perm!(
        session,
        "proveedores:update",
        format!("Actualizando estatus comercial del proveedor ID: {}", id)
    )?;
    proveedor_service::change_status(&search_service, &id, &new_status).await
}

/// Actualiza los datos de contacto o fiscales de un proveedor existente.
#[command]
pub async fn update_proveedor(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
    input: UpdateProveedorInput,
) -> Result<ProveedorResponse, ProveedorError> {
    require_perm!(
        session,
        "proveedores:update",
        format!("Editando información de proveedor {}", id)
    )?;
    proveedor_service::update_proveedor(&search_service, id, input).await
}

#[command]
pub async fn get_proveedor_by_id(
    session: State<'_, SessionState>,
    id: String,
) -> Result<ProveedorResponse, ProveedorError> {
    require_perm!(session, "proveedores:read")?;
    proveedor_service::get_proveedor_by_id(&id).await
}

/// Baja Administrativa: Archiva el perfil del proveedor del catálogo operativo.
#[command]
pub async fn delete_proveedor(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<(), ProveedorError> {
    require_perm!(session, "proveedores:delete", format!("Archivando perfil de proveedor {}", id))?;
    proveedor_service::delete_proveedor(&search_service, &id).await
}

/// Restablecimiento: Recupera un proveedor anteriormente archivado.
#[command]
pub async fn restore_proveedor(
    session: State<'_, SessionState>,
    search_service: State<'_, Arc<SearchService>>,
    id: String,
) -> Result<ProveedorResponse, ProveedorError> {
    require_perm!(
        session,
        "proveedores:delete",
        format!("Restaurando perfil de proveedor {}", id)
    )?;
    proveedor_service::restore_proveedor(&search_service, &id).await
}

/// Consulta histórica de proveedores que han sido dados de baja administrativa.
#[command]
pub async fn get_archived_proveedores(
    session: State<'_, SessionState>,
) -> Result<Vec<ProveedorResponse>, ProveedorError> {
    require_perm!(session, "proveedores:read", "Consultando catálogo de proveedores archivados")?;
    proveedor_service::get_archived_proveedores().await
}
