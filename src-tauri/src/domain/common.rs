/// Capa de Dominio: Lógica Compartida y Utilidades Comunes.
///
/// Este módulo centraliza las reglas de negocio puras que son transversales
/// a todos los tipos de ingreso (Contratistas, Proveedores y Visitas).
///
/// Sigue el principio DRY (Don't Repeat Yourself) para asegurar que validaciones
/// críticas como tiempos de permanencia y gestión de gafetes sean consistentes
/// en todo el sistema.
use chrono::DateTime;

// Re-exportación de estructuras desde models
pub use crate::models::ingreso::{CommonError, DecisionReporteGafete};

// --------------------------------------------------------------------------
// GESTIÓN DE GAFETES: REGLAS DE NEGOCIO
// --------------------------------------------------------------------------

/// Normaliza un input de gafete (texto) a su representación numérica interna (INT).
///
/// Reglas:
/// - "S/G" (case insensitive) -> 0
/// - "0" -> 0
/// - "005" -> 5
/// - "123" -> 123
pub fn normalizar_gafete_a_int(input: &str) -> Result<i32, String> {
    let limpio = input.trim().to_uppercase();

    // Alias S/G
    if limpio == "S/G" || limpio == "0" {
        return Ok(0);
    }

    // Intentar parsear número
    match limpio.parse::<i32>() {
        Ok(n) if n >= 0 => Ok(n),
        _ => Err(format!(
            "Formato de gafete inválido: '{}'. Debe ser un número positivo o 'S/G'.",
            input
        )),
    }
}

// ... evaluar_devolucion_gafete ahora debe trabajar con INTs o normalizar antes?
// Si gafete es INT en DB, aqui deberiamos recibir Option<i32>.
// Pero los parámetros actuales son Option<&str>.
// Necesito actualizar `evaluar_devolucion_gafete` para recibir `Option<i32>`.

pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<i32>,
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<i32>, // Input ya normalizado a int
) -> DecisionReporteGafete {
    if !tenia_gafete {
        return DecisionReporteGafete::default();
    }

    // Incidencia: Salida sin devolución
    if !reporto_devolucion {
        return DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some("Salida registrada sin devolver gafete físico".to_string()),
            gafete_numero: gafete_asignado,
        };
    }

    // Incidencia: Discrepancia de identificación
    if let (Some(asignado), Some(devuelto)) = (gafete_asignado, gafete_devuelto_numero) {
        if asignado != devuelto {
            return DecisionReporteGafete {
                debe_generar_reporte: true,
                motivo: Some(format!(
                    "Discrepancia: Devolvió gafete incorrecto (Detectado: {} / Esperado: {})",
                    devuelto, asignado
                )),
                gafete_numero: Some(asignado),
            };
        }
    }

    DecisionReporteGafete::default()
}

/// Valida que el número de gafete devuelto coincida con el asignado.
pub fn validar_gafete_coincide(
    gafete_asignado: i32,
    gafete_devuelto: i32,
) -> Result<(), CommonError> {
    if gafete_asignado != gafete_devuelto {
        // Para el mensaje de error convertimos a string (0 mostrarlo como S/G seria ideal pero 0 esta ok)
        let asignado_str =
            if gafete_asignado == 0 { "S/G".to_string() } else { gafete_asignado.to_string() };
        let devuelto_str =
            if gafete_devuelto == 0 { "S/G".to_string() } else { gafete_devuelto.to_string() };

        return Err(CommonError::GafeteNoCoincide {
            devuelto: devuelto_str,
            asignado: asignado_str,
        });
    }
    Ok(())
}

// --------------------------------------------------------------------------
// CONTROL DE TIEMPO: REGLAS DE NEGOCIO
// --------------------------------------------------------------------------

/// Valida que el flujo temporal de una estancia sea lógicamente correcto.
///
/// Garantiza que nadie pueda "salir antes de entrar", validando las estampas
/// de tiempo capturadas por el sistema.
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

/// Calcula la duración exacta de una estancia en minutos.
///
/// Utilizado para auditoría, generación de métricas y detección de estancias
/// que exceden el límite máximo permitido.
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

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalizacion_coherente() {
        assert_eq!(normalizar_gafete_a_int("  123  ").unwrap(), 123);
        assert_eq!(normalizar_gafete_a_int("005").unwrap(), 5);
        assert_eq!(normalizar_gafete_a_int("  0010  ").unwrap(), 10);
        assert_eq!(normalizar_gafete_a_int("s/g").unwrap(), 0);
        assert_eq!(normalizar_gafete_a_int("0").unwrap(), 0);
    }

    #[test]
    fn test_deteccion_perdida_gafete() {
        let decision = evaluar_devolucion_gafete(true, Some(1), false, None);
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.unwrap().contains("sin devolver"));
    }

    #[test]
    fn test_deteccion_intercambio_gafete() {
        let decision = evaluar_devolucion_gafete(true, Some(1), true, Some(999));
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.unwrap().contains("incorrecto"));
    }

    #[test]
    fn test_flujo_tiempo_invalido() {
        let ingreso = "2024-01-01T17:00:00Z";
        let salida = "2024-01-01T08:00:00Z";
        let result = validar_tiempo_salida(ingreso, salida);
        assert!(matches!(result.unwrap_err(), CommonError::SalidaAnteriorAIngreso));
    }
}
