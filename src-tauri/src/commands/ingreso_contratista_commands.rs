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
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput,
    ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::repositories::{
    contratista::{SurrealContratistaRepository, SurrealSecurityRepository},
    gafete::SurrealGafeteRepository,
    ingreso_contratista::SurrealIngresoContratistaRepository,
};
use crate::services::ingreso_contratista_service::IngresoContratistaService;
use crate::services::session::SessionState;

use tauri::{command, AppHandle, Emitter, State};

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
    require_perm!(session, "ingresos:read")?;
    create_service().validar_ingreso_contratista(contratista_id).await
}

/// [Comando Tauri] Registra la entrada física de un contratista.
/// El `usuario_id` se extrae de la sesión para evitar suplantación.
#[command]
pub async fn create_ingreso_contratista(
    app: AppHandle,
    session: State<'_, SessionState>,
    input: CreateIngresoContratistaInput,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let user = require_session!(session);
    require_perm!(session, "ingresos:create")?;
    let result = create_service().crear_ingreso_contratista(input, user.id.clone()).await?;

    // Emit event to refresh gafete grid
    let _ = app.emit("gafetes:refresh", ());

    Ok(result)
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
    require_perm!(session, "ingresos:read")?;

    create_service()
        .validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(IngresoContratistaError::Validation)
}

/// [Comando Tauri] Finaliza el registro de permanencia y libera recursos.
/// El `usuario_id` se extrae de la sesión para evitar suplantación.
#[command]
pub async fn register_exit_contratista(
    app: AppHandle,
    session: State<'_, SessionState>,
    input: RegistrarSalidaInput,
) -> Result<IngresoResponse, IngresoContratistaError> {
    let user = require_session!(session);
    require_perm!(session, "ingresos:update")?;
    let result = create_service().registrar_salida(input, user.id.clone()).await?;

    // Emit event to refresh gafete grid
    let _ = app.emit("gafetes:refresh", ());

    Ok(result)
}

// --------------------------------------------------------------------------
// MONITOREO DE PLANTA
// --------------------------------------------------------------------------

/// [Comando Tauri] Obtiene los ingresos de contratistas que están actualmente activos.
#[command]
pub async fn get_ingresos_contratistas_activos(
    session: State<'_, SessionState>,
) -> Result<Vec<IngresoResponse>, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "ingresos:read")?;
    create_service().get_activos().await
}

/// [Comando Tauri] Obtiene el historial de salidas de contratistas en un rango de fechas.
#[command]
pub async fn get_ingresos_contratistas_historial(
    session: State<'_, SessionState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "ingresos:read")?;
    create_service().get_salidas_en_rango(&fecha_inicio, &fecha_fin).await
}

/// [Comando Tauri] Consulta alertas por tiempos de permanencia excedidos.
#[command]
pub async fn check_time_alerts(
    session: State<'_, SessionState>,
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    require_session!(session);
    require_perm!(session, "ingresos:read")?;
    create_service().verificar_tiempos_excedidos().await
}
