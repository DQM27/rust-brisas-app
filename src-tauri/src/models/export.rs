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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "pdf" => Ok(ExportFormat::Pdf),
            "excel" => Ok(ExportFormat::Excel),
            "csv" => Ok(ExportFormat::Csv),
            _ => Err(format!("Formato desconocido: {}", s)),
        }
    }
}

/// Orientación de página (solo para PDF)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PageOrientation {
    Portrait,  // Vertical
    Landscape, // Horizontal
}

impl Default for PageOrientation {
    fn default() -> Self {
        PageOrientation::Landscape
    }
}

/// Delimitador para CSV
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CsvDelimiter {
    Comma,     // ,
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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "comma" | "," => Ok(CsvDelimiter::Comma),
            "semicolon" | ";" => Ok(CsvDelimiter::Semicolon),
            "tab" | "\\t" => Ok(CsvDelimiter::Tab),
            "pipe" | "|" => Ok(CsvDelimiter::Pipe),
            _ => Err(format!("Delimitador desconocido: {}", s)),
        }
    }
}

impl Default for CsvDelimiter {
    fn default() -> Self {
        CsvDelimiter::Comma
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
    pub title: Option<String>,       // Título del documento
    pub orientation: Option<String>, // "portrait" | "landscape"
    pub show_preview: Option<bool>,  // Si mostrar preview (PDF.js)
    pub template_id: Option<String>, // ID del template a usar
    pub font_size: Option<i32>,      // 8-20 pts
    pub font_family: Option<String>, // "Inter", "Arial", etc.

    // Opcionales para CSV
    pub delimiter: Option<String>, // "comma" | "semicolon" | "tab" | "pipe"
    pub include_bom: Option<bool>, // BOM para Excel UTF-8

    // Opcionales generales
    pub target_path: Option<String>, // Path absoluto donde guardar el archivo
}

/// Configuración específica para PDF
#[derive(Debug, Clone)]
pub struct PdfConfig {
    pub title: String,
    pub orientation: PageOrientation,
    pub headers: Vec<String>,
    pub show_preview: bool,
    pub template_id: Option<String>,
    pub font_size: i32,      // 8-20 pts
    pub font_family: String, // Nombre de la fuente
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
        }
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
        Self {
            filename: "export.xlsx".to_string(),
            headers: Vec::new(),
        }
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

/// Datos normalizados listos para exportar
#[derive(Debug, Clone)]
pub struct ExportData {
    pub format: ExportFormat,
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, String>>, // Todo normalizado a String
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
