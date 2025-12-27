use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisita {
    pub id: String,
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String, // 'ADENTRO', 'SALIO'
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoVisitaInput {
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoVisitaFullInput {
    // Datos Visitante
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa: Option<String>,

    // Datos Ingreso
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,

    // Opcional: Cita ID si viene de cita (aunque si es full, suele ser manual)
    pub cita_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngresoVisitaPopulated {
    // Ingreso Fields
    pub id: String,
    pub visitante_id: String,
    pub cita_id: Option<String>,
    pub anfitrion: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,

    // Visitante Fields (Joined)
    pub visitante_nombre: String,
    pub visitante_apellido: String,
    pub visitante_cedula: String,
    pub visitante_empresa: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoVisitaResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub visitante: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoVisita>,
}

use crate::domain::errors::IngresoVisitaError;

// Re-exports from common module
pub use crate::domain::common::{normalizar_numero_gafete, DecisionReporteGafete};

// Import common functions for internal use
use crate::domain::common as common_domain;

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoVisitaError> {
    if fecha_salida.is_some() {
        return Err(IngresoVisitaError::NoActiveIngreso);
    }
    Ok(())
}

/// Wrapper que usa el error específico de visita
pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoVisitaError> {
    common_domain::validar_tiempo_salida(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))
}

/// Wrapper que usa el error específico de visita
pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoVisitaError> {
    common_domain::calcular_tiempo_permanencia(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoVisitaError::Validation(e.to_string()))
}

/// Delega a common::evaluar_devolucion_gafete
pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<&str>,
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<&str>,
) -> DecisionReporteGafete {
    common_domain::evaluar_devolucion_gafete(
        tenia_gafete,
        gafete_asignado,
        reporto_devolucion,
        gafete_devuelto_numero,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_tiempo_salida() {
        let ingreso = "2023-12-22T08:00:00Z";
        let salida_valida = "2023-12-22T09:00:00Z";
        let salida_invalida = "2023-12-22T07:59:59Z";

        assert!(validar_tiempo_salida(ingreso, salida_valida).is_ok());
        assert!(validar_tiempo_salida(ingreso, salida_invalida).is_err());
    }

    #[test]
    fn test_calcular_tiempo_permanencia() {
        let ingreso = "2023-12-22T08:00:00Z";
        let salida = "2023-12-22T08:15:00Z";
        assert_eq!(calcular_tiempo_permanencia(ingreso, salida).unwrap(), 15);
    }

    #[test]
    fn test_evaluar_devolucion_gafete() {
        // OK
        let res = evaluar_devolucion_gafete(true, Some("V-5"), true, Some("V-5"));
        assert!(!res.debe_generar_reporte);

        // Sin devolver
        let res = evaluar_devolucion_gafete(true, Some("V-5"), false, None);
        assert!(res.debe_generar_reporte);

        // Gafete incorrecto
        let res = evaluar_devolucion_gafete(true, Some("V-5"), true, Some("V-6"));
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("incorrecto"));
    }
}
