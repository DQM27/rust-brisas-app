//! # Modelos: Vehículos (Entidades y DTOs)
//!
//! Este módulo define las estructuras de datos para la gestión vehicular,
//! incluyendo modelos de persistencia, versiones hidratadas (FETCH)
//! y contratos para la comunicación con el frontend (Tauri).

use crate::models::contratista::ContratistaFetched;
use crate::models::proveedor::ProveedorFetched;
use crate::models::visitante::VisitanteFetched;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa un vehículo registrado en el sistema.
///
/// Un vehículo siempre está asociado directamente a un propietario (Contratista,
/// Proveedor o Visitante) para control de acceso y trazabilidad.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehiculo {
    pub id: RecordId,
    pub propietario: RecordId,
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

/// Versión "poblada" del vehículo con datos completos del propietario (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiculoFetched {
    pub id: RecordId,
    pub propietario: PropietarioFetched,
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

/// Enum polimórfico para manejar los distintos tipos de propietarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropietarioFetched {
    Contratista(ContratistaFetched),
    Proveedor(ProveedorFetched),
    Visitante(VisitanteFetched),
}

// --------------------------------------------------------------------------
// ENUMS (Tipos Estrictos)
// --------------------------------------------------------------------------

/// Categoría del vehículo para control de acceso.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TipoVehiculo {
    Motocicleta,
    Automovil,
}

impl TipoVehiculo {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Motocicleta => "motocicleta",
            Self::Automovil => "automovil",
        }
    }

    pub const fn display(&self) -> &str {
        match self {
            Self::Motocicleta => "Motocicleta",
            Self::Automovil => "Automóvil",
        }
    }
}

impl std::str::FromStr for TipoVehiculo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "motocicleta" => Ok(Self::Motocicleta),
            "automóvil" | "automovil" => Ok(Self::Automovil),
            _ => Err(format!("Tipo de vehículo desconocido: {s}")),
        }
    }
}

impl std::fmt::Display for TipoVehiculo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

/// Datos necesarios para registrar un nuevo vehículo.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVehiculoInput {
    pub propietario_id: String,
    pub tipo_vehiculo: String,
    pub placa: String,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

/// Datos para actualizar la información de un vehículo.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVehiculoInput {
    pub tipo_vehiculo: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct VehiculoCreateDTO {
    pub propietario: RecordId,
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

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VehiculoResponse {
    pub id: String,
    pub propietario_id: String,
    pub propietario_nombre: String,
    pub propietario_cedula: String,
    pub propietario_tipo: String, // contratista, proveedor, visitante
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
            propietario_id: v.propietario.to_string(),
            propietario_nombre: String::new(),
            propietario_cedula: String::new(),
            propietario_tipo: v.propietario.table().to_string(),
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
            propietario_id: String::new(),
            propietario_nombre: String::new(),
            propietario_cedula: String::new(),
            propietario_tipo: String::new(),
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

        match v.propietario {
            PropietarioFetched::Contratista(c) => {
                res.propietario_id = c.id.to_string();
                res.propietario_nombre = format!("{} {}", c.nombre, c.apellido);
                res.propietario_cedula = c.cedula;
                res.propietario_tipo = "contratista".to_string();
                res.empresa_nombre = c.empresa.nombre;
            }
            PropietarioFetched::Proveedor(p) => {
                res.propietario_id = p.id.to_string();
                res.propietario_nombre = format!("{} {}", p.nombre, p.apellido);
                res.propietario_cedula = p.cedula;
                res.propietario_tipo = "proveedor".to_string();
                res.empresa_nombre = p.empresa.nombre;
            }
            PropietarioFetched::Visitante(vis) => {
                res.propietario_id = vis.id.to_string();
                res.propietario_nombre = format!("{} {}", vis.nombre, vis.apellido);
                res.propietario_cedula = vis.cedula;
                res.propietario_tipo = "visitante".to_string();
                res.empresa_nombre =
                    vis.empresa.map_or_else(|| "N/A".to_string(), |e| e.nombre);
            }
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
