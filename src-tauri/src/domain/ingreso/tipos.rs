// ==========================================
// src/domain/ingreso/tipos.rs
// ==========================================
// Tipos y estructuras compartidas entre módulos de ingreso

use serde::{Deserialize, Serialize};

// ==========================================
// CONSTANTES DE NEGOCIO
// ==========================================

/// Tiempo máximo de permanencia en instalaciones (horas)
pub const TIEMPO_MAXIMO_HORAS: i64 = 14;

/// Tiempo para alerta temprana (minutos) - 30 minutos antes del límite
pub const TIEMPO_ALERTA_TEMPRANA_MINUTOS: i64 = 13 * 60 + 30; // 13h 30min

/// Tiempo máximo en minutos
pub const TIEMPO_MAXIMO_MINUTOS: i64 = TIEMPO_MAXIMO_HORAS * 60; // 840 minutos

// ==========================================
// ENUMS DE ESTADO
// ==========================================

/// Estado del tiempo de permanencia de un contratista
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPermanencia {
    /// Todo normal, tiempo < 13h 30min
    Normal,
    /// Alerta temprana, tiempo >= 13h 30min y < 14h
    AlertaTemprana,
    /// Tiempo excedido, >= 14h
    TiempoExcedido,
}

impl EstadoPermanencia {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoPermanencia::Normal => "normal",
            EstadoPermanencia::AlertaTemprana => "alerta_temprana",
            EstadoPermanencia::TiempoExcedido => "tiempo_excedido",
        }
    }
}

// ==========================================
// ESTRUCTURAS DE RESULTADO
// ==========================================

/// Resultado de validación de entrada
#[derive(Debug, Clone)]
pub struct ResultadoValidacionEntrada {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>, // warnings no bloqueantes
}

// ==========================================
// TIPOS PARA STRATEGY PATTERN
// ==========================================

/// Datos necesarios para validar un ingreso (agnóstico del tipo)
#[derive(Debug, Clone)]
pub struct DatosValidacion {
    pub cedula: String,
    pub nombre: Option<String>,
    pub apellido: Option<String>,
    pub tipo_ingreso: String,

    // Para contratistas
    pub contratista_id: Option<String>,

    // Para visitas
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo_visita: Option<String>,

    // Para proveedores
    pub empresa_id: Option<String>,
    pub motivo_proveedor: Option<String>,

    // Comunes
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub gafete_numero: Option<String>,
    pub vehiculo_id: Option<String>,
    pub placa_temporal: Option<String>,
}

/// Resultado de validación con datos adicionales
#[derive(Debug, Clone)]
pub struct ResultadoValidacion {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub datos_adicionales: Option<serde_json::Value>,
}

/// Datos preparados para inserción en DB
#[derive(Debug, Clone)]
pub struct DatosIngreso {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub empresa_proveedor_id: Option<String>,
    pub anfitrion: Option<String>,
    pub area_visitada: Option<String>,
    pub motivo_visita: Option<String>,
    pub motivo_proveedor: Option<String>,
    pub tipo_ingreso: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_id: Option<String>,
    pub placa_temporal: Option<String>,
    pub gafete_numero: Option<String>,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
}

/// Resultado de evaluación de devolución de gafete
#[derive(Debug, Clone)]
pub struct DecisionReporteGafete {
    pub debe_generar_reporte: bool,
    pub motivo: Option<String>,
    pub gafete_numero: Option<String>,
}

/// Información de alerta de tiempo
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaTiempo {
    pub estado: EstadoPermanencia,
    pub minutos_transcurridos: i64,
    pub minutos_restantes: i64,
    pub mensaje: Option<String>,
}