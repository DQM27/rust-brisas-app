// ==========================================
// src/models/cita.rs
// Enterprise Quality SurrealDB Implementation
// ==========================================

use crate::models::user::User;
use crate::models::visitante::Visitante;
use serde::{Deserialize, Serialize};
use surrealdb::{Datetime, RecordId};

// ==========================================
// MODELO DE BASE DE DATOS (SurrealDB Native)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cita {
    pub id: RecordId,
    pub visitante_id: Option<RecordId>,
    pub usuario_id: RecordId,
    pub motivo: String,
    pub fecha_inicio: Datetime,
    pub fecha_fin: Datetime,
    pub estado: String,
    pub activa: bool,
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// ==========================================
// MODELO FETCHED (con relaciones pobladas)
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitaFetched {
    pub id: RecordId,
    pub visitante_id: Option<Visitante>,
    pub usuario_id: User,
    pub motivo: String,
    pub fecha_inicio: Datetime,
    pub fecha_fin: Datetime,
    pub estado: String,
    pub activa: bool,
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// ==========================================
// DTOs DE ENTRADA (Frontend -> Command)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCitaInput {
    pub visitante_id: Option<String>,
    pub fecha_cita: String,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    // Datos de visitante nuevo (si no existe)
    pub visitante_cedula: Option<String>,
    pub visitante_nombre: Option<String>,
    pub visitante_apellido: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCitaInput {
    pub fecha_cita: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo: Option<String>,
    pub estado: Option<String>,
}

// ==========================================
// DTOs PARA PERSISTENCIA (Service -> DB)
// ==========================================

#[derive(Debug, Serialize)]
pub struct CitaCreateDTO {
    pub visitante_id: Option<RecordId>,
    pub usuario_id: RecordId,
    pub motivo: String,
    pub fecha_inicio: Datetime,
    pub fecha_fin: Datetime,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct CitaUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motivo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_inicio: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_fin: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anfitrion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_visitada: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// ==========================================
// DTOs DE SALIDA (Service -> Frontend)
// ==========================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitaResponse {
    pub id: String,
    pub visitante_id: Option<String>,
    pub visitante_cedula: Option<String>,
    pub visitante_nombre: Option<String>,
    pub visitante_apellido: Option<String>,
    pub visitante_nombre_completo: Option<String>,
    pub usuario_id: String,
    pub usuario_nombre: Option<String>,
    pub motivo: String,
    pub fecha_cita: String,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub estado: String,
    pub activa: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl CitaResponse {
    /// Construye un `CitaResponse` a partir de un `CitaFetched` con relaciones pobladas
    pub fn from_fetched(c: CitaFetched) -> Self {
        let (
            visitante_id,
            visitante_cedula,
            visitante_nombre,
            visitante_apellido,
            visitante_nombre_completo,
        ) = if let Some(ref v) = c.visitante_id {
            let nombre_completo = format!("{} {}", v.nombre, v.apellido);
            (
                Some(v.id.to_string()),
                Some(v.cedula.clone()),
                Some(v.nombre.clone()),
                Some(v.apellido.clone()),
                Some(nombre_completo),
            )
        } else {
            // Fallback to raw fields from Cita if visitor wasn't fetched
            (
                None,
                c.visitante_cedula.clone(),
                c.visitante_nombre.clone(),
                None,
                c.visitante_nombre.clone(),
            )
        };

        let usuario_nombre = Some(format!("{} {}", c.usuario_id.nombre, c.usuario_id.apellido));

        Self {
            id: c.id.to_string(),
            visitante_id,
            visitante_cedula,
            visitante_nombre,
            visitante_apellido,
            visitante_nombre_completo,
            usuario_id: c.usuario_id.id.to_string(),
            usuario_nombre,
            motivo: c.motivo,
            fecha_cita: c.fecha_inicio.to_string(),
            anfitrion: c.anfitrion,
            area_visitada: c.area_visitada,
            estado: c.estado,
            activa: c.activa,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
        }
    }
}

impl From<Cita> for CitaResponse {
    fn from(c: Cita) -> Self {
        Self {
            id: c.id.to_string(),
            visitante_id: c.visitante_id.map(|v| v.to_string()),
            visitante_cedula: c.visitante_cedula,
            visitante_nombre: c.visitante_nombre.clone(),
            visitante_apellido: None,
            visitante_nombre_completo: c.visitante_nombre,
            usuario_id: c.usuario_id.to_string(),
            usuario_nombre: None,
            motivo: c.motivo,
            fecha_cita: c.fecha_inicio.to_string(),
            anfitrion: c.anfitrion,
            area_visitada: c.area_visitada,
            estado: c.estado,
            activa: c.activa,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
        }
    }
}

// ==========================================
// LIST RESPONSE
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitaListResponse {
    pub citas: Vec<CitaResponse>,
    pub total: usize,
    pub pendientes: usize,
    pub completadas: usize,
    pub canceladas: usize,
}
