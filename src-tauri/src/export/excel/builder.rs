// ==========================================
// src/export/excel/builder.rs
// ==========================================
// Generador Excel usando rust_xlsxwriter
// Crea archivos .xlsx con formato profesional

use crate::export::errors::{ExportError, ExportResult};
use crate::models::export::{ExcelConfig, ExportValue};
use rust_xlsxwriter::{Format, Workbook, Worksheet};
use std::collections::HashMap;
use std::path::PathBuf;

// ==========================================
// FUNCIÓN PRINCIPAL
// ==========================================

/// Genera un archivo Excel y retorna el path
pub fn generate_excel(
    headers: &[String],
    rows: &[HashMap<String, ExportValue>],
    config: &ExcelConfig,
    target_path: Option<String>,
) -> ExportResult<String> {
    // 1. Crear workbook
    let mut workbook = Workbook::new();

    // 2. Agregar worksheet
    // ✅ FIX: add_worksheet() retorna &mut Worksheet directamente
    let worksheet = workbook.add_worksheet();

    // 3. Escribir contenido
    write_excel_content(worksheet, headers, rows)?;

    // 4. Determinar path de salida (priorizar target_path del usuario)
    let output_path = get_output_path(&config.filename, target_path)?;

    // 5. Guardar archivo
    workbook
        .save(&output_path)
        .map_err(|e| ExportError::XlsxWriteError(format!("Error guardando Excel: {}", e)))?;

    // 6. Retornar path como string
    Ok(output_path
        .to_str()
        .ok_or_else(|| ExportError::FileSystemError("Path inválido".to_string()))?
        .to_string())
}

// ==========================================
// ESCRITURA DEL CONTENIDO
// ==========================================

/// Escribe headers y rows en el worksheet con formato
fn write_excel_content(
    worksheet: &mut Worksheet,
    headers: &[String],
    rows: &[HashMap<String, ExportValue>],
) -> ExportResult<()> {
    // 1. Crear formatos
    let header_format = create_header_format()?;
    let cell_format = create_cell_format()?;

    // 2. Escribir headers (fila 0)
    for (col_idx, header) in headers.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col_idx as u16, header, &header_format)
            .map_err(|e| ExportError::XlsxWriteError(format!("Error escribiendo header: {}", e)))?;
    }

    // 3. Escribir rows (empezando en fila 1)
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, header) in headers.iter().enumerate() {
            let value_opt = row.get(header);

            let result = match value_opt {
                Some(ExportValue::Text(s)) => worksheet.write_string_with_format(
                    (row_idx + 1) as u32,
                    col_idx as u16,
                    s,
                    &cell_format,
                ),
                Some(ExportValue::Number(n)) => worksheet.write_number_with_format(
                    (row_idx + 1) as u32,
                    col_idx as u16,
                    *n,
                    &cell_format,
                ),
                Some(ExportValue::Bool(b)) => worksheet.write_boolean_with_format(
                    (row_idx + 1) as u32,
                    col_idx as u16,
                    *b,
                    &cell_format,
                ),
                None => worksheet.write_string_with_format(
                    (row_idx + 1) as u32,
                    col_idx as u16,
                    "",
                    &cell_format,
                ),
            };

            result.map_err(|e| {
                ExportError::XlsxWriteError(format!(
                    "Error escribiendo celda [{}, {}]: {}",
                    row_idx + 1,
                    col_idx,
                    e
                ))
            })?;
        }
    }

    // 4. Autofit columnas
    autofit_columns(worksheet, headers, rows)?;

    // 5. Freeze headers (congelar primera fila)
    worksheet
        .set_freeze_panes(1, 0)
        .map_err(|e| ExportError::XlsxFormatError(format!("Error congelando headers: {}", e)))?;

    Ok(())
}

// ==========================================
// FORMATOS
// ==========================================

/// Crea formato para headers (negrita, fondo gris claro, bordes)
fn create_header_format() -> ExportResult<Format> {
    // ✅ FIX: Format methods consumen self, usar chaining
    let format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0xD9D9D9)) // Gris claro
        .set_border(rust_xlsxwriter::FormatBorder::Thin)
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_align(rust_xlsxwriter::FormatAlign::VerticalCenter);

    Ok(format)
}

