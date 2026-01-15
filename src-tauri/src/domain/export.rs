/// Capa de Dominio: Reglas para Exportación de Datos.
///
/// Este módulo define la lógica pura para la preparación y validación de datos
/// destinados a exportación en diversos formatos (PDF, CSV, Excel).
/// No tiene dependencias de infraestructura ni de base de datos.
use crate::models::export::{
    CsvDelimiter, ExportFormat, ExportRequest, ExportValue, PageOrientation, PdfConfig,
};
use chrono::DateTime;
use std::borrow::Cow;
use std::collections::HashMap;

// --------------------------------------------------------------------------
// BUILDER (Lógica de Construcción)
// --------------------------------------------------------------------------

/// Builder para facilitar la creación de `PdfConfig`.
/// Mantenido en la capa de dominio para preservar el modelo anémico.
#[derive(Debug, Clone, Default)]
pub struct PdfConfigBuilder {
    config: PdfConfig,
}

impl PdfConfigBuilder {
    /// Inicia un nuevo builder con la configuración por defecto.
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    #[must_use]
    pub const fn orientation(mut self, orientation: PageOrientation) -> Self {
        self.config.orientation = orientation;
        self
    }

    #[must_use]
    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.config.headers = headers;
        self
    }

    #[must_use]
    pub const fn show_preview(mut self, show: bool) -> Self {
        self.config.show_preview = show;
        self
    }

    #[must_use]
    pub fn template_id(mut self, id: impl Into<String>) -> Self {
        self.config.template_id = Some(id.into());
        self
    }

    #[must_use]
    pub fn font_size(mut self, size: i32) -> Self {
        self.config.font_size = size.clamp(8, 20);
        self
    }

    #[must_use]
    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.config.font_family = family.into();
        self
    }

    #[must_use]
    pub const fn margins(mut self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.config.margin_top = top;
        self.config.margin_bottom = bottom;
        self.config.margin_left = left;
        self.config.margin_right = right;
        self
    }

    #[must_use]
    pub const fn uniform_margins(mut self, margin: f32) -> Self {
        self.config.margin_top = margin;
        self.config.margin_bottom = margin;
        self.config.margin_left = margin;
        self.config.margin_right = margin;
        self
    }

    #[must_use]
    pub fn banner_color(mut self, color: impl Into<String>) -> Self {
        self.config.banner_color = color.into();
        self
    }

    #[must_use]
    pub fn generated_by(mut self, name: impl Into<String>) -> Self {
        self.config.generated_by = name.into();
        self
    }

    #[must_use]
    pub fn build(self) -> PdfConfig {
        self.config
    }
}

// --------------------------------------------------------------------------
// CONSTANTES DE LÍMITES Y SEGURIDAD
// --------------------------------------------------------------------------

/// Número máximo de filas permitidas en una exportación.
pub const MAX_ROWS: usize = 100_000;

/// Tamaño máximo estimado de datos en memoria (50MB).
pub const MAX_SIZE: usize = 50 * 1024 * 1024;

/// Longitud máxima del título del documento.
pub const TITULO_MAX_LEN: usize = 200;

/// Caracteres prohibidos en títulos (prevención de inyecciones y corrupción).
const CHARS_PROHIBIDOS: &[char] = &['<', '>', '{', '}', '|', '\\', '^', '~', '[', ']', '`', '\0'];

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que la cadena represente un formato de exportación soportado.
pub fn validar_formato(formato: &str) -> Result<ExportFormat, String> {
    formato.parse()
}

/// Garantiza que la lista de encabezados sea válida y no contenga duplicados.
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
            return Err(format!("Header duplicado: {header}"));
        }
    }

    Ok(())
}

/// Valida que el conjunto de datos a exportar cumpla con los límites de seguridad.
pub fn validar_rows(rows: &[HashMap<String, serde_json::Value>]) -> Result<(), String> {
    if rows.is_empty() {
        return Err("No hay datos para exportar".to_string());
    }

    if rows.len() > MAX_ROWS {
        return Err(format!("Demasiadas filas. Máximo: {}, recibido: {}", MAX_ROWS, rows.len()));
    }

    Ok(())
}

/// Verifica que todas las filas contengan información procesable.
pub fn validar_consistencia_columnas(
    _headers: &[String],
    rows: &[HashMap<String, serde_json::Value>],
) -> Result<(), String> {
    for (idx, row) in rows.iter().enumerate() {
        if row.is_empty() {
            return Err(format!("La fila {} está vacía", idx + 1));
        }
    }

    Ok(())
}

/// Valida la orientación de página solicitada para formatos visuales (PDF).
pub fn validar_orientacion(orientacion: &str) -> Result<PageOrientation, String> {
    match orientacion.to_lowercase().as_str() {
        "portrait" | "vertical" => Ok(PageOrientation::Portrait),
        "landscape" | "horizontal" => Ok(PageOrientation::Landscape),
        _ => Err(format!("Orientación inválida: {orientacion}")),
    }
}

/// Valida el delimitador de campos para formatos de texto plano (CSV).
pub fn validar_delimitador(delimitador: &str) -> Result<CsvDelimiter, String> {
    delimitador.parse()
}

