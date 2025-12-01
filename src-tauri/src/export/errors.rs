// ==========================================
// src/export/errors.rs
// ==========================================
// Tipos de error específicos del módulo export
// NUNCA panic!, siempre Result

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum ExportError {
    // ==========================================
    // Errores de validación (input del usuario)
    // ==========================================
    InvalidFormat(String),
    EmptyHeaders,
    EmptyData,
    InvalidData(String),
    MismatchedColumns(String),
    
    // ==========================================
    // Errores de configuración
    // ==========================================
    InvalidOrientation(String),
    InvalidDelimiter(String),
    InvalidTitle(String),
    
    // ==========================================
    // Errores de generación (PDF)
    // ==========================================
    TypstCompilationError(String),
    TemplateGenerationError(String),
    
    // ==========================================
    // Errores de generación (Excel)
    // ==========================================
    XlsxWriteError(String),
    XlsxFormatError(String),
    
    // ==========================================
    // Errores de generación (CSV)
    // ==========================================
    CsvWriteError(String),
    
    // ==========================================
    // Errores de sistema
    // ==========================================
    FileSystemError(String),
    IoError(String),
    
    // ==========================================
    // Error catch-all
    // ==========================================
    Unknown(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Validación
            Self::InvalidFormat(msg) => write!(f, "Formato inválido: {}", msg),
            Self::EmptyHeaders => write!(f, "Los headers no pueden estar vacíos"),
            Self::EmptyData => write!(f, "No hay datos para exportar"),
            Self::InvalidData(msg) => write!(f, "Datos inválidos: {}", msg),
            Self::MismatchedColumns(msg) => write!(f, "Columnas no coinciden: {}", msg),
            
            // Configuración
            Self::InvalidOrientation(msg) => write!(f, "Orientación inválida: {}", msg),
            Self::InvalidDelimiter(msg) => write!(f, "Delimitador inválido: {}", msg),
            Self::InvalidTitle(msg) => write!(f, "Título inválido: {}", msg),
            
            // PDF
            Self::TypstCompilationError(msg) => write!(f, "Error compilando PDF: {}", msg),
            Self::TemplateGenerationError(msg) => write!(f, "Error generando template: {}", msg),
            
            // Excel
            Self::XlsxWriteError(msg) => write!(f, "Error escribiendo Excel: {}", msg),
            Self::XlsxFormatError(msg) => write!(f, "Error de formato Excel: {}", msg),
            
            // CSV
            Self::CsvWriteError(msg) => write!(f, "Error escribiendo CSV: {}", msg),
            
            // Sistema
            Self::FileSystemError(msg) => write!(f, "Error de archivo: {}", msg),
            Self::IoError(msg) => write!(f, "Error de I/O: {}", msg),
            
            // Unknown
            Self::Unknown(msg) => write!(f, "Error desconocido: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {}

// ==========================================
// Conversiones automáticas de errores externos
// ==========================================

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::IoError(err.to_string())
    }
}

// ==========================================
// Tipo Result conveniente
// ==========================================

pub type ExportResult<T> = Result<T, ExportError>;