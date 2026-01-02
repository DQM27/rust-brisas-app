// ==========================================
// src/export/errors.rs
// ==========================================
// Tipos de error específicos del módulo export
// NUNCA panic!, siempre Result

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Clone)]
pub enum ExportError {
    // ==========================================
    // Errores de validación (input del usuario)
    // ==========================================
    #[error("Formato inválido: {0}")]
    InvalidFormat(String),

    #[error("Los headers no pueden estar vacíos")]
    EmptyHeaders,

    #[error("No hay datos para exportar")]
    EmptyData,

    #[error("Datos inválidos: {0}")]
    InvalidData(String),

    #[error("Columnas no coinciden: {0}")]
    MismatchedColumns(String),

    // ==========================================
    // Errores de configuración
    // ==========================================
    #[error("Orientación inválida: {0}")]
    InvalidOrientation(String),

    #[error("Delimitador inválido: {0}")]
    InvalidDelimiter(String),

    #[error("Título inválido: {0}")]
    InvalidTitle(String),

    // ==========================================
    // Errores de generación (PDF)
    // ==========================================
    #[error("Error compilando PDF: {0}")]
    TypstCompilationError(String),

    #[error("Error generando template: {0}")]
    TemplateGenerationError(String),

    // ==========================================
    // Errores de generación (Excel)
    // ==========================================
    #[error("Error escribiendo Excel: {0}")]
    XlsxWriteError(String),

    #[error("Error de formato Excel: {0}")]
    XlsxFormatError(String),

    // ==========================================
    // Errores de generación (CSV)
    // ==========================================
    #[error("Error escribiendo CSV: {0}")]
    CsvWriteError(String),

    // ==========================================
    // Errores de sistema
    // ==========================================
    #[error("Error de archivo: {0}")]
    FileSystemError(String),

    #[error("Error de I/O: {0}")]
    IoError(String),

    // ==========================================
    // Errores de Perfiles
    // ==========================================
    #[error("Perfil no encontrado")]
    ProfileNotFound,

    #[error("Error de datos de perfil: {0}")]
    ProfileSerializationError(String),

    #[error("Operación inválida en perfil: {0}")]
    InvalidProfileOperation(String),

    // ==========================================
    // Error catch-all
    // ==========================================
    #[error("Error desconocido: {0}")]
    Unknown(String),
}

// ==========================================
// Conversiones automáticas de errores externos
// ==========================================

// NOTA: No podemos derivar #[from] directamente porque std::io::Error no es Clone/Serialize
// y necesitamos que ExportError lo sea para enviarlo al frontend.
impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

// ==========================================
// Tipo Result conveniente
// ==========================================

pub type ExportResult<T> = Result<T, ExportError>;
