// ==========================================
// src/models/empresa.rs
// ==========================================

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Empresa {
    pub id: String,
    pub nombre: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmpresaInput {
    pub nombre: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmpresaInput {
    pub nombre: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmpresaResponse {
    pub id: String,
    pub nombre: String,
    pub is_active: bool,
    pub total_contratistas: usize,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Empresa> for EmpresaResponse {
    fn from(e: Empresa) -> Self {
        Self {
            id: e.id,
            nombre: e.nombre,
            is_active: e.is_active,
            total_contratistas: 0, // Se llena después con query
            created_at: e.created_at,
            updated_at: e.updated_at,
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
