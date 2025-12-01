// ==========================================
// src/export/csv/generator.rs
// ==========================================
// Generador CSV sin dependencias externas
// Usa solo std::fs y String manipulation

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::CsvConfig;
use std::collections::HashMap;
use std::path::PathBuf;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera un archivo CSV y retorna el path
pub fn generate_csv(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &CsvConfig,
) -> ExportResult<String> {
    // 1. Construir contenido CSV
    let csv_content = build_csv_content(headers, rows, config)?;

    // 2. Determinar path de salida
    let output_path = get_output_path(&config.filename)?;

    // 3. Escribir archivo
    std::fs::write(&output_path, csv_content)
        .map_err(|e| ExportError::CsvWriteError(format!("Error escribiendo archivo: {}", e)))?;

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
    rows: &[HashMap<String, String>],
    config: &CsvConfig,
) -> ExportResult<String> {
    let mut content = String::new();

    // 1. UTF-8 BOM (para Excel)
    if config.include_bom {
        content.push_str("\u{FEFF}");
    }

    // 2. Headers
    let header_line = encode_csv_line(headers, config.delimiter.as_char());
    content.push_str(&header_line);
    content.push('\n');

    // 3. Rows
    for row in rows {
        let values: Vec<String> = headers
            .iter()
            .map(|header| row.get(header).cloned().unwrap_or_default())
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
        format!("\"{}\"", escaped)
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

/// Estima el tamaño del CSV resultante (para validación)
pub fn estimate_csv_size(
    headers: &[String],
    rows: &[HashMap<String, String>],
    config: &CsvConfig,
) -> usize {
    let delimiter_char = config.delimiter.as_char();
    let bom_size = if config.include_bom { 3 } else { 0 };

    // Headers
    let header_size: usize = headers.iter().map(|h| h.len() + 2).sum(); // +2 para quotes
    let header_delimiters = (headers.len().saturating_sub(1)) * delimiter_char.len_utf8();

    // Rows
    let mut rows_size = 0;
    for row in rows {
        for header in headers {
            if let Some(value) = row.get(header) {
                rows_size += value.len() + 2; // +2 para potential quotes
            }
        }
        rows_size += (headers.len().saturating_sub(1)) * delimiter_char.len_utf8();
        rows_size += 1; // newline
    }

    bom_size + header_size + header_delimiters + 1 + rows_size
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::export::CsvDelimiter;

    #[test]
    fn test_escape_csv_field_simple() {
        assert_eq!(escape_csv_field("simple", ','), "simple");
        assert_eq!(escape_csv_field("123", ','), "123");
    }

    #[test]
    fn test_escape_csv_field_with_comma() {
        assert_eq!(escape_csv_field("hello, world", ','), "\"hello, world\"");
    }

    #[test]
    fn test_escape_csv_field_with_quotes() {
        assert_eq!(
            escape_csv_field("say \"hello\"", ','),
            "\"say \"\"hello\"\"\""
        );
    }

    #[test]
    fn test_escape_csv_field_with_newline() {
        assert_eq!(escape_csv_field("line1\nline2", ','), "\"line1\nline2\"");
    }

    #[test]
    fn test_encode_csv_line() {
        let values = vec!["Name".to_string(), "Age".to_string(), "City".to_string()];
        assert_eq!(encode_csv_line(&values, ','), "Name,Age,City");
    }

    #[test]
    fn test_encode_csv_line_with_special_chars() {
        let values = vec!["Smith, John".to_string(), "30".to_string()];
        assert_eq!(encode_csv_line(&values, ','), "\"Smith, John\",30");
    }

    #[test]
    fn test_build_csv_content() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let mut row1 = HashMap::new();
        row1.insert("Name".to_string(), "John".to_string());
        row1.insert("Age".to_string(), "30".to_string());

        let mut row2 = HashMap::new();
        row2.insert("Name".to_string(), "Jane".to_string());
        row2.insert("Age".to_string(), "25".to_string());

        let rows = vec![row1, row2];

        let config = CsvConfig {
            filename: "test.csv".to_string(),
            headers: headers.clone(),
            delimiter: CsvDelimiter::Comma,
            include_bom: false,
        };

        let content = build_csv_content(&headers, &rows, &config).unwrap();
        let expected = "Name,Age\nJohn,30\nJane,25\n";
        assert_eq!(content, expected);
    }

    #[test]
    fn test_build_csv_content_with_bom() {
        let headers = vec!["Name".to_string()];
        let rows = vec![];

        let config = CsvConfig {
            filename: "test.csv".to_string(),
            headers: headers.clone(),
            delimiter: CsvDelimiter::Comma,
            include_bom: true,
        };

        let content = build_csv_content(&headers, &rows, &config).unwrap();
        assert!(content.starts_with('\u{FEFF}'));
    }

    #[test]
    fn test_different_delimiters() {
        let values = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        
        assert_eq!(encode_csv_line(&values, ','), "A,B,C");
        assert_eq!(encode_csv_line(&values, ';'), "A;B;C");
        assert_eq!(encode_csv_line(&values, '\t'), "A\tB\tC");
        assert_eq!(encode_csv_line(&values, '|'), "A|B|C");
    }
}