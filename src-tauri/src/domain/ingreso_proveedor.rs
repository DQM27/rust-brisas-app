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

use crate::models::ingreso::AlertaGafeteResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidacionIngresoProveedorResponse {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<AlertaGafeteResponse>,
    pub proveedor: Option<serde_json::Value>,
    pub tiene_ingreso_abierto: bool,
    pub ingreso_abierto: Option<IngresoProveedor>,
}

use crate::domain::errors::IngresoProveedorError;
use chrono::DateTime;

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

pub fn normalizar_numero_gafete(input: &str) -> String {
    input.trim().to_uppercase()
}

pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoProveedorError> {
    if fecha_salida.is_some() {
        return Err(IngresoProveedorError::NoActiveIngreso);
    }
    Ok(())
}

pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoProveedorError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| IngresoProveedorError::Validation("Fecha ingreso inválida".to_string()))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| IngresoProveedorError::Validation("Fecha salida inválida".to_string()))?;

    if salida < ingreso {
        return Err(IngresoProveedorError::Validation(
            "La fecha de salida no puede ser anterior a la de ingreso".to_string(),
        ));
    }
    Ok(())
}

pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoProveedorError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| IngresoProveedorError::Validation("Fecha ingreso inválida".to_string()))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| IngresoProveedorError::Validation("Fecha salida inválida".to_string()))?;

    let duracion = salida.signed_duration_since(ingreso);
    Ok(duracion.num_minutes())
}

// Reusing DecisionReporteGafete struct if generic, or redefine?
// IngresoContratista defined it locally. Maybe we should move it to models/ingreso.rs?
// For now, define locally to avoid cross-crate dependency hell refactoring.
#[derive(Debug, Clone)]
pub struct DecisionReporteGafete {
    pub debe_generar_reporte: bool,
    pub motivo: Option<String>,
    pub gafete_numero: Option<String>,
}

pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<&str>,
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<&str>,
) -> Result<DecisionReporteGafete, IngresoProveedorError> {
    if !tenia_gafete {
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: false,
            motivo: None,
            gafete_numero: None,
        });
    }

    if !reporto_devolucion {
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some("Salida registrada sin devolver gafete".to_string()),
            gafete_numero: gafete_asignado.map(|s| s.to_string()),
        });
    }

    if let (Some(asignado), Some(devuelto)) = (gafete_asignado, gafete_devuelto_numero) {
        if normalizar_numero_gafete(asignado) != normalizar_numero_gafete(devuelto) {
            return Ok(DecisionReporteGafete {
                debe_generar_reporte: true,
                motivo: Some(format!(
                    "Devolvió gafete incorrecto: {} vs {}",
                    devuelto, asignado
                )),
                gafete_numero: Some(asignado.to_string()),
            });
        }
    }

    Ok(DecisionReporteGafete {
        debe_generar_reporte: false,
        motivo: None,
        gafete_numero: None,
    })
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
        let res = evaluar_devolucion_gafete(true, Some("P-10"), true, Some("P-10")).unwrap();
        assert!(!res.debe_generar_reporte);

        // Sin devolver
        let res = evaluar_devolucion_gafete(true, Some("P-10"), false, None).unwrap();
        assert!(res.debe_generar_reporte);

        // Gafete incorrecto
        let res = evaluar_devolucion_gafete(true, Some("P-10"), true, Some("P-11")).unwrap();
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("incorrecto"));
    }
}
