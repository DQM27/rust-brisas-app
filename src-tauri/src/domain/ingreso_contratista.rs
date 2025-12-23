// src/domain/ingreso_contratista.rs

use crate::domain::errors::IngresoContratistaError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ==========================================
// CONSTANTES DE NEGOCIO
// ==========================================

/// Tiempo máximo de permanencia en instalaciones (horas)
pub const TIEMPO_MAXIMO_HORAS: i64 = 14;

/// Tiempo para alerta temprana (minutos) - 30 minutos antes del límite
pub const TIEMPO_ALERTA_TEMPRANA_MINUTOS: i64 = 13 * 60 + 30; // 13h 30min

/// Tiempo máximo en minutos
pub const TIEMPO_MAXIMO_MINUTOS: i64 = TIEMPO_MAXIMO_HORAS * 60; // 840 minutos

// ==========================================
// ENUMS DE ESTADO
// ==========================================

/// Estado del tiempo de permanencia de un contratista
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EstadoPermanencia {
    /// Todo normal, tiempo < 13h 30min
    Normal,
    /// Alerta temprana, tiempo >= 13h 30min y < 14h
    AlertaTemprana,
    /// Tiempo excedido, >= 14h
    TiempoExcedido,
}

impl EstadoPermanencia {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoPermanencia::Normal => "normal",
            EstadoPermanencia::AlertaTemprana => "alerta_temprana",
            EstadoPermanencia::TiempoExcedido => "tiempo_excedido",
        }
    }
}

// ==========================================
// ESTRUCTURAS DE VALIDACIÓN
// ==========================================

/// Resultado de validación de entrada
#[derive(Debug, Clone)]
pub struct ResultadoValidacionEntrada {
    pub puede_ingresar: bool,
    pub motivo_rechazo: Option<String>,
    pub alertas: Vec<String>, // warnings no bloqueantes
}

/// Alerta de tiempo
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertaTiempo {
    pub estado: EstadoPermanencia,
    pub minutos_transcurridos: i64,
    pub minutos_restantes: i64,
    pub mensaje: Option<String>,
}

/// Resultado de evaluación de devolución de gafete
#[derive(Debug, Clone)]
pub struct DecisionReporteGafete {
    pub debe_generar_reporte: bool,
    pub motivo: Option<String>,
    pub gafete_numero: Option<String>,
}

// ==========================================
// LOGICA DE DOMINIO: ENTRADA
// ==========================================

/// Evalúa todas las reglas para determinar si un contratista puede entrar
pub fn evaluar_elegibilidad_entrada(
    esta_bloqueado: bool,
    motivo_bloqueo: Option<String>,
    tiene_ingreso_abierto: bool,
    estado_contratista: &str,
    praind_vigente: bool,
    cantidad_alertas_gafete: usize,
) -> ResultadoValidacionEntrada {
    let puede_ingresar = true;
    let motivo_rechazo = None;
    let mut alertas = Vec::new();

    // 1. REGLA BLOQUEANTE: Lista Negra
    if esta_bloqueado {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!(
                "CONTRATISTA BLOQUEADO: {}",
                motivo_bloqueo.unwrap_or_default()
            )),
            alertas,
        };
    }

    // 2. REGLA BLOQUEANTE: Ingreso Abierto (Duplicado)
    if tiene_ingreso_abierto {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some("El contratista ya tiene un ingreso activo".to_string()),
            alertas,
        };
    }

    // 3. REGLA BLOQUEANTE: Estado Inactivo
    if estado_contratista != "activo" {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some(format!(
                "El contratista no está activo (Estado: {})",
                estado_contratista
            )),
            alertas,
        };
    }

    // 4. REGLA BLOQUEANTE: PRAIND Vencido
    if !praind_vigente {
        return ResultadoValidacionEntrada {
            puede_ingresar: false,
            motivo_rechazo: Some("PRAIND vencido o no válido".to_string()),
            alertas,
        };
    }

    // 5. REGLA NO BLOQUEANTE: Alertas de Gafete
    if cantidad_alertas_gafete > 0 {
        alertas.push(format!("Tiene {} alerta(s) de gafete pendiente", cantidad_alertas_gafete));
    }

    ResultadoValidacionEntrada { puede_ingresar, motivo_rechazo, alertas }
}

pub fn verificar_praind_vigente(
    fecha_vencimiento_str: &str,
) -> Result<bool, IngresoContratistaError> {
    if fecha_vencimiento_str.is_empty() {
        return Ok(false);
    }

    // Intentar parsear solo fecha YYYY-MM-DD
    let fecha_venc =
        chrono::NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d").map_err(|_| {
            IngresoContratistaError::Validation(format!(
                "Formato de fecha de vencimiento inválido: {}. Se espera YYYY-MM-DD",
                fecha_vencimiento_str
            ))
        })?;

    let hoy = Utc::now().date_naive();

    // Es vigente si la fecha de vencimiento es mayor o igual a hoy
    Ok(fecha_venc >= hoy)
}

