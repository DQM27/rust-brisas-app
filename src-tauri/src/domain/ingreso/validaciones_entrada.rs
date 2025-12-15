// ==========================================
// src/domain/ingreso/validaciones_entrada.rs
// ==========================================
// Validaciones y reglas de negocio PURAS para la fase de ENTRADA

use super::tipos::ResultadoValidacionEntrada;
use crate::models::ingreso::CreateIngresoContratistaInput;

// ==========================================
// VALIDACIONES DE CAMPOS
// ==========================================

/// Valida el formato de un número de gafete
///
/// Reglas:
/// - No puede estar vacío (después de trim)
/// - No puede exceder 20 caracteres
pub fn validar_formato_gafete(numero: &str) -> Result<(), String> {
    let limpio = numero.trim();

    if limpio.is_empty() {
        return Err("El número de gafete no puede estar vacío".to_string());
    }

    if limpio.len() > 20 {
        return Err("El número de gafete no puede exceder 20 caracteres".to_string());
    }

    Ok(())
}

// ==========================================
// NORMALIZACIONES
// ==========================================

/// Normaliza un número de gafete a formato estándar (trim + uppercase)
///
/// Ejemplos:
/// - "  a-15  " → "A-15"
/// - "c-25" → "C-25"
pub fn normalizar_numero_gafete(numero: &str) -> String {
    numero.trim().to_uppercase()
}

// ==========================================
// VALIDACIONES DE INPUTS
// ==========================================

/// Valida el input completo de creación de ingreso
pub fn validar_input_entrada(input: &CreateIngresoContratistaInput) -> Result<(), String> {
    // Validar gafete si se proporciona
    if let Some(ref gafete_num) = input.gafete_numero {
        validar_formato_gafete(gafete_num)?;
    }

    // Más validaciones de campos si es necesario
    Ok(())
}

// ==========================================
// REGLAS DE NEGOCIO - PRAIND
// ==========================================

/// Verifica si un PRAIND está vigente comparando contra fecha actual
///
/// Formato esperado: "YYYY-MM-DD"
/// Retorna true si fecha_vencimiento >= hoy
pub fn verificar_praind_vigente(fecha_vencimiento: &str) -> Result<bool, String> {
    use chrono::{NaiveDate, Utc};

    let fecha_venc = match NaiveDate::parse_from_str(fecha_vencimiento, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return Ok(false), // Invalid format treated as expired/not valid
    };

    let hoy = Utc::now().date_naive();
    Ok(fecha_venc >= hoy)
}

/// Calcula días restantes hasta vencimiento de PRAIND
///
/// Retorna número negativo si ya venció
pub fn dias_hasta_vencimiento_praind(fecha_vencimiento: &str) -> Result<i64, String> {
    use chrono::{NaiveDate, Utc};

    let fecha_venc = NaiveDate::parse_from_str(fecha_vencimiento, "%Y-%m-%d")
        .map_err(|_| format!("Formato de fecha PRAIND inválido: {}", fecha_vencimiento))?;

    let hoy = Utc::now().date_naive();
    let dias = (fecha_venc - hoy).num_days();

    Ok(dias)
}

// ==========================================
// REGLA DE NEGOCIO PRINCIPAL
// ==========================================

