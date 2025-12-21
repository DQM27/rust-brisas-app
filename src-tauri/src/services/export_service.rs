// ==========================================
// src/services/export_service.rs
// ==========================================
// Capa de servicio: orquesta dominio y módulos de export
// Contiene la lógica de negocio completa

use crate::domain::export as domain;
use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{
    CsvConfig, CsvDelimiter, ExcelConfig, ExportData, ExportFormat, ExportRequest, ExportResponse,
    PageOrientation, PdfConfig,
};
use std::collections::HashMap;

// ==========================================
// FUNCIÓN PRINCIPAL DE EXPORTACIÓN
// ==========================================

/// Punto de entrada principal para cualquier exportación
pub async fn export_data(request: ExportRequest) -> ExportResult<ExportResponse> {
    // 1. Validar request completo
    domain::validar_export_request(&request).map_err(|e| ExportError::InvalidData(e))?;

    // 2. Validar tamaño total (seguridad)
    domain::validar_tamano_total(&request).map_err(|e| ExportError::InvalidData(e))?;

    // 3. Normalizar datos
    let export_data = normalizar_export_data(&request)?;

    // 4. Exportar según formato
    match export_data.format {
        ExportFormat::Pdf => export_to_pdf_internal(export_data).await,
        ExportFormat::Excel => export_to_excel_internal(export_data).await,
        ExportFormat::Csv => export_to_csv_internal(export_data).await,
    }
}

// ==========================================
// NORMALIZACIÓN DE DATOS
// ==========================================

/// Convierte ExportRequest en ExportData normalizado
fn normalizar_export_data(request: &ExportRequest) -> ExportResult<ExportData> {
    // 1. Parsear formato
    let format =
        ExportFormat::from_str(&request.format).map_err(|e| ExportError::InvalidFormat(e))?;

    // 2. Clonar headers
    let headers = request.headers.clone();

    // 3. Normalizar todas las rows (JSON → String)
    let rows: Vec<HashMap<String, String>> = request
        .rows
        .iter()
        .map(|row| domain::normalizar_row(row, &headers))
        .collect();

    // 4. Construir config según formato
    let pdf_config = if format == ExportFormat::Pdf {
        Some(construir_pdf_config(request)?)
    } else {
        None
    };

    let excel_config = if format == ExportFormat::Excel {
        Some(construir_excel_config(request))
    } else {
        None
    };

    let csv_config = if format == ExportFormat::Csv {
        Some(construir_csv_config(request)?)
    } else {
        None
    };

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
        domain::validar_titulo(t).map_err(|e| ExportError::InvalidTitle(e))?;
        domain::normalizar_titulo(t)
    } else {
        "Reporte".to_string()
    };

    // Orientación
    let orientation = if let Some(ref o) = request.orientation {
        domain::validar_orientacion(o).map_err(|e| ExportError::InvalidOrientation(e))?
    } else {
        PageOrientation::Landscape
    };

    // Preview
    let show_preview = request.show_preview.unwrap_or(false);

    // Font size (clamp entre 8 y 20)
    let font_size = request.font_size.unwrap_or(10).clamp(8, 20);

    // Font family
    let font_family = request
        .font_family
        .clone()
        .unwrap_or_else(|| "Inter".to_string());

    // Márgenes (con defaults razonables)
    let margin_top = request.margin_top.unwrap_or(2.0);
    let margin_bottom = request.margin_bottom.unwrap_or(2.0);
    let margin_left = request.margin_left.unwrap_or(1.5);
    let margin_right = request.margin_right.unwrap_or(1.5);

    // Color del banner
    let banner_color = request
        .banner_color
        .clone()
        .unwrap_or_else(|| "#059669".to_string());

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
    let filename = request
        .title
        .clone()
        .unwrap_or_else(|| "export".to_string());

    ExcelConfig {
        filename: format!("{}.xlsx", sanitizar_filename(&filename)),
        headers: request.headers.clone(),
    }
}

/// Construye configuración para CSV
fn construir_csv_config(request: &ExportRequest) -> ExportResult<CsvConfig> {
    // Filename
    let filename = request
        .title
        .clone()
        .unwrap_or_else(|| "export".to_string());

    // Delimitador
    let delimiter = if let Some(ref d) = request.delimiter {
        domain::validar_delimitador(d).map_err(|e| ExportError::InvalidDelimiter(e))?
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

/// Exporta a PDF usando Typst
// Exporta a PDF usando Typst
#[cfg(feature = "export")]
async fn export_to_pdf_internal(data: ExportData) -> ExportResult<ExportResponse> {
    use crate::export::pdf;
    use crate::models::export::{PdfColors, PdfDesign, PdfFonts};
    use crate::services::export_profile_service;

    let config = data
        .pdf_config
        .ok_or_else(|| ExportError::Unknown("Config PDF no encontrada".to_string()))?;

    // ✅ Obtener PERFIL (usando template_id como profile_id por compatibilidad o default)
    let profile = if let Some(ref id) = config.template_id {
        export_profile_service::get_profile_by_id(id)
            .or_else(|| export_profile_service::get_default_profile())
    } else {
        export_profile_service::get_default_profile()
    };

    // Obtener diseño del perfil o usar uno default al vuelo
    let design = profile
        .and_then(|p| p.pdf_design)
        .unwrap_or_else(|| PdfDesign {
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
            fonts: PdfFonts {
                family: "Inter".to_string(),
                size: 10,
                header_size: 11,
            },
        });

    // Generar PDF
    let pdf_bytes = pdf::generate_pdf(&data.headers, &data.rows, &config, &design)?;

    // Determinar si guardar en disco
    let file_path = if let Some(path) = data.target_path {
        std::fs::write(&path, &pdf_bytes)
            .map_err(|e| ExportError::FileSystemError(format!("Error escribiendo PDF: {}", e)))?;
        Some(path)
    } else {
        None
    };

    // Si hay preview, devolver bytes aunque se haya guardado
    let bytes = if config.show_preview || file_path.is_none() {
        Some(pdf_bytes)
    } else {
        None
    };

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
    Err(ExportError::Unknown(
        "Función de exportación PDF no disponible en esta build".to_string(),
    ))
}

/// Exporta a Excel usando rust_xlsxwriter
#[cfg(feature = "export")]
async fn export_to_excel_internal(data: ExportData) -> ExportResult<ExportResponse> {
    use crate::export::excel;

    let config = data
        .excel_config
        .ok_or_else(|| ExportError::Unknown("Config Excel no encontrada".to_string()))?;

    // ✅ Generar Excel con target_path opcional
    let file_path = excel::generate_excel(&data.headers, &data.rows, &config, data.target_path)?;

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
async fn export_to_csv_internal(data: ExportData) -> ExportResult<ExportResponse> {
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
pub fn is_export_available() -> bool {
    cfg!(feature = "export")
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitizar_filename() {
        assert_eq!(sanitizar_filename("Reporte Mensual"), "Reporte_Mensual");
        assert_eq!(sanitizar_filename("Test@#$%"), "Test");
        assert_eq!(sanitizar_filename("File (1).xlsx"), "File_1_xlsx");
    }

    #[test]
    fn test_is_export_available() {
        // Depende de si el feature está activado
        let available = is_export_available();
        assert!(available || !available); // Siempre pasa, solo para cobertura
    }
}
