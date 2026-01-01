/// Puertos de Entrada: Consultas Generales de Ingresos y Gestión de Alertas (Global Bridge).
///
/// Este módulo proporciona una visión consolidada de todos los movimientos de acceso
/// en planta, facilitando el monitoreo centralizado y la respuesta ante incidencias
/// o anomalías detectadas por el sistema de seguridad.
use crate::domain::errors::AlertaError;
use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};
use crate::services::alerta_service;
use crate::services::ingreso_general_service;
use crate::services::session::SessionState;
use tauri::State;

// ==========================================
// MONITORIZACIÓN GLOBAL DE INGRESOS
// ==========================================

/// Recupera un registro de ingreso específico por su identificador único.
#[tauri::command]
pub async fn get_ingreso_by_id(id: String) -> Result<IngresoResponse, String> {
    ingreso_general_service::get_ingreso_by_id(&id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "El registro de ingreso solicitado no existe".to_string())
}

/// Obtiene el historial completo de ingresos con métricas de rendimiento (Dashboard principal).
#[tauri::command]
pub async fn get_all_ingresos() -> Result<IngresoListResponse, String> {
    ingreso_general_service::get_all_ingresos_with_stats().await.map_err(|e| e.to_string())
}

/// Filtra exclusivamente las personas que se encuentran dentro de las instalaciones en tiempo real.
#[tauri::command]
pub async fn get_ingresos_abiertos() -> Result<Vec<IngresoResponse>, String> {
    ingreso_general_service::get_ingresos_abiertos().await.map_err(|e| e.to_string())
}

/// Localiza un ingreso activo mediante el escaneo físico del gafete.
#[tauri::command]
pub async fn get_ingreso_by_gafete(gafete_numero: String) -> Result<IngresoResponse, String> {
    ingreso_general_service::get_ingreso_by_gafete(&gafete_numero)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No se detectó ningún ingreso activo vinculado a este gafete".to_string())
}

/// Realiza consultas históricas de flujo de personal en rangos de tiempo específicos.
#[tauri::command]
pub async fn get_salidas_en_rango(
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, String> {
    ingreso_general_service::get_salidas_en_rango(&fecha_inicio, &fecha_fin)
        .await
        .map_err(|e| e.to_string())
}

/// Reporte de actividad diaria: Agrupa todas las salidas ocurridas en una fecha calendario.
#[tauri::command]
pub async fn get_salidas_del_dia(fecha: String) -> Result<Vec<IngresoResponse>, String> {
    let start = format!("{}T00:00:00Z", fecha);
    let end = format!("{}T23:59:59Z", fecha);
    ingreso_general_service::get_salidas_en_rango(&start, &end).await.map_err(|e| e.to_string())
}

// ==========================================
// GESTIÓN DE ALERTAS DE SEGURIDAD
// ==========================================

/// Identifica alertas críticas (incumplimiento de normas, bloqueos) asociadas a una cédula.
#[tauri::command]
pub async fn get_alertas_pendientes_by_cedula(
    cedula: String,
) -> Result<Vec<AlertaGafeteResponse>, AlertaError> {
    let alertas = alerta_service::find_pendientes_by_cedula(&cedula).await?;
    let response = alertas.into_iter().map(AlertaGafeteResponse::from).collect();
    Ok(response)
}

/// Central de Alertas: Lista todas las incidencias de seguridad registradas en el sistema.
#[tauri::command]
pub async fn get_all_alertas_gafetes() -> Result<Vec<AlertaGafeteResponse>, AlertaError> {
    let alertas = alerta_service::find_all(None).await?;
    let response = alertas.into_iter().map(AlertaGafeteResponse::from).collect();
    Ok(response)
}

/// Protocolo de Resolución: Permite a un supervisor cerrar una alerta tras una inspección manual.
#[tauri::command]
pub async fn resolver_alerta_gafete(
    session: State<'_, SessionState>,
    input: ResolverAlertaInput,
) -> Result<(), AlertaError> {
    let user = session
        .get_user()
        .ok_or(AlertaError::Validation("Sesión de supervisor no válida o expirada".to_string()))?;

    let mut payload = input;
    payload.usuario_id = Some(user.id.clone());
    alerta_service::resolver(payload).await?;

    Ok(())
}
