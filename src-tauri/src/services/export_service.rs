//! # Servicio de Exportación de Datos
//!
//! Orquestador principal para la generación de reportes en múltiples formatos (PDF, Excel, CSV).
//!
//! ## Responsabilidades
//! - Coordinar la normalización de datos.
//! - Gestionar la configuración por formato.
//! - Ejecutar la generación de archivos de forma eficiente (thread blocking para CPU-intensive).
//! - Proveer feedback claro (logging y errores tipados).
//!
//! ## Dependencias
//! - `crate::export`: Módulos de infraestructura de exportación.
//! - `crate::services::export_profile_service`: Acceso a perfiles y templates.

use crate::domain::export as domain;
use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{
    CsvConfig, CsvDelimiter, ExcelConfig, ExportData, ExportFormat, ExportRequest, ExportResponse,
    PageOrientation, PdfConfig,
};
use log::{error, info, warn};

// ==========================================
// FUNCIÓN PRINCIPAL DE EXPORTACIÓN
// ==========================================

/// Punto de entrada principal para cualquier exportación.
///
/// Orquesta todo el flujo: validación, normalización y delegación
/// a los generadores específicos (PDF, Excel, CSV).
///
/// # Argumentos
/// * `request` - Solicitud completa con datos y configuración.
///
/// # Retorno
/// `ExportResult<ExportResponse>` con los bytes generados o la ruta del archivo.
///
/// # Errores
/// * `ExportError::InvalidData` - Si los datos superan límites o son incoherentes.
/// * `ExportError::InvalidFormat` - Si el formato solicitado no es soportado.
/// * `ExportError::Unknown` - Fallos internos de generación.
///
/// # Logging
/// - `INFO`: Inicio y fin de la exportación.
/// - `WARN`: Si se detectan parámetros inusuales (ej. demasiadas filas).
pub async fn export_data(request: ExportRequest) -> ExportResult<ExportResponse> {
    info!(
        "Iniciando exportación: formato={:?}, filas={}, título={:?}",
        request.format,
        request.rows.len(),
        request.title
    );

    // 1. Validar request completo
    domain::validar_export_request(&request).map_err(ExportError::InvalidData)?;

    // 2. Validar tamaño total (seguridad)
    if let Err(e) = domain::validar_tamano_total(&request) {
        warn!("Intento de exportación rechazada por tamaño: {e:?}");
        return Err(ExportError::InvalidData(e));
    }

    // 3. Normalizar datos
    let export_data = normalizar_export_data(&request)?;

    // 4. Exportar según formato
    let result = match export_data.format {
        ExportFormat::Pdf => export_to_pdf_internal(export_data).await,
        ExportFormat::Excel => export_to_excel_internal(export_data).await,
        ExportFormat::Csv => export_to_csv_internal(export_data),
    };

    if result.is_ok() {
        info!("Exportación finalizada exitosamente.");
    } else {
        error!("Falló la exportación: {:?}", result.as_ref().err());
    }

    result
}

// ==========================================
// NORMALIZACIÓN DE DATOS
// ==========================================

/// Convierte `ExportRequest` en `ExportData` normalizado
fn normalizar_export_data(request: &ExportRequest) -> ExportResult<ExportData> {
    // 1. Parsear formato
    let format = request.format.parse().map_err(ExportError::InvalidFormat)?;

    // 2. Clonar headers
    let headers = request.headers.clone();

    // 3. Normalizar todas las rows (JSON → ExportValue)
    let rows: Vec<std::collections::HashMap<String, crate::models::export::ExportValue>> =
        request.rows.iter().map(|row| domain::normalizar_row(row, &headers)).collect();

    // 4. Construir config según formato
    let pdf_config =
        if format == ExportFormat::Pdf { Some(construir_pdf_config(request)?) } else { None };

    let excel_config =
        if format == ExportFormat::Excel { Some(construir_excel_config(request)) } else { None };

    let csv_config =
        if format == ExportFormat::Csv { Some(construir_csv_config(request)?) } else { None };

    Ok(ExportData {
        format,
        headers,
        rows,
        pdf_config,
        excel_config,
        csv_config,
        target_path: request.target_path.clone(),
    })
}

