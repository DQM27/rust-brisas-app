// ==========================================
// src/domain/ingreso/validaciones_salida.rs
// ==========================================
// Validaciones y reglas de negocio PURAS para la fase de SALIDA

use super::tipos::DecisionReporteGafete;
use chrono::NaiveDateTime;

// ==========================================
// VALIDACIONES DE TIEMPO
// ==========================================

/// Valida que la fecha de salida no sea anterior a la de ingreso
/// 
/// Formato esperado: RFC3339 ("2024-01-15T10:30:00-06:00")
pub fn validar_tiempo_salida(
    fecha_hora_ingreso: &str,
    fecha_hora_salida: &str,
) -> Result<(), String> {
    let ingreso = NaiveDateTime::parse_from_str(fecha_hora_ingreso, "%+")
        .map_err(|e| format!("Error parseando fecha de ingreso: {}", e))?;

    let salida = NaiveDateTime::parse_from_str(fecha_hora_salida, "%+")
        .map_err(|e| format!("Error parseando fecha de salida: {}", e))?;

    if salida < ingreso {
        return Err("La fecha de salida no puede ser anterior a la fecha de ingreso".to_string());
    }

    Ok(())
}

/// Calcula el tiempo de permanencia en minutos entre ingreso y salida
/// 
/// Formato esperado: RFC3339
/// Retorna: minutos transcurridos (siempre positivo si la validación pasó)
pub fn calcular_tiempo_permanencia(
    fecha_hora_ingreso: &str,
    fecha_hora_salida: &str,
) -> Result<i64, String> {
    let ingreso = NaiveDateTime::parse_from_str(fecha_hora_ingreso, "%+")
        .map_err(|e| format!("Error parseando fecha de ingreso: {}", e))?;

    let salida = NaiveDateTime::parse_from_str(fecha_hora_salida, "%+")
        .map_err(|e| format!("Error parseando fecha de salida: {}", e))?;

    let minutos = (salida - ingreso).num_minutes();
    Ok(minutos)
}

// ==========================================
// VALIDACIONES DE GAFETE
// ==========================================

/// Valida que el gafete devuelto coincida con el gafete asignado
/// 
/// Reglas:
/// - Si no tenía gafete asignado → cualquier devolución es error
/// - Si tenía gafete → el devuelto DEBE coincidir (case-insensitive después de normalizar)
/// - Si tenía gafete pero no devuelve nada → error (se maneja en evaluar_devolucion_gafete)
pub fn validar_gafete_coincide(
    gafete_asignado: Option<&str>,
    gafete_devuelto: Option<&str>,
) -> Result<(), String> {
    match (gafete_asignado, gafete_devuelto) {
        (None, Some(_)) => {
            Err("No se puede devolver un gafete que no fue asignado".to_string())
        }
        (Some(asignado), Some(devuelto)) => {
            let asignado_norm = asignado.trim().to_uppercase();
            let devuelto_norm = devuelto.trim().to_uppercase();

            if asignado_norm != devuelto_norm {
                Err(format!(
                    "El gafete devuelto ({}) no coincide con el asignado ({})",
                    devuelto, asignado
                ))
            } else {
                Ok(())
            }
        }
        (Some(_), None) => {
            // Este caso se maneja en evaluar_devolucion_gafete (genera reporte)
            Ok(())
        }
        (None, None) => Ok(()), // No tenía gafete y no devuelve nada: OK
    }
}

// ==========================================
// REGLA DE NEGOCIO PRINCIPAL - GAFETE
// ==========================================

/// Evalúa la devolución de gafete y determina si debe generarse reporte
/// 
/// Casos:
/// 1. NO tenía gafete asignado:
///    - devolvio_gafete=true → ERROR (no puede devolver algo que no recibió)
///    - devolvio_gafete=false → OK, sin reporte
/// 
/// 2. SÍ tenía gafete asignado:
///    - devolvio_gafete=true + gafete_devuelto coincide → OK, sin reporte
///    - devolvio_gafete=true + gafete_devuelto NO coincide → ERROR
///    - devolvio_gafete=false → OK, PERO generar reporte
pub fn evaluar_devolucion_gafete(
    tenia_gafete_asignado: bool,
    gafete_numero_asignado: Option<&str>,
    devolvio_gafete: bool,
    gafete_devuelto: Option<&str>,
) -> Result<DecisionReporteGafete, String> {
    // Caso 1: NO tenía gafete asignado
    if !tenia_gafete_asignado {
        if devolvio_gafete {
            return Err(
                "No se puede marcar como 'devolvió gafete' cuando no se asignó ninguno".to_string()
            );
        }

        // Todo OK, sin reporte
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: false,
            motivo: None,
            gafete_numero: None,
        });
    }

    // Caso 2: SÍ tenía gafete asignado
    let gafete_num = gafete_numero_asignado
        .ok_or_else(|| "Inconsistencia: tenía gafete pero número es None".to_string())?;

    if devolvio_gafete {
        // Guardia dice que SÍ devolvió → validar que coincida
        if let Some(devuelto) = gafete_devuelto {
            validar_gafete_coincide(Some(gafete_num), Some(devuelto))?;

            // Gafete devuelto correctamente, sin reporte
            return Ok(DecisionReporteGafete {
                debe_generar_reporte: false,
                motivo: None,
                gafete_numero: None,
            });
        } else {
            return Err(
                "Si marcó que devolvió el gafete, debe especificar cuál gafete devolvió".to_string()
            );
        }
    } else {
        // Guardia dice que NO devolvió → GENERAR REPORTE
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some(format!("Gafete {} no fue devuelto al salir", gafete_num)),
            gafete_numero: Some(gafete_num.to_string()),
        });
    }
}

