// ==========================================
// src/commands/export_commands.rs
// ==========================================

use crate::domain::errors::ExportError;
use crate::models::export::{ExportRequest, ExportResponse};
use crate::services::export_service;

// ==========================================
// COMANDO PRINCIPAL DE EXPORTACIÓN
// ==========================================

#[tauri::command]
pub async fn export_data(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    Ok(export_service::export_data(request).await?)
}

// ==========================================
// COMANDO DE VERIFICACIÓN DE DISPONIBILIDAD
// ==========================================

#[tauri::command]
pub async fn check_export_available() -> Result<bool, ExportError> {
    Ok(export_service::is_export_available())
}

#[tauri::command]
pub async fn get_available_export_formats() -> Result<Vec<String>, ExportError> {
    use crate::export;

    let formats = export::available_formats().iter().map(|&s| s.to_string()).collect();

    Ok(formats)
}

#[tauri::command]
pub async fn is_export_format_available(format: String) -> Result<bool, ExportError> {
    use crate::export;

    Ok(export::is_format_available(&format))
}

// ==========================================
// COMANDOS OPCIONALES (si quieres separar por formato)
// ==========================================

#[cfg(feature = "export-pdf")]
#[tauri::command]
pub async fn export_to_pdf(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut pdf_request = request;
    pdf_request.format = "pdf".to_string();

    Ok(export_service::export_data(pdf_request).await?)
}

#[cfg(feature = "export-excel")]
#[tauri::command]
pub async fn export_to_excel(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut excel_request = request;
    excel_request.format = "excel".to_string();

    Ok(export_service::export_data(excel_request).await?)
}

#[tauri::command]
pub async fn export_to_csv(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut csv_request = request;
    csv_request.format = "csv".to_string();

    Ok(export_service::export_data(csv_request).await?)
}

// ==========================================
// PREVIEW PARA DIÁLOGO AVANZADO
// ==========================================

#[tauri::command]
pub async fn export_preview(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut preview_request = request;
    preview_request.format = "pdf".to_string();
    preview_request.show_preview = Some(true);
    preview_request.target_path = None;

    Ok(export_service::export_data(preview_request).await?)
}

// ==========================================
// STUBS CUANDO LOS FEATURES ESTÁN DESHABILITADOS
// ==========================================

#[cfg(not(feature = "export-pdf"))]
#[tauri::command]
pub async fn export_to_pdf(_request: ExportRequest) -> Result<ExportResponse, ExportError> {
    Err(ExportError::UnsupportedFormat("Exportación a PDF no disponible en esta build".to_string()))
}

#[cfg(not(feature = "export-excel"))]
#[tauri::command]
pub async fn export_to_excel(_request: ExportRequest) -> Result<ExportResponse, ExportError> {
    Err(ExportError::UnsupportedFormat(
        "Exportación a Excel no disponible en esta build".to_string(),
    ))
}
