/// Puertos de Entrada: Consultas Generales de Ingresos y Gestión de Alertas (Global Bridge).
///
/// Este módulo proporciona una visión consolidada de todos los movimientos de acceso
/// en planta, facilitando el monitoreo centralizado y la respuesta ante incidencias
/// o anomalías detectadas por el sistema de seguridad.
use crate::domain::errors::{AlertaError, IngresoError};
use crate::models::ingreso::{
    AlertaGafeteResponse, IngresoListResponse, IngresoResponse, ResolverAlertaInput,
};
use crate::services::alerta_service;
use crate::services::ingreso_general_service;
use crate::services::session::SessionState;
use tauri::{command, State};

// ==========================================
// MONITORIZACIÓN GLOBAL DE INGRESOS
// ==========================================

/// Recupera un registro de ingreso específico por su identificador único.
/// [Comando Tauri]
#[command]
pub async fn get_ingreso_by_id(id: String) -> Result<IngresoResponse, IngresoError> {
    ingreso_general_service::get_ingreso_by_id(&id).await?.ok_or(IngresoError::NotFound)
}

/// Obtiene el historial completo de ingresos con métricas de rendimiento (Dashboard principal).
/// [Comando Tauri]
#[command]
pub async fn get_all_ingresos() -> Result<IngresoListResponse, IngresoError> {
    ingreso_general_service::get_all_ingresos_with_stats().await
}

/// Filtra exclusivamente las personas que se encuentran dentro de las instalaciones en tiempo real.
/// [Comando Tauri]
#[command]
pub async fn get_ingresos_abiertos() -> Result<Vec<IngresoResponse>, IngresoError> {
    ingreso_general_service::get_ingresos_abiertos().await
}

/// Localiza un ingreso activo mediante el escaneo físico del gafete.
/// [Comando Tauri]
#[command]
pub async fn get_ingreso_by_gafete(gafete_numero: String) -> Result<IngresoResponse, IngresoError> {
    ingreso_general_service::get_ingreso_by_gafete(&gafete_numero)
        .await?
        .ok_or(IngresoError::NotFound)
}

/// Realiza consultas históricas de flujo de personal en rangos de tiempo específicos.
/// [Comando Tauri]
#[command]
pub async fn get_salidas_en_rango(
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<IngresoResponse>, IngresoError> {
    ingreso_general_service::get_salidas_en_rango(&fecha_inicio, &fecha_fin).await
}

/// Reporte de actividad diaria: Agrupa todas las salidas ocurridas en una fecha calendario.
/// [Comando Tauri]
#[command]
pub async fn get_salidas_del_dia(fecha: String) -> Result<Vec<IngresoResponse>, IngresoError> {
    let start = format!("{fecha}T00:00:00Z");
    let end = format!("{fecha}T23:59:59Z");
    ingreso_general_service::get_salidas_en_rango(&start, &end).await
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
