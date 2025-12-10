// ==========================================
// src/models/lista_negra.rs
// ==========================================
// Solo modelos, DTOs y helpers básicos - SIN validaciones ni lógica

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

/// Representa una persona bloqueada en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegra {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub fecha_fin_bloqueo: Option<String>,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToListaNegraInput {
    pub contratista_id: Option<String>,
    pub cedula: Option<String>,
    pub nombre: Option<String>,
    pub segundo_nombre: Option<String>,
    pub apellido: Option<String>,
    pub segundo_apellido: Option<String>,
    pub motivo_bloqueo: String,
    pub fecha_fin_bloqueo: Option<String>,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateListaNegraInput {
    pub motivo_bloqueo: Option<String>,
    pub fecha_fin_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraResponse {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub segundo_nombre: Option<String>,
    pub apellido: String,
    pub segundo_apellido: Option<String>,
    pub nombre_completo: String,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub fecha_fin_bloqueo: Option<String>,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub es_bloqueo_permanente: bool,
    pub dias_transcurridos: i64,
    pub empresa_nombre: Option<String>,
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

        let es_bloqueo_permanente = ln.fecha_fin_bloqueo.is_none();

        let fecha_inicio =
            NaiveDateTime::parse_from_str(&ln.fecha_inicio_bloqueo, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| Utc::now().naive_utc());

        let dias_transcurridos = (Utc::now().naive_utc() - fecha_inicio).num_days();

        Self {
            id: ln.id,
            contratista_id: ln.contratista_id,
            cedula: ln.cedula,
            nombre: ln.nombre.clone(),
            segundo_nombre: ln.segundo_nombre.clone(),
            apellido: ln.apellido.clone(),
            segundo_apellido: ln.segundo_apellido.clone(),
            nombre_completo,
            motivo_bloqueo: ln.motivo_bloqueo,
            fecha_inicio_bloqueo: ln.fecha_inicio_bloqueo,
            fecha_fin_bloqueo: ln.fecha_fin_bloqueo,
            bloqueado_por: ln.bloqueado_por,
            observaciones: ln.observaciones,
            is_active: ln.is_active,
            es_bloqueo_permanente,
            dias_transcurridos,
            empresa_nombre: None, // Se llena en el servicio
            created_at: ln.created_at,
            updated_at: ln.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraListResponse {
    pub bloqueados: Vec<ListaNegraResponse>,
    pub total: usize,
    pub activos: usize,
    pub permanentes: usize,
    pub temporales: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockCheckResponse {
    pub is_blocked: bool,
    pub motivo: Option<String>,
    pub bloqueado_desde: Option<String>,
    pub bloqueado_hasta: Option<String>,
    pub bloqueado_por: Option<String>,
}
