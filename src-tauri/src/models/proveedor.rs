// ==========================================
// src/models/proveedor.rs
// ==========================================

use crate::models::empresa::Empresa;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa a un proveedor registrado en el sistema.
///
/// Un proveedor es una entidad que suministra bienes o servicios y tiene acceso recurrente.
/// A diferencia del contratista, su relación suele ser de entrega/logística.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proveedor {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    /// Referencia a la empresa que representa.
    pub empresa: RecordId,
    pub estado: EstadoProveedor,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
    #[serde(alias = "deleted_at")]
    pub deleted_at: Option<Datetime>,
}

/// Versión "poblada" del proveedor con datos completos de la empresa (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveedorFetched {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    #[serde(alias = "segundo_nombre")]
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    #[serde(alias = "segundo_apellido")]
    pub segundo_apellido: Option<String>,
    pub empresa: Empresa,
    pub estado: EstadoProveedor,
    #[serde(alias = "created_at")]
    pub created_at: Datetime,
    #[serde(alias = "updated_at")]
    pub updated_at: Datetime,
    #[serde(alias = "deleted_at")]
    pub deleted_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// ENUMS (Tipos Estrictos)
// --------------------------------------------------------------------------

/// Estado operativo del proveedor en la plataforma.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(try_from = "String")]
pub enum EstadoProveedor {
    Activo,
    Inactivo,
    Suspendido,
}

impl EstadoProveedor {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Activo => "ACTIVO",
            Self::Inactivo => "INACTIVO",
            Self::Suspendido => "SUSPENDIDO",
        }
    }
}

impl std::str::FromStr for EstadoProveedor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ACTIVO" => Ok(Self::Activo),
            "INACTIVO" => Ok(Self::Inactivo),
            "SUSPENDIDO" => Ok(Self::Suspendido),
            _ => Err(format!("Estado desconocido: {s}")),
        }
    }
}

impl TryFrom<String> for EstadoProveedor {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

/// Datos necesarios para registrar un nuevo proveedor.
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
    // Datos vehiculares opcionales
    pub tipo_vehiculo: Option<String>,
    pub placa: Option<String>,
    pub marca: Option<String>,
    pub modelo: Option<String>,
    pub color: Option<String>,
}

/// Datos para la actualización parcial de un proveedor.
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

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA
// --------------------------------------------------------------------------

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

#[derive(Debug, Serialize, Default)]
pub struct ProveedorUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segundo_apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empresa: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<EstadoProveedor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responsess)
// --------------------------------------------------------------------------

/// Respuesta detallada con toda la información del proveedor.
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
    // Detalles vehiculares aplanados
    pub vehiculo_tipo: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub vehiculo_marca: Option<String>,
    pub vehiculo_modelo: Option<String>,
    pub vehiculo_color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
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
            empresa_nombre: String::new(), // Se llena en la capa de servicio
            estado: p.estado,
            puede_ingresar,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: p.created_at.to_string(),
            updated_at: p.updated_at.to_string(),
            deleted_at: p.deleted_at.map(|d| d.to_string()),
        }
    }
}

impl ProveedorResponse {
    /// Crea la respuesta a partir de un proveedor con datos relacionados (FETCH).
    pub fn from_fetched(p: ProveedorFetched) -> Self {
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
            empresa_id: p.empresa.id.to_string(),
            empresa_nombre: p.empresa.nombre.clone(),
            estado: p.estado.clone(),
            puede_ingresar,
            vehiculo_tipo: None,
            vehiculo_placa: None,
            vehiculo_marca: None,
            vehiculo_modelo: None,
            vehiculo_color: None,
            created_at: p.created_at.to_string(),
            updated_at: p.updated_at.to_string(),
            deleted_at: p.deleted_at.map(|d| d.to_string()),
        }
    }
}