/// Crea formato para celdas normales (solo bordes)
fn create_cell_format() -> ExportResult<Format> {
    // ✅ FIX: Chaining
    let format = Format::new()
        .set_border(rust_xlsxwriter::FormatBorder::Thin)
        .set_align(rust_xlsxwriter::FormatAlign::VerticalCenter);

    Ok(format)
}

// ... (formats remain same) ...

// ==========================================
// AUTOFIT DE COLUMNAS
// ==========================================

/// Ajusta automáticamente el ancho de las columnas
fn autofit_columns(
    worksheet: &mut Worksheet,
    headers: &[String],
    rows: &[HashMap<String, ExportValue>],
) -> ExportResult<()> {
    for (col_idx, header) in headers.iter().enumerate() {
        // Calcular ancho máximo para esta columna
        let max_width = calculate_column_width(header, rows);

        // Aplicar ancho (con límites razonables)
        let width = max_width.clamp(10.0, 50.0);

        worksheet.set_column_width(col_idx as u16, width).map_err(|e| {
            ExportError::XlsxFormatError(format!("Error ajustando columna {}: {}", col_idx, e))
        })?;
    }

    Ok(())
}

/// Calcula el ancho apropiado para una columna
fn calculate_column_width(header: &str, rows: &[HashMap<String, ExportValue>]) -> f64 {
    // Ancho del header
    let mut max_len = header.len();

    // Revisar todas las filas
    for row in rows {
        if let Some(value) = row.get(header) {
            let len = match value {
                ExportValue::Text(s) => s.len(),
                ExportValue::Number(n) => n.to_string().len(),
                ExportValue::Bool(b) => b.to_string().len(),
            };
            max_len = max_len.max(len);
        }
    }

    // Convertir a puntos de Excel (aproximación: 1 char ≈ 1.2 puntos)
    (max_len as f64) * 1.2
}

// ==========================================
// PATH MANAGEMENT
// ==========================================

/// Obtiene el path de salida (prioritiza target_path del usuario, luego Downloads)
fn get_output_path(filename: &str, target_path: Option<String>) -> ExportResult<PathBuf> {
    // ✅ Prioridad 1: Usar target_path si fue proporcionado por el usuario
    if let Some(path) = target_path {
        let path_buf = PathBuf::from(path);

        // Validar que el directorio padre existe
        if let Some(parent) = path_buf.parent() {
            if !parent.exists() {
                return Err(ExportError::FileSystemError(format!(
                    "El directorio no existe: {}",
                    parent.display()
                )));
            }
        }

        return Ok(path_buf);
    }

    // ✅ Prioridad 2: Usar el directorio de Downloads
    let filename = ensure_xlsx_extension(filename);

    if let Some(downloads_dir) = get_downloads_dir() {
        let path = downloads_dir.join(&filename);
        return Ok(path);
    }

    // ✅ Fallback: directorio temporal
    let temp_dir = std::env::temp_dir();
    Ok(temp_dir.join(&filename))
}

/// Asegura que el filename tenga extensión .xlsx
fn ensure_xlsx_extension(filename: &str) -> String {
    if filename.to_lowercase().ends_with(".xlsx") {
        filename.to_string()
    } else {
        format!("{}.xlsx", filename.trim_end_matches(".xlsx"))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_xlsx_extension() {
        assert_eq!(ensure_xlsx_extension("file.xlsx"), "file.xlsx");
        assert_eq!(ensure_xlsx_extension("file"), "file.xlsx");
        assert_eq!(ensure_xlsx_extension("file.XLS"), "file.XLS.xlsx");
        assert_eq!(ensure_xlsx_extension("file.txt"), "file.txt.xlsx");
    }

    #[test]
    fn test_calculate_column_width() {
        let header = "Name";
        let mut row1 = HashMap::new();
        row1.insert("Name".to_string(), ExportValue::Text("John".to_string()));

        let mut row2 = HashMap::new();
        row2.insert("Name".to_string(), ExportValue::Text("Alexander".to_string()));

        let rows = vec![row1, row2];

        let width = calculate_column_width(header, &rows);
        // "Alexander" = 9 chars * 1.2 = 10.8
        assert!(width > 10.0);
    }

    #[test]
    fn test_create_header_format() {
        let format = create_header_format();
        assert!(format.is_ok());
    }

    #[test]
    fn test_create_cell_format() {
        let format = create_cell_format();
        assert!(format.is_ok());
    }
}
