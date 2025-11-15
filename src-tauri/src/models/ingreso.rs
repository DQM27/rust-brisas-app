// ==========================================
// src/models/ingreso.rs
// ==========================================
use serde::{Deserialize, Serialize};

/// Tipo de ingreso
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoIngreso {
    Contratista,
    Temporal,
}

impl TipoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            TipoIngreso::Contratista => "contratista",
            TipoIngreso::Temporal => "temporal",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(TipoIngreso::Contratista),
            "temporal" => Ok(TipoIngreso::Temporal),
            _ => Err(format!("Tipo de ingreso desconocido: {}", s)),
        }
    }
}

/// Tipo de autorización
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TipoAutorizacion {
    Praind,
    Correo,
}

impl TipoAutorizacion {
    pub fn as_str(&self) -> &str {
        match self {
            TipoAutorizacion::Praind => "praind",
            TipoAutorizacion::Correo => "correo",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "praind" => Ok(TipoAutorizacion::Praind),
            "correo" => Ok(TipoAutorizacion::Correo),
            _ => Err(format!("Tipo de autorización desconocido: {}", s)),
        }
    }
}

/// Modo de ingreso (transporte)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModoIngreso {
    Caminando,
    Vehiculo,
    VehiculoTemporal,
}

impl ModoIngreso {
    pub fn as_str(&self) -> &str {
        match self {
            ModoIngreso::Caminando => "caminando",
            ModoIngreso::Vehiculo => "vehiculo",
            ModoIngreso::VehiculoTemporal => "vehiculo_temporal",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(ModoIngreso::Caminando),
            "vehiculo" => Ok(ModoIngreso::Vehiculo),
            "vehiculo_temporal" => Ok(ModoIngreso::VehiculoTemporal),
            _ => Err(format!("Modo de ingreso desconocido: {}", s)),
        }
    }
}

/// Modelo de dominio - Representa un ingreso/salida
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingreso {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: String,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub vehiculo_id: Option<String>,
    pub placa_temporal: Option<String>,
    pub gafete_id: String,
    pub gafete_numero: String,
    pub fecha_hora_ingreso: String,
    pub fecha_hora_salida: Option<String>,
    pub tiempo_permanencia_minutos: Option<i64>,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ==========================================
// DTOs de entrada (Commands/Input)
// ==========================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub vehiculo_id: Option<String>,  // Si ingresa en vehículo registrado
    pub gafete_id: String,            // Default: "sin-gafete-default" (S/G)
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,   // Guardia logueado
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoTemporalInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub placa_temporal: Option<String>,  // Si ingresa en vehículo
    pub gafete_id: String,               // Default: "sin-gafete-default"
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaInput {
    pub devolvio_gafete: bool,           // true = devolvió, false = perdido
    pub usuario_salida_id: String,       // Guardia logueado
    pub observaciones_salida: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaConGafetePerdidoInput {
    pub monto_cobro: f64,
    pub usuario_salida_id: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CerrarIngresoAnteriorInput {
    pub cerrar_anterior: bool,  // true = cerrar y crear nuevo, false = cancelar
}

// ==========================================
// DTOs de salida (Response/ViewModel)
// ==========================================

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoResponse {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: TipoIngreso,
    pub tipo_autorizacion: TipoAutorizacion,
    pub modo_ingreso: ModoIngreso,
    pub vehiculo_id: Option<String>,
    pub vehiculo_placa: Option<String>,         // JOIN o placa_temporal
    pub placa_temporal: Option<String>,
    pub gafete_id: String,
    pub gafete_numero: String,
    pub fecha_hora_ingreso: String,
    pub fecha_hora_salida: Option<String>,
    pub tiempo_permanencia_minutos: Option<i64>,
    pub tiempo_permanencia_texto: Option<String>,  // "2h 30m"
    pub usuario_ingreso_id: String,
    pub usuario_ingreso_nombre: String,         // JOIN con users
    pub usuario_salida_id: Option<String>,
    pub usuario_salida_nombre: Option<String>,  // JOIN con users
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    pub observaciones: Option<String>,
    pub esta_adentro: bool,                     // true si hora_salida IS NULL
    pub tiene_gafete_asignado: bool,            // true si gafete != S/G
    pub created_at: String,
    pub updated_at: String,
}

impl From<Ingreso> for IngresoResponse {
    fn from(i: Ingreso) -> Self {
        let esta_adentro = i.fecha_hora_salida.is_none();
        let tiene_gafete_asignado = i.gafete_numero.to_uppercase() != "S/G";
        
        let tiempo_permanencia_texto = i.tiempo_permanencia_minutos.map(|mins| {
            let horas = mins / 60;
            let minutos = mins % 60;
            if horas > 0 {
                format!("{}h {}m", horas, minutos)
            } else {
                format!("{}m", minutos)
            }
        });
        
        Self {
            id: i.id,
            contratista_id: i.contratista_id,
            cedula: i.cedula.clone(),
            nombre: i.nombre.clone(),
            apellido: i.apellido.clone(),
            nombre_completo: format!("{} {}", i.nombre, i.apellido),
            empresa_nombre: i.empresa_nombre,
            tipo_ingreso: TipoIngreso::from_str(&i.tipo_ingreso).unwrap_or(TipoIngreso::Temporal),
            tipo_autorizacion: TipoAutorizacion::from_str(&i.tipo_autorizacion).unwrap_or(TipoAutorizacion::Correo),
            modo_ingreso: ModoIngreso::from_str(&i.modo_ingreso).unwrap_or(ModoIngreso::Caminando),
            vehiculo_id: i.vehiculo_id,
            vehiculo_placa: None,  // Se llena en comando con JOIN
            placa_temporal: i.placa_temporal,
            gafete_id: i.gafete_id,
            gafete_numero: i.gafete_numero,
            fecha_hora_ingreso: i.fecha_hora_ingreso,
            fecha_hora_salida: i.fecha_hora_salida,
            tiempo_permanencia_minutos: i.tiempo_permanencia_minutos,
            tiempo_permanencia_texto,
            usuario_ingreso_id: i.usuario_ingreso_id,
            usuario_ingreso_nombre: String::new(),  // Se llena en comando
            usuario_salida_id: i.usuario_salida_id,
            usuario_salida_nombre: None,  // Se llena en comando
            praind_vigente_al_ingreso: i.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: i.estado_contratista_al_ingreso,
            observaciones: i.observaciones,
            esta_adentro,
            tiene_gafete_asignado,
            created_at: i.created_at,
            updated_at: i.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoListResponse {
    pub ingresos: Vec<IngresoResponse>,
    pub total: usize,
    pub adentro: usize,
    pub salieron: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoAbiertoConAlerta {
    pub tiene_ingreso_abierto: bool,
    pub ingreso: Option<IngresoResponse>,
    pub mensaje: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,  // Deudas de gafetes, PRAIND próximo a vencer, etc.
    pub contratista: Option<serde_json::Value>,  // Datos del contratista si existe
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoResponse>,
}

// ==========================================
// Validaciones de dominio
// ==========================================

pub mod validaciones {
    use super::*;
    
    pub fn validar_cedula(cedula: &str) -> Result<(), String> {
        let limpia = cedula.trim();
        
        if limpia.is_empty() {
            return Err("La cédula no puede estar vacía".to_string());
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
        
        Ok(())
    }
    
    pub fn validar_create_temporal_input(input: &CreateIngresoTemporalInput) -> Result<(), String> {
        validar_cedula(&input.cedula)?;
        validar_nombre(&input.nombre)?;
        validar_nombre(&input.apellido)?;
        validar_nombre(&input.empresa_nombre)?;
        Ok(())
    }
}