/// Construye configuración para PDF
fn construir_pdf_config(request: &ExportRequest) -> ExportResult<PdfConfig> {
    // Título
    let title = if let Some(ref t) = request.title {
        domain::validar_titulo(t).map_err(ExportError::InvalidTitle)?;
        domain::normalizar_titulo(t)
    } else {
        "Reporte".to_string()
    };

    // Orientación
    let orientation = if let Some(ref o) = request.orientation {
        domain::validar_orientacion(o).map_err(ExportError::InvalidOrientation)?
    } else {
        PageOrientation::Landscape
    };

    // Preview
    let show_preview = request.show_preview.unwrap_or(false);

    // Font size (clamp entre 8 y 20)
    let font_size = request.font_size.unwrap_or(10).clamp(8, 20);

    // Font family
    let font_family = request.font_family.clone().unwrap_or_else(|| "Inter".to_string());

    // Márgenes (con defaults razonables)
    let margin_top = request.margin_top.unwrap_or(2.0);
    let margin_bottom = request.margin_bottom.unwrap_or(2.0);
    let margin_left = request.margin_left.unwrap_or(1.5);
    let margin_right = request.margin_right.unwrap_or(1.5);

    // Color del banner
    let banner_color = request.banner_color.clone().unwrap_or_else(|| "#059669".to_string());

    Ok(PdfConfig {
        title,
        orientation,
        headers: request.headers.clone(),
        show_preview,
        template_id: request.template_id.clone(),
        font_size,
        font_family,
        margin_top,
        margin_bottom,
        margin_left,
        margin_right,
        banner_color,
        generated_by: request.generated_by.clone().unwrap_or_default(),
    })
}

/// Construye configuración para Excel
fn construir_excel_config(request: &ExportRequest) -> ExcelConfig {
    let filename = request.title.clone().unwrap_or_else(|| "export".to_string());

    ExcelConfig {
        filename: format!("{}.xlsx", sanitizar_filename(&filename)),
        headers: request.headers.clone(),
    }
}

/// Construye configuración para CSV
fn construir_csv_config(request: &ExportRequest) -> ExportResult<CsvConfig> {
    // Filename
    let filename = request.title.clone().unwrap_or_else(|| "export".to_string());

    // Delimitador
    let delimiter = if let Some(ref d) = request.delimiter {
        domain::validar_delimitador(d).map_err(ExportError::InvalidDelimiter)?
    } else {
        CsvDelimiter::Comma
    };

    // BOM para Excel UTF-8
    let include_bom = request.include_bom.unwrap_or(true);

    Ok(CsvConfig {
        filename: format!("{}.csv", sanitizar_filename(&filename)),
        headers: request.headers.clone(),
        delimiter,
        include_bom,
    })
}

// ==========================================
// EXPORTACIÓN POR FORMATO
// ==========================================

/// Exporta a PDF usando Typst (con `spawn_blocking` para no bloquear UI)
#[cfg(feature = "export")]
async fn export_to_pdf_internal(data: ExportData) -> ExportResult<ExportResponse> {
    use crate::export::pdf;
    use crate::models::export::{PdfColors, PdfDesign, PdfFonts};
    use crate::services::export_profile_service;

    let config = data
        .pdf_config
        .ok_or_else(|| ExportError::Unknown("Config PDF no encontrada".to_string()))?;

    let profile = config
        .template_id
        .as_ref()
        .and_then(|id| export_profile_service::get_profile_by_id(id))
        .or_else(export_profile_service::get_default_profile);

    // Obtener diseño del perfil o usar uno default al vuelo
    let design = profile.and_then(|p| p.pdf_design).unwrap_or_else(|| PdfDesign {
        page_size: "us-letter".to_string(),
        orientation: "landscape".to_string(),
        margin_x: 1.5,
        margin_x_unit: "cm".to_string(),
        margin_y: 2.0,
        margin_y_unit: "cm".to_string(),
        colors: PdfColors {
            header_fill: "#e8e8e8".to_string(),
            header_text: "#000000".to_string(),
            row_text: "#000000".to_string(),
            border: "#000000".to_string(),
        },
        fonts: PdfFonts { family: "Inter".to_string(), size: 10, header_size: 11 },
    });

    // ✅ PERFORMANCE: Run CPU-intensive Typst compilation on blocking thread
    let headers = data.headers.clone();
    let rows = data.rows.clone();
    let config_clone = config.clone();
    let design_clone = design.clone();

    let pdf_bytes = tokio::task::spawn_blocking(move || {
        pdf::generate_pdf(&headers, &rows, &config_clone, &design_clone)
    })
    .await
    .map_err(|e| ExportError::Unknown(format!("Error en thread de PDF: {e}")))??;

    // Determinar si guardar en disco
    let file_path = if let Some(path) = data.target_path {
        std::fs::write(&path, &pdf_bytes)
            .map_err(|e| ExportError::FileSystemError(format!("Error escribiendo PDF: {e}")))?;
        Some(path)
    } else {
        None
    };

    // Si hay preview, devolver bytes aunque se haya guardado
    let bytes = if config.show_preview || file_path.is_none() { Some(pdf_bytes) } else { None };

    Ok(ExportResponse {
        success: true,
        format: "pdf".to_string(),
        bytes,
        file_path,
        message: "PDF generado exitosamente".to_string(),
    })
}

