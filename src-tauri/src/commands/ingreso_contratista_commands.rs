/// Puertos de Entrada: Gestión de Admisión de Contratistas (UI Bridge).
///
/// Este módulo expone las funciones de Rust al frontend (Svelte) mediante Tauri.
/// Actúa como un orquestador de bajo acoplamiento que recibe las peticiones de
/// la interfaz de usuario, las delega al servicio especializado y retorna las
/// respuestas tipadas para su visualización.
use crate::domain::errors::IngresoContratistaError;
use crate::models::ingreso::{
    CreateIngresoContratistaInput, IngresoResponse, RegistrarSalidaInput, ValidacionIngresoResponse,
};
use crate::services::ingreso_contratista_service as service;

// ==========================================
// 1. PROTOCOLOS DE ENTRADA
// ==========================================

/// Ejecuta el pre-chequeo de identidad y seguridad antes de mostrar el formulario de ingreso.
#[tauri::command]
pub async fn validate_ingreso_contratista(
    contratista_id: String,
) -> Result<ValidacionIngresoResponse, IngresoContratistaError> {
    service::validar_ingreso_contratista(contratista_id).await
}

/// Registra físicamente la entrada de un contratista a las instalaciones.
#[tauri::command]
pub async fn create_ingreso_contratista(
    input: CreateIngresoContratistaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::crear_ingreso_contratista(input, usuario_id).await
}

// ==========================================
// 2. PROTOCOLOS DE SALIDA
// ==========================================

/// Realiza una validación previa a la salida (Ej: verificar estado del gafete).
#[tauri::command]
pub async fn validate_exit_contratista(
    ingreso_id: String,
    gafete_devuelto: Option<String>,
) -> Result<service::ResultadoValidacionSalida, IngresoContratistaError> {
    service::validar_puede_salir(&ingreso_id, gafete_devuelto.as_deref())
        .await
        .map_err(|e| IngresoContratistaError::Validation(e))
}

/// Finaliza el registro de permanencia y libera los recursos.
#[tauri::command]
pub async fn register_exit_contratista(
    input: RegistrarSalidaInput,
    usuario_id: String,
) -> Result<IngresoResponse, IngresoContratistaError> {
    service::registrar_salida(input, usuario_id).await
}

// ==========================================
// 3. MONITOREO DE PLANTA
// ==========================================

/// Obtiene el estado de ocupación actual (Quiénes están dentro y cuánto tiempo llevan).
#[tauri::command]
pub async fn get_permanencia_status(
) -> Result<Vec<service::IngresoConEstadoResponse>, IngresoContratistaError> {
    service::get_ingresos_abiertos_con_alertas().await
}

/// Consulta reactiva de alertas por tiempos de permanencia excedidos.
#[tauri::command]
pub async fn check_time_alerts(
) -> Result<Vec<service::AlertaTiempoExcedido>, IngresoContratistaError> {
    service::verificar_tiempos_excedidos().await
}

// ==========================================
// 4. GESTIÓN EXCEPCIONAL
// ==========================================
