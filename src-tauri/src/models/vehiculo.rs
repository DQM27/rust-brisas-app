// ==========================================
// src/models/vehiculo.rs
// ==========================================

use crate::models::contratista::ContratistaFetched;
use crate::models::proveedor::ProveedorFetched;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehiculo {
    pub id: RecordId,
    pub contratista: Option<RecordId>,
    pub proveedor: Option<RecordId>,
    pub visitante: Option<RecordId>,
    #[serde(alias = "tipo_vehiculo")]
    pub tipo_vehiculo: TipoVehiculo,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    #[serde(alias = "is_active")]
    pub is_active: bool,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiculoFetched {
    pub id: RecordId,
    pub contratista: Option<ContratistaFetched>,
    pub proveedor: Option<ProveedorFetched>,
    pub visitante: Option<RecordId>, // No fetched visitor yet
    #[serde(alias = "tipo_vehiculo")]
    pub tipo_vehiculo: TipoVehiculo,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    #[serde(alias = "is_active")]
    pub is_active: bool,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
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
    pub contratista_id: Option<String>,
    pub proveedor_id: Option<String>,
    pub tipo_vehiculo: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehiculoInput {
    pub tipo_vehiculo: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

// ==========================================
// DTO PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct VehiculoCreateDTO {
    pub contratista: Option<RecordId>,
    pub proveedor: Option<RecordId>,
    pub visitante: Option<RecordId>,
    pub tipo_vehiculo: TipoVehiculo,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct VehiculoUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_vehiculo: Option<TipoVehiculo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modelo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
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

use std::fmt;

impl fmt::Display for TipoVehiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

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
            id: v.id.to_string(),
            contratista_id: v.contratista.as_ref().map(|t| t.to_string()).unwrap_or_default(),
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
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
        }
    }
}

impl VehiculoResponse {
    pub fn from_fetched(v: VehiculoFetched) -> Self {
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

        let mut res = Self {
            id: v.id.to_string(),
            contratista_id: String::new(),
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
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
        };

        if let Some(c) = v.contratista {
            res.contratista_id = c.id.to_string();
            res.contratista_nombre = format!("{} {}", c.nombre, c.apellido);
            res.contratista_cedula = c.cedula;
            res.empresa_nombre = c.empresa.nombre;
        } else if let Some(p) = v.proveedor {
            res.contratista_id = p.id.to_string();
            res.contratista_nombre = format!("{} {}", p.nombre, p.apellido);
            res.contratista_cedula = p.cedula;
            res.empresa_nombre = p.empresa.nombre;
        }

        res
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
