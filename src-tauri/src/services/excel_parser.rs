// ==========================================
// src/services/excel_parser.rs
// ==========================================
// Servicio para parsear y normalizar archivos Excel
// Maneja detección de nombres compuestos y validación

use calamine::{Reader, open_workbook_auto, Data};

use crate::models::blacklist_import::{
    ExcelRowRaw,
    BlacklistImportResponse,
    ImportResultResponse,
    ImportError,
    ValidationStatus,
    ExcelPreviewResponse,
    PreviewValidationSummary,
};
use crate::domain::blacklist_import::{
    validar_cedula,
    validar_empresa,
    normalizar_cedula,
    normalizar_texto,
    capitalizar_nombre,
    requiere_validacion_manual,
    separar_nombre_automatico,
};

// ==========================================
// ESTRUCTURAS AUXILIARES
// ==========================================

/// Representa una fila del Excel después de parseo inicial
#[derive(Debug, Clone)]
pub struct ParsedExcelRow {
    pub row_number: usize,
    pub cedula: Option<String>,
    pub nombre_completo: Option<String>,
    pub empresa: Option<String>,
    pub motivo: Option<String>,
    pub fecha_inicio: Option<String>,
    pub observaciones: Option<String>,
}

/// Resultado de normalización de una fila
#[derive(Debug, Clone)]
pub struct NormalizedRow {
    pub row_number: usize,
    pub cedula: String,
    pub primer_nombre: String,
    pub segundo_nombre: Option<String>,
    pub primer_apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: String,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub observaciones: Option<String>,
    pub validation_status: ValidationStatus,
    pub validation_message: Option<String>,
}

// ==========================================
// CONFIGURACIÓN DE COLUMNAS ESPERADAS
// ==========================================

/// Mapeo de índices de columnas (puede ser configurable)
pub struct ColumnMapping {
    pub cedula: usize,
    pub nombre_completo: usize,
    pub empresa: usize,
    pub motivo: Option<usize>,
    pub fecha_inicio: Option<usize>,
    pub observaciones: Option<usize>,
}

impl Default for ColumnMapping {
    fn default() -> Self {
        Self {
            cedula: 0,           // Columna A
            nombre_completo: 1,  // Columna B
            empresa: 2,          // Columna C
            motivo: Some(3),     // Columna D (opcional)
            fecha_inicio: Some(4), // Columna E (opcional)
            observaciones: Some(5), // Columna F (opcional)
        }
    }
}

// ==========================================
// LECTURA DEL ARCHIVO EXCEL
// ==========================================

/// Lee un archivo Excel y retorna las filas parseadas
pub fn read_excel_file(
    file_path: &str,
    mapping: &ColumnMapping,
    skip_header: bool,
) -> Result<Vec<ParsedExcelRow>, String> {
    // Abrir el archivo Excel
    let mut workbook = open_workbook_auto(file_path)
        .map_err(|e| format!("Error abriendo archivo Excel: {}", e))?;

    // Obtener la primera hoja
    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err("El archivo Excel no contiene hojas".to_string());
    }

    let sheet_name = &sheet_names[0];
    
    // worksheet_range devuelve Result<Range, Error>
    let range = workbook
        .worksheet_range(sheet_name)
        .map_err(|e| format!("Error leyendo hoja '{}': {}", sheet_name, e))?;

    let mut rows = Vec::new();
    let start_row = if skip_header { 1 } else { 0 };

    // Iterar sobre las filas
    for (idx, row) in range.rows().enumerate().skip(start_row) {
        let row_number = idx + 1;

        // Extraer valores de las columnas según el mapping
        let cedula = get_cell_value(row, mapping.cedula);
        let nombre_completo = get_cell_value(row, mapping.nombre_completo);
        let empresa = get_cell_value(row, mapping.empresa);
        let motivo = mapping.motivo.and_then(|i| get_cell_value(row, i));
        let fecha_inicio = mapping.fecha_inicio.and_then(|i| get_cell_value(row, i));
        let observaciones = mapping.observaciones.and_then(|i| get_cell_value(row, i));

        // Solo agregar filas que tengan al menos cédula o nombre
        if cedula.is_some() || nombre_completo.is_some() {
            rows.push(ParsedExcelRow {
                row_number,
                cedula,
                nombre_completo,
                empresa,
                motivo,
                fecha_inicio,
                observaciones,
            });
        }
    }

    if rows.is_empty() {
        return Err("El archivo Excel no contiene datos válidos".to_string());
    }

    Ok(rows)
}

/// Extrae el valor de una celda como String
fn get_cell_value(row: &[Data], index: usize) -> Option<String> {
    row.get(index).and_then(|cell| {
        match cell {
            Data::Empty => None,
            Data::String(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }
            Data::Float(f) => Some(f.to_string()),
            Data::Int(i) => Some(i.to_string()),
            Data::Bool(b) => Some(b.to_string()),
            Data::DateTime(dt) => {
                // Excel almacena fechas como f64 (días desde 1899-12-30)
                // Convertir a formato YYYY-MM-DD
                let days_since_1900 = dt.as_f64().floor() as i64;
                let base = chrono::NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
                
                base.checked_add_signed(chrono::Duration::days(days_since_1900))
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .or_else(|| Some(format!("{}", dt.as_f64()))) // Fallback
            }
            Data::DateTimeIso(dt) => Some(dt.to_string()),
            Data::DurationIso(d) => Some(d.to_string()),
            Data::Error(e) => {
                eprintln!("Error en celda: {:?}", e);
                None
            }
        }
    })
}
// ==========================================
// NORMALIZACIÓN DE FILAS
// ==========================================

