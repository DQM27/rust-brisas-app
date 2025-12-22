// ==========================================
// src/models/visitante.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Visitante {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub has_vehicle: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVisitanteInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub has_vehicle: bool,
}
