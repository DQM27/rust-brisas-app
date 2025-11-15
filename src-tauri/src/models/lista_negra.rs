// ==========================================
// src/models/lista_negra.rs
// ==========================================
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Modelo de dominio - Representa una persona bloqueada
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegra {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
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
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToListaNegraInput {
    pub contratista_id: Option<String>, // Si ya existe en BD
    pub cedula: Option<String>,         // Requerido si no hay contratista_id
    pub nombre: Option<String>,         // Requerido si no hay contratista_id
    pub apellido: Option<String>,       // Requerido si no hay contratista_id
    pub motivo_bloqueo: String,
    pub fecha_fin_bloqueo: Option<String>, // NULL = permanente
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateListaNegraInput {
    pub motivo_bloqueo: Option<String>,
    pub fecha_fin_bloqueo: Option<String>,
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraResponse {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub motivo_bloqueo: String,
    pub fecha_inicio_bloqueo: String,
    pub fecha_fin_bloqueo: Option<String>,
    pub bloqueado_por: String,
    pub observaciones: Option<String>,
    pub is_active: bool,
    pub es_bloqueo_permanente: bool,
    pub dias_transcurridos: i64,
    pub empresa_nombre: Option<String>, // Si tiene contratista_id
    pub created_at: String,
    pub updated_at: String,
}

impl From<ListaNegra> for ListaNegraResponse {
    fn from(ln: ListaNegra) -> Self {
        let nombre = ln.nombre.clone().unwrap_or_default();
        let apellido = ln.apellido.clone().unwrap_or_default();
        let es_bloqueo_permanente = ln.fecha_fin_bloqueo.is_none();
        
        let fecha_inicio = NaiveDateTime::parse_from_str(&ln.fecha_inicio_bloqueo, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_else(|_| Utc::now().naive_utc());
        
        let dias_transcurridos = (Utc::now().naive_utc() - fecha_inicio).num_days();
        
        Self {
            id: ln.id,
            contratista_id: ln.contratista_id,
            cedula: ln.cedula,
            nombre: nombre.clone(),
            apellido: apellido.clone(),
            nombre_completo: format!("{} {}", nombre, apellido).trim().to_string(),
            motivo_bloqueo: ln.motivo_bloqueo,
            fecha_inicio_bloqueo: ln.fecha_inicio_bloqueo,
            fecha_fin_bloqueo: ln.fecha_fin_bloqueo,
            bloqueado_por: ln.bloqueado_por,
            observaciones: ln.observaciones,
            is_active: ln.is_active,
            es_bloqueo_permanente,
            dias_transcurridos,
            empresa_nombre: None, // Se llena en el comando con JOIN
            created_at: ln.created_at,
            updated_at: ln.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaNegraListResponse {
    pub bloqueados: Vec<ListaNegraResponse>,
    pub total: usize,
    pub activos: usize,
    pub permanentes: usize,
    pub temporales: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockCheckResponse {
    pub is_blocked: bool,
    pub motivo: Option<String>,
    pub bloqueado_desde: Option<String>,
    pub bloqueado_hasta: Option<String>,
    pub bloqueado_por: Option<String>,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use super::*;
    use chrono::NaiveDateTime;
    
    pub fn validar_cedula(cedula: &str) -> Result<(), String> {
        let limpia = cedula.trim();
        
        if limpia.is_empty() {
            return Err("La cédula no puede estar vacía".to_string());
        }
        
        if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
            return Err("La cédula solo puede contener números y guiones".to_string());
        }
        
        if limpia.len() < 7 || limpia.len() > 20 {
            return Err("La cédula debe tener entre 7 y 20 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_nombre(nombre: &str) -> Result<(), String> {
        let limpio = nombre.trim();
        
        if limpio.is_empty() {
            return Err("El nombre no puede estar vacío".to_string());
        }
        
        if limpio.len() > 50 {
            return Err("El nombre no puede exceder 50 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_motivo(motivo: &str) -> Result<(), String> {
        let limpio = motivo.trim();
        
        if limpio.is_empty() {
            return Err("Debe especificar un motivo de bloqueo".to_string());
        }
        
        if limpio.len() > 500 {
            return Err("El motivo no puede exceder 500 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_fecha_fin(fecha_str: &str) -> Result<NaiveDateTime, String> {
        NaiveDateTime::parse_from_str(fecha_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD HH:MM:SS".to_string())
    }
    
    pub fn validar_add_input(input: &AddToListaNegraInput) -> Result<(), String> {
        // Si tiene contratista_id, no necesita cédula/nombre/apellido
        if input.contratista_id.is_some() {
            validaciones::validar_motivo(&input.motivo_bloqueo)?;
            return Ok(());
        }
        
        // Si NO tiene contratista_id, requiere cédula + nombre + apellido
        let cedula = input.cedula.as_ref()
            .ok_or("Debe proporcionar cédula si no especifica contratista_id")?;
        validar_cedula(cedula)?;
        
        let nombre = input.nombre.as_ref()
            .ok_or("Debe proporcionar nombre si no especifica contratista_id")?;
        validar_nombre(nombre)?;
        
        let apellido = input.apellido.as_ref()
            .ok_or("Debe proporcionar apellido si no especifica contratista_id")?;
        validar_nombre(apellido)?;
        
        validaciones::validar_motivo(&input.motivo_bloqueo)?;
        
        // Validar fecha_fin si existe
        if let Some(ref fecha) = input.fecha_fin_bloqueo {
            validar_fecha_fin(fecha)?;
        }
        
        Ok(())
    }
}