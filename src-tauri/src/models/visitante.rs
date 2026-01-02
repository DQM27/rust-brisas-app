//! # Modelos: Visitantes (Entidades y DTOs)
//!
//! Este módulo define las estructuras de datos para la gestión de visitantes
//! ocasionales, incluyendo modelos de persistencia y contratos de comunicación.

use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// MODELO DE DOMINIO
// --------------------------------------------------------------------------

/// Representa una persona externa que ingresa temporalmente a las instalaciones.
///
/// En modo estricto, todo visitante debe estar asociado a una empresa registrada
/// en el sistema para garantizar la trazabilidad y consistencia de datos.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Visitante {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    /// Referencia obligatoria a la empresa de origen (record<empresa>).
    pub empresa: Option<RecordId>, // Mantenemos Option por si se borra la empresa, pero el input exigirá ID.
    pub has_vehicle: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted_at: Option<Datetime>,
}

/// Versión "poblada" del visitante con los datos completos de su empresa (FETCH).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitanteFetched {
    pub id: RecordId,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: Option<crate::models::empresa::Empresa>,
    pub has_vehicle: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub deleted_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Commands)
// --------------------------------------------------------------------------

/// Datos necesarios para registrar un nuevo visitante (Estricto).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVisitanteInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,

    /// ID de la empresa registrada. Es obligatorio seleccionar una empresa existente.
    pub empresa_id: String,

    pub has_vehicle: bool,

    // Campos opcionales de Vehículo para registro inline
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
pub struct VisitanteCreateDTO {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: RecordId, // Obligatorio en creación
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
    pub empresa: Option<RecordId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vehicle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Responses)
// --------------------------------------------------------------------------

/// Respuesta detallada de información de visitante.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitanteResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub segundo_nombre: Option<String>,
    pub segundo_apellido: Option<String>,
    pub empresa: String, // Nombre de la empresa para display
    pub empresa_id: Option<String>,
    pub has_vehicle: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
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
            empresa: "Desconocida".to_string(), // Se rellena en capa superior o servicio
            empresa_id: v.empresa.map(|t| t.to_string()),
            has_vehicle: v.has_vehicle,
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
            deleted_at: v.deleted_at.map(|d| d.to_string()),
        }
    }
}

impl VisitanteResponse {
    pub fn from_fetched(v: VisitanteFetched) -> Self {
        Self {
            id: v.id.to_string(),
            cedula: v.cedula,
            nombre: v.nombre,
            apellido: v.apellido,
            segundo_nombre: v.segundo_nombre,
            segundo_apellido: v.segundo_apellido,
            empresa: v
                .empresa
                .as_ref()
                .map(|e| e.nombre.clone())
                .unwrap_or_else(|| "Sin Empresa".to_string()),
            empresa_id: v.empresa.as_ref().map(|e| e.id.to_string()),
            has_vehicle: v.has_vehicle,
            created_at: v.created_at.to_string(),
            updated_at: v.updated_at.to_string(),
            deleted_at: v.deleted_at.map(|d| d.to_string()),
        }
    }
}
