/// Motor de Exportación: Extracción y Reportes de Datos (Tauri Bridge).
///
/// Proporciona los puntos de entrada para generar reportes en diversos formatos
/// (PDF, Excel, CSV), permitiendo filtrar y previsualizar la información del sistema
/// antes de su archivado o auditoría.
use crate::domain::errors::ExportError;
use crate::models::export::{ExportRequest, ExportResponse};
use crate::services::export_service;

/// Comando universal para la generación de archivos de exportación.
#[tauri::command]
pub async fn export_data(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    Ok(export_service::export_data(request).await?)
}

/// Verifica si el subsistema de exportación está activo y listo para operar.
#[tauri::command]
pub async fn check_export_available() -> Result<bool, ExportError> {
    Ok(export_service::is_export_available())
}

/// Retorna la lista de formatos de exportación soportados por la build actual.
#[tauri::command]
pub async fn get_available_export_formats() -> Result<Vec<String>, ExportError> {
    use crate::export;

    let formats = export::available_formats().iter().map(|&s| s.to_string()).collect();

    Ok(formats)
}

/// Comprueba si un formato específico de archivo está disponible para su generación.
#[tauri::command]
pub async fn is_export_format_available(format: String) -> Result<bool, ExportError> {
    use crate::export;

    Ok(export::is_format_available(&format))
}

// --------------------------------------------------------------------------
// COMANDOS ESPECIALIZADOS POR FORMATO
// --------------------------------------------------------------------------

/// Especialización para exportación directa a formato PDF.
#[cfg(feature = "export-pdf")]
#[tauri::command]
pub async fn export_to_pdf(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut pdf_request = request;
    pdf_request.format = "pdf".to_string();

    Ok(export_service::export_data(pdf_request).await?)
}

/// Especialización para exportación directa a hojas de cálculo Excel.
#[cfg(feature = "export-excel")]
#[tauri::command]
pub async fn export_to_excel(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut excel_request = request;
    excel_request.format = "excel".to_string();

    Ok(export_service::export_data(excel_request).await?)
}

/// Especialización para exportación directa a archivos CSV planos.
#[tauri::command]
pub async fn export_to_csv(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut csv_request = request;
    csv_request.format = "csv".to_string();

    Ok(export_service::export_data(csv_request).await?)
}

/// Genera una previsualización rápida sin guardar un archivo permanente en disco.
#[tauri::command]
pub async fn export_preview(request: ExportRequest) -> Result<ExportResponse, ExportError> {
    let mut preview_request = request;
    preview_request.format = "pdf".to_string();
    preview_request.show_preview = Some(true);
    preview_request.target_path = None;

    Ok(export_service::export_data(preview_request).await?)
}

// --------------------------------------------------------------------------
// MANEJADORES DE ERROR CUANDO NO HAY SOPORTE (FALLBACKS)
// --------------------------------------------------------------------------

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
