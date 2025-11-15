// ==========================================
// src/models/ingreso.rs
// ==========================================
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

// ==========================================
// MODELO DE DOMINIO
// ==========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingreso {
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub tipo_ingreso: TipoIngreso,
    pub tipo_autorizacion: TipoAutorizacion,
    pub modo_ingreso: ModoIngreso,
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

impl Ingreso {
    /// Verifica si el ingreso está abierto (persona adentro)
    pub fn esta_abierto(&self) -> bool {
        self.fecha_hora_salida.is_none()
    }
    
    /// Nombre completo de la persona
    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }
    
    /// Calcula el tiempo de permanencia si ya salió
    pub fn calcular_permanencia(&self) -> Option<i64> {
        self.tiempo_permanencia_minutos
    }
    
    /// Verifica si es un ingreso temporal
    pub fn es_temporal(&self) -> bool {
        self.tipo_ingreso == TipoIngreso::Temporal
    }
    
    /// Verifica si ingresó con vehículo
    pub fn ingreso_con_vehiculo(&self) -> bool {
        matches!(
            self.modo_ingreso,
            ModoIngreso::Vehiculo | ModoIngreso::VehiculoTemporal
        )
    }
}

// ==========================================
// ENUMS
// ==========================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TipoIngreso {
    Contratista,
    Temporal,
}

