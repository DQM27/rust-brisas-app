// ==========================================
// src/models/blacklist_import.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

/// Representa una entrada de prueba de importación de lista negra
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistImportTest {
    pub id: String,
    pub cedula: String,
    
    // Nombres estructurados
    pub primer_nombre: String,
    pub segundo_nombre: Option<String>,
    
    // Apellidos estructurados
    pub primer_apellido: String,
    pub segundo_apellido: Option<String>,
    
    // Nombre completo (generado por BD)
    pub nombre_completo: String,
    
    // Datos del bloqueo
    pub empresa: String,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub observaciones: Option<String>,
    
    // Metadata de importación
    pub imported_at: String,
    pub imported_by: String,
    
    // Auditoría
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// ENUM DE ESTADO DE VALIDACIÓN
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    Valid,        // Validación automática exitosa
    NeedsReview,  // Requiere revisión manual (nombre compuesto detectado)
    Invalid,      // Datos inválidos
}

impl ValidationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ValidationStatus::Valid => "valid",
            ValidationStatus::NeedsReview => "needs_review",
            ValidationStatus::Invalid => "invalid",
        }
    }
    
    pub fn display(&self) -> &str {
        match self {
            ValidationStatus::Valid => "Válido",
            ValidationStatus::NeedsReview => "Requiere Revisión",
            ValidationStatus::Invalid => "Inválido",
        }
    }
}

// ==========================================
// DTOs DE ENTRADA (desde Excel)
// ==========================================

/// Representa una fila cruda del Excel antes de normalizar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelRowRaw {
    pub cedula: Option<String>,
    pub nombre_completo: Option<String>,
    pub empresa: Option<String>,
    pub motivo: Option<String>,
    pub fecha_inicio: Option<String>,
    pub observaciones: Option<String>,
}

/// Input para crear una entrada normalizada manualmente
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlacklistImportInput {
    pub cedula: String,
    pub primer_nombre: String,
    pub segundo_nombre: Option<String>,
    pub primer_apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: String,
    pub motivo_bloqueo: Option<String>,
    pub fecha_inicio_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

/// Input para actualizar una entrada que requiere revisión
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBlacklistImportInput {
    pub primer_nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub primer_apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub motivo_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

/// Response con datos normalizados y metadata de validación
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistImportResponse {
    pub id: String,
    pub cedula: String,
    pub primer_nombre: String,
    pub segundo_nombre: Option<String>,
    pub primer_apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa: String,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub observaciones: Option<String>,
    pub validation_status: ValidationStatus,
    pub validation_message: Option<String>,
    pub imported_at: String,
    pub imported_by: String,
}

impl From<BlacklistImportTest> for BlacklistImportResponse {
    fn from(entry: BlacklistImportTest) -> Self {
        Self {
            id: entry.id,
            cedula: entry.cedula,
            primer_nombre: entry.primer_nombre,
            segundo_nombre: entry.segundo_nombre,
            primer_apellido: entry.primer_apellido,
            segundo_apellido: entry.segundo_apellido,
            nombre_completo: entry.nombre_completo,
            empresa: entry.empresa,
            motivo_bloqueo: entry.motivo_bloqueo,
            fecha_inicio_bloqueo: entry.fecha_inicio_bloqueo,
            observaciones: entry.observaciones,
            validation_status: ValidationStatus::Valid,
            validation_message: None,
            imported_at: entry.imported_at,
            imported_by: entry.imported_by,
        }
    }
}

/// Response del proceso de importación completo
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultResponse {
    pub total_rows: usize,
    pub successful: usize,
    pub needs_review: usize,
    pub failed: usize,
    pub entries: Vec<BlacklistImportResponse>,
    pub errors: Vec<ImportError>,
}

/// Detalle de errores durante la importación
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportError {
    pub row_number: usize,
    pub cedula: Option<String>,
    pub error_type: String,
    pub message: String,
}

/// Response con estadísticas de la tabla de prueba
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistImportStats {
    pub total_entries: usize,
    pub by_empresa: Vec<EmpresaStats>,
    pub recent_imports: Vec<BlacklistImportResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaStats {
    pub empresa: String,
    pub count: usize,
}

/// DTO para preview de datos del Excel antes de importar
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelPreviewResponse {
    pub total_rows: usize,
    pub detected_columns: Vec<String>,
    pub sample_rows: Vec<ExcelRowRaw>,
    pub validation_summary: PreviewValidationSummary,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewValidationSummary {
    pub valid_rows: usize,
    pub needs_review_rows: usize,
    pub invalid_rows: usize,
}