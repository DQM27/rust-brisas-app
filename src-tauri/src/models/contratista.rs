// ==========================================
// src/models/contratista.rs
// ==========================================
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Modelo de dominio - Representa un contratista en la base de datos
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contratista {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa: String,
    pub fecha_vencimiento_praind: String, // ISO 8601 format
    pub estado: EstadoContratista,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoContratista {
    Activo,
    Inactivo,
    Suspendido,
}

impl EstadoContratista {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoContratista::Activo => "activo",
            EstadoContratista::Inactivo => "inactivo",
            EstadoContratista::Suspendido => "suspendido",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoContratista::Activo),
            "inactivo" => Ok(EstadoContratista::Inactivo),
            "suspendido" => Ok(EstadoContratista::Suspendido),
            _ => Err(format!("Estado desconocido: {}", s)),
        }
    }
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContratistaInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa: String,
    pub fecha_vencimiento_praind: String, // "YYYY-MM-DD"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContratistaInput {
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub empresa: Option<String>,
    pub fecha_vencimiento_praind: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambiarEstadoInput {
    pub estado: String, // "activo", "inactivo", "suspendido"
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContratistaResponse {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub empresa: String,
    pub fecha_vencimiento_praind: String,
    pub estado: EstadoContratista,
    pub puede_ingresar: bool,
    pub praind_vencido: bool,
    pub dias_hasta_vencimiento: i64,
    pub requiere_atencion: bool, // true si vence en <= 30 días
    pub created_at: String,
    pub updated_at: String,
}

impl From<Contratista> for ContratistaResponse {
    fn from(c: Contratista) -> Self {
        let fecha_vencimiento = NaiveDate::parse_from_str(&c.fecha_vencimiento_praind, "%Y-%m-%d")
            .unwrap_or_else(|_| Utc::now().date_naive());
        
        let hoy = Utc::now().date_naive();
        let dias_hasta_vencimiento = (fecha_vencimiento - hoy).num_days();
        let praind_vencido = dias_hasta_vencimiento < 0;
        let requiere_atencion = dias_hasta_vencimiento <= 30 && dias_hasta_vencimiento >= 0;
        let puede_ingresar = c.estado == EstadoContratista::Activo && !praind_vencido;
        
        Self {
            id: c.id,
            cedula: c.cedula.clone(),
            nombre: c.nombre.clone(),
            apellido: c.apellido.clone(),
            nombre_completo: format!("{} {}", c.nombre, c.apellido),
            empresa: c.empresa,
            fecha_vencimiento_praind: c.fecha_vencimiento_praind,
            estado: c.estado,
            puede_ingresar,
            praind_vencido,
            dias_hasta_vencimiento,
            requiere_atencion,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContratistaListResponse {
    pub contratistas: Vec<ContratistaResponse>,
    pub total: usize,
    pub activos: usize,
    pub con_praind_vencido: usize,
    pub requieren_atencion: usize,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use chrono::NaiveDate;
    
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
    
    pub fn validar_apellido(apellido: &str) -> Result<(), String> {
        let limpio = apellido.trim();
        
        if limpio.is_empty() {
            return Err("El apellido no puede estar vacío".to_string());
        }
        
        if limpio.len() > 50 {
            return Err("El apellido no puede exceder 50 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_empresa(empresa: &str) -> Result<(), String> {
        let limpia = empresa.trim();
        
        if limpia.is_empty() {
            return Err("El nombre de la empresa no puede estar vacío".to_string());
        }
        
        if limpia.len() > 100 {
            return Err("El nombre de la empresa no puede exceder 100 caracteres".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_fecha(fecha_str: &str) -> Result<NaiveDate, String> {
        NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d")
            .map_err(|_| "Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    }
    
    pub fn validar_create_input(input: &super::CreateContratistaInput) -> Result<(), String> {
        validar_cedula(&input.cedula)?;
        validar_nombre(&input.nombre)?;
        validar_apellido(&input.apellido)?;
        validar_empresa(&input.empresa)?;
        validar_fecha(&input.fecha_vencimiento_praind)?;
        Ok(())
    }
}