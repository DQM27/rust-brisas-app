// ==========================================
// src/models/export.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==========================================
// ENUMS
// ==========================================

/// Formato de exportación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Pdf,
    Excel,
    Csv,
}

impl ExportFormat {
    pub fn as_str(&self) -> &str {
        match self {
            ExportFormat::Pdf => "pdf",
            ExportFormat::Excel => "excel",
            ExportFormat::Csv => "csv",
        }
    }
}

impl std::str::FromStr for ExportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pdf" => Ok(ExportFormat::Pdf),
            "excel" => Ok(ExportFormat::Excel),
            "csv" => Ok(ExportFormat::Csv),
            _ => Err(format!("Formato desconocido: {}", s)),
        }
    }
}

/// Orientación de página (solo para PDF)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PageOrientation {
    Portrait, // Vertical
    #[default]
    Landscape, // Horizontal
}

/// Delimitador para CSV
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CsvDelimiter {
    #[default]
    Comma, // ,
    Semicolon, // ;
    Tab,       // \t
    Pipe,      // |
}

impl CsvDelimiter {
    pub fn as_char(&self) -> char {
        match self {
            CsvDelimiter::Comma => ',',
            CsvDelimiter::Semicolon => ';',
            CsvDelimiter::Tab => '\t',
            CsvDelimiter::Pipe => '|',
        }
    }
}

impl std::str::FromStr for CsvDelimiter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "comma" | "," => Ok(CsvDelimiter::Comma),
            "semicolon" | ";" => Ok(CsvDelimiter::Semicolon),
            "tab" | "\\t" => Ok(CsvDelimiter::Tab),
            "pipe" | "|" => Ok(CsvDelimiter::Pipe),
            _ => Err(format!("Delimitador desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

/// Request principal de exportación
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub format: String,                                // "pdf" | "excel" | "csv"
    pub headers: Vec<String>,                          // Headers de columnas
    pub rows: Vec<HashMap<String, serde_json::Value>>, // Datos (flexible)

    // Opcionales para PDF
    pub title: Option<String>,        // Título del documento
    pub orientation: Option<String>,  // "portrait" | "landscape"
    pub show_preview: Option<bool>,   // Si mostrar preview (PDF.js)
    pub template_id: Option<String>,  // ID del template a usar
    pub font_size: Option<i32>,       // 8-20 pts
    pub font_family: Option<String>,  // "Inter", "Arial", etc.
    pub margin_top: Option<f32>,      // Márgen superior (cm)
    pub margin_bottom: Option<f32>,   // Márgen inferior (cm)
    pub margin_left: Option<f32>,     // Márgen izquierdo (cm)
    pub margin_right: Option<f32>,    // Márgen derecho (cm)
    pub banner_color: Option<String>, // Color del banner hex

    // Opcionales para CSV
    pub delimiter: Option<String>, // "comma" | "semicolon" | "tab" | "pipe"
    pub include_bom: Option<bool>, // BOM para Excel UTF-8

    // Opcionales generales
    pub target_path: Option<String>, // Path absoluto donde guardar el archivo
    pub generated_by: Option<String>, // Nombre del usuario que genera el reporte
}

/// Configuración específica para PDF
#[derive(Debug, Clone)]
pub struct PdfConfig {
    pub title: String,
    pub orientation: PageOrientation,
    pub headers: Vec<String>,
    pub show_preview: bool,
    pub template_id: Option<String>,
    pub font_size: i32,       // 8-20 pts
    pub font_family: String,  // Nombre de la fuente
    pub margin_top: f32,      // Márgen superior (cm)
    pub margin_bottom: f32,   // Márgen inferior (cm)
    pub margin_left: f32,     // Márgen izquierdo (cm)
    pub margin_right: f32,    // Márgen derecho (cm)
    pub banner_color: String, // Color hex del banner
    pub generated_by: String, // Nombre del usuario
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            title: "Reporte".to_string(),
            orientation: PageOrientation::Landscape,
            headers: Vec::new(),
            show_preview: false,
            template_id: None,
            font_size: 10,
            font_family: "Inter".to_string(),
            margin_top: 2.0,
            margin_bottom: 2.0,
            margin_left: 1.5,
            margin_right: 1.5,
            banner_color: "#059669".to_string(),
            generated_by: "".to_string(),
        }
    }
}

impl PdfConfig {
    /// Creates a new builder for PdfConfig with sensible defaults.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = PdfConfig::builder()
    ///     .title("Monthly Report")
    ///     .orientation(PageOrientation::Portrait)
    ///     .font_size(12)
    ///     .build();
    /// ```
    #[must_use]
    pub fn builder() -> PdfConfigBuilder {
        PdfConfigBuilder::default()
    }
}

/// Builder for `PdfConfig` using the fluent builder pattern.
///
/// Provides a convenient way to construct `PdfConfig` with
/// only the fields you need, using sensible defaults for the rest.
#[derive(Debug, Clone, Default)]
pub struct PdfConfigBuilder {
    config: PdfConfig,
}

impl PdfConfigBuilder {
    /// Sets the document title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Sets the page orientation.
    #[must_use]
    pub fn orientation(mut self, orientation: PageOrientation) -> Self {
        self.config.orientation = orientation;
        self
    }