pub fn normalizar_numero_gafete(input: &str) -> String {
    input.trim().to_uppercase()
}

pub fn validar_input_entrada(input: &impl InputEntrada) -> Result<(), IngresoContratistaError> {
    if input.tipo_ingreso() != "contratista" {
        return Err(IngresoContratistaError::Validation(
            "Tipo de ingreso inválido para este servicio".to_string(),
        ));
    }
    // Validaciones basicas adicionales si hicieran falta
    Ok(())
}

// Trait helper para validar inputs genéricos
pub trait InputEntrada {
    fn tipo_ingreso(&self) -> &str;
}

// ==========================================
// LOGICA DE DOMINIO: PERMANENCIA
// ==========================================

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

pub fn calcular_minutos_restantes(minutos_transcurridos: i64) -> i64 {
    TIEMPO_MAXIMO_MINUTOS - minutos_transcurridos
}

pub fn evaluar_estado_permanencia(minutos_transcurridos: i64) -> EstadoPermanencia {
    if minutos_transcurridos >= TIEMPO_MAXIMO_MINUTOS {
        EstadoPermanencia::TiempoExcedido
    } else if minutos_transcurridos >= TIEMPO_ALERTA_TEMPRANA_MINUTOS {
        EstadoPermanencia::AlertaTemprana
    } else {
        EstadoPermanencia::Normal
    }
}

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

// ==========================================
// LOGICA DE DOMINIO: SALIDA
// ==========================================

pub fn validar_ingreso_abierto(
    fecha_salida: &Option<String>,
) -> Result<(), IngresoContratistaError> {
    if fecha_salida.is_some() {
        return Err(IngresoContratistaError::NoActiveIngreso);
    }
    Ok(())
}

pub fn validar_tiempo_salida(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<(), String> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| "Datos corruptos: fecha ingreso inválida".to_string())?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| "Fecha salida inválida".to_string())?;

    if salida < ingreso {
        return Err("La fecha de salida no puede ser anterior a la de ingreso".to_string());
    }
    Ok(())
}

pub fn calcular_tiempo_permanencia(
    fecha_ingreso_str: &str,
    fecha_salida_str: &str,
) -> Result<i64, String> {
    let ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| "Fecha ingreso inválida".to_string())?;

    let salida = DateTime::parse_from_rfc3339(fecha_salida_str)
        .map_err(|_| "Fecha salida inválida".to_string())?;

    let duracion = salida.signed_duration_since(ingreso);
    Ok(duracion.num_minutes())
}

pub fn validar_gafete_coincide(
    asignado: Option<&str>,
    devuelto: Option<&str>,
) -> Result<(), String> {
    match (asignado, devuelto) {
        (Some(a), Some(d)) => {
            if normalizar_numero_gafete(a) != normalizar_numero_gafete(d) {
                return Err(format!(
                    "El gafete devuelto ({}) no coincide con el asignado ({})",
                    d, a
                ));
            }
        }
        _ => {} // Si no tenía o no devolvió, no hay conflicto de coincidencia aquí
    }
    Ok(())
}

pub fn evaluar_devolucion_gafete(
    tenia_gafete: bool,
    gafete_asignado: Option<&str>, // Número de gafete
    reporto_devolucion: bool,
    gafete_devuelto_numero: Option<&str>, // El numero que dice que devolvió
) -> Result<DecisionReporteGafete, String> {
    // Si no tenía gafete asignado, no hay nada que evaluar
    if !tenia_gafete {
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: false,
            motivo: None,
            gafete_numero: None,
        });
    }

    // SI tenía gafete asignado:

    // Caso 1: Dice que NO lo devolvió (Check desmarcado en frontend)
    if !reporto_devolucion {
        return Ok(DecisionReporteGafete {
            debe_generar_reporte: true,
            motivo: Some("Salida registrada sin devolver gafete".to_string()),
            gafete_numero: gafete_asignado.map(|s| s.to_string()),
        });
    }

    // Caso 2: Dice que SÍ lo devolvió, pero hay discrepancia de números (validación extra)
    if let (Some(asignado), Some(devuelto)) = (gafete_asignado, gafete_devuelto_numero) {
        if normalizar_numero_gafete(asignado) != normalizar_numero_gafete(devuelto) {
            // Esto debería haber fallado antes en `validar_gafete_coincide`, pero por seguridad:
            return Ok(DecisionReporteGafete {
                debe_generar_reporte: true,
                motivo: Some(format!("Devolvió gafete incorrecto: {} vs {}", devuelto, asignado)),
                gafete_numero: Some(asignado.to_string()),
            });
        }
    }

    // Caso 3: Todo OK
    Ok(DecisionReporteGafete { debe_generar_reporte: false, motivo: None, gafete_numero: None })
}

// ==========================================
// LOGICA DE DOMINIO: VISITANTES
// ==========================================

