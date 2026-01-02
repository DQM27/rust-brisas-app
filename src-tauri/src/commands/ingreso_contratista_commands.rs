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
//! Todos los comandos requieren sesión activa.

use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoConEstadoResponse, IngresoResponse,
    RegistrarSalidaInput, ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;
use crate::services::session::SessionState;
use tauri::{command, State};

// Macro para verificar sesión activa
macro_rules! require_session {
    ($session:expr) => {{
        let _user = $session.get_user().ok_or_else(|| {
            IngresoContratistaError::Validation("Sesión no válida o expirada".to_string())
        })?;
    }};
}

// --------------------------------------------------------------------------
// PROTOCOLOS DE ENTRADA
// --------------------------------------------------------------------------

/// [Comando Tauri] Pre-chequeo de identidad y seguridad antes del ingreso.
///
/// ## Validaciones
/// - Estado del contratista (activo)
/// - Lista negra
/// - Vigencia PRAIND
/// - Ingreso activo previo
///
/// ## Parámetros
/// * `contratista_id` - ID del contratista a validar
///
/// ## Retorno
/// Respuesta de validación con estado de autorización
#[command]
pub async fn validate_ingreso_contratista(
    session: State<'_, SessionState>,
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    require_session!(session);
    service::validar_ingreso_contratista(contratista_id).await
}

/// [Comando Tauri] Registra la entrada física de un contratista.
///
/// ## Parámetros
/// * `input` - Datos del ingreso (gafete, vehículo, etc.)
/// * `usuario_id` - ID del guardia que registra
///
/// ## Retorno
/// Datos del ingreso creado con timestamp
#[command]
pub async fn create_ingreso_contratista(
    session: State<'_, SessionState>,
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    require_session!(session);
    service::crear_ingreso_contratista(input, usuario_id).await
}

// --------------------------------------------------------------------------
// PROTOCOLOS DE SALIDA
// --------------------------------------------------------------------------

/// [Comando Tauri] Validación previa a la salida (estado del gafete).
///
/// ## Parámetros
/// * `ingreso_id` - ID del ingreso activo
/// * `gafete_devuelto` - Número de gafete devuelto (opcional)
///
/// ## Retorno
/// Resultado de validación con errores/advertencias
#[command]
pub async fn validate_exit_contratista(
    session: State<'_, SessionState>,
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<ResultadoValidacionSalida, IngresoContratistaError> {
    require_session!(session);
    service::validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(|e| IngresoContratistaError::Validation(e))
}

/// [Comando Tauri] Finaliza el registro de permanencia y libera recursos.
///
/// ## Parámetros
/// * `input` - Datos de la salida (observaciones)
/// * `usuario_id` - ID del guardia que registra
///
/// ## Retorno
/// Ingreso actualizado con hora de salida
#[command]
pub async fn register_exit_contratista(
    session: State<'_, SessionState>,
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    require_session!(session);
    service::registrar_salida(input, usuario_id).await
}

// --------------------------------------------------------------------------
// MONITOREO DE PLANTA
// --------------------------------------------------------------------------

/// [Comando Tauri] Estado de ocupación actual de la planta.
///
/// ## Retorno
/// Lista de ingresos activos con tiempo transcurrido
#[command]
pub async fn get_permanencia_status(
    session: State<'_, SessionState>,
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    require_session!(session);
    service::get_ingresos_abiertos_con_alertas().await
}

/// [Comando Tauri] Consulta alertas por tiempos de permanencia excedidos.
///
/// ## Retorno
/// Lista de alertas para contratistas que exceden el límite (>14h)
#[command]
pub async fn check_time_alerts(
    session: State<'_, SessionState>,
) -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    require_session!(session);
    service::verificar_tiempos_excedidos().await
}