/// Normaliza una fila del Excel aplicando validaciones y defaults
pub fn normalize_excel_row(
    parsed_row: &ParsedExcelRow,
) -> Result<NormalizedRow, ImportError> {
    let row_number = parsed_row.row_number;

    // Validar campos obligatorios
    let cedula_raw = parsed_row.cedula.as_ref().ok_or_else(|| ImportError {
        row_number,
        cedula: None,
        error_type: "CAMPO_FALTANTE".to_string(),
        message: "La cédula es obligatoria".to_string(),
    })?;

    let nombre_completo_raw = parsed_row.nombre_completo.as_ref().ok_or_else(|| ImportError {
        row_number,
        cedula: Some(cedula_raw.clone()),
        error_type: "CAMPO_FALTANTE".to_string(),
        message: "El nombre completo es obligatorio".to_string(),
    })?;

    let empresa_raw = parsed_row.empresa.as_ref().ok_or_else(|| ImportError {
        row_number,
        cedula: Some(cedula_raw.clone()),
        error_type: "CAMPO_FALTANTE".to_string(),
        message: "La empresa es obligatoria".to_string(),
    })?;

    // Validar y normalizar cédula
    validar_cedula(cedula_raw).map_err(|msg| ImportError {
        row_number,
        cedula: Some(cedula_raw.clone()),
        error_type: "CEDULA_INVALIDA".to_string(),
        message: msg,
    })?;
    let cedula = normalizar_cedula(cedula_raw);

    // Validar empresa
    validar_empresa(empresa_raw).map_err(|msg| ImportError {
        row_number,
        cedula: Some(cedula.clone()),
        error_type: "EMPRESA_INVALIDA".to_string(),
        message: msg,
    })?;
    let empresa = normalizar_texto(empresa_raw);

    // Normalizar nombre completo y separar en partes
    let nombre_normalizado = capitalizar_nombre(nombre_completo_raw);

    // Detectar si requiere validación manual
    let requiere_validacion = requiere_validacion_manual(&nombre_normalizado);

    let (primer_nombre, segundo_nombre, primer_apellido, segundo_apellido, validation_status, validation_message) = 
        if requiere_validacion {
            // Nombre con preposiciones - requiere validación manual
            (
                nombre_normalizado.clone(),
                None,
                String::from("REQUIERE_VALIDACION"),
                None,
                ValidationStatus::NeedsReview,
                Some(format!("El nombre '{}' contiene preposiciones y requiere validación manual", nombre_normalizado)),
            )
        } else {
            // Intentar separar automáticamente
            match separar_nombre_automatico(&nombre_normalizado) {
                Ok((p1, s1, a1, s2)) => {
                    (p1, s1, a1, s2, ValidationStatus::Valid, None)
                }
                Err(msg) => {
                    // Error en separación automática (ej: 5+ palabras)
                    (
                        nombre_normalizado.clone(),
                        None,
                        String::from("REQUIERE_VALIDACION"),
                        None,
                        ValidationStatus::NeedsReview,
                        Some(msg),
                    )
                }
            }
        };

    // Aplicar defaults para campos opcionales
    let motivo_bloqueo = parsed_row.motivo
        .as_ref()
        .filter(|m| !m.trim().is_empty())
        .map(|m| m.trim().to_string())
        .unwrap_or_else(|| "No especificado".to_string());

    let fecha_inicio_bloqueo = parsed_row.fecha_inicio
        .as_ref()
        .filter(|f| !f.trim().is_empty())
        .map(|f| f.trim().to_string())
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    let observaciones = parsed_row.observaciones
        .as_ref()
        .filter(|o| !o.trim().is_empty())
        .map(|o| o.trim().to_string());

    Ok(NormalizedRow {
        row_number,
        cedula,
        primer_nombre,
        segundo_nombre,
        primer_apellido,
        segundo_apellido,
        empresa,
        motivo_bloqueo,
        fecha_inicio_bloqueo,
        observaciones,
        validation_status,
        validation_message,
    })
}

