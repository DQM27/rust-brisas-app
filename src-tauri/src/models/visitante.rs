// ==========================================
// MODELO DE DOMINIO
// ==========================================

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Visitante {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    pub empresa: Option<RecordId>,
    #[serde(alias = "has_vehicle")]
    pub has_vehicle: bool,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
}

// ==========================================
// DTOs DE ENTRADA (Frontend -> Command)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVisitanteInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>, // Nombre de la empresa (legacy/fallback)
    pub empresa_id: Option<String>, // ID de la empresa (link)
    pub has_vehicle: bool,
}

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct VisitanteCreateDTO {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<RecordId>,
    pub has_vehicle: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct VisitanteUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empresa: Option<Option<RecordId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vehicle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// ==========================================
// DTOs DE SALIDA (Service -> Frontend)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitanteResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<String>,
    pub empresa_id: Option<String>,
    pub has_vehicle: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Visitante> for VisitanteResponse {
    fn from(v: Visitante) -> Self {
        Self {
            id: v.id.to_string(),
            cedula: v.cedula,
            nombre: v.nombre,
            apellido: v.apellido,
            segundo_nombre: v.segundo_nombre,
            segundo_apellido: v.segundo_apellido,
            empresa: None, // Will be filled by service if name is needed
            empresa_id: v.empresa.map(|t| t.to_string()),
            has_vehicle: v.has_vehicle,
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
        }
    }
}
