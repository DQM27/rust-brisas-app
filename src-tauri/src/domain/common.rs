//! # Módulo Común de Dominio
//!
//! Este módulo centraliza la lógica de dominio compartida entre tipos de ingreso:
//! - Contratistas
//! - Proveedores
//! - Visitas
//!
//! Elimina duplicación de código siguiendo el principio DRY.

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ==========================================
// ERRORES COMUNES
// ==========================================

/// Errores comunes para validaciones de dominio compartidas
#[derive(Error, Debug, Clone, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum CommonError {
    #[error("Datos corruptos: fecha de ingreso inválida")]
    FechaIngresoInvalida,

    #[error("Fecha de salida inválida")]
    FechaSalidaInvalida,

    #[error("La fecha de salida no puede ser anterior a la de ingreso")]
    SalidaAnteriorAIngreso,

    #[error("El gafete devuelto ({devuelto}) no coincide con el asignado ({asignado})")]
    GafeteNoCoincide { devuelto: String, asignado: String },
}

// ==========================================
// GAFETES: STRUCTS COMUNES
// ==========================================

/// Resultado de evaluar si se debe generar un reporte de gafete no devuelto.
/// Usado por todos los tipos de ingreso al procesar la salida.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionReporteGafete {
    /// Si es true, el servicio debe crear una alerta de gafete no devuelto
    pub debe_generar_reporte: bool,
    /// Motivo descriptivo para el reporte
    pub motivo: Option<String>,
    /// Número del gafete involucrado
    pub gafete_numero: Option<String>,
}

impl Default for DecisionReporteGafete {
    fn default() -> Self {
        Self { debe_generar_reporte: false, motivo: None, gafete_numero: None }
    }
}

// ==========================================
// GAFETES: FUNCIONES COMUNES
// ==========================================

/// Normaliza un número de gafete para comparaciones consistentes.
/// Elimina espacios y convierte a mayúsculas.
///
/// # Ejemplos
/// ```
/// use brisas_app_lib::domain::common::normalizar_numero_gafete;
/// assert_eq!(normalizar_numero_gafete(" v-123 "), "V-123");
/// ```
pub fn normalizar_numero_gafete(input: &str) -> String {
    input.trim().to_uppercase()
}

/// Evalúa si se debe generar un reporte de gafete no devuelto.
///
/// # Argumentos
/// - `tenia_gafete`: Si la persona tenía un gafete asignado al ingresar
/// - `gafete_asignado`: Número del gafete asignado (si aplica)
/// - `reporto_devolucion`: Si indicó que devolvió el gafete al salir
/// - `gafete_devuelto_numero`: Número del gafete que dice haber devuelto
///
/// # Retorna
/// `DecisionReporteGafete` indicando si se debe crear un reporte y por qué
pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<&str>,
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<&str>,
) -> DecisionReporteGafete {
    // Si no tenía gafete asignado, no hay nada que evaluar
    if !tenia_gafete {
        return DecisionReporteGafete::default();
    }

    // SI tenía gafete asignado:

    // Caso 1: Dice que NO lo devolvió
    if !reporto_devolucion {
        return DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some("Salida registrada sin devolver gafete".to_string()),
            gafete_numero: gafete_asignado.map(|s| s.to_string()),
        };
    }

    // Caso 2: Dice que SÍ lo devolvió, pero hay discrepancia de números
    if let (Some(asignado), Some(devuelto)) = (gafete_asignado, gafete_devuelto_numero) {
        if normalizar_numero_gafete(asignado) != normalizar_numero_gafete(devuelto) {
            return DecisionReporteGafete {
                debe_generar_reporte: true,
                motivo: Some(format!("Devolvió gafete incorrecto: {} vs {}", devuelto, asignado)),
                gafete_numero: Some(asignado.to_string()),
            };
        }
    }

    // Caso 3: Todo OK
    DecisionReporteGafete::default()
}

/// Valida que el gafete devuelto coincida con el asignado.
///
/// # Errores
/// Retorna error si los números no coinciden (después de normalización)
pub fn validar_gafete_coincide(
    asignado: Option<&str>,
    devuelto: Option<&str>,
) -> Result<(), CommonError> {
    match (asignado, devuelto) {
        (Some(a), Some(d)) => {
            if normalizar_numero_gafete(a) != normalizar_numero_gafete(d) {
                return Err(CommonError::GafeteNoCoincide {
                    devuelto: d.to_string(),
                    asignado: a.to_string(),
                });
            }
        }
        _ => {} // Si no tenía o no devolvió, no hay conflicto aquí
    }
    Ok(())
}

// ==========================================
// TIEMPO: FUNCIONES COMUNES
// ==========================================

