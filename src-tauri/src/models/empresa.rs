// ==========================================
// src/models/empresa.rs
// ==========================================

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Entidad empresarial (Proveedores, Contratistas, Clientes).
/// Actúa como agrupación para personal externo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Empresa {
    pub id: RecordId,
    pub nombre: String,
    pub direccion: Option<String>,
    #[serde(alias = "is_active")]
    pub is_active: bool,
    #[serde(alias = "created_at", default)]
    pub created_at: Option<surrealdb::Datetime>,
    #[serde(alias = "updated_at", default)]
    pub updated_at: Option<surrealdb::Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA
// --------------------------------------------------------------------------

/// Datos requeridos para registrar una empresa.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmpresaInput {
    pub nombre: String,
    pub direccion: Option<String>,
}

/// Datos para actualización parcial de empresas.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmpresaInput {
    pub nombre: Option<String>,
    pub direccion: Option<String>,
    pub is_active: Option<bool>,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct EmpresaCreateDTO {
    pub nombre: String,
    pub direccion: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct EmpresaUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<surrealdb::Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA
// --------------------------------------------------------------------------

/// Respuesta pública al cliente con datos de la empresa.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaResponse {
    pub id: String,
    pub nombre: String,
    pub direccion: Option<String>,
    pub is_active: bool,
    pub total_contratistas: usize,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Empresa> for EmpresaResponse {
    fn from(e: Empresa) -> Self {
        Self {
            id: e.id.to_string(),
            nombre: e.nombre,
            direccion: e.direccion,
            is_active: e.is_active,
            total_contratistas: 0, // Se llena en capa de servicio si es necesario
            created_at: e
                .created_at.map_or_else(|| chrono::Utc::now().to_rfc3339(), |d| d.to_string()),
            updated_at: e
                .updated_at.map_or_else(|| chrono::Utc::now().to_rfc3339(), |d| d.to_string()),
        }
    }
}

/// Lista paginada de empresas.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaListResponse {
    pub empresas: Vec<EmpresaResponse>,
    pub total: usize,
    pub activas: usize,
}