/// Normaliza un batch completo de filas
pub fn normalize_excel_rows(
    parsed_rows: Vec<ParsedExcelRow>,
) -> ImportResultResponse {
    let total_rows = parsed_rows.len();
    let mut successful = 0;
    let mut needs_review = 0;
    let mut failed = 0;
    let mut entries = Vec::new();
    let mut errors = Vec::new();

    for parsed_row in parsed_rows {
        match normalize_excel_row(&parsed_row) {
            Ok(normalized) => {
                match normalized.validation_status {
                    ValidationStatus::Valid => {
                        successful += 1;
                    }
                    ValidationStatus::NeedsReview => {
                        needs_review += 1;
                    }
                    ValidationStatus::Invalid => {
                        failed += 1;
                    }
                }

                // Convertir NormalizedRow a BlacklistImportResponse
                let response = BlacklistImportResponse {
                    id: String::new(), // Se generará al insertar
                    cedula: normalized.cedula,
                    primer_nombre: normalized.primer_nombre,
                    segundo_nombre: normalized.segundo_nombre,
                    primer_apellido: normalized.primer_apellido,
                    segundo_apellido: normalized.segundo_apellido,
                    nombre_completo: String::new(), // Se genera en BD
                    empresa: normalized.empresa,
                    motivo_bloqueo: normalized.motivo_bloqueo,
                    fecha_inicio_bloqueo: normalized.fecha_inicio_bloqueo,
                    observaciones: normalized.observaciones,
                    validation_status: normalized.validation_status,
                    validation_message: normalized.validation_message,
                    imported_at: String::new(), // Se genera al insertar
                    imported_by: String::new(), // Se asigna al insertar
                };

                entries.push(response);
            }
            Err(error) => {
                failed += 1;
                errors.push(error);
            }
        }
    }

    ImportResultResponse {
        total_rows,
        successful,
        needs_review,
        failed,
        entries,
        errors,
    }
}

// ==========================================
// PREVIEW DEL EXCEL
// ==========================================

/// Genera un preview del Excel sin insertar en BD
pub fn preview_excel_file(
    file_path: &str,
    mapping: &ColumnMapping,
    skip_header: bool,
    max_preview_rows: usize,
) -> Result<ExcelPreviewResponse, String> {
    // Leer archivo
    let parsed_rows = read_excel_file(file_path, mapping, skip_header)?;
    
    let total_rows = parsed_rows.len();

    // Detectar columnas
    let detected_columns = vec![
        "Cédula".to_string(),
        "Nombre Completo".to_string(),
        "Empresa".to_string(),
        "Motivo".to_string(),
        "Fecha Inicio".to_string(),
        "Observaciones".to_string(),
    ];

    // Tomar muestra de filas para preview
    let sample_rows: Vec<ExcelRowRaw> = parsed_rows
        .iter()
        .take(max_preview_rows)
        .map(|row| ExcelRowRaw {
            cedula: row.cedula.clone(),
            nombre_completo: row.nombre_completo.clone(),
            empresa: row.empresa.clone(),
            motivo: row.motivo.clone(),
            fecha_inicio: row.fecha_inicio.clone(),
            observaciones: row.observaciones.clone(),
        })
        .collect();

    // Validar todas las filas para generar summary
    let validation_result = normalize_excel_rows(parsed_rows);

    let validation_summary = PreviewValidationSummary {
        valid_rows: validation_result.successful,
        needs_review_rows: validation_result.needs_review,
        invalid_rows: validation_result.failed,
    };

    Ok(ExcelPreviewResponse {
        total_rows,
        detected_columns,
        sample_rows,
        validation_summary,
    })
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_excel_row_valid() {
        let parsed = ParsedExcelRow {
            row_number: 1,
            cedula: Some("1-2345-6789".to_string()),
            nombre_completo: Some("juan carlos perez gomez".to_string()),
            empresa: Some("Constructora XYZ".to_string()),
            motivo: None,
            fecha_inicio: None,
            observaciones: None,
        };

        let result = normalize_excel_row(&parsed);
        assert!(result.is_ok());

        let normalized = result.unwrap();
        assert_eq!(normalized.cedula, "1-2345-6789");
        assert_eq!(normalized.primer_nombre, "Juan");
        assert_eq!(normalized.segundo_nombre, Some("Carlos".to_string()));
        assert_eq!(normalized.primer_apellido, "Perez");
        assert_eq!(normalized.segundo_apellido, Some("Gomez".to_string()));
        assert_eq!(normalized.motivo_bloqueo, "No especificado");
        assert_eq!(normalized.validation_status, ValidationStatus::Valid);
    }

    #[test]
    fn test_normalize_excel_row_needs_review() {
        let parsed = ParsedExcelRow {
            row_number: 1,
            cedula: Some("1-2345-6789".to_string()),
            nombre_completo: Some("maria de los angeles rodriguez".to_string()),
            empresa: Some("Empresa ABC".to_string()),
            motivo: None,
            fecha_inicio: None,
            observaciones: None,
        };

        let result = normalize_excel_row(&parsed);
        assert!(result.is_ok());

        let normalized = result.unwrap();
        assert_eq!(normalized.validation_status, ValidationStatus::NeedsReview);
        assert!(normalized.validation_message.is_some());
    }

    #[test]
    fn test_normalize_excel_row_missing_cedula() {
        let parsed = ParsedExcelRow {
            row_number: 1,
            cedula: None,
            nombre_completo: Some("Juan Perez".to_string()),
            empresa: Some("Empresa ABC".to_string()),
            motivo: None,
            fecha_inicio: None,
            observaciones: None,
        };

        let result = normalize_excel_row(&parsed);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.error_type, "CAMPO_FALTANTE");
    }
}