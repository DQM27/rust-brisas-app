/// Capa de Dominio: Gestión de Permanencia y Seguridad de Contratistas.
///
/// Este módulo centraliza las reglas de negocio puras para el control de acceso de
/// contratistas, incluyendo el cálculo de tiempos de permanencia, gestión de
/// alertas por vencimiento de documentos (PRAIND) y cierres manuales.
use crate::domain::errors::IngresoContratistaError;
use chrono::{DateTime, Utc};

// Re-exportaciones de estructuras desde models
pub use crate::models::ingreso::contratista::{
    AlertaTiempo, EstadoPermanencia, MotivoCierre, MotivoExcepcional, ResultadoCierreManual,
    ResultadoIngresoExcepcional,
};

// Re-exportaciones de funciones comunes
pub use crate::domain::common::{
    calcular_tiempo_permanencia, evaluar_devolucion_gafete, normalizar_numero_gafete,
    validar_gafete_coincide, validar_tiempo_salida, DecisionReporteGafete,
};

// --------------------------------------------------------------------------
// CONSTANTES DE SEGURIDAD INDUSTRIAL
// --------------------------------------------------------------------------

/// Tiempo máximo de permanencia permitido en planta (horas).
pub const TIEMPO_MAXIMO_HORAS: i64 = 14;

/// Tiempo para disparo de alerta temprana de salida (13h 30min).
pub const TIEMPO_ALERTA_TEMPRANA_MINUTOS: i64 = 13 * 60 + 30;

/// Tiempo máximo convertido a minutos para cálculos internos (840 min).
pub const TIEMPO_MAXIMO_MINUTOS: i64 = TIEMPO_MAXIMO_HORAS * 60;

/// Ventana de anticipación para alertas de vencimiento de documentos (30 días).
pub const DIAS_ALERTA_PRAIND: i64 = 30;

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: PERMANENCIA
// --------------------------------------------------------------------------

/// Calcula los minutos exactos transcurridos desde el ingreso.
pub fn calcular_tiempo_transcurrido(
    fecha_ingreso_str: &str,
) -> Result<i64, IngresoContratistaError> {
    let fecha_ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| {
            IngresoContratistaError::Validation("Formato de fecha de ingreso inválido".to_string())
        })?
        .with_timezone(&Utc);

    let ahora = Utc::now();
    let duration = ahora.signed_duration_since(fecha_ingreso);

    Ok(duration.num_minutes())
}

/// Calcula el margen de tiempo restante antes de incurrir en infracción.
pub fn calcular_minutos_restantes(minutos_transcurridos: i64) -> i64 {
    TIEMPO_MAXIMO_MINUTOS - minutos_transcurridos
}

/// Determina la categoría de permanencia basada en el tiempo transcurrido.
pub fn evaluar_estado_permanencia(minutos_transcurridos: i64) -> EstadoPermanencia {
    if minutos_transcurridos >= TIEMPO_MAXIMO_MINUTOS {
        EstadoPermanencia::TiempoExcedido
    } else if minutos_transcurridos >= TIEMPO_ALERTA_TEMPRANA_MINUTOS {
        EstadoPermanencia::AlertaTemprana
    } else {
        EstadoPermanencia::Normal
    }
}

