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

/// Normaliza un número de gafete para asegurar comparaciones precisas.
///
/// Reglas:
/// 1. Elimina espacios (`trim`).
/// 2. Convierte a mayúsculas.
/// 3. Si es "0", lo convierte automáticamente a "S/G".
/// 4. Si es numérico diferente de 0, elimina ceros a la izquierda.
pub fn normalizar_numero_gafete(input: &str) -> String {
    let limpio = input.trim().to_uppercase();

    // Alias rápido: 0 -> S/G
    if limpio == "0" {
        return "S/G".to_string();
    }

    // Si es numérico, parsear para quitar ceros (ej: "04" -> "4")
    if let Ok(num) = limpio.parse::<u32>() {
        return num.to_string();
    }

    limpio
}

/// Evalúa el estado de un gafete al cierre de una visita para detectar anomalías.
///
/// # Lógica de Decisión:
/// 1. Si no hubo gafete asignado, no se genera reporte.
/// 2. Si el usuario reporta que NO devolvió el gafete, se genera reporte de pérdida.
/// 3. Si reporta que SÍ lo devolvió, pero el número no coincide con el original,
///    se genera reporte por discrepancia (posible intercambio erróneo).
pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<&str>,
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<&str>,
) -> DecisionReporteGafete {
    if !tenia_gafete {
        return DecisionReporteGafete::default();
    }

    // Incidencia: Salida sin devolución
    if !reporto_devolucion {
        return DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some("Salida registrada sin devolver gafete físico".to_string()),
            gafete_numero: gafete_asignado.map(|s| s.to_string()),
        };
    }

    // Incidencia: Discrepancia de identificación
    if let (Some(asignado), Some(devuelto)) = (gafete_asignado, gafete_devuelto_numero) {
        if normalizar_numero_gafete(asignado) != normalizar_numero_gafete(devuelto) {
            return DecisionReporteGafete {
                debe_generar_reporte: true,
                motivo: Some(format!(
                    "Discrepancia: Devolvió gafete incorrecto (Detectado: {} / Esperado: {})",
                    devuelto, asignado
                )),
                gafete_numero: Some(asignado.to_string()),
            };
        }
    }

    DecisionReporteGafete::default()
}

/// Valida que el número de gafete devuelto coincida con el asignado.
///
/// Esta es una versión simplificada que retorna un Result<(), CommonError>
/// para uso en validaciones estrictas durante el cierre de ingreso.
pub fn validar_gafete_coincide(
    gafete_asignado: &str,
    gafete_devuelto: &str,
) -> Result<(), CommonError> {
    if normalizar_numero_gafete(gafete_asignado) != normalizar_numero_gafete(gafete_devuelto) {
        return Err(CommonError::GafeteNoCoincide {
            devuelto: gafete_devuelto.to_string(),
            asignado: gafete_asignado.to_string(),
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
        assert_eq!(normalizar_numero_gafete("  v-123  "), "V-123");
        // Verifica eliminación de ceros
        assert_eq!(normalizar_numero_gafete("005"), "5");
        assert_eq!(normalizar_numero_gafete("  0010  "), "10");
        // Verifica S/G
        assert_eq!(normalizar_numero_gafete("s/g"), "S/G");
        assert_eq!(normalizar_numero_gafete("0"), "S/G"); // Alias
    }

    #[test]
    fn test_deteccion_perdida_gafete() {
        let decision = evaluar_devolucion_gafete(true, Some("V-001"), false, None);
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.unwrap().contains("sin devolver"));
    }

    #[test]
    fn test_deteccion_intercambio_gafete() {
        let decision = evaluar_devolucion_gafete(true, Some("V-001"), true, Some("V-999"));
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
