use serde::{Deserialize, Serialize};

// --------------------------------------------------------------------------
// DEFINICIONES DE MODELO PARA MOTOR DE VALIDACIÓN
// --------------------------------------------------------------------------

/// Niveles de severidad para listas negras y alertas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NivelSeveridad {
    Alto,
    Medio,
    Bajo,
}

impl NivelSeveridad {
    /// Convierte un string arbitrario a un nivel de severidad.
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "ALTO" | "HIGH" => Self::Alto,
            "MEDIO" | "MEDIUM" => Self::Medio,
            "BAJO" | "LOW" => Self::Bajo,
            _ => Self::Alto, // Default to Alto for safety
        }
    }
}

/// Estados de validación del motor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    Allowed,
    Denied,
    Warning,
}

/// Razones canónicas de rechazo o advertencia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationReason {
    None,
    Blacklisted,
    AlreadyInside,
    ExpiredDocuments,
    GafeteAlert,
}

/// Resultado de la ejecución del motor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub reason: ValidationReason,
    pub message: String,
}

/// Tipo de acceso que solicita la persona.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TipoAcceso {
    Contratista,
    Visitante,
    Proveedor,
    Manual, // Para casos especiales
}

/// Estado de los documentos o contratos de la persona.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstadoAutorizacion {
    Activo,
    Vencido,
    Inactivo,
    Suspendido,
    PorDefinir,
}

impl EstadoAutorizacion {
    /// Convierte un string arbitrario (BD) a un estado del motor.
    /// Esto actúa como un "anti-corruption layer" para normalizar los estados.
    pub fn from_str_lossy(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "activo" | "active" | "authorized" | "ok" => Self::Activo,
            "vencido" | "expired" => Self::Vencido,
            "inactivo" | "inactive" => Self::Inactivo,
            "suspendido" | "suspended" => Self::Suspendido,
            _ => Self::PorDefinir,
        }
    }
}

/// Contexto puro para evaluación de reglas.
#[derive(Debug, Clone)]
pub struct MotorContexto {
    pub ident_cedula: String,
    pub ident_nombre: String,
    pub tipo_acceso: TipoAcceso,
    pub lista_negra: Option<InfoListaNegra>,
    pub ingreso_activo: Option<InfoIngresoActivoInt>,
    pub estado_autorizacion: EstadoAutorizacion,
    pub alerta_gafete: Option<String>,
}

/// Detalle de restricción (Lista Negra).
#[derive(Debug, Clone)]
pub struct InfoListaNegra {
    pub motivo: String,
    pub severidad: NivelSeveridad,
}

/// Detalle de permanencia actual.
#[derive(Debug, Clone)]
pub struct InfoIngresoActivoInt {
    pub id: String,
    pub fecha_ingreso: String,
    pub gafete_numero: i32,
}