#[cfg(not(feature = "export"))]
async fn export_to_pdf_internal(_data: ExportData) -> ExportResult<ExportResponse> {
    Err(ExportError::Unknown("Función de exportación PDF no disponible en esta build".to_string()))
}

/// Exporta a Excel usando `rust_xlsxwriter` (con `spawn_blocking` para no bloquear UI)
#[cfg(feature = "export")]
async fn export_to_excel_internal(data: ExportData) -> ExportResult<ExportResponse> {
    use crate::export::excel;

    let config = data
        .excel_config
        .ok_or_else(|| ExportError::Unknown("Config Excel no encontrada".to_string()))?;

    // ✅ PERFORMANCE: Run CPU-intensive Excel generation on blocking thread
    let headers = data.headers.clone();
    let rows = data.rows.clone();
    let target_path = data.target_path.clone();

    let file_path = tokio::task::spawn_blocking(move || {
        excel::generate_excel(&headers, &rows, &config, target_path)
    })
    .await
    .map_err(|e| ExportError::Unknown(format!("Error en thread de Excel: {e}")))??;

    Ok(ExportResponse {
        success: true,
        format: "excel".to_string(),
        bytes: None,
        file_path: Some(file_path),
        message: "Excel generado exitosamente".to_string(),
    })
}

#[cfg(not(feature = "export"))]
async fn export_to_excel_internal(_data: ExportData) -> ExportResult<ExportResponse> {
    Err(ExportError::Unknown(
        "Función de exportación Excel no disponible en esta build".to_string(),
    ))
}

/// Exporta a CSV (sin dependencias externas)
fn export_to_csv_internal(data: ExportData) -> ExportResult<ExportResponse> {
    use crate::export::csv;

    let config = data
        .csv_config
        .ok_or_else(|| ExportError::Unknown("Config CSV no encontrada".to_string()))?;

    // Generar CSV y obtener path
    let file_path = csv::generate_csv(&data.headers, &data.rows, &config, data.target_path)?;

    Ok(ExportResponse {
        success: true,
        format: "csv".to_string(),
        bytes: None,
        file_path: Some(file_path),
        message: "CSV generado exitosamente".to_string(),
    })
}

// ==========================================
// HELPERS INTERNOS
// ==========================================

/// Sanitiza un filename (remueve caracteres especiales)
fn sanitizar_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '_',
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
        .trim_matches('_')
        .to_string()
}

/// Verifica si el módulo de export está disponible
pub const fn is_export_available() -> bool {
    cfg!(feature = "export")
}

// ==========================================
// TESTS UNITARIOS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitizar_filename() {
        assert_eq!(sanitizar_filename("Reporte Mensual"), "Reporte_Mensual");
        assert_eq!(sanitizar_filename("Test@#$%"), "Test");
        assert_eq!(sanitizar_filename("File (1).xlsx"), "File_-1--xlsx");
    }

    #[test]
    fn test_is_export_available() {
        // Just verify it returns a valid bool (compile-time check)
        let _available: bool = is_export_available();
    }

    #[test]
    fn test_defaults_pdf_config() {
        let mut req = ExportRequest::default();
        req.format = "pdf".to_string();

        // Defaults cuando no se envía title ni orientation
        let config = construir_pdf_config(&req).unwrap();
        assert_eq!(config.title, "Reporte");
        assert!(matches!(config.orientation, PageOrientation::Landscape));
        assert_eq!(config.font_size, 10);
        assert!((config.margin_top - 2.0_f32).abs() < f32::EPSILON);
    }
}
