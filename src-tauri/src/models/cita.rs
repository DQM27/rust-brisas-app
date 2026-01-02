/// Modelo de Base de Datos: Gestión de Citas.
///
/// Este módulo define la estructura de datos para el agendamiento de visitas,
/// manejando estados estrictos (`EstadoCita`) y relaciones con Visitantes y Usuarios.
use crate::models::user::User;
use crate::models::visitante::Visitante;
use serde::{Deserialize, Serialize};
use std::fmt;
use surrealdb::{Datetime, RecordId};

// --------------------------------------------------------------------------
// ENUMS DE DOMINIO
// --------------------------------------------------------------------------

/// Define los estados posibles del ciclo de vida de una Cita.
///
/// Este Enum centraliza la lógica de estados válidos, evitando cadenas mágicas
/// ("Magic Strings") dispersas por el código.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EstadoCita {
    /// La cita ha sido creada y está a la espera de ocurrir.
    #[serde(rename = "pendiente")]
    Programada,

    /// El visitante ha ingresado y la cita está ocurriendo.
    #[serde(rename = "en_curso")]
    EnCurso,

    /// La cita concluyó exitosamente.
    #[serde(rename = "completada")]
    Finalizada,

    /// La cita fue cancelada antes de ocurrir o durante.
    #[serde(rename = "cancelada")]
    Cancelada,

    /// La cita no ocurrió en la fecha prevista y expiró.
    #[serde(rename = "vencida")]
    Vencida,

    /// Estado de fallback para valores no reconocidos en la base de datos.
    #[serde(untagged)]
    Desconocido(String),
}

// Implementación de Display para facilitar logs y respuestas
impl fmt::Display for EstadoCita {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Programada => write!(f, "PROGRAMADA"),
            Self::EnCurso => write!(f, "EN_CURSO"),
            Self::Finalizada => write!(f, "FINALIZADA"),
            Self::Cancelada => write!(f, "CANCELADA"),
            Self::Vencida => write!(f, "VENCIDA"),
            Self::Desconocido(s) => write!(f, "{s}"),
        }
    }
}

// Convertidor para facilitar el uso con strings
impl From<String> for EstadoCita {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "PROGRAMADA" => Self::Programada,
            "EN_CURSO" => Self::EnCurso,
            "FINALIZADA" => Self::Finalizada,
            "CANCELADA" => Self::Cancelada,
            "VENCIDA" => Self::Vencida,
            _ => Self::Desconocido(s),
        }
    }
}

impl From<&str> for EstadoCita {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

// --------------------------------------------------------------------------
// MODELO DE BASE DE DATOS (SurrealDB Native)
// --------------------------------------------------------------------------

/// Modelo de Cita (`SurrealDB`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cita {
    pub id: RecordId,
    /// Visitante opcional (si es nuevo o no registrado).
    pub visitante_id: Option<RecordId>,
    pub usuario_id: RecordId,
    pub motivo: String,
    pub fecha_inicio: Datetime,
    pub fecha_fin: Datetime,
    /// Estado del flujo (programada -> `en_curso` -> finalizada).
    pub estado: EstadoCita,
    pub activa: bool,

    // Snapshots de datos históricos
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,

    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// --------------------------------------------------------------------------
// MODELO FETCHED (con relaciones pobladas)
// --------------------------------------------------------------------------

/// Versión enriquecida de `Cita` con relaciones `FETCH` resueltas.
///
/// Usado cuando se consulta la base de datos con `FETCH visitante_id, usuario_id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitaFetched {
    pub id: RecordId,
    pub visitante_id: Option<Visitante>,
    pub usuario_id: User,
    pub motivo: String,
    pub fecha_inicio: Datetime,
    pub fecha_fin: Datetime,
    pub estado: EstadoCita,
    pub activa: bool,
    pub visitante_nombre: Option<String>,
    pub visitante_cedula: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// --------------------------------------------------------------------------
// DTOs DE ENTRADA (Frontend -> Command)
// --------------------------------------------------------------------------

/// Datos necesarios para agendar una nueva cita.
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

/// Datos para actualizar una cita existente.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCitaInput {
    pub fecha_cita: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo: Option<String>,
    pub estado: Option<EstadoCita>, // Uso estricto del Enum
}

// --------------------------------------------------------------------------
// DTOs PARA PERSISTENCIA (Service -> DB)
// --------------------------------------------------------------------------

/// Estructura interna para insertar una Cita en DB.
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
    // Estado y activa se manejan por defecto en creación o en lógica de servicio
}

/// Estructura interna para parches (PATCH) de Cita.
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
    pub estado: Option<EstadoCita>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// --------------------------------------------------------------------------
// DTOs DE SALIDA (Service -> Frontend)
// --------------------------------------------------------------------------

/// Respuesta API estándar para objetos Cita.
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
    pub estado: String, // Se serializa como string para el Frontend
    pub activa: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl CitaResponse {
    /// Construye un `CitaResponse` a partir de un `CitaFetched` con relaciones pobladas.
    ///
    /// Mapea lógica de presentación: nombres completos, formatos de fecha por defecto, etc.
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
            // Fallback: usar datos "snapshot" si no hay relación viva
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
            estado: c.estado.to_string(), // Conversión Display -> String
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
            estado: c.estado.to_string(),
            activa: c.activa,
            created_at: c.created_at.to_string(),
            updated_at: c.updated_at.to_string(),
        }
    }
}

// --------------------------------------------------------------------------
// LIST RESPONSE
// --------------------------------------------------------------------------

/// Respuesta paginada y estadística para listados de citas.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CitaListResponse {
    pub citas: Vec<CitaResponse>,
    pub total: usize,
    pub pendientes: usize,
    pub completadas: usize,
    pub canceladas: usize,
}