/// Construye un objeto de alerta con metadatos descriptivos para la UI.
pub fn construir_alerta_tiempo(minutos_transcurridos: i64) -> AlertaTiempo {
    let estado = evaluar_estado_permanencia(minutos_transcurridos);
    let minutos_restantes = calcular_minutos_restantes(minutos_transcurridos);

    let mensaje = match estado {
        EstadoPermanencia::TiempoExcedido => {
            Some(format!("TIEMPO EXCEDIDO por {} min", minutos_restantes.abs()))
        }
        EstadoPermanencia::AlertaTemprana => {
            Some(format!("Alerta: Quedan {} min", minutos_restantes))
        }
        EstadoPermanencia::Normal => None,
    };

    AlertaTiempo { estado, minutos_transcurridos, minutos_restantes, mensaje }
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: SALIDA
// --------------------------------------------------------------------------

/// Valida que el ingreso esté abierto antes de registrar salida.
pub fn validar_ingreso_abierto(
    fecha_salida: &Option<String>,
) -> Result<(), IngresoContratistaError> {
    if fecha_salida.is_some() {
        return Err(IngresoContratistaError::NoActiveIngreso);
    }
    Ok(())
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: ELEGIBILIDAD DE ENTRADA (VISITANTES)
// --------------------------------------------------------------------------

/// Evalúa todas las reglas para determinar si un visitante puede entrar.
/// Mismas reglas que contratistas EXCEPTO: no valida PRAIND ni estado.
#[deprecated(note = "Use motor_validacion::ejecutar_validacion_motor with MotorContexto instead")]
pub fn evaluar_elegibilidad_visita(
    esta_bloqueado: bool,
    motivo_bloqueo: Option<String>,
    tiene_ingreso_abierto: bool,
    cantidad_alertas_gafete: usize,
) -> crate::models::ingreso::ResultadoValidacionEntrada {
    use crate::models::ingreso::ResultadoValidacionEntrada;

    let mut alertas = Vec::new();

    // 1. REGLA BLOQUEANTE: Lista Negra
    if esta_bloqueado {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!(
                "VISITANTE BLOQUEADO: {}",
                motivo_bloqueo.unwrap_or_default()
            )),
            alertas,
        };
    }

    // 2. REGLA BLOQUEANTE: Ingreso Abierto (Duplicado)
    if tiene_ingreso_abierto {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some("El visitante ya tiene un ingreso activo".to_string()),
            alertas,
        };
    }

    // 3. REGLA BLOQUEANTE: Más de 1 gafete pendiente (límite 2 = bloqueado)
    if cantidad_alertas_gafete >= 2 {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!(
                "Debe {} gafetes. Regularice antes de ingresar",
                cantidad_alertas_gafete
            )),
            alertas,
        };
    }

    // 4. REGLA NO BLOQUEANTE: 1 alerta de gafete = warning
    if cantidad_alertas_gafete == 1 {
        alertas.push("⚠️ Tiene 1 gafete pendiente de devolución".to_string());
    }

    ResultadoValidacionEntrada { puede_ingresar: true, motivo_rechazo: None, alertas }
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: PRAIND ALERTAS
// --------------------------------------------------------------------------

/// Evalúa si el PRAIND está por vencer (dentro de los próximos 30 días)
pub fn praind_por_vencer(fecha_vencimiento_str: &str) -> Result<bool, IngresoContratistaError> {
    if fecha_vencimiento_str.is_empty() {
        return Ok(false);
    }

    let fecha_venc =
        chrono::NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d").map_err(|_| {
            IngresoContratistaError::Validation(format!(
                "Formato de fecha inválido: {}",
                fecha_vencimiento_str
            ))
        })?;

    let hoy = Utc::now().date_naive();
    let dias_restantes = (fecha_venc - hoy).num_days();

    // Está por vencer si: 0 <= días_restantes <= 30
    Ok(dias_restantes >= 0 && dias_restantes <= DIAS_ALERTA_PRAIND)
}

/// Calcula los días restantes hasta el vencimiento del PRAIND
pub fn dias_hasta_vencimiento_praind(
    fecha_vencimiento_str: &str,
) -> Result<i64, IngresoContratistaError> {
    if fecha_vencimiento_str.is_empty() {
        return Err(IngresoContratistaError::Validation("Fecha vacía".to_string()));
    }

    let fecha_venc =
        chrono::NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d").map_err(|_| {
            IngresoContratistaError::Validation(format!(
                "Formato de fecha inválido: {}",
                fecha_vencimiento_str
            ))
        })?;

    let hoy = Utc::now().date_naive();
    Ok((fecha_venc - hoy).num_days())
}

