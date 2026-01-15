// ==========================================
// src/export/csv/generator.rs
// ==========================================
// Generador CSV sin dependencias externas
// Usa solo std::fs y String manipulation

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{CsvConfig, ExportValue};
use std::collections::HashMap;
use std::path::PathBuf;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera un archivo CSV y retorna el path
pub fn generate_csv(
    headers: &[String],
    rows: &[HashMap<String, ExportValue>],
    config: &CsvConfig,
    target_path: Option<String>,
) -> ExportResult<String> {
    // 1. Construir contenido CSV
    let csv_content = build_csv_content(headers, rows, config)?;

    // 2. Determinar path de salida
    let output_path = if let Some(path) = target_path {
        PathBuf::from(path)
    } else {
        get_output_path(&config.filename)?
    };

    // 3. Escribir archivo
    std::fs::write(&output_path, csv_content)
        .map_err(|e| ExportError::CsvWriteError(format!("Error escribiendo archivo: {e}")))?;

    // 4. Retornar path como string
    Ok(output_path
        .to_str()
        .ok_or_else(|| ExportError::FileSystemError("Path inválido".to_string()))?
        .to_string())
}

// ==========================================
// CONSTRUCCIÓN DEL CONTENIDO
// ==========================================

/// Construye el contenido completo del CSV
fn build_csv_content(
    headers: &[String],
    rows: &[HashMap<String, ExportValue>],
    config: &CsvConfig,
) -> ExportResult<String> {
    let mut content = String::new();

    // 1. UTF-8 BOM (para Excel)
    if config.include_bom {
        content.push('\u{FEFF}');
    }

    // 2. Headers
    let header_line = encode_csv_line(headers, config.delimiter.as_char());
    content.push_str(&header_line);
    content.push('\n');

    // 3. Rows
    for row in rows {
        let values: Vec<String> = headers
            .iter()
            .map(|header| row.get(header).map(std::string::ToString::to_string).unwrap_or_default())
            .collect();

        let row_line = encode_csv_line(&values, config.delimiter.as_char());
        content.push_str(&row_line);
        content.push('\n');
    }

    Ok(content)
}

/// Codifica una línea CSV con escape de caracteres especiales
fn encode_csv_line(values: &[String], delimiter: char) -> String {
    values
        .iter()
        .map(|value| escape_csv_field(value, delimiter))
        .collect::<Vec<_>>()
        .join(&delimiter.to_string())
}

/// Escapa un campo CSV según RFC 4180
fn escape_csv_field(field: &str, delimiter: char) -> String {
    // Caracteres que requieren quoting
    let needs_quoting = field.contains('"')
        || field.contains(delimiter)
        || field.contains('\n')
        || field.contains('\r');

    if needs_quoting {
        // Escapar comillas dobles duplicándolas
        let escaped = field.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        field.to_string()
    }
}

// ==========================================
// PATH MANAGEMENT
// ==========================================

/// Obtiene el path de salida (Downloads o directorio temporal)
fn get_output_path(filename: &str) -> ExportResult<PathBuf> {
    // Intentar usar el directorio de Downloads
    if let Some(downloads_dir) = get_downloads_dir() {
        let path = downloads_dir.join(filename);
        return Ok(path);
    }

    // Fallback: directorio temporal
    let temp_dir = std::env::temp_dir();
    Ok(temp_dir.join(filename))
}

/// Obtiene el directorio de Downloads del usuario
fn get_downloads_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        dirs::download_dir()
    }

    #[cfg(target_os = "macos")]
    {
        dirs::download_dir()
    }

    #[cfg(target_os = "linux")]
    {
        dirs::download_dir()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

// ==========================================
// HELPERS
// ==========================================

// ==========================================
// TESTS
// ==========================================