// ==========================================
// VALIDACIONES DE ESTADO
// ==========================================

/// Valida que el ingreso esté en estado "abierto" (sin fecha de salida)
pub fn validar_ingreso_abierto(fecha_hora_salida: &Option<String>) -> Result<(), String> {
    if fecha_hora_salida.is_some() {
        return Err("El ingreso ya fue cerrado previamente".to_string());
    }
    Ok(())
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // Tests de Validación de Tiempo
    // ==========================================

    #[test]
    fn test_validar_tiempo_salida_correcto() {
        let ingreso = "2024-01-15T10:00:00-06:00";
        let salida = "2024-01-15T18:00:00-06:00";

        assert!(validar_tiempo_salida(ingreso, salida).is_ok());
    }

    #[test]
    fn test_validar_tiempo_salida_mismo_momento() {
        let fecha = "2024-01-15T10:00:00-06:00";
        assert!(validar_tiempo_salida(fecha, fecha).is_ok());
    }

    #[test]
    fn test_validar_tiempo_salida_anterior() {
        let ingreso = "2024-01-15T18:00:00-06:00";
        let salida = "2024-01-15T10:00:00-06:00"; // antes del ingreso

        assert!(validar_tiempo_salida(ingreso, salida).is_err());
    }

    #[test]
    fn test_calcular_tiempo_permanencia() {
        let ingreso = "2024-01-15T08:00:00-06:00";
        let salida = "2024-01-15T17:30:00-06:00"; // 9h 30min

        let minutos = calcular_tiempo_permanencia(ingreso, salida).unwrap();
        assert_eq!(minutos, 570); // 9*60 + 30
    }

    #[test]
    fn test_calcular_tiempo_permanencia_mismo_momento() {
        let fecha = "2024-01-15T10:00:00-06:00";
        let minutos = calcular_tiempo_permanencia(fecha, fecha).unwrap();
        assert_eq!(minutos, 0);
    }

    // ==========================================
    // Tests de Validación de Gafete
    // ==========================================

    #[test]
    fn test_validar_gafete_coincide_ok() {
        assert!(validar_gafete_coincide(Some("A-15"), Some("A-15")).is_ok());
        assert!(validar_gafete_coincide(Some("A-15"), Some("a-15")).is_ok()); // case-insensitive
        assert!(validar_gafete_coincide(Some("A-15"), Some(" A-15 ")).is_ok()); // trim
    }

    #[test]
    fn test_validar_gafete_no_coincide() {
        let resultado = validar_gafete_coincide(Some("A-15"), Some("B-20"));
        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no coincide"));
    }

    #[test]
    fn test_validar_gafete_devuelve_sin_asignar() {
        let resultado = validar_gafete_coincide(None, Some("A-15"));
        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no fue asignado"));
    }

    #[test]
    fn test_validar_gafete_sin_asignar_sin_devolver() {
        assert!(validar_gafete_coincide(None, None).is_ok());
    }

    #[test]
    fn test_validar_gafete_asignado_no_devuelve() {
        // Este caso NO es error aquí, se maneja en evaluar_devolucion_gafete
        assert!(validar_gafete_coincide(Some("A-15"), None).is_ok());
    }

    // ==========================================
    // Tests de Evaluación de Devolución de Gafete
    // ==========================================

    #[test]
    fn test_evaluar_sin_gafete_asignado_sin_devolver() {
        let decision = evaluar_devolucion_gafete(
            false, // no tenía gafete
            None,
            false, // no devolvió
            None,
        )
        .unwrap();

        assert!(!decision.debe_generar_reporte);
        assert!(decision.motivo.is_none());
    }

    #[test]
    fn test_evaluar_sin_gafete_asignado_pero_dice_devolver() {
        let resultado = evaluar_devolucion_gafete(
            false,       // no tenía gafete
            None,
            true,        // DICE que devolvió
            Some("A-15"),
        );

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no se asignó"));
    }

    #[test]
    fn test_evaluar_con_gafete_devuelve_correcto() {
        let decision = evaluar_devolucion_gafete(
            true,        // SÍ tenía gafete
            Some("A-15"),
            true,        // SÍ devolvió
            Some("A-15"), // gafete correcto
        )
        .unwrap();

        assert!(!decision.debe_generar_reporte);
        assert!(decision.motivo.is_none());
    }

    #[test]
    fn test_evaluar_con_gafete_devuelve_incorrecto() {
        let resultado = evaluar_devolucion_gafete(
            true,        // SÍ tenía gafete
            Some("A-15"),
            true,        // dice que devolvió
            Some("B-20"), // GAFETE INCORRECTO
        );

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no coincide"));
    }

    #[test]
    fn test_evaluar_con_gafete_no_devuelve() {
        let decision = evaluar_devolucion_gafete(
            true,        // SÍ tenía gafete
            Some("A-15"),
            false,       // NO devolvió
            None,
        )
        .unwrap();

        // DEBE generar reporte
        assert!(decision.debe_generar_reporte);
        assert!(decision.motivo.is_some());
        assert!(decision.motivo.unwrap().contains("A-15"));
        assert_eq!(decision.gafete_numero, Some("A-15".to_string()));
    }

    #[test]
    fn test_evaluar_con_gafete_dice_devolver_sin_especificar() {
        let resultado = evaluar_devolucion_gafete(
            true,        // SÍ tenía gafete
            Some("A-15"),
            true,        // dice que devolvió
            None,        // pero no especifica cuál
        );

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("debe especificar"));
    }

    // ==========================================
    // Tests de Validación de Estado
    // ==========================================

    #[test]
    fn test_validar_ingreso_abierto_ok() {
        assert!(validar_ingreso_abierto(&None).is_ok());
    }

    #[test]
    fn test_validar_ingreso_ya_cerrado() {
        let fecha_salida = Some("2024-01-15T18:00:00-06:00".to_string());
        let resultado = validar_ingreso_abierto(&fecha_salida);

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("ya fue cerrado"));
    }

    // ==========================================
    // Tests de Integración (Escenarios Reales)
    // ==========================================

    #[test]
    fn test_escenario_salida_normal_con_gafete() {
        // Contratista tenía gafete A-15, lo devuelve correctamente
        let ingreso = "2024-01-15T08:00:00-06:00";
        let salida = "2024-01-15T16:30:00-06:00";

        // Validar tiempo
        assert!(validar_tiempo_salida(ingreso, salida).is_ok());
        let minutos = calcular_tiempo_permanencia(ingreso, salida).unwrap();
        assert_eq!(minutos, 510); // 8h 30min

        // Validar gafete
        let decision = evaluar_devolucion_gafete(
            true,
            Some("A-15"),
            true,
            Some("A-15"),
        )
        .unwrap();

        assert!(!decision.debe_generar_reporte);
    }

    #[test]
    fn test_escenario_salida_sin_devolver_gafete() {
        // Contratista tenía gafete A-15, NO lo devuelve
        let ingreso = "2024-01-15T08:00:00-06:00";
        let salida = "2024-01-15T16:30:00-06:00";

        // Validar tiempo
        assert!(validar_tiempo_salida(ingreso, salida).is_ok());

        // Validar gafete
        let decision = evaluar_devolucion_gafete(
            true,
            Some("A-15"),
            false, // NO devolvió
            None,
        )
        .unwrap();

        // DEBE generar reporte
        assert!(decision.debe_generar_reporte);
        assert_eq!(decision.gafete_numero, Some("A-15".to_string()));
    }

    #[test]
    fn test_escenario_salida_sin_gafete() {
        // Contratista NO tenía gafete asignado
        let ingreso = "2024-01-15T08:00:00-06:00";
        let salida = "2024-01-15T16:30:00-06:00";

        // Validar tiempo
        assert!(validar_tiempo_salida(ingreso, salida).is_ok());

        // Validar gafete
        let decision = evaluar_devolucion_gafete(
            false, // no tenía gafete
            None,
            false, // no devolvió (obvio)
            None,
        )
        .unwrap();

        assert!(!decision.debe_generar_reporte);
    }

    #[test]
    fn test_escenario_salida_gafete_incorrecto() {
        // Contratista tenía A-15, devuelve B-20 (incorrecto)
        let ingreso = "2024-01-15T08:00:00-06:00";
        let salida = "2024-01-15T16:30:00-06:00";

        // Validar tiempo
        assert!(validar_tiempo_salida(ingreso, salida).is_ok());

        // Validar gafete
        let resultado = evaluar_devolucion_gafete(
            true,
            Some("A-15"),
            true,
            Some("B-20"), // INCORRECTO
        );

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().contains("no coincide"));
    }

    #[test]
    fn test_escenario_salida_antes_ingreso() {
        // Error: fecha de salida antes de ingreso
        let ingreso = "2024-01-15T16:00:00-06:00";
        let salida = "2024-01-15T08:00:00-06:00"; // ANTES

        assert!(validar_tiempo_salida(ingreso, salida).is_err());
    }

    #[test]
    fn test_escenario_ingreso_ya_cerrado() {
        let fecha_salida = Some("2024-01-15T16:00:00-06:00".to_string());
        
        let resultado = validar_ingreso_abierto(&fecha_salida);
        assert!(resultado.is_err());
    }
}