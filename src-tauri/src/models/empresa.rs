// ==========================================
// src/models/empresa.rs
// ==========================================

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Empresa {
    pub id: Thing,
    pub nombre: String,
    pub direccion: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            created_at: e.created_at.to_rfc3339(),
            updated_at: e.updated_at.to_rfc3339(),
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
