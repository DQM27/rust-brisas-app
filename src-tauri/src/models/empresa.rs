// ==========================================
// src/models/empresa.rs
// ==========================================

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

use surrealdb::RecordId;

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

// ==========================================
// DTOs DE ENTRADA (Frontend -> Command)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmpresaInput {
    pub nombre: String,
    pub direccion: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmpresaInput {
    pub nombre: Option<String>,
    pub direccion: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

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

// ==========================================
// DTOs DE SALIDA (Service -> Frontend)
// ==========================================

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
            total_contratistas: 0,
            created_at: e
                .created_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: e
                .updated_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaListResponse {
    pub empresas: Vec<EmpresaResponse>,
    pub total: usize,
    pub activas: usize,
}
