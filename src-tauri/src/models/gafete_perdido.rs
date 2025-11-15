// ==========================================
// src/models/gafete_perdido.rs
// ==========================================
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Estados de pago de un gafete perdido
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPago {
    Pendiente,
    Pagado,
    Condonado,
}

impl EstadoPago {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoPago::Pendiente => "pendiente",
            EstadoPago::Pagado => "pagado",
            EstadoPago::Condonado => "condonado",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "pendiente" => Ok(EstadoPago::Pendiente),
            "pagado" => Ok(EstadoPago::Pagado),
            "condonado" => Ok(EstadoPago::Condonado),
            _ => Err(format!("Estado de pago desconocido: {}", s)),
        }
    }
}

/// Modelo de dominio - Representa un gafete perdido
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GafetePerdido {
    pub id: String,
    pub gafete_id: String,
    pub contratista_id: String,
    pub ingreso_id: Option<String>,
    pub fecha_perdida: String,
    pub monto_cobro: f64,
    pub estado_pago: String,
    pub fecha_pago: Option<String>,
    pub observaciones: Option<String>,
    pub reportado_por: String,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportarGafetePerdidoInput {
    pub gafete_id: String,
    pub contratista_id: String,
    pub ingreso_id: Option<String>,
    pub monto_cobro: f64,
    pub observaciones: Option<String>,
    pub reportado_por: String,  // ID del usuario (guardia/admin)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarPagoInput {
    pub fecha_pago: Option<String>,  // Si es None, usa fecha actual
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CondonarDeudaInput {
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafetePerdidoResponse {
    pub id: String,
    pub gafete_id: String,
    pub gafete_numero: String,              // JOIN con gafetes
    pub contratista_id: String,
    pub contratista_nombre: String,         // JOIN con contratistas
    pub contratista_cedula: String,         // JOIN con contratistas
    pub empresa_nombre: String,             // JOIN con empresas
    pub ingreso_id: Option<String>,
    pub fecha_perdida: String,
    pub monto_cobro: f64,
    pub estado_pago: EstadoPago,
    pub fecha_pago: Option<String>,
    pub observaciones: Option<String>,
    pub reportado_por: String,
    pub reportado_por_nombre: String,       // JOIN con users
    pub dias_sin_pagar: i64,
    pub esta_pendiente: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<GafetePerdido> for GafetePerdidoResponse {
    fn from(gp: GafetePerdido) -> Self {
        let estado_pago = EstadoPago::from_str(&gp.estado_pago).unwrap_or(EstadoPago::Pendiente);
        let esta_pendiente = estado_pago == EstadoPago::Pendiente;
        
        let fecha_perdida_parsed = chrono::NaiveDateTime::parse_from_str(&gp.fecha_perdida, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_else(|_| Utc::now().naive_utc());
        
        let dias_sin_pagar = if esta_pendiente {
            (Utc::now().naive_utc() - fecha_perdida_parsed).num_days()
        } else {
            0
        };
        
        Self {
            id: gp.id,
            gafete_id: gp.gafete_id,
            gafete_numero: String::new(),  // Se llena en comando con JOIN
            contratista_id: gp.contratista_id,
            contratista_nombre: String::new(),  // Se llena en comando con JOIN
            contratista_cedula: String::new(),  // Se llena en comando con JOIN
            empresa_nombre: String::new(),      // Se llena en comando con JOIN
            ingreso_id: gp.ingreso_id,
            fecha_perdida: gp.fecha_perdida,
            monto_cobro: gp.monto_cobro,
            estado_pago,
            fecha_pago: gp.fecha_pago,
            observaciones: gp.observaciones,
            reportado_por: gp.reportado_por,
            reportado_por_nombre: String::new(),  // Se llena en comando con JOIN
            dias_sin_pagar,
            esta_pendiente,
            created_at: gp.created_at,
            updated_at: gp.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GafetesPerdidosListResponse {
    pub perdidos: Vec<GafetePerdidoResponse>,
    pub total: usize,
    pub pendientes: usize,
    pub pagados: usize,
    pub condonados: usize,
    pub monto_total_pendiente: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeudasContratistaResponse {
    pub contratista_id: String,
    pub contratista_nombre: String,
    pub contratista_cedula: String,
    pub gafetes_perdidos: Vec<GafetePerdidoResponse>,
    pub total_deuda: f64,
    pub cantidad_gafetes_perdidos: usize,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use super::*;
    
    pub fn validar_monto(monto: f64) -> Result<(), String> {
        if monto < 0.0 {
            return Err("El monto no puede ser negativo".to_string());
        }
        
        if monto > 1000000.0 {
            return Err("El monto excede el límite permitido".to_string());
        }
        
        Ok(())
    }
    
    pub fn validar_reportar_input(input: &ReportarGafetePerdidoInput) -> Result<(), String> {
        if input.gafete_id.trim().is_empty() {
            return Err("Debe especificar un gafete".to_string());
        }
        
        if input.contratista_id.trim().is_empty() {
            return Err("Debe especificar un contratista".to_string());
        }
        
        validar_monto(input.monto_cobro)?;
        
        Ok(())
    }
}