// ==========================================
// src/models/gafete.rs
// ==========================================
use serde::{Deserialize, Serialize};

/// Estados posibles de un gafete
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoGafete {
    Disponible,
    Asignado,
    Perdido,
}

impl EstadoGafete {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoGafete::Disponible => "disponible",
            EstadoGafete::Asignado => "asignado",
            EstadoGafete::Perdido => "perdido",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "disponible" => Ok(EstadoGafete::Disponible),
            "asignado" => Ok(EstadoGafete::Asignado),
            "perdido" => Ok(EstadoGafete::Perdido),
            _ => Err(format!("Estado de gafete desconocido: {}", s)),
        }
    }
}

/// Modelo de dominio - Representa un gafete físico
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gafete {
    pub id: String,
    pub numero: String,
    pub estado: String,
    pub contratista_asignado_id: Option<String>,
    pub ingreso_actual_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGafeteInput {
    pub numero: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGafeteInput {
    pub numero: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsignarGafeteInput {
    pub contratista_id: String,
    pub ingreso_id: String,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafeteResponse {
    pub id: String,
    pub numero: String,
    pub estado: EstadoGafete,
    pub contratista_asignado_id: Option<String>,
    pub contratista_nombre: Option<String>,      // JOIN si está asignado
    pub contratista_cedula: Option<String>,      // JOIN si está asignado
    pub ingreso_actual_id: Option<String>,
    pub es_sin_gafete: bool,                     // true si numero == "S/G"
    pub puede_asignarse: bool,                   // true si estado == disponible
    pub created_at: String,
    pub updated_at: String,
}

impl From<Gafete> for GafeteResponse {
    fn from(g: Gafete) -> Self {
        let estado = EstadoGafete::from_str(&g.estado).unwrap_or(EstadoGafete::Disponible);
        let es_sin_gafete = g.numero.to_uppercase() == "S/G";
        let puede_asignarse = estado == EstadoGafete::Disponible && !es_sin_gafete;
        
        Self {
            id: g.id,
            numero: g.numero,
            estado,
            contratista_asignado_id: g.contratista_asignado_id,
            contratista_nombre: None,  // Se llena en comando con JOIN
            contratista_cedula: None,  // Se llena en comando con JOIN
            ingreso_actual_id: g.ingreso_actual_id,
            es_sin_gafete,
            puede_asignarse,
            created_at: g.created_at,
            updated_at: g.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafeteListResponse {
    pub gafetes: Vec<GafeteResponse>,
    pub total: usize,
    pub disponibles: usize,
    pub asignados: usize,
    pub perdidos: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafeteStockResponse {
    pub total_gafetes: usize,
    pub disponibles: usize,
    pub asignados: usize,
    pub perdidos: usize,
    pub porcentaje_disponibilidad: f64,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use super::*;
    
    pub fn validar_numero(numero: &str) -> Result<(), String> {
        let limpio = numero.trim().to_uppercase();
        
        if limpio.is_empty() {
            return Err("El número de gafete no puede estar vacío".to_string());
        }
        
        if limpio.len() > 20 {
            return Err("El número de gafete no puede exceder 20 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_create_input(input: &CreateGafeteInput) -> Result<(), String> {
        validar_numero(&input.numero)?;
        Ok(())
    }
}