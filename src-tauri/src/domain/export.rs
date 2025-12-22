// ==========================================
// src/domain/export.rs
// ==========================================
// Capa de dominio: validaciones y reglas de negocio puras
// Sin dependencias de DB ni servicios externos

use crate::models::export::{CsvDelimiter, ExportFormat, ExportRequest, PageOrientation};
use std::collections::HashMap;

// ==========================================
// VALIDACIONES DE CAMPOS INDIVIDUALES
// ==========================================

/// Valida que el formato sea válido
pub fn validar_formato(formato: &str) -> Result<ExportFormat, String> {
    formato.parse()
}

/// Valida que los headers no estén vacíos
pub fn validar_headers(headers: &[String]) -> Result<(), String> {
    if headers.is_empty() {
        return Err("Los headers no pueden estar vacíos".to_string());
    }

    // Verificar que no haya headers vacíos
    for (idx, header) in headers.iter().enumerate() {
        if header.trim().is_empty() {
            return Err(format!("El header en posición {} está vacío", idx + 1));
        }
    }

    // Verificar headers duplicados
    let mut seen = std::collections::HashSet::new();
    for header in headers {
        let normalizado = normalizar_header(header);
        if !seen.insert(normalizado.clone()) {
            return Err(format!("Header duplicado: {}", header));
        }
    }

    Ok(())
}

/// Valida que haya datos para exportar
pub fn validar_rows(rows: &[HashMap<String, serde_json::Value>]) -> Result<(), String> {
    if rows.is_empty() {
        return Err("No hay datos para exportar".to_string());
    }

    // Límite razonable para evitar crashes de memoria
    const MAX_ROWS: usize = 100_000;
    if rows.len() > MAX_ROWS {
        return Err(format!(
            "Demasiadas filas. Máximo: {}, recibido: {}",
            MAX_ROWS,
            rows.len()
        ));
    }

    Ok(())
}

/// Valida que las filas tengan las columnas correctas
pub fn validar_consistencia_columnas(
    headers: &[String],
    rows: &[HashMap<String, serde_json::Value>],
) -> Result<(), String> {
    // ✅ FIX: Agregar underscore
    let _headers_normalizados: Vec<String> = headers.iter().map(|h| normalizar_header(h)).collect();

    for (idx, row) in rows.iter().enumerate() {
        if row.is_empty() {
            return Err(format!("La fila {} está vacía", idx + 1));
        }
    }

    Ok(())
}

/// Valida orientación de página (PDF)
pub fn validar_orientacion(orientacion: &str) -> Result<PageOrientation, String> {
    match orientacion.to_lowercase().as_str() {
        "portrait" | "vertical" => Ok(PageOrientation::Portrait),
        "landscape" | "horizontal" => Ok(PageOrientation::Landscape),
        _ => Err(format!("Orientación inválida: {}", orientacion)),
    }
}

/// Valida delimitador CSV
pub fn validar_delimitador(delimitador: &str) -> Result<CsvDelimiter, String> {
    delimitador.parse()
}

/// Valida título del documento
pub fn validar_titulo(titulo: &str) -> Result<(), String> {
    let limpio = titulo.trim();

    if limpio.is_empty() {
        return Err("El título no puede estar vacío".to_string());
    }

    if limpio.len() > 200 {
        return Err("El título no puede exceder 200 caracteres".to_string());
    }

    // Validar caracteres especiales que puedan romper el PDF
    if limpio.contains('\0') {
        return Err("El título contiene caracteres inválidos".to_string());
    }

    Ok(())
}

// ==========================================
// VALIDACIÓN COMPLETA DEL REQUEST
// ==========================================

/// Valida todos los campos del ExportRequest
pub fn validar_export_request(request: &ExportRequest) -> Result<(), String> {
    // 1. Validar formato
    validar_formato(&request.format)?;

    // 2. Validar headers
    validar_headers(&request.headers)?;

    // 3. Validar que haya datos
    validar_rows(&request.rows)?;

    // 4. Validar consistencia entre headers y rows
    validar_consistencia_columnas(&request.headers, &request.rows)?;

    // 5. Validar configuraciones opcionales según formato
    let formato: ExportFormat = request.format.parse()?;

    match formato {
        ExportFormat::Pdf => {
            // Validar orientación si viene
            if let Some(ref orient) = request.orientation {
                validar_orientacion(orient)?;
            }

            // Validar título si viene
            if let Some(ref titulo) = request.title {
                validar_titulo(titulo)?;
            }
        }
        ExportFormat::Csv => {
            // Validar delimitador si viene
            if let Some(ref delim) = request.delimiter {
                validar_delimitador(delim)?;
            }
        }
        ExportFormat::Excel => {
            // Excel no tiene validaciones especiales por ahora
        }
    }

    Ok(())
}

// ==========================================
// HELPERS DE NORMALIZACIÓN
// ==========================================

