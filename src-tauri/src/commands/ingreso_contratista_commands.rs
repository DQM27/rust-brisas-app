/// Puertos de Entrada: Gestión de Admisión de Contratistas (UI Bridge).
///
/// Este módulo expone las funciones de Rust al frontend mediante Tauri.
/// Actúa como orquestador de bajo acoplamiento para control de acceso.
use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    AlertaTiempoExcedido, CreateIngresoContratistaInput, IngresoConEstadoResponse, IngresoResponse,
    RegistrarSalidaInput, ResultadoValidacionSalida, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;
use tauri::command;

// --------------------------------------------------------------------------
// PROTOCOLOS DE ENTRADA
// --------------------------------------------------------------------------

/// [Comando Tauri] Pre-chequeo de identidad y seguridad antes del ingreso.
///
/// # Argumentos
/// * `contratista_id` - ID del contratista a validar.
///
/// # Retorno
/// Respuesta de validación con estado de autorización.
#[command]
pub async fn validate_ingreso_contratista(
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    service::validar_ingreso_contratista(contratista_id).await
}

/// [Comando Tauri] Registra la entrada física de un contratista.
///
/// # Argumentos
/// * `input` - Datos del ingreso (gafete, vehículo, etc.).
/// * `usuario_id` - ID del guardia que registra.
///
/// # Retorno
/// Datos del ingreso creado.
#[command]
pub async fn create_ingreso_contratista(
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::crear_ingreso_contratista(input, usuario_id).await
}

// --------------------------------------------------------------------------
// PROTOCOLOS DE SALIDA
// --------------------------------------------------------------------------

/// [Comando Tauri] Validación previa a la salida (estado del gafete).
///
/// # Argumentos
/// * `ingreso_id` - ID del ingreso activo.
/// * `gafete_devuelto` - Número de gafete devuelto (opcional).
///
/// # Retorno
/// Resultado de validación con errores/advertencias.
#[command]
pub async fn validate_exit_contratista(
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<ResultadoValidacionSalida, IngresoContratistaError> {
    service::validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(|e| IngresoContratistaError::Validation(e))
}

/// [Comando Tauri] Finaliza el registro de permanencia y libera recursos.
///
/// # Argumentos
/// * `input` - Datos de la salida.
/// * `usuario_id` - ID del guardia que registra.
///
/// # Retorno
/// Ingreso actualizado con hora de salida.
#[command]
pub async fn register_exit_contratista(
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::registrar_salida(input, usuario_id).await
}

// --------------------------------------------------------------------------
// MONITOREO DE PLANTA
// --------------------------------------------------------------------------

/// [Comando Tauri] Estado de ocupación actual de la planta.
///
/// # Retorno
/// Lista de ingresos activos con tiempo transcurrido.
#[command]
pub async fn get_permanencia_status(
) -> Result<Vec<IngresoConEstadoResponse>, IngresoContratistaError> {
    service::get_ingresos_abiertos_con_alertas().await
}

/// [Comando Tauri] Consulta alertas por tiempos de permanencia excedidos.
///
/// # Retorno
/// Lista de alertas para contratistas que exceden el límite.
#[command]
pub async fn check_time_alerts() -> Result<Vec<AlertaTiempoExcedido>, IngresoContratistaError> {
    service::verificar_tiempos_excedidos().await
}
