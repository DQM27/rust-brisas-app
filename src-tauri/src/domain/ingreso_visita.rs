use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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

use crate::domain::errors::IngresoVisitaError;
use chrono::DateTime;

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

pub fn normalizar_numero_gafete(input: &str) -> String {
    input.trim().to_uppercase()
}

pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), IngresoVisitaError> {
    if fecha_salida.is_some() {
        return Err(IngresoVisitaError::NoActiveIngreso);
    }
    Ok(())
}

pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), IngresoVisitaError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| IngresoVisitaError::Validation("Fecha ingreso inválida".to_string()))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| IngresoVisitaError::Validation("Fecha salida inválida".to_string()))?;

    if salida < ingreso {
        return Err(IngresoVisitaError::Validation(
            "La fecha de salida no puede ser anterior a la de ingreso".to_string(),
        ));
    }
    Ok(())
}

pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, IngresoVisitaError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| IngresoVisitaError::Validation("Fecha ingreso inválida".to_string()))?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| IngresoVisitaError::Validation("Fecha salida inválida".to_string()))?;

    let duracion = salida.signed_duration_since(ingreso);
    Ok(duracion.num_minutes())
}

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
) -> Result<DecisionReporteGafete, IngresoVisitaError> {
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
        let salida = "2023-12-22T08:15:00Z";
        assert_eq!(calcular_tiempo_permanencia(ingreso, salida).unwrap(), 15);
    }

    #[test]
    fn test_evaluar_devolucion_gafete() {
        // OK
        let res = evaluar_devolucion_gafete(true, Some("V-5"), true, Some("V-5")).unwrap();
        assert!(!res.debe_generar_reporte);

        // Sin devolver
        let res = evaluar_devolucion_gafete(true, Some("V-5"), false, None).unwrap();
        assert!(res.debe_generar_reporte);

        // Gafete incorrecto
        let res = evaluar_devolucion_gafete(true, Some("V-5"), true, Some("V-6")).unwrap();
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("incorrecto"));
    }
}
