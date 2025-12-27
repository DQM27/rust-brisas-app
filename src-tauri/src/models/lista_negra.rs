// ==========================================
// src/models/lista_negra.rs
// ==========================================
// Modelos para el sistema de Lista Negra (bloqueo universal por cédula)

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// ==========================================
// ENUMS
// ==========================================

/// Nivel de severidad del bloqueo (para feedback visual al guardia)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum NivelSeveridad {
    /// 🔴 Persona peligrosa - Llamar supervisor inmediatamente
    Alto,
    /// 🟡 Persona conflictiva - Precaución
    Medio,
    /// 🟢 Solo prohibido acceso - Trato normal
    Bajo,
}

impl fmt::Display for NivelSeveridad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NivelSeveridad::Alto => write!(f, "ALTO"),
            NivelSeveridad::Medio => write!(f, "MEDIO"),
            NivelSeveridad::Bajo => write!(f, "BAJO"),
        }
    }
}

impl std::str::FromStr for NivelSeveridad {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ALTO" => Ok(NivelSeveridad::Alto),
            "MEDIO" => Ok(NivelSeveridad::Medio),
            "BAJO" => Ok(NivelSeveridad::Bajo),
            _ => Err(format!("Nivel de severidad inválido: {}", s)),
        }
    }
}

impl Default for NivelSeveridad {
    fn default() -> Self {
        NivelSeveridad::Bajo
    }
}

// ==========================================
// MODELO DE DOMINIO
// ==========================================

/// Representa una persona bloqueada en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegra {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    /// FK a empresas (si la empresa está registrada)
    pub empresa_id: Option<String>,
    /// Nombre libre de empresa (si no está registrada)
    pub empresa_nombre: Option<String>,
    /// Nivel de severidad (ALTO/MEDIO/BAJO)
    pub nivel_severidad: String,
    /// Motivo del bloqueo (solo visible para Admin/Supervisor)
    pub motivo_bloqueo: String,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

/// Input para agregar persona a lista negra
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToListaNegraInput {
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    /// FK a empresas (si existe)
    pub empresa_id: Option<String>,
    /// Nombre de empresa (si no existe en sistema)
    pub empresa_nombre: Option<String>,
    /// Nivel de severidad (ALTO/MEDIO/BAJO)
    pub nivel_severidad: String,
    /// Motivo del bloqueo
    pub motivo_bloqueo: String,
    /// ID del usuario que bloquea
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
}

/// Input para actualizar bloqueo existente
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateListaNegraInput {
    pub nivel_severidad: Option<String>,
    pub motivo_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

/// Response completa para Admin/Supervisor (incluye motivo)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub empresa_id: Option<String>,
    pub empresa_nombre: Option<String>,
    pub nivel_severidad: String,
    /// Solo visible para Admin/Supervisor
    pub motivo_bloqueo: String,
    pub bloqueado_por: String,
    pub bloqueado_por_nombre: Option<String>, // Se llena en servicio
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub dias_bloqueado: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ListaNegra> for ListaNegraResponse {
    fn from(ln: ListaNegra) -> Self {
        let nombre_completo = format!(
            "{} {} {} {}",
            ln.nombre,
            ln.segundo_nombre.clone().unwrap_or_default(),
            ln.apellido,
            ln.segundo_apellido.clone().unwrap_or_default()
        )
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

        let created = NaiveDateTime::parse_from_str(&ln.created_at, "%Y-%m-%dT%H:%M:%S%.f%z")
            .or_else(|_| NaiveDateTime::parse_from_str(&ln.created_at, "%Y-%m-%d %H:%M:%S"))
            .unwrap_or_else(|_| Utc::now().naive_utc());

        let dias_bloqueado = (Utc::now().naive_utc() - created).num_days();

        Self {
            id: ln.id,
            cedula: ln.cedula,
            nombre: ln.nombre,
            segundo_nombre: ln.segundo_nombre,
            apellido: ln.apellido,
            segundo_apellido: ln.segundo_apellido,
            nombre_completo,
            empresa_id: ln.empresa_id,
            empresa_nombre: ln.empresa_nombre,
            nivel_severidad: ln.nivel_severidad,
            motivo_bloqueo: ln.motivo_bloqueo,
            bloqueado_por: ln.bloqueado_por,
            bloqueado_por_nombre: None, // Se llena en el servicio
            observaciones: ln.observaciones,
            is_active: ln.is_active,
            dias_bloqueado,
            created_at: ln.created_at,
            updated_at: ln.updated_at,
        }
    }
}

/// Response simplificada para Guardias (SIN motivo, solo nivel/color)
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlockCheckResponse {
    pub is_blocked: bool,
    /// Nivel de severidad (para mostrar color al guardia)
    pub nivel_severidad: Option<String>,
    /// Fecha desde que está bloqueado
    pub bloqueado_desde: Option<String>,
}

/// Lista de bloqueados con estadísticas
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraListResponse {
    pub bloqueados: Vec<ListaNegraResponse>,
    pub total: usize,
    pub activos: usize,
    pub por_nivel: NivelStats,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NivelStats {
    pub alto: usize,
    pub medio: usize,
    pub bajo: usize,
}

/// Resultado de búsqueda de persona para bloquear
/// Permite pre-llenar el formulario de bloqueo
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonaSearchResult {
    /// Tipo de persona: contratista, proveedor, visita
    pub tipo_persona: String,
    /// ID de la entidad (contratista_id, proveedor_id, visitante_id)
    pub entity_id: String,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    /// ID de la empresa (si aplica)
    pub empresa_id: Option<String>,
    /// Nombre de la empresa
    pub empresa_nombre: Option<String>,
    /// Si ya está bloqueado actualmente
    pub ya_bloqueado: bool,
}