/// Valida que la fecha de salida sea posterior a la de ingreso.
///
/// # Argumentos
/// - `fecha_ingreso_str`: Fecha de ingreso en formato RFC 3339
/// - `fecha_salida_str`: Fecha de salida en formato RFC 3339
///
/// # Errores
/// - Si alguna fecha tiene formato inválido
/// - Si la salida es anterior al ingreso
pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), CommonError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| CommonError::FechaIngresoInvalida)?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| CommonError::FechaSalidaInvalida)?;

    if salida < ingreso {
        return Err(CommonError::SalidaAnteriorAIngreso);
    }
    Ok(())
}

/// Calcula el tiempo de permanencia en minutos entre ingreso y salida.
///
/// # Argumentos
/// - `fecha_ingreso_str`: Fecha de ingreso en formato RFC 3339
/// - `fecha_salida_str`: Fecha de salida en formato RFC 3339
///
/// # Retorna
/// Duración en minutos (puede ser negativo si las fechas están invertidas)
pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, CommonError> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| CommonError::FechaIngresoInvalida)?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| CommonError::FechaSalidaInvalida)?;

    let duracion = salida.signed_duration_since(ingreso);
    Ok(duracion.num_minutes())
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests de normalizar_numero_gafete ---

    #[test]
    fn test_normalizar_gafete_espacios() {
        assert_eq!(normalizar_numero_gafete("  v-123  "), "V-123");
    }

    #[test]
    fn test_normalizar_gafete_mayusculas() {
        assert_eq!(normalizar_numero_gafete("contratista-5"), "CONTRATISTA-5");
    }

    #[test]
    fn test_normalizar_gafete_vacio() {
        assert_eq!(normalizar_numero_gafete("   "), "");
    }

    // --- Tests de evaluar_devolucion_gafete ---

    #[test]
    fn test_evaluar_sin_gafete_no_reporta() {
        let decision = evaluar_devolucion_gafete(false, None, false, None);
        assert!(!decision.debe_generar_reporte);
    }

    #[test]
    fn test_evaluar_no_devolvio_reporta() {
        let decision = evaluar_devolucion_gafete(true, Some("V-001"), false, None);
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.unwrap().contains("sin devolver"));
        assert_eq!(decision.gafete_numero, Some("V-001".to_string()));
    }

    #[test]
    fn test_evaluar_gafete_incorrecto_reporta() {
        let decision = evaluar_devolucion_gafete(true, Some("V-001"), true, Some("V-999"));
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.unwrap().contains("incorrecto"));
    }

    #[test]
    fn test_evaluar_devolucion_correcta_no_reporta() {
        let decision = evaluar_devolucion_gafete(true, Some("V-001"), true, Some("v-001"));
        assert!(!decision.debe_generar_reporte);
    }

    // --- Tests de validar_gafete_coincide ---

    #[test]
    fn test_validar_gafete_coincide_ok() {
        assert!(validar_gafete_coincide(Some("V-001"), Some("v-001")).is_ok());
    }

    #[test]
    fn test_validar_gafete_no_coincide() {
        let result = validar_gafete_coincide(Some("V-001"), Some("V-999"));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommonError::GafeteNoCoincide { .. }));
    }

    #[test]
    fn test_validar_gafete_none_ok() {
        assert!(validar_gafete_coincide(None, None).is_ok());
        assert!(validar_gafete_coincide(Some("V-001"), None).is_ok());
    }

    // --- Tests de tiempo ---

    #[test]
    fn test_validar_tiempo_salida_ok() {
        let ingreso = "2024-01-01T08:00:00Z";
        let salida = "2024-01-01T17:00:00Z";
        assert!(validar_tiempo_salida(ingreso, salida).is_ok());
    }

    #[test]
    fn test_validar_tiempo_salida_anterior_error() {
        let ingreso = "2024-01-01T17:00:00Z";
        let salida = "2024-01-01T08:00:00Z";
        let result = validar_tiempo_salida(ingreso, salida);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommonError::SalidaAnteriorAIngreso));
    }

    #[test]
    fn test_calcular_permanencia() {
        let ingreso = "2024-01-01T08:00:00Z";
        let salida = "2024-01-01T17:30:00Z";
        let minutos = calcular_tiempo_permanencia(ingreso, salida).unwrap();
        assert_eq!(minutos, 570); // 9.5 horas = 570 minutos
    }

    #[test]
    fn test_calcular_permanencia_fecha_invalida() {
        let result = calcular_tiempo_permanencia("invalid", "2024-01-01T17:00:00Z");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommonError::FechaIngresoInvalida));
    }
}