/// Determina si el contratista debe ser suspendido por PRAIND vencido
/// Retorna true si la fecha de vencimiento es anterior a hoy (00:00)
pub fn debe_suspender_por_praind(
    fecha_vencimiento_str: &str,
) -> Result<bool, IngresoContratistaError> {
    let dias = dias_hasta_vencimiento_praind(fecha_vencimiento_str)?;
    Ok(dias < 0)
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: CIERRE MANUAL
// --------------------------------------------------------------------------

/// Evalúa si un ingreso puede cerrarse manualmente y qué acciones tomar
pub fn evaluar_cierre_manual(
    fecha_hora_ingreso: &str,
    motivo: &MotivoCierre,
) -> Result<ResultadoCierreManual, IngresoContratistaError> {
    let minutos_transcurridos = calcular_tiempo_transcurrido(fecha_hora_ingreso)?;
    let estado_permanencia = evaluar_estado_permanencia(minutos_transcurridos);

    // Siempre se puede cerrar manualmente
    let puede_cerrar = true;

    // Genera reporte si el cierre es sospechoso
    let genera_reporte =
        matches!(motivo, MotivoCierre::SalioSinRegistrar | MotivoCierre::PersonaNoLocalizada)
            || estado_permanencia == EstadoPermanencia::TiempoExcedido;

    let tipo_reporte = if genera_reporte { Some("cierre_manual".to_string()) } else { None };

    let mensaje = if estado_permanencia == EstadoPermanencia::TiempoExcedido {
        Some(format!(
            "Tiempo excedido: {} minutos ({} horas)",
            minutos_transcurridos,
            minutos_transcurridos / 60
        ))
    } else {
        None
    };

    Ok(ResultadoCierreManual { puede_cerrar, genera_reporte, tipo_reporte, mensaje })
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: INGRESO EXCEPCIONAL
// --------------------------------------------------------------------------

/// Evalúa si un ingreso excepcional es válido
///
/// Un ingreso excepcional permite la entrada de un contratista que normalmente
/// no podría ingresar (suspendido, PRAIND vencido, etc.) con autorización.
pub fn evaluar_ingreso_excepcional(
    motivo_bloqueo_original: &str,
    autorizado_por_id: &str,
    motivo: &MotivoExcepcional,
    notas: Option<&str>,
) -> ResultadoIngresoExcepcional {
    // Siempre se permite si hay un supervisor que autoriza
    // La responsabilidad recae en el guardia y el supervisor autorizado

    let ahora = Utc::now();
    let fin_del_dia = ahora.date_naive().and_hms_opt(23, 59, 59).unwrap();

    ResultadoIngresoExcepcional {
        permitido: true,
        motivo_original_bloqueo: motivo_bloqueo_original.to_string(),
        autorizado_por: autorizado_por_id.to_string(),
        motivo_excepcional: motivo.clone(),
        notas: notas.map(|s| s.to_string()),
        valido_hasta: fin_del_dia.to_string(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluar_elegibilidad_bloqueado() {
        let res =
            evaluar_elegibilidad_entrada(true, Some("Robo".to_string()), false, "activo", true, 0);
        assert!(!res.puede_ingresar);
        assert!(res.motivo_rechazo.unwrap().contains("BLOQUEADO"));
    }

    #[test]
    fn test_evaluar_elegibilidad_ingreso_abierto() {
        let res = evaluar_elegibilidad_entrada(false, None, true, "activo", true, 0);
        assert!(!res.puede_ingresar);
        assert!(res.motivo_rechazo.unwrap().contains("activo"));
    }

    #[test]
    fn test_evaluar_elegibilidad_praind_vencido() {
        let res = evaluar_elegibilidad_entrada(false, None, false, "activo", false, 0);
        assert!(!res.puede_ingresar);
        assert!(res.motivo_rechazo.unwrap().contains("PRAIND"));
    }

    #[test]
    fn test_evaluar_elegibilidad_valida() {
        let res = evaluar_elegibilidad_entrada(false, None, false, "activo", true, 2);
        assert!(res.puede_ingresar);
        assert_eq!(res.alertas.len(), 1); // Tiene 2 alertas de gafete
    }

    #[test]
    fn test_verificar_praind_vigente() {
        let hoy = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        assert!(verificar_praind_vigente(&hoy).unwrap());
        assert!(!verificar_praind_vigente("2000-01-01").unwrap());
    }

    #[test]
    fn test_evaluar_estado_permanencia() {
        assert_eq!(evaluar_estado_permanencia(100), EstadoPermanencia::Normal);
        assert_eq!(evaluar_estado_permanencia(820), EstadoPermanencia::AlertaTemprana);
        assert_eq!(evaluar_estado_permanencia(850), EstadoPermanencia::TiempoExcedido);
    }

    #[test]
    fn test_validar_tiempo_salida() {
        let ingreso = "2023-12-22T08:00:00Z";
        let salida_valida = "2023-12-22T10:00:00Z";
        let salida_invalida = "2023-12-22T07:00:00Z";

        assert!(validar_tiempo_salida(ingreso, salida_valida).is_ok());
        assert!(validar_tiempo_salida(ingreso, salida_invalida).is_err());
    }

    #[test]
    fn test_evaluar_devolucion_gafete() {
        // Todo OK
        let res = evaluar_devolucion_gafete(true, Some("G-1"), true, Some("G-1"));
        assert!(!res.debe_generar_reporte);

        // No lo devolvió
        let res = evaluar_devolucion_gafete(true, Some("G-1"), false, None);
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("sin devolver"));

        // Devolvió uno distinto
        let res = evaluar_devolucion_gafete(true, Some("G-1"), true, Some("G-2"));
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("incorrecto"));
    }

    // ==========================================
    // TESTS NUEVOS: PRAIND ALERTAS
    // ==========================================

    #[test]
    fn test_praind_por_vencer() {
        // Fecha en 15 días - debería estar por vencer
        let en_15_dias =
            (Utc::now().date_naive() + chrono::Duration::days(15)).format("%Y-%m-%d").to_string();
        assert!(praind_por_vencer(&en_15_dias).unwrap());

        // Fecha en 60 días - no debería estar por vencer
        let en_60_dias =
            (Utc::now().date_naive() + chrono::Duration::days(60)).format("%Y-%m-%d").to_string();
        assert!(!praind_por_vencer(&en_60_dias).unwrap());

        // Fecha pasada - no está "por vencer", ya venció
        assert!(!praind_por_vencer("2000-01-01").unwrap());
    }

    #[test]
    fn test_dias_hasta_vencimiento_praind() {
        let hoy = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        assert_eq!(dias_hasta_vencimiento_praind(&hoy).unwrap(), 0);

        let manana =
            (Utc::now().date_naive() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        assert_eq!(dias_hasta_vencimiento_praind(&manana).unwrap(), 1);
    }

    #[test]
    fn test_debe_suspender_por_praind() {
        // Ayer - debe suspender
        let ayer =
            (Utc::now().date_naive() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        assert!(debe_suspender_por_praind(&ayer).unwrap());

        // Mañana - no debe suspender
        let manana =
            (Utc::now().date_naive() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        assert!(!debe_suspender_por_praind(&manana).unwrap());

        // Hoy - no debe suspender (vence a las 23:59)
        let hoy = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        assert!(!debe_suspender_por_praind(&hoy).unwrap());
    }

    // ==========================================
    // TESTS NUEVOS: CIERRE MANUAL
    // ==========================================

    #[test]
    fn test_motivo_cierre_from_str() {
        assert_eq!(
            "olvido_registrar_salida".parse::<MotivoCierre>().unwrap(),
            MotivoCierre::OlvidoRegistrarSalida
        );
        assert_eq!(
            "salio_sin_registrar".parse::<MotivoCierre>().unwrap(),
            MotivoCierre::SalioSinRegistrar
        );
        assert!("invalido".parse::<MotivoCierre>().is_err());
    }

    #[test]
    fn test_motivo_excepcional_from_str() {
        assert_eq!(
            "orden_seguridad_industrial".parse::<MotivoExcepcional>().unwrap(),
            MotivoExcepcional::OrdenSeguridadIndustrial
        );
        assert_eq!(
            "emergencia_operativa".parse::<MotivoExcepcional>().unwrap(),
            MotivoExcepcional::EmergenciaOperativa
        );
    }

    #[test]
    fn test_evaluar_ingreso_excepcional() {
        let resultado = evaluar_ingreso_excepcional(
            "PRAIND vencido",
            "supervisor-123",
            &MotivoExcepcional::OrdenSeguridadIndustrial,
            Some("Orden verbal del gerente"),
        );

        assert!(resultado.permitido);
        assert_eq!(resultado.autorizado_por, "supervisor-123");
        assert_eq!(resultado.motivo_original_bloqueo, "PRAIND vencido");
    }
}