impl TipoIngreso {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Contratista => "contratista",
            Self::Temporal => "temporal",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "contratista" => Ok(Self::Contratista),
            "temporal" => Ok(Self::Temporal),
            _ => Err(format!("Tipo de ingreso desconocido: '{}'", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TipoAutorizacion {
    Praind,
    Correo,
}

impl TipoAutorizacion {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Praind => "praind",
            Self::Correo => "correo",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "praind" => Ok(Self::Praind),
            "correo" => Ok(Self::Correo),
            _ => Err(format!("Tipo de autorización desconocido: '{}'", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModoIngreso {
    Caminando,
    Vehiculo,
    VehiculoTemporal,
}

impl ModoIngreso {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Caminando => "caminando",
            Self::Vehiculo => "vehiculo",
            Self::VehiculoTemporal => "vehiculo_temporal",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "caminando" => Ok(Self::Caminando),
            "vehiculo" => Ok(Self::Vehiculo),
            "vehiculo_temporal" => Ok(Self::VehiculoTemporal),
            _ => Err(format!("Modo de ingreso desconocido: '{}'", s)),
        }
    }
}

// ==========================================
// DTOs DE ENTRADA
// ==========================================

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoContratistaInput {
    pub contratista_id: String,
    pub vehiculo_id: Option<String>,
    pub gafete_id: String,
    pub usuario_ingreso_id: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoTemporalInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_nombre: String,
    pub placa_temporal: Option<String>,
    pub gafete_id: String,
    pub usuario_ingreso_id: String,
    pub observaciones: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaInput {
    pub usuario_salida_id: String,
    pub devolvio_gafete: bool,
    pub observaciones_salida: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrarSalidaConGafetePerdidoInput {
    pub usuario_salida_id: String,
    pub monto_cobro: f64,
    pub observaciones: Option<String>,
}

// ==========================================
// DTOs DE SALIDA
// ==========================================

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoResponse {
    // Datos básicos
    pub id: String,
    pub contratista_id: Option<String>,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub nombre_completo: String,
    pub empresa_nombre: String,
    
    // Tipo y modo
    pub tipo_ingreso: TipoIngreso,
    pub tipo_autorizacion: TipoAutorizacion,
    pub modo_ingreso: ModoIngreso,
    
    // Vehículo
    pub vehiculo_id: Option<String>,
    pub vehiculo_placa: Option<String>,
    pub placa_temporal: Option<String>,
    
    // Gafete
    pub gafete_id: String,
    pub gafete_numero: String,
    
    // Fechas y tiempos
    pub fecha_hora_ingreso: String,
    pub fecha_hora_salida: Option<String>,
    pub tiempo_permanencia_minutos: Option<i64>,
    
    // Usuarios
    pub usuario_ingreso_id: String,
    pub usuario_ingreso_nombre: Option<String>,
    pub usuario_salida_id: Option<String>,
    pub usuario_salida_nombre: Option<String>,
    
    // Estado al ingresar
    pub praind_vigente_al_ingreso: Option<bool>,
    pub estado_contratista_al_ingreso: Option<String>,
    
    // Campos calculados
    pub esta_adentro: bool,
    
    // Metadata
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl IngresoResponse {
    pub fn new(
        ingreso: Ingreso,
        usuario_ingreso_nombre: Option<String>,
        usuario_salida_nombre: Option<String>,
        vehiculo_placa: Option<String>,
    ) -> Self {
        Self {
            id: ingreso.id,
            contratista_id: ingreso.contratista_id,
            cedula: ingreso.cedula.clone(),
            nombre: ingreso.nombre.clone(),
            apellido: ingreso.apellido.clone(),
            nombre_completo: ingreso.nombre_completo(),
            empresa_nombre: ingreso.empresa_nombre,
            tipo_ingreso: ingreso.tipo_ingreso,
            tipo_autorizacion: ingreso.tipo_autorizacion,
            modo_ingreso: ingreso.modo_ingreso,
            vehiculo_id: ingreso.vehiculo_id,
            vehiculo_placa,
            placa_temporal: ingreso.placa_temporal,
            gafete_id: ingreso.gafete_id,
            gafete_numero: ingreso.gafete_numero,
            fecha_hora_ingreso: ingreso.fecha_hora_ingreso,
            fecha_hora_salida: ingreso.fecha_hora_salida,
            tiempo_permanencia_minutos: ingreso.tiempo_permanencia_minutos,
            usuario_ingreso_id: ingreso.usuario_ingreso_id,
            usuario_ingreso_nombre,
            usuario_salida_id: ingreso.usuario_salida_id,
            usuario_salida_nombre,
            praind_vigente_al_ingreso: ingreso.praind_vigente_al_ingreso,
            estado_contratista_al_ingreso: ingreso.estado_contratista_al_ingreso,
            esta_adentro: ingreso.esta_abierto(),
            observaciones: ingreso.observaciones,
            created_at: ingreso.created_at,
            updated_at: ingreso.updated_at,
        }
    }
}

impl From<Ingreso> for IngresoResponse {
    fn from(ingreso: Ingreso) -> Self {
        Self::new(ingreso, None, None, None)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoListResponse {
    pub ingresos: Vec<IngresoResponse>,
    pub total: usize,
    pub adentro: usize,
    pub salieron: usize,
}

impl IngresoListResponse {
    pub fn new(ingresos: Vec<IngresoResponse>) -> Self {
        let total = ingresos.len();
        let adentro = ingresos.iter().filter(|i| i.esta_adentro).count();
        let salieron = total - adentro;
        
        Self {
            ingresos,
            total,
            adentro,
            salieron,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub contratista: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoResponse>,
}

// ==========================================
// CONSTANTES
// ==========================================

pub const GAFETE_SIN_GAFETE: &str = "S/G";

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tipo_ingreso_from_str() {
        assert_eq!(
            TipoIngreso::from_str("contratista").unwrap(),
            TipoIngreso::Contratista
        );
        assert_eq!(
            TipoIngreso::from_str("TEMPORAL").unwrap(),
            TipoIngreso::Temporal
        );
        assert!(TipoIngreso::from_str("invalido").is_err());
    }
    
    #[test]
    fn test_modo_ingreso() {
        assert_eq!(ModoIngreso::Caminando.as_str(), "caminando");
        assert_eq!(ModoIngreso::Vehiculo.as_str(), "vehiculo");
    }
    
    #[test]
    fn test_ingreso_esta_abierto() {
        let ingreso = Ingreso {
            id: "1".to_string(),
            contratista_id: Some("c1".to_string()),
            cedula: "123".to_string(),
            nombre: "Juan".to_string(),
            apellido: "Pérez".to_string(),
            empresa_nombre: "Empresa 1".to_string(),
            tipo_ingreso: TipoIngreso::Contratista,
            tipo_autorizacion: TipoAutorizacion::Praind,
            modo_ingreso: ModoIngreso::Caminando,
            vehiculo_id: None,
            placa_temporal: None,
            gafete_id: "g1".to_string(),
            gafete_numero: "001".to_string(),
            fecha_hora_ingreso: "2024-01-01 10:00:00".to_string(),
            fecha_hora_salida: None,
            tiempo_permanencia_minutos: None,
            usuario_ingreso_id: "u1".to_string(),
            usuario_salida_id: None,
            praind_vigente_al_ingreso: Some(true),
            estado_contratista_al_ingreso: Some("activo".to_string()),
            observaciones: None,
            created_at: "2024-01-01T10:00:00Z".to_string(),
            updated_at: "2024-01-01T10:00:00Z".to_string(),
        };
        
        assert!(ingreso.esta_abierto());
        assert!(!ingreso.es_temporal());
    }
}