// ==========================================
// src/commands/email_commands.rs
// ==========================================
// Capa de comandos Tauri: delega al servicio

use crate::models::reporte::{CreateReporteInput, ReporteListResponse, ReporteResponse};
use crate::services::email_service;
use sqlx::SqlitePool;
use tauri::State;

/// Crea y envia un reporte (sugerencia, error o mejora)
#[tauri::command]
pub async fn send_suggestion(
    pool: State<'_, SqlitePool>,
    subject: String,
    message: String,
    contact_info: Option<String>,
    attachment_base64: Option<String>,
    attachment_name: Option<String>,
) -> Result<ReporteResponse, String> {
    // Convertir parametros legacy al nuevo formato
    let input = CreateReporteInput {
        tipo: "sugerencia".to_string(),
        asunto: subject,
        mensaje: message,
        contacto: contact_info,
        adjunto_base64: attachment_base64,
        nombre_adjunto: attachment_name,
    };

    email_service::crear_y_enviar_reporte(&pool, input).await
}

/// Crea y envia un reporte de error
#[tauri::command]
pub async fn send_error_report(
    pool: State<'_, SqlitePool>,
    subject: String,
    message: String,
    contact_info: Option<String>,
    attachment_base64: Option<String>,
    attachment_name: Option<String>,
) -> Result<ReporteResponse, String> {
    let input = CreateReporteInput {
        tipo: "error".to_string(),
        asunto: subject,
        mensaje: message,
        contacto: contact_info,
        adjunto_base64: attachment_base64,
        nombre_adjunto: attachment_name,
    };

    email_service::crear_y_enviar_reporte(&pool, input).await
}

/// Crea y envia un reporte generico (tipo flexible)
#[tauri::command]
pub async fn create_reporte(
    pool: State<'_, SqlitePool>,
    input: CreateReporteInput,
) -> Result<ReporteResponse, String> {
    email_service::crear_y_enviar_reporte(&pool, input).await
}

/// Obtiene todos los reportes
#[tauri::command]
pub async fn get_all_reportes(pool: State<'_, SqlitePool>) -> Result<ReporteListResponse, String> {
    email_service::get_all_reportes(&pool).await
}

/// Obtiene un reporte por ID
#[tauri::command]
pub async fn get_reporte(pool: State<'_, SqlitePool>, id: String) -> Result<ReporteResponse, String> {
    email_service::get_reporte_by_id(&pool, &id).await
}

/// Obtiene reportes filtrados por tipo
#[tauri::command]
pub async fn get_reportes_by_tipo(
    pool: State<'_, SqlitePool>,
    tipo: String,
) -> Result<Vec<ReporteResponse>, String> {
    email_service::get_reportes_by_tipo(&pool, &tipo).await
}

/// Reintenta el envio de un reporte fallido
#[tauri::command]
pub async fn retry_reporte(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ReporteResponse, String> {
    email_service::reintentar_envio(&pool, &id).await
}
