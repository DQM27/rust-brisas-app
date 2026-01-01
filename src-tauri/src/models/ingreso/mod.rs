/// Módulo: Ingreso.
///
/// Este módulo centraliza todos los submodelos y tipos relacionados con el control de acceso
/// (entradas y salidas) de personal y visitantes.
///
/// Submódulos:
/// - `contratista`: Ingresos de personal contratista (PRAIND).
/// - `proveedor`: Ingresos de proveedores (entregas, servicios).
/// - `visita`: Ingresos de visitantes ocasionales.
/// - `alerta`: Gestión de incidentes de gafetes y seguridad.
/// - `response`: Estructuras de respuesta unificadas (DTOs de salida).
// src/models/ingreso/mod.rs
pub mod alerta;
pub mod contratista;
pub mod proveedor;
pub mod response;
pub mod visita;

pub use alerta::*;
pub use contratista::*;
pub use proveedor::*;
pub use response::*;
pub use visita::*;

use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

// ==========================================
// ENUMS DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoIngreso {
    Contratista,
    Visita,
    Proveedor,
}

impl TipoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            TipoIngreso::Contratista => "contratista",
            TipoIngreso::Visita => "visita",
            TipoIngreso::Proveedor => "proveedor",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            TipoIngreso::Contratista => "Contratista",
            TipoIngreso::Visita => "Visita",
            TipoIngreso::Proveedor => "Proveedor",
        }
    }
}

impl std::str::FromStr for TipoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoIngreso::Contratista),
            "visita" => Ok(TipoIngreso::Visita),
            "proveedor" => Ok(TipoIngreso::Proveedor),
            _ => Err(format!("Tipo de ingreso desconocido: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoAutorizacion {
    Praind,
    Correo,
}

impl TipoAutorizacion {
    pub fn as_str(&self) -> &str {
        match self {
            TipoAutorizacion::Praind => "praind",
            TipoAutorizacion::Correo => "correo",
        }
    }
}

impl std::str::FromStr for TipoAutorizacion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "praind" => Ok(TipoAutorizacion::Praind),
            "correo" => Ok(TipoAutorizacion::Correo),
            _ => Err(format!("Tipo de autorización desconocido: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModoIngreso {
    Caminando,
    Vehiculo,
}

impl ModoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "caminando",
            ModoIngreso::Vehiculo => "vehiculo",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "Caminando",
            ModoIngreso::Vehiculo => "Vehículo",
        }
    }
}

impl std::str::FromStr for ModoIngreso {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(ModoIngreso::Caminando),
            "vehiculo" => Ok(ModoIngreso::Vehiculo),
            _ => Err(format!("Modo de ingreso desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs COMPARTIDOS
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(tag = "tipo", rename_all = "lowercase")]
pub enum CreateIngresoInput {
    Contratista(CreateIngresoContratistaInput),
    Visita(CreateIngresoVisitaInput),
    Proveedor(CreateIngresoProveedorInput),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaInput {
    pub ingreso_id: String,
    pub devolvio_gafete: bool,
    pub usuario_salida_id: String,
    pub observaciones_salida: Option<String>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct IngresoUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_hora_salida: Option<surrealdb::Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usuario_salida: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

pub type IngresoContratistaUpdateDTO = IngresoUpdateDTO;
pub type IngresoProveedorUpdateDTO = IngresoUpdateDTO;
pub type IngresoVisitaUpdateDTO = IngresoUpdateDTO;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoListResponse {
    pub ingresos: Vec<IngresoResponse>,
    pub total: usize,
    pub adentro: usize,
    pub salieron: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub contratista: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoResponse>,
}