/// Valida que el título del documento cumpla con los requisitos estéticos y técnicos.
pub fn validar_titulo(titulo: &str) -> Result<(), String> {
    let limpio = titulo.trim();

    if limpio.is_empty() {
        return Err("El título no puede estar vacío".to_string());
    }

    if limpio.len() > TITULO_MAX_LEN {
        return Err(format!("El título no puede exceder {TITULO_MAX_LEN} caracteres"));
    }

    // Validar caracteres prohibidos
    if limpio.chars().any(|c| CHARS_PROHIBIDOS.contains(&c)) {
        return Err("El título contiene caracteres no permitidos".to_string());
    }

    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIÓN INTEGRAL DEL REQUEST
// --------------------------------------------------------------------------

/// Realiza una auditoría completa de una solicitud de exportación.
pub fn validar_export_request(request: &ExportRequest) -> Result<(), String> {
    validar_formato(&request.format)?;
    validar_headers(&request.headers)?;
    validar_rows(&request.rows)?;
    validar_consistencia_columnas(&request.headers, &request.rows)?;

    let formato: ExportFormat = request.format.parse()?;

    match formato {
        ExportFormat::Pdf => {
            if let Some(ref orient) = request.orientation {
                validar_orientacion(orient)?;
            }
            if let Some(ref titulo) = request.title {
                validar_titulo(titulo)?;
            }
        }
        ExportFormat::Csv => {
            if let Some(ref delim) = request.delimiter {
                validar_delimitador(delim)?;
            }
        }
        ExportFormat::Excel => {}
    }

    Ok(())
}

// --------------------------------------------------------------------------
// NORMALIZACIÓN Y TRANSFORMACIÓN
// --------------------------------------------------------------------------

/// Normaliza un encabezado para comparaciones internas (trim + lowercase).
pub fn normalizar_header(header: &str) -> String {
    header.trim().to_lowercase()
}

/// Limpia un título de espacios en blanco innecesarios.
pub fn normalizar_titulo(titulo: &str) -> String {
    titulo.trim().to_string()
}

/// Convierte un valor JSON genérico a un valor tipado de exportación.
pub fn normalizar_value(value: &serde_json::Value, header: &str) -> ExportValue {
    match value {
        serde_json::Value::Null => ExportValue::Text(String::new()),
        serde_json::Value::Bool(b) => ExportValue::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                ExportValue::Number(f)
            } else {
                ExportValue::Number(0.0)
            }
        }
        serde_json::Value::String(s) => {
            let formatted = try_format_date(s, header);
            ExportValue::Text(formatted)
        }
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            ExportValue::Text(value.to_string())
        }
    }
}

/// Convierte un valor JSON a su representación en cadena de forma eficiente.
pub fn json_value_to_string(value: &serde_json::Value) -> Cow<'_, str> {
    match value {
        serde_json::Value::Null => Cow::Borrowed(""),
        serde_json::Value::Bool(b) => Cow::Owned(b.to_string()),
        serde_json::Value::Number(n) => Cow::Owned(n.to_string()),
        serde_json::Value::String(s) => Cow::Borrowed(s.as_str()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => Cow::Owned(value.to_string()),
    }
}

/// Mapea una fila completa de datos crudos a una fila normalizada.
pub fn normalizar_row(
    row: &HashMap<String, serde_json::Value>,
    headers: &[String],
) -> HashMap<String, ExportValue> {
    let mut normalized = HashMap::new();

    for header in headers {
        let raw_value = row.get(header).unwrap_or(&serde_json::Value::Null);
        let value = normalizar_value(raw_value, header);
        normalized.insert(header.clone(), value);
    }

    normalized
}

/// Intenta aplicar formatos de fecha amigables si detecta un ISO 8601.
fn try_format_date(value: &str, header: &str) -> String {
    if value.len() < 10 || !value.contains('-') {
        return value.to_string();
    }

    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        let local_dt = dt.with_timezone(&chrono::Local);
        let header_lower = header.to_lowercase();

        if header_lower.contains("hora") {
            return local_dt.format("%H:%M").to_string();
        } else if header_lower.contains("fecha") {
            return local_dt.format("%d/%m/%Y").to_string();
        }
        return local_dt.format("%d/%m/%Y %H:%M").to_string();
    }

    value.to_string()
}

// --------------------------------------------------------------------------
// LÍMITES Y SEGURIDAD OPERATIVA
// --------------------------------------------------------------------------

/// Estima el tamaño en memoria para prevenir desbordamientos durante la generación.
pub fn validar_tamano_total(request: &ExportRequest) -> Result<(), String> {
    let headers_size: usize = request.headers.iter().map(std::string::String::len).sum();

    let mut rows_size: usize = 0;
    for row in &request.rows {
        for value in row.values() {
            rows_size += json_value_to_string(value).len();
        }
    }

    let total_size = headers_size + rows_size;

    if total_size > MAX_SIZE {
        return Err(format!(
            "Datos demasiado grandes. Máximo: {}MB, estimado: {}MB",
            MAX_SIZE / 1024 / 1024,
            total_size / 1024 / 1024
        ));
    }

    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