/// Evalúa todas las reglas para determinar si un visitante puede entrar
/// Mismas reglas que contratistas EXCEPTO: no valida PRAIND ni estado
pub fn evaluar_elegibilidad_visita(
    esta_bloqueado: bool,
    motivo_bloqueo: Option<String>,
    tiene_ingreso_abierto: bool,
    cantidad_alertas_gafete: usize,
) -> ResultadoValidacionEntrada {
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

// ==========================================
// LOGICA DE DOMINIO: PRAIND ALERTAS
// ==========================================

/// Días de anticipación para alerta de vencimiento PRAIND
pub const DIAS_ALERTA_PRAIND: i64 = 30;

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

// ==========================================
// LOGICA DE DOMINIO: CIERRE MANUAL
// ==========================================

/// Motivo de cierre manual de un ingreso
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotivoCierre {
    /// El guardia olvidó registrar la salida al momento
    OlvidoRegistrarSalida,
    /// Se confirmó que la persona salió sin registrar
    SalioSinRegistrar,
    /// No se encontró a la persona en las instalaciones
    PersonaNoLocalizada,
    /// Un supervisor autorizó el cierre (caso excepcional)
    AutorizacionEspecial,
}

impl MotivoCierre {
    pub fn as_str(&self) -> &str {
        match self {
            MotivoCierre::OlvidoRegistrarSalida => "olvido_registrar_salida",
            MotivoCierre::SalioSinRegistrar => "salio_sin_registrar",
            MotivoCierre::PersonaNoLocalizada => "persona_no_localizada",
            MotivoCierre::AutorizacionEspecial => "autorizacion_especial",
        }
    }

    pub fn descripcion(&self) -> &str {
        match self {
            MotivoCierre::OlvidoRegistrarSalida => "Se olvidó registrar la salida",
            MotivoCierre::SalioSinRegistrar => "La persona salió sin registrar",
            MotivoCierre::PersonaNoLocalizada => "No se localizó a la persona en instalaciones",
            MotivoCierre::AutorizacionEspecial => "Cierre autorizado por supervisor",
        }
    }
}

impl std::str::FromStr for MotivoCierre {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "olvido_registrar_salida" => Ok(MotivoCierre::OlvidoRegistrarSalida),
            "salio_sin_registrar" => Ok(MotivoCierre::SalioSinRegistrar),
            "persona_no_localizada" => Ok(MotivoCierre::PersonaNoLocalizada),
            "autorizacion_especial" => Ok(MotivoCierre::AutorizacionEspecial),
            _ => Err(format!("Motivo de cierre desconocido: {}", s)),
        }
    }
}

/// Resultado de evaluación de cierre manual
#[derive(Debug, Clone)]
pub struct ResultadoCierreManual {
    pub puede_cerrar: bool,
    pub genera_reporte: bool,
    pub tipo_reporte: Option<String>,
    pub mensaje: Option<String>,
}

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

// ==========================================
// LOGICA DE DOMINIO: INGRESO EXCEPCIONAL
// ==========================================

/// Motivo para un ingreso excepcional (cuando normalmente no podría entrar)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotivoExcepcional {
    /// Orden directa de Seguridad Industrial
    OrdenSeguridadIndustrial,
    /// Emergencia operativa que requiere presencia
    EmergenciaOperativa,
    /// Documentos en trámite con autorización temporal
    DocumentosEnTramite,
    /// Otro motivo especificado en texto libre
    Otro,
}

impl MotivoExcepcional {
    pub fn as_str(&self) -> &str {
        match self {
            MotivoExcepcional::OrdenSeguridadIndustrial => "orden_seguridad_industrial",
            MotivoExcepcional::EmergenciaOperativa => "emergencia_operativa",
            MotivoExcepcional::DocumentosEnTramite => "documentos_en_tramite",
            MotivoExcepcional::Otro => "otro",
        }
    }
}

impl std::str::FromStr for MotivoExcepcional {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "orden_seguridad_industrial" => Ok(MotivoExcepcional::OrdenSeguridadIndustrial),
            "emergencia_operativa" => Ok(MotivoExcepcional::EmergenciaOperativa),
            "documentos_en_tramite" => Ok(MotivoExcepcional::DocumentosEnTramite),
            "otro" => Ok(MotivoExcepcional::Otro),
            _ => Err(format!("Motivo excepcional desconocido: {}", s)),
        }
    }
}

/// Resultado de evaluación de ingreso excepcional
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoIngresoExcepcional {
    pub permitido: bool,
    pub motivo_original_bloqueo: String,
    pub autorizado_por: String,
    pub motivo_excepcional: MotivoExcepcional,
    pub notas: Option<String>,
    pub valido_hasta: String, // Válido solo hasta 23:59 del día
}

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
        let res = evaluar_devolucion_gafete(true, Some("G-1"), true, Some("G-1")).unwrap();
        assert!(!res.debe_generar_reporte);

        // No lo devolvió
        let res = evaluar_devolucion_gafete(true, Some("G-1"), false, None).unwrap();
        assert!(res.debe_generar_reporte);
        assert!(res.motivo.unwrap().contains("sin devolver"));

        // Devolvió uno distinto
        let res = evaluar_devolucion_gafete(true, Some("G-1"), true, Some("G-2")).unwrap();
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
