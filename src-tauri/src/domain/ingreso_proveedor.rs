use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IngresoProveedor {
    pub id: String,
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub proveedor_id: Option<String>,
    pub empresa_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub tipo_autorizacion: Option<String>,
    pub modo_ingreso: Option<String>,
    pub placa_vehiculo: Option<String>,
    pub fecha_ingreso: String,
    pub fecha_salida: Option<String>,
    pub estado: String,
    pub usuario_ingreso_id: String,
    pub usuario_salida_id: Option<String>,
    pub observaciones: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    // Populated fields (from JOINs)
    #[serde(default)]
    pub usuario_ingreso_nombre: String,
    #[serde(default)]
    pub usuario_salida_nombre: String,
    #[serde(default)]
    pub empresa_nombre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngresoProveedorInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub area_visitada: String,
    pub motivo: String,
    pub gafete: Option<String>,
    pub tipo_autorizacion: String,
    pub modo_ingreso: String,
    pub tipo_vehiculo: Option<String>,
    pub placa_vehiculo: Option<String>,
    pub marca_vehiculo: Option<String>,
    pub modelo_vehiculo: Option<String>,
    pub color_vehiculo: Option<String>,
    pub observaciones: Option<String>,
    pub usuario_ingreso_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ProveedorSnapshot {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub empresa_nombre: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoProveedorResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>,
    pub proveedor: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoProveedor>,
}

use crate::domain::errors::IngresoProveedorError;

// Re-exports from common module
pub use crate::domain::common::{normalizar_numero_gafete, DecisionReporteGafete};

// Import common functions for internal use
use crate::domain::common as common_domain;

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoProveedorError> {
    if fecha_salida.is_some() {
        return Err(IngresoProveedorError::NoActiveIngreso);
    }
    Ok(())
}

/// Wrapper que usa el error específico de proveedor
pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoProveedorError> {
    common_domain::validar_tiempo_salida(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoProveedorError::Validation(e))
}

/// Wrapper que usa el error específico de proveedor
pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoProveedorError> {
    common_domain::calcular_tiempo_permanencia(fecha_ingreso_str, fecha_salida_str)
        .map_err(|e| IngresoProveedorError::Validation(e))
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
        let salida = "2023-12-22T08:30:00Z";
        assert_eq!(calcular_tiempo_permanencia(ingreso, salida).unwrap(), 30);
    }

    #[test]
    fn test_evaluar_devolucion_gafete() {
        // OK
        let res = evaluar_devolucion_gafete(true, Some("P-10"), true, Some("P-10"));
        assert!(!res.debe_generar_reporte);

        // Sin devolver
        let res = evaluar_devolucion_gafete(true, Some("P-10"), false, None);
        assert!(res.debe_generar_reporte);

        // Gafete incorrecto
        let res = evaluar_devolucion_gafete(true, Some("P-10"), true, Some("P-11"));
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("incorrecto"));
    }
}