/// Evalúa si un contratista puede ingresar basado en todas las validaciones
///
/// Esta es la función CORE que concentra toda la lógica de elegibilidad
pub fn evaluar_elegibilidad_entrada(
    bloqueado: bool,
    motivo_bloqueo: Option<String>,
    tiene_ingreso_abierto: bool,
    estado_contratista: &str,
    praind_vigente: bool,
    alertas_gafete_previas: usize,
) -> ResultadoValidacionEntrada {
    let mut alertas = Vec::new();

    // 1. Verificar lista negra (BLOQUEANTE)
    if bloqueado {
        let motivo = motivo_bloqueo.unwrap_or_else(|| "Motivo no especificado".to_string());
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!("Contratista bloqueado: {}", motivo)),
            alertas,
        };
    }

    // 2. Verificar ingreso duplicado (BLOQUEANTE)
    if tiene_ingreso_abierto {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso abierto".to_string()),
            alertas,
        };
    }

    // 3. Verificar estado del contratista (BLOQUEANTE)
    if estado_contratista.to_lowercase() != "activo" {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!("Contratista con estado: {}", estado_contratista)),
            alertas,
        };
    }

    // 4. Verificar PRAIND (BLOQUEANTE)
    if !praind_vigente {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some("PRAIND vencido".to_string()),
            alertas,
        };
    }

    // 5. Verificar alertas de gafetes previas (WARNING, no bloqueante)
    if alertas_gafete_previas > 0 {
        alertas.push(format!(
            "Tiene {} gafete(s) sin devolver de ingresos anteriores",
            alertas_gafete_previas
        ));
    }

    // ✅ Todas las validaciones pasaron
    ResultadoValidacionEntrada {
        puede_ingresar: true,
        motivo_rechazo: None,
        alertas,
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // Tests de Normalización
    // ==========================================

    #[test]
    fn test_normalizar_gafete_numero() {
        assert_eq!(normalizar_numero_gafete("  a-15  "), "A-15");
        assert_eq!(normalizar_numero_gafete("c-25"), "C-25");
        assert_eq!(normalizar_numero_gafete("ABC-123"), "ABC-123");
        assert_eq!(normalizar_numero_gafete("  "), "");
    }

    // ==========================================
    // Tests de Validación de Formato
    // ==========================================

    #[test]
    fn test_validar_formato_gafete_valido() {
        assert!(validar_formato_gafete("A-15").is_ok());
        assert!(validar_formato_gafete("C-25").is_ok());
        assert!(validar_formato_gafete("ABC-123").is_ok());
    }

    #[test]
    fn test_validar_formato_gafete_vacio() {
        assert!(validar_formato_gafete("").is_err());
        assert!(validar_formato_gafete("   ").is_err());
    }

    #[test]
    fn test_validar_formato_gafete_muy_largo() {
        let gafete_largo = "A".repeat(25);
        assert!(validar_formato_gafete(&gafete_largo).is_err());
    }

    // ==========================================
    // Tests de PRAIND
    // ==========================================

    #[test]
    fn test_verificar_praind_vigente() {
        // Fecha futura (2030)
        assert_eq!(verificar_praind_vigente("2030-12-31").unwrap(), true);

        // Fecha pasada
        assert_eq!(verificar_praind_vigente("2020-01-01").unwrap(), false);
    }

    #[test]
    fn test_verificar_praind_formato_invalido() {
        assert!(verificar_praind_vigente("31-12-2030").is_err());
        assert!(verificar_praind_vigente("invalid-date").is_err());
    }

    #[test]
    fn test_dias_hasta_vencimiento_praind() {
        // Fecha futura debe retornar positivo
        let dias = dias_hasta_vencimiento_praind("2030-12-31").unwrap();
        assert!(dias > 0);

        // Fecha pasada debe retornar negativo
        let dias = dias_hasta_vencimiento_praind("2020-01-01").unwrap();
        assert!(dias < 0);
    }

    // ==========================================
    // Tests de Elegibilidad
    // ==========================================

    #[test]
    fn test_elegibilidad_todo_correcto() {
        let resultado = evaluar_elegibilidad_entrada(
            false,    // no bloqueado
            None,     // sin motivo
            false,    // no tiene ingreso abierto
            "Activo", // estado activo
            true,     // praind vigente
            0,        // sin alertas previas
        );

        assert!(resultado.puede_ingresar);
        assert!(resultado.motivo_rechazo.is_none());
        assert!(resultado.alertas.is_empty());
    }

    #[test]
    fn test_elegibilidad_bloqueado() {
        let resultado = evaluar_elegibilidad_entrada(
            true, // BLOQUEADO
            Some("Deuda pendiente".to_string()),
            false,
            "Activo",
            true,
            0,
        );

        assert!(!resultado.puede_ingresar);
        assert!(resultado.motivo_rechazo.is_some());
        assert!(resultado.motivo_rechazo.unwrap().contains("bloqueado"));
    }

    #[test]
    fn test_elegibilidad_ingreso_duplicado() {
        let resultado = evaluar_elegibilidad_entrada(
            false, None, true, // YA TIENE INGRESO ABIERTO
            "Activo", true, 0,
        );

        assert!(!resultado.puede_ingresar);
        assert!(resultado
            .motivo_rechazo
            .unwrap()
            .contains("ingreso abierto"));
    }

    #[test]
    fn test_elegibilidad_estado_inactivo() {
        let resultado = evaluar_elegibilidad_entrada(
            false, None, false, "Inactivo", // ESTADO INACTIVO
            true, 0,
        );

        assert!(!resultado.puede_ingresar);
        assert!(resultado.motivo_rechazo.unwrap().contains("estado"));
    }

    #[test]
    fn test_elegibilidad_praind_vencido() {
        let resultado = evaluar_elegibilidad_entrada(
            false, None, false, "Activo", false, // PRAIND VENCIDO
            0,
        );

        assert!(!resultado.puede_ingresar);
        assert!(resultado.motivo_rechazo.unwrap().contains("PRAIND"));
    }

    #[test]
    fn test_elegibilidad_con_alertas_previas() {
        let resultado = evaluar_elegibilidad_entrada(
            false, None, false, "Activo", true, 2, // TIENE 2 ALERTAS PREVIAS
        );

        // PUEDE ingresar (es warning, no bloqueante)
        assert!(resultado.puede_ingresar);
        assert!(resultado.motivo_rechazo.is_none());

        // PERO debe tener alertas
        assert_eq!(resultado.alertas.len(), 1);
        assert!(resultado.alertas[0].contains("gafete"));
    }

    #[test]
    fn test_elegibilidad_multiples_problemas() {
        // Si hay múltiples problemas, debe retornar el primero en orden de prioridad
        let resultado = evaluar_elegibilidad_entrada(
            true, // bloqueado (mayor prioridad)
            Some("Deuda".to_string()),
            true,       // también tiene ingreso abierto
            "Inactivo", // también está inactivo
            false,      // también praind vencido
            3,
        );

        assert!(!resultado.puede_ingresar);
        // Debe retornar el error de bloqueado (mayor prioridad)
        assert!(resultado.motivo_rechazo.unwrap().contains("bloqueado"));
    }
}
