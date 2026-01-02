// ==========================================
// src/models/export.rs
// ==========================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --------------------------------------------------------------------------
// ENUMS
// --------------------------------------------------------------------------

/// Formatos soportados para la exportación de datos.
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

/// Orientación de la página para PDFs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PageOrientation {
    Portrait,
    #[default]
    Landscape,
}

/// Delimitadores soportados para archivos CSV.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CsvDelimiter {
    #[default]
    Comma,
    Semicolon,
    Tab,
    Pipe,
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

// --------------------------------------------------------------------------
// DTOs DE ENTRADA
// --------------------------------------------------------------------------

/// Solicitud principal para procesos de exportación.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub format: String,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, serde_json::Value>>,
    pub title: Option<String>,
    pub orientation: Option<String>,
    pub show_preview: Option<bool>,
    pub template_id: Option<String>,
    pub font_size: Option<i32>,
    pub font_family: Option<String>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub banner_color: Option<String>,
    pub delimiter: Option<String>,
    pub include_bom: Option<bool>,
    pub target_path: Option<String>,
    pub generated_by: Option<String>,
}

impl Default for ExportRequest {
    fn default() -> Self {
        Self {
            format: "pdf".to_string(), // Default safe
            headers: Vec::new(),
            rows: Vec::new(),
            title: None,
            orientation: None,
            show_preview: None,
            template_id: None,
            font_size: None,
            font_family: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            banner_color: None,
            delimiter: None,
            include_bom: None,
            target_path: None,
            generated_by: None,
        }
    }
}

/// Configuración específica para generación de PDF.
#[derive(Debug, Clone)]
pub struct PdfConfig {
    pub title: String,
    pub orientation: PageOrientation,
    pub headers: Vec<String>,
    pub show_preview: bool,
    pub template_id: Option<String>,
    pub font_size: i32,
    pub font_family: String,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub banner_color: String,
    pub generated_by: String,
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

/// Configuración para exportación a Excel.
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

/// Configuración para exportación a CSV.
#[derive(Debug, Clone)]
pub struct CsvConfig {
    pub filename: String,
    pub headers: Vec<String>,
    pub delimiter: CsvDelimiter,
    pub include_bom: bool,
}

impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            filename: "export.csv".to_string(),
            headers: Vec::new(),
            delimiter: CsvDelimiter::Comma,
            include_bom: true,
        }
    }
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfExportResponse {
    pub success: bool,
    pub bytes: Option<Vec<u8>>,
    pub file_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelExportResponse {
    pub success: bool,
    pub file_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvExportResponse {
    pub success: bool,
    pub file_path: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResponse {
    pub success: bool,
    pub format: String,
    pub bytes: Option<Vec<u8>>,
    pub file_path: Option<String>,
    pub message: String,
}

// --------------------------------------------------------------------------
// MODELO INTERNO NORMALIZADO
// --------------------------------------------------------------------------

/// Valor polimórfico para celdas de exportación.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExportValue {
    Text(String),
    Number(f64),
    Bool(bool),
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

/// Contenedor agnóstico de datos para procesadores de exportación.
#[derive(Debug, Clone)]
pub struct ExportData {
    pub format: ExportFormat,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, ExportValue>>,
    pub pdf_config: Option<PdfConfig>,
    pub excel_config: Option<ExcelConfig>,
    pub csv_config: Option<CsvConfig>,
    pub target_path: Option<String>,
}

// --------------------------------------------------------------------------
// PERFILES DE EXPORTACIÓN
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfDesign {
    pub page_size: String,
    pub orientation: String,
    pub margin_x: f64,
    pub margin_x_unit: String,
    pub margin_y: f64,
    pub margin_y_unit: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvOptions {
    pub delimiter: String,
    pub include_bom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportProfile {
    pub id: String,
    pub name: String,
    pub format: String,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_design: Option<PdfDesign>,
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