    /// Sets the column headers.
    #[must_use]
    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.config.headers = headers;
        self
    }

    /// Enables or disables preview mode.
    #[must_use]
    pub fn show_preview(mut self, show: bool) -> Self {
        self.config.show_preview = show;
        self
    }

    /// Sets the template ID.
    #[must_use]
    pub fn template_id(mut self, id: impl Into<String>) -> Self {
        self.config.template_id = Some(id.into());
        self
    }

    /// Sets the font size (8-20 pts).
    #[must_use]
    pub fn font_size(mut self, size: i32) -> Self {
        self.config.font_size = size.clamp(8, 20);
        self
    }

    /// Sets the font family.
    #[must_use]
    pub fn font_family(mut self, family: impl Into<String>) -> Self {
        self.config.font_family = family.into();
        self
    }

    /// Sets all margins at once (in cm).
    #[must_use]
    pub fn margins(mut self, top: f32, bottom: f32, left: f32, right: f32) -> Self {
        self.config.margin_top = top;
        self.config.margin_bottom = bottom;
        self.config.margin_left = left;
        self.config.margin_right = right;
        self
    }

    /// Sets uniform margins on all sides (in cm).
    #[must_use]
    pub fn uniform_margins(mut self, margin: f32) -> Self {
        self.config.margin_top = margin;
        self.config.margin_bottom = margin;
        self.config.margin_left = margin;
        self.config.margin_right = margin;
        self
    }

    /// Sets the banner color (hex format, e.g., "#059669").
    #[must_use]
    pub fn banner_color(mut self, color: impl Into<String>) -> Self {
        self.config.banner_color = color.into();
        self
    }

    /// Sets the "generated by" user name.
    #[must_use]
    pub fn generated_by(mut self, name: impl Into<String>) -> Self {
        self.config.generated_by = name.into();
        self
    }

    /// Builds the final `PdfConfig`.
    #[must_use]
    pub fn build(self) -> PdfConfig {
        self.config
    }
}

/// Configuración específica para Excel
#[derive(Debug, Clone)]
pub struct ExcelConfig {
    pub filename: String,
    pub headers: Vec<String>,
}

impl Default for ExcelConfig {
    fn default() -> Self {
        Self { filename: "export.xlsx".to_string(), headers: Vec::new() }
    }
}

/// Configuración específica para CSV
#[derive(Debug, Clone)]
pub struct CsvConfig {
    pub filename: String,
    pub headers: Vec<String>,
    pub delimiter: CsvDelimiter,
    pub include_bom: bool, // UTF-8 BOM para Excel
}

impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            filename: "export.csv".to_string(),
            headers: Vec::new(),
            delimiter: CsvDelimiter::Comma,
            include_bom: true, // Por default activado para Excel
        }
    }
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

/// Respuesta de exportación PDF
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfExportResponse {
    pub success: bool,
    pub bytes: Option<Vec<u8>>,    // Bytes del PDF
    pub file_path: Option<String>, // Path si se guardó
    pub message: String,
}

/// Respuesta de exportación Excel
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelExportResponse {
    pub success: bool,
    pub file_path: Option<String>, // Path donde se guardó
    pub message: String,
}

/// Respuesta de exportación CSV
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvExportResponse {
    pub success: bool,
    pub file_path: Option<String>, // Path donde se guardó
    pub message: String,
}

/// Respuesta genérica unificada (para un solo comando)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResponse {
    pub success: bool,
    pub format: String,            // "pdf" | "excel" | "csv"
    pub bytes: Option<Vec<u8>>,    // Solo para PDF
    pub file_path: Option<String>, // Para todos
    pub message: String,
}

// ==========================================
// MODELO INTERNO NORMALIZADO
// ==========================================

/// Valor tipado para exportación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExportValue {
    Text(String),
    Number(f64),
    Bool(bool),
    // Por ahora las fechas las manejamos como strings ISO
    // En el futuro podríamos agregar Date(DateTime<Local>)
}

impl Default for ExportValue {
    fn default() -> Self {
        ExportValue::Text(String::new())
    }
}

impl std::fmt::Display for ExportValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportValue::Text(s) => write!(f, "{}", s),
            ExportValue::Number(n) => write!(f, "{}", n),
            ExportValue::Bool(b) => write!(f, "{}", b),
        }
    }
}

/// Datos normalizados listos para exportar
#[derive(Debug, Clone)]
pub struct ExportData {
    pub format: ExportFormat,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, ExportValue>>, // Tipado fuerte
    pub pdf_config: Option<PdfConfig>,
    pub excel_config: Option<ExcelConfig>,
    pub csv_config: Option<CsvConfig>,
    pub target_path: Option<String>,
}

// ==========================================
// PERFILES DE EXPORTACIÓN
// ==========================================

/// Diseño completo para PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfDesign {
    pub page_size: String,   // "us-letter" | "a4" | "legal"
    pub orientation: String, // "portrait" | "landscape"
    pub margin_x: f64,
    pub margin_x_unit: String, // "mm" | "cm" | "in" | "pt"
    pub margin_y: f64,
    pub margin_y_unit: String, // "mm" | "cm" | "in" | "pt"
    pub colors: PdfColors,
    pub fonts: PdfFonts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfColors {
    pub header_fill: String,
    pub header_text: String,
    pub row_text: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfFonts {
    pub family: String,
    pub size: i32,
    pub header_size: i32,
}

/// Opciones específicas para CSV
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvOptions {
    pub delimiter: String, // "comma" | "semicolon" | "tab" | "pipe"
    pub include_bom: bool,
}

/// Perfil de exportación unificado (incluye formato + diseño + opciones)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportProfile {
    pub id: String,
    pub name: String,
    pub format: String, // "pdf" | "excel" | "csv"
    pub is_default: bool,

    // Opciones comunes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_preview: Option<bool>,

    // Opciones PDF (incluye diseño completo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_design: Option<PdfDesign>,

    // Opciones CSV
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_options: Option<CsvOptions>,
}

impl ExportProfile {
    pub fn new(id: String, name: String, format: String) -> Self {
        Self {
            id,
            name,
            format,
            is_default: false,
            title: None,
            show_preview: None,
            pdf_design: None,
            csv_options: None,
        }
    }
}
