// ==========================================
// src/models/reporte.rs
// ==========================================
// Solo modelos, DTOs y helpers basicos - SIN validaciones ni logica

use serde::{Deserialize, Serialize};

// ==========================================
// ENUMS
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoReporte {
    Error,
    Sugerencia,
    Mejora,
}

impl TipoReporte {
    pub fn as_str(&self) -> &str {
        match self {
            TipoReporte::Error => "error",
            TipoReporte::Sugerencia => "sugerencia",
            TipoReporte::Mejora => "mejora",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(TipoReporte::Error),
            "sugerencia" => Some(TipoReporte::Sugerencia),
            "mejora" => Some(TipoReporte::Mejora),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoReporte {
    Pendiente,
    Enviado,
    Fallido,
}

impl EstadoReporte {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoReporte::Pendiente => "pendiente",
            EstadoReporte::Enviado => "enviado",
            EstadoReporte::Fallido => "fallido",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pendiente" => Some(EstadoReporte::Pendiente),
            "enviado" => Some(EstadoReporte::Enviado),
            "fallido" => Some(EstadoReporte::Fallido),
            _ => None,
        }
    }
}

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reporte {
    pub id: String,
    pub tipo: TipoReporte,
    pub asunto: String,
    pub mensaje: String,
    pub contacto: Option<String>,
    pub tiene_adjunto: bool,
    pub nombre_adjunto: Option<String>,
    pub estado: EstadoReporte,
    pub error_envio: Option<String>,
    pub enviado_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReporteInput {
    pub tipo: String,
    pub asunto: String,
    pub mensaje: String,
    pub contacto: Option<String>,
    pub adjunto_base64: Option<String>,
    pub nombre_adjunto: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReporteResponse {
    pub id: String,
    pub tipo: String,
    pub asunto: String,
    pub mensaje: String,
    pub contacto: Option<String>,
    pub tiene_adjunto: bool,
    pub nombre_adjunto: Option<String>,
    pub estado: String,
    pub error_envio: Option<String>,
    pub enviado_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Reporte> for ReporteResponse {
    fn from(r: Reporte) -> Self {
        Self {
            id: r.id,
            tipo: r.tipo.as_str().to_string(),
            asunto: r.asunto,
            mensaje: r.mensaje,
            contacto: r.contacto,
            tiene_adjunto: r.tiene_adjunto,
            nombre_adjunto: r.nombre_adjunto,
            estado: r.estado.as_str().to_string(),
            error_envio: r.error_envio,
            enviado_at: r.enviado_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReporteListResponse {
    pub reportes: Vec<ReporteResponse>,
    pub total: usize,
    pub enviados: usize,
    pub pendientes: usize,
    pub fallidos: usize,
}
