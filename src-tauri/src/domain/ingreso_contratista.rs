/// Capa de Dominio: Gestión de Permanencia y Seguridad de Contratistas.
///
/// Este módulo centraliza las reglas de negocio puras para el control de acceso de
/// contratistas, incluyendo el cálculo de tiempos de permanencia, gestión de
/// alertas por vencimiento de documentos (PRAIND) y cierres manuales.
use crate::domain::errors::IngresoContratistaError;
use chrono::Utc;

// Re-exportaciones de estructuras desde models
pub use crate::models::ingreso::contratista::{
    AlertaTiempo, EstadoPermanencia, MotivoCierre, MotivoExcepcional, ResultadoCierreManual,
    ResultadoIngresoExcepcional,
};

// Re-exportaciones de funciones comunes
pub use crate::domain::common::{
    calcular_minutos_restantes,
    calcular_tiempo_desde_ingreso,
    calcular_tiempo_permanencia,
    construir_alerta_tiempo,
    evaluar_devolucion_gafete,
    // New common logic
    evaluar_estado_permanencia,
    normalizar_gafete_a_int,
    parsear_fecha_simple,
    validar_gafete_coincide,
    validar_tiempo_salida,
    DecisionReporteGafete,
    TIEMPO_ALERTA_TEMPRANA_MINUTOS,
    TIEMPO_MAXIMO_HORAS,
    TIEMPO_MAXIMO_MINUTOS,
};

/// Ventana de anticipación para alertas de vencimiento de documentos (30 días).
pub const DIAS_ALERTA_PRAIND: i64 = 30;

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: PERMANENCIA
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------

// LÓGICA DE CONTROL: PERMANENCIA
// --------------------------------------------------------------------------

/// Calcula los minutos exactos transcurridos desde el ingreso.
pub fn calcular_tiempo_transcurrido(
    fecha_ingreso_str: &str,
) -> Result<i64, IngresoContratistaError> {
    crate::domain::common::calcular_tiempo_desde_ingreso(fecha_ingreso_str)
        .map_err(|e| IngresoContratistaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: SALIDA
// --------------------------------------------------------------------------

/// Valida que el ingreso esté abierto antes de registrar salida.
pub fn validar_ingreso_abierto(
    fecha_salida: &Option<String>,
) -> Result<(), IngresoContratistaError> {
    crate::domain::common::validar_ingreso_abierto(fecha_salida)
        .map_err(|e| IngresoContratistaError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// LÓGICA DE CONTROL: PRAIND ALERTAS
// --------------------------------------------------------------------------

/// Evalúa si el PRAIND está por vencer (dentro de los próximos 30 días)
pub fn praind_por_vencer(fecha_vencimiento_str: &str) -> Result<bool, IngresoContratistaError> {
    if fecha_vencimiento_str.is_empty() {
        return Ok(false);
    }

    let fecha_venc = parsear_fecha_simple(fecha_vencimiento_str)
        .map_err(|e| IngresoContratistaError::Validation(e.to_string()))?;

    let hoy = Utc::now().date_naive();
    let dias_restantes = (fecha_venc - hoy).num_days();

    // Está por vencer si: 0 <= días_restantes <= 30
    Ok((0..=DIAS_ALERTA_PRAIND).contains(&dias_restantes))
}

/// Calcula los días restantes hasta el vencimiento del PRAIND
pub fn dias_hasta_vencimiento_praind(
    fecha_vencimiento_str: &str,
) -> Result<i64, IngresoContratistaError> {
    if fecha_vencimiento_str.is_empty() {
        return Err(IngresoContratistaError::Validation("Fecha vacía".to_string()));
    }

    let fecha_venc = parsear_fecha_simple(fecha_vencimiento_str)
        .map_err(|e| IngresoContratistaError::Validation(e.to_string()))?;

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
        notas: notas.map(std::string::ToString::to_string),
        valido_hasta: fin_del_dia.to_string(),
    }
}

