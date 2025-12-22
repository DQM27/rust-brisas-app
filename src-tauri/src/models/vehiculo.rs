// ==========================================
// src/models/vehiculo.rs
// ==========================================
// Solo modelos, DTOs y enums - SIN validaciones ni lógica

use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

/// Representa un vehículo registrado
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehiculo {
    pub id: String,
    pub contratista_id: Option<String>, // Ahora es Opcional
    pub proveedor_id: Option<String>,   // Nuevo campo
    pub tipo_vehiculo: TipoVehiculo,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// ENUM DE TIPO DE VEHÍCULO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoVehiculo {
    Motocicleta,
    Automovil,
}

impl TipoVehiculo {
    pub fn as_str(&self) -> &str {
        match self {
            TipoVehiculo::Motocicleta => "motocicleta",
            TipoVehiculo::Automovil => "automovil",
        }
    }

    pub fn display(&self) -> &str {
        match self {
            TipoVehiculo::Motocicleta => "Motocicleta",
            TipoVehiculo::Automovil => "Automóvil",
        }
    }
}

impl std::str::FromStr for TipoVehiculo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "motocicleta" => Ok(TipoVehiculo::Motocicleta),
            "automóvil" | "automovil" => Ok(TipoVehiculo::Automovil),
            _ => Err(format!("Tipo de vehículo desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVehiculoInput {
    pub contratista_id: String,
    pub tipo_vehiculo: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehiculoInput {
    pub tipo_vehiculo: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VehiculoResponse {
    pub id: String,
    pub contratista_id: String,
    pub contratista_nombre: String,
    pub contratista_cedula: String,
    pub empresa_nombre: String,
    pub tipo_vehiculo: TipoVehiculo,
    pub tipo_vehiculo_display: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub descripcion_completa: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// Implementación de Display para TipoVehiculo
use std::fmt;

impl fmt::Display for TipoVehiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

// ... (CreateVehiculoInput code remains same)

impl From<Vehiculo> for VehiculoResponse {
    fn from(v: Vehiculo) -> Self {
        let marca_str = v.marca.clone().unwrap_or_else(|| "N/A".to_string());
        let modelo_str = v.modelo.clone().unwrap_or_else(|| "N/A".to_string());
        let color_str = v.color.clone().unwrap_or_else(|| "N/A".to_string());

        let descripcion_completa = if v.marca.is_some() || v.modelo.is_some() {
            format!(
                "{} - Placa {} - {} {} ({})",
                v.tipo_vehiculo.display(),
                v.placa,
                marca_str,
                modelo_str,
                color_str
            )
        } else {
            format!("{} - Placa {}", v.tipo_vehiculo.display(), v.placa)
        };

        Self {
            id: v.id,
            contratista_id: v.contratista_id.unwrap_or_default(), // Handle None as empty string
            contratista_nombre: String::new(),
            contratista_cedula: String::new(),
            empresa_nombre: String::new(),
            tipo_vehiculo: v.tipo_vehiculo.clone(),
            tipo_vehiculo_display: v.tipo_vehiculo.display().to_string(),
            placa: v.placa,
            marca: v.marca,
            modelo: v.modelo,
            color: v.color,
            descripcion_completa,
            is_active: v.is_active,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehiculoListResponse {
    pub vehiculos: Vec<VehiculoResponse>,
    pub total: usize,
    pub activos: usize,
    pub inactivos: usize,
    pub por_tipo: TipoVehiculoStats,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TipoVehiculoStats {
    pub motocicletas: usize,
    pub automoviles: usize,
}