/// Normaliza un header (trim + lowercase para comparación)
pub fn normalizar_header(header: &str) -> String {
    header.trim().to_lowercase()
}

/// Normaliza un título (trim + sanitizar)
pub fn normalizar_titulo(titulo: &str) -> String {
    titulo.trim().to_string()
}

/// Convierte un valor JSON a String de forma segura
pub fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            // Para arrays/objetos, usar JSON string
            value.to_string()
        }
    }
}

use chrono::DateTime;

/// Normaliza una fila completa (convierte todos los valores a String)
pub fn normalizar_row(
    row: &HashMap<String, serde_json::Value>,
    headers: &[String],
) -> HashMap<String, String> {
    let mut normalized = HashMap::new();

    for header in headers {
        let raw_value = row
            .get(header)
            .map(json_value_to_string)
            .unwrap_or_default();

        // Intentar formatear si parece una fecha
        let value = try_format_date(&raw_value, header);

        normalized.insert(header.clone(), value);
    }

    normalized
}

/// Intenta formatear una cadena si es una fecha válida ISO 8601
fn try_format_date(value: &str, header: &str) -> String {
    // Optimización: si no tiene longitud de fecha mínima o separador, retornar original
    if value.len() < 10 || !value.contains('-') {
        return value.to_string();
    }

    // Intentar parsear como DateTime RFC3339 (formato estándar de JSON/JS)
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        let local_dt = dt.with_timezone(&chrono::Local);
        let header_lower = header.to_lowercase();

        if header_lower.contains("hora") {
            // Solo hora: 14:30
            return local_dt.format("%H:%M").to_string();
        } else if header_lower.contains("fecha") {
            // Solo fecha: 21/12/2025
            return local_dt.format("%d/%m/%Y").to_string();
        } else {
            // Fecha y Hora: 21/12/2025 14:30
            return local_dt.format("%d/%m/%Y %H:%M").to_string();
        }
    }

    value.to_string()
}

// ==========================================
// VALIDACIONES DE LÍMITES Y SEGURIDAD
// ==========================================

/// Valida que el tamaño total de datos sea razonable
pub fn validar_tamano_total(request: &ExportRequest) -> Result<(), String> {
    // Estimar tamaño aproximado en bytes
    let headers_size: usize = request.headers.iter().map(|h| h.len()).sum();

    let mut rows_size: usize = 0;
    for row in &request.rows {
        for value in row.values() {
            rows_size += json_value_to_string(value).len();
        }
    }

    let total_size = headers_size + rows_size;

    // Límite de 50MB para evitar crashes
    const MAX_SIZE: usize = 50 * 1024 * 1024; // 50MB
    if total_size > MAX_SIZE {
        return Err(format!(
            "Datos demasiado grandes. Máximo: {}MB, estimado: {}MB",
            MAX_SIZE / 1024 / 1024,
            total_size / 1024 / 1024
        ));
    }

    Ok(())
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_formato_valido() {
        assert!(validar_formato("pdf").is_ok());
        assert!(validar_formato("excel").is_ok());
        assert!(validar_formato("csv").is_ok());
        assert!(validar_formato("PDF").is_ok()); // Case insensitive
    }

    #[test]
    fn test_validar_formato_invalido() {
        assert!(validar_formato("word").is_err());
        assert!(validar_formato("").is_err());
    }

    #[test]
    fn test_validar_headers_validos() {
        let headers = vec!["Nombre".to_string(), "Email".to_string()];
        assert!(validar_headers(&headers).is_ok());
    }

    #[test]
    fn test_validar_headers_vacios() {
        let headers: Vec<String> = vec![];
        assert!(validar_headers(&headers).is_err());
    }

    #[test]
    fn test_validar_headers_duplicados() {
        let headers = vec!["Nombre".to_string(), "nombre".to_string()]; // Duplicado (case insensitive)
        assert!(validar_headers(&headers).is_err());
    }

    #[test]
    fn test_validar_rows_vacias() {
        let rows: Vec<HashMap<String, serde_json::Value>> = vec![];
        assert!(validar_rows(&rows).is_err());
    }

    #[test]
    fn test_validar_titulo() {
        assert!(validar_titulo("Reporte Mensual").is_ok());
        assert!(validar_titulo("").is_err());
        assert!(validar_titulo(&"A".repeat(201)).is_err());
    }

    #[test]
    fn test_normalizar_header() {
        assert_eq!(normalizar_header("  Nombre  "), "nombre");
        assert_eq!(normalizar_header("EMAIL"), "email");
    }

    #[test]
    fn test_json_value_to_string() {
        assert_eq!(json_value_to_string(&serde_json::json!(null)), "");
        assert_eq!(json_value_to_string(&serde_json::json!(true)), "true");
        assert_eq!(json_value_to_string(&serde_json::json!(42)), "42");
        assert_eq!(json_value_to_string(&serde_json::json!("test")), "test");
    }
}
