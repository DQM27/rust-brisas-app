// src/domain/ingreso_contratista.rs

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
    let mut puede_ingresar = true;
    let mut motivo_rechazo = None;
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
        alertas.push(format!(
            "Tiene {} alerta(s) de gafete pendiente",
            cantidad_alertas_gafete
        ));
    }

    ResultadoValidacionEntrada {
        puede_ingresar,
        motivo_rechazo,
        alertas,
    }
}

pub fn verificar_praind_vigente(fecha_vencimiento_str: &str) -> Result<bool, String> {
    if fecha_vencimiento_str.is_empty() {
        return Ok(false);
    }

    // Intentar parsear solo fecha YYYY-MM-DD
    let fecha_venc =
        chrono::NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d").map_err(|_| {
            format!(
                "Formato de fecha de vencimiento inválido: {}. Se espera YYYY-MM-DD",
                fecha_vencimiento_str
            )
        })?;

    let hoy = Utc::now().date_naive();

    // Es vigente si la fecha de vencimiento es mayor o igual a hoy
    Ok(fecha_venc >= hoy)
}

pub fn normalizar_numero_gafete(input: &str) -> String {
    input.trim().to_uppercase()
}

pub fn validar_input_entrada(input: &impl InputEntrada) -> Result<(), String> {
    if input.tipo_ingreso() != "contratista" {
        return Err("Tipo de ingreso inválido para este servicio".to_string());
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

pub fn calcular_tiempo_transcurrido(fecha_ingreso_str: &str) -> Result<i64, String> {
    let fecha_ingreso = DateTime::parse_from_rfc3339(fecha_ingreso_str)
        .map_err(|_| "Formato de fecha de ingreso inválido".to_string())?
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
        EstadoPermanencia::TiempoExcedido => Some(format!(
            "TIEMPO EXCEDIDO por {} min",
            minutos_restantes.abs()
        )),
        EstadoPermanencia::AlertaTemprana => {
            Some(format!("Alerta: Quedan {} min", minutos_restantes))
        }
        EstadoPermanencia::Normal => None,
    };

    AlertaTiempo {
        estado,
        minutos_transcurridos,
        minutos_restantes,
        mensaje,
    }
}

// ==========================================
// LOGICA DE DOMINIO: SALIDA
// ==========================================

pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), String> {
    if fecha_salida.is_some() {
        return Err("El ingreso ya fue cerrado (registrado como salida)".to_string());
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
                motivo: Some(format!(
                    "Devolvió gafete incorrecto: {} vs {}",
                    devuelto, asignado
                )),
                gafete_numero: Some(asignado.to_string()),
            });
        }
    }

    // Caso 3: Todo OK
    Ok(DecisionReporteGafete {
        debe_generar_reporte: false,
        motivo: None,
        gafete_numero: None,
    })
}
