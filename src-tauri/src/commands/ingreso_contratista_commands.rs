//! # Commands Tauri: Ingreso Contratista (Control de Acceso)
//!
//! Comandos para gestión de entradas/salidas de contratistas.
//!
//! ## Categorías
//! - **Validación**: `validate_*` - Pre-chequeos sin persistencia
//! - **Registro**: `create_*`, `register_*` - Operaciones de escritura
//! - **Monitoreo**: `get_permanencia_status`, `check_time_alerts`
//!
//! ## Seguridad
//! Todos los comandos requieren sesión activa y permisos específicos.

use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoConEstadoResponse, IngresoResponse,
    RegistrarSalidaInput, ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::repositories::{
    contratista::{SurrealContratistaRepository, SurrealSecurityRepository},
    gafete::SurrealGafeteRepository,
    ingreso_contratista::SurrealIngresoContratistaRepository,
};
use crate::services::ingreso_contratista_service::IngresoContratistaService;
use crate::services::session::SessionState;
use crate::{require_perm, require_session};
use tauri::{command, State};

// --------------------------------------------------------------------------
// HELPERS: Construcción del Servicio
// --------------------------------------------------------------------------

/// Crea una instancia del servicio con las implementaciones concretas de `SurrealDB`.
const fn create_service() -> IngresoContratistaService<
    SurrealIngresoContratistaRepository,
    SurrealGafeteRepository,
    SurrealContratistaRepository,
    SurrealSecurityRepository,
> {
    IngresoContratistaService::new(
        SurrealIngresoContratistaRepository,
        SurrealGafeteRepository,
        SurrealContratistaRepository,
        SurrealSecurityRepository,
    )
}

// --------------------------------------------------------------------------
// PROTOCOLOS DE ENTRADA
// --------------------------------------------------------------------------

/// [Comando Tauri] Pre-chequeo de identidad y seguridad antes del ingreso.
#[command]
pub async fn validate_ingreso_contratista(
    session: State<'_, SessionState>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "IngresoContratista:Read")?;
    create_service().validar_ingreso_contratista(contratista_id).await
}

/// [Comando Tauri] Registra la entrada física de un contratista.
/// El `usuario_id` se extrae de la sesión para evitar suplantación.
#[command]
pub async fn create_ingreso_contratista(
    session: State<'_, SessionState>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let user = require_session!(session);
    require_perm!(session, "IngresoContratista:Create")?;
    create_service().crear_ingreso_contratista(input, user.id.clone()).await
}

// --------------------------------------------------------------------------
// PROTOCOLOS DE SALIDA
// --------------------------------------------------------------------------

/// [Comando Tauri] Validación previa a la salida (estado del gafete).
#[command]
pub async fn validate_exit_contratista(
    session: State<'_, SessionState>,
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<ResultadoValidacionSalida, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "IngresoContratista:Read")?;

    create_service()
        .validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(IngresoContratistaError::Validation)
}

/// [Comando Tauri] Finaliza el registro de permanencia y libera recursos.
/// El `usuario_id` se extrae de la sesión para evitar suplantación.
#[command]
pub async fn register_exit_contratista(
    session: State<'_, SessionState>,
    input: RegistrarSalidaInput,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let user = require_session!(session);
    require_perm!(session, "IngresoContratista:Update")?;
    create_service().registrar_salida(input, user.id.clone()).await
}

// --------------------------------------------------------------------------
// MONITOREO DE PLANTA
// --------------------------------------------------------------------------

/// [Comando Tauri] Estado de ocupación actual de la planta.
#[command]
pub async fn get_permanencia_status(
    session: State<'_, SessionState>,
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "IngresoContratista:Read")?;
    create_service().get_ingresos_abiertos_con_alertas().await
}

/// [Comando Tauri] Consulta alertas por tiempos de permanencia excedidos.
#[command]
pub async fn check_time_alerts(
    session: State<'_, SessionState>,
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "IngresoContratista:Read")?;
    create_service().verificar_tiempos_excedidos().await
}
