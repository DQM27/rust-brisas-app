// ==========================================
// src/models/proveedor.rs
// ==========================================

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proveedor {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: RecordId,
    pub estado: EstadoProveedor,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoProveedor {
    Activo,
    Inactivo,
    Suspendido,
}

impl EstadoProveedor {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoProveedor::Activo => "ACTIVO",
            EstadoProveedor::Inactivo => "INACTIVO",
            EstadoProveedor::Suspendido => "SUSPENDIDO",
        }
    }
}

impl std::str::FromStr for EstadoProveedor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVO" => Ok(EstadoProveedor::Activo),
            "INACTIVO" => Ok(EstadoProveedor::Inactivo),
            "SUSPENDIDO" => Ok(EstadoProveedor::Suspendido),
            _ => Err(format!("Estado desconocido: {}", s)),
        }
    }
}

impl TryFrom<String> for EstadoProveedor {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa_id: String,
    pub tiene_vehiculo: Option<bool>,
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProveedorInput {
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa_id: Option<String>,
    pub estado: Option<String>,
    pub tiene_vehiculo: Option<bool>,
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct ProveedorCreateDTO {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub empresa: RecordId,
    pub estado: EstadoProveedor,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProveedorResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: String,
    pub empresa_nombre: String,
    pub estado: EstadoProveedor,
    pub puede_ingresar: bool,
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Proveedor> for ProveedorResponse {
    fn from(p: Proveedor) -> Self {
        let mut nombre_completo = p.nombre.clone();
        if let Some(ref s) = p.segundo_nombre {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }
        nombre_completo.push(' ');
        nombre_completo.push_str(&p.apellido);
        if let Some(ref s) = p.segundo_apellido {
            nombre_completo.push(' ');
            nombre_completo.push_str(s);
        }

        let puede_ingresar = p.estado == EstadoProveedor::Activo;

        Self {
            id: p.id.to_string(),
            cedula: p.cedula,
            nombre: p.nombre,
            segundo_nombre: p.segundo_nombre,
            apellido: p.apellido,
            segundo_apellido: p.segundo_apellido,
            nombre_completo,
            empresa_id: p.empresa.to_string(),
            empresa_nombre: String::new(), // Will be filled by service
            estado: p.estado,
            puede_ingresar,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: p.created_at.to_string(),
            updated_at: p.updated_at.to_string(),
        }
    }
}
