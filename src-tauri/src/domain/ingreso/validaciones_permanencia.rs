// ==========================================
// src/domain/ingreso/validaciones_permanencia.rs
// ==========================================
// Validaciones y reglas de negocio PURAS para la fase de PERMANENCIA
// (mientras el contratista está dentro de las instalaciones)

use super::tipos::{
    AlertaTiempo, EstadoPermanencia, TIEMPO_ALERTA_TEMPRANA_MINUTOS, TIEMPO_MAXIMO_MINUTOS,
};
use chrono::{NaiveDateTime, Utc};

// ==========================================
// CÁLCULO DE TIEMPO
// ==========================================

/// Calcula el tiempo transcurrido desde el ingreso hasta ahora
/// 
/// Formato esperado: RFC3339 ("2024-01-15T10:30:00-06:00")
/// Retorna: minutos transcurridos
pub fn calcular_tiempo_transcurrido(fecha_hora_ingreso: &str) -> Result<i64, String> {
    let fecha_ingreso = NaiveDateTime::parse_from_str(fecha_hora_ingreso, "%+")
        .map_err(|e| format!("Error parseando fecha de ingreso: {}", e))?;

    let ahora = Utc::now().naive_utc();
    let minutos = (ahora - fecha_ingreso).num_minutes();

    Ok(minutos)
}

/// Calcula el tiempo transcurrido entre dos fechas específicas
/// 
/// Útil para calcular tiempo en un momento específico (no necesariamente "ahora")
pub fn calcular_tiempo_entre_fechas(
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<i64, String> {
    let inicio = NaiveDateTime::parse_from_str(fecha_inicio, "%+")
        .map_err(|e| format!("Error parseando fecha inicio: {}", e))?;

    let fin = NaiveDateTime::parse_from_str(fecha_fin, "%+")
        .map_err(|e| format!("Error parseando fecha fin: {}", e))?;

    let minutos = (fin - inicio).num_minutes();
    Ok(minutos)
}

// ==========================================
// EVALUACIÓN DE ESTADO
// ==========================================

/// Determina el estado de permanencia según el tiempo transcurrido
/// 
/// Reglas:
/// - Normal: < 13h 30min (810 minutos)
/// - AlertaTemprana: >= 13h 30min Y < 14h (810-839 minutos)
/// - TiempoExcedido: >= 14h (840+ minutos)
pub fn evaluar_estado_permanencia(minutos_transcurridos: i64) -> EstadoPermanencia {
    if minutos_transcurridos >= TIEMPO_MAXIMO_MINUTOS {
        EstadoPermanencia::TiempoExcedido
    } else if minutos_transcurridos >= TIEMPO_ALERTA_TEMPRANA_MINUTOS {
        EstadoPermanencia::AlertaTemprana
    } else {
        EstadoPermanencia::Normal
    }
}

/// Calcula minutos restantes hasta el límite
/// 
/// Retorna negativo si ya excedió el tiempo
pub fn calcular_minutos_restantes(minutos_transcurridos: i64) -> i64 {
    TIEMPO_MAXIMO_MINUTOS - minutos_transcurridos
}

// ==========================================
// GENERACIÓN DE ALERTAS
// ==========================================

/// Genera el mensaje de alerta apropiado según el estado
pub fn mensaje_alerta_permanencia(estado: &EstadoPermanencia, minutos_restantes: i64) -> Option<String> {
    match estado {
        EstadoPermanencia::Normal => None,
        EstadoPermanencia::AlertaTemprana => {
            Some(format!(
                "⚠️ Tiempo límite próximo: {} minutos restantes para salir",
                minutos_restantes
            ))
        }
        EstadoPermanencia::TiempoExcedido => {
            let minutos_excedidos = minutos_restantes.abs();
            Some(format!(
                "🚨 TIEMPO EXCEDIDO: {} minutos sobre el límite de 14 horas",
                minutos_excedidos
            ))
        }
    }
}

/// Construye la estructura completa de alerta de tiempo
pub fn construir_alerta_tiempo(minutos_transcurridos: i64) -> AlertaTiempo {
    let estado = evaluar_estado_permanencia(minutos_transcurridos);
    let minutos_restantes = calcular_minutos_restantes(minutos_transcurridos);
    let mensaje = mensaje_alerta_permanencia(&estado, minutos_restantes);

    AlertaTiempo {
        estado,
        minutos_transcurridos,
        minutos_restantes,
        mensaje,
    }
}

// ==========================================
// VERIFICACIONES PERIÓDICAS
// ==========================================

/// Determina si se debe verificar cambios en lista negra
/// 
/// Regla: verificar cada 60 minutos
pub fn requiere_verificacion_lista_negra(minutos_desde_ultima_verificacion: i64) -> bool {
    const INTERVALO_VERIFICACION_MINUTOS: i64 = 60;
    minutos_desde_ultima_verificacion >= INTERVALO_VERIFICACION_MINUTOS
}

/// Determina si se debe enviar notificación de alerta
/// 
/// Regla: solo notificar en transición de estado (no repetir constantemente)
pub fn debe_notificar_alerta(
    estado_anterior: &EstadoPermanencia,
    estado_actual: &EstadoPermanencia,
) -> bool {
    estado_anterior != estado_actual
        && matches!(
            estado_actual,
            EstadoPermanencia::AlertaTemprana | EstadoPermanencia::TiempoExcedido
        )
}

// ==========================================
// FORMATEO DE TIEMPO
// ==========================================

/// Formatea minutos a texto legible (Xh Ymin)
/// 
/// Ejemplos:
/// - 65 → "1h 5min"
/// - 130 → "2h 10min"
/// - 45 → "45min"
/// - 840 → "14h"
pub fn formatear_tiempo_permanencia(minutos: i64) -> String {
    let horas = minutos / 60;
    let mins = minutos % 60;

    if horas > 0 && mins > 0 {
        format!("{}h {}min", horas, mins)
    } else if horas > 0 {
        format!("{}h", horas)
    } else {
        format!("{}min", mins)
    }
}

// ==========================================
// TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // Tests de Evaluación de Estado
    // ==========================================

    #[test]
    fn test_evaluar_estado_normal() {
        // Menos de 13h 30min
        assert_eq!(
            evaluar_estado_permanencia(0),
            EstadoPermanencia::Normal
        );
        assert_eq!(
            evaluar_estado_permanencia(400),
            EstadoPermanencia::Normal
        );
        assert_eq!(
            evaluar_estado_permanencia(809), // justo antes del límite
            EstadoPermanencia::Normal
        );
    }

    #[test]
    fn test_evaluar_estado_alerta_temprana() {
        // Entre 13h 30min y 14h
        assert_eq!(
            evaluar_estado_permanencia(810), // exactamente 13h 30min
            EstadoPermanencia::AlertaTemprana
        );
        assert_eq!(
            evaluar_estado_permanencia(820),
            EstadoPermanencia::AlertaTemprana
        );
        assert_eq!(
            evaluar_estado_permanencia(839), // justo antes de 14h
            EstadoPermanencia::AlertaTemprana
        );
    }

    #[test]
    fn test_evaluar_estado_tiempo_excedido() {
        // 14h o más
        assert_eq!(
            evaluar_estado_permanencia(840), // exactamente 14h
            EstadoPermanencia::TiempoExcedido
        );
        assert_eq!(
            evaluar_estado_permanencia(900),
            EstadoPermanencia::TiempoExcedido
        );
        assert_eq!(
            evaluar_estado_permanencia(1500),
            EstadoPermanencia::TiempoExcedido
        );
    }

    // ==========================================
    // Tests de Cálculo de Tiempo Restante
    // ==========================================

    #[test]
    fn test_calcular_minutos_restantes() {
        assert_eq!(calcular_minutos_restantes(0), 840); // 14h restantes
        assert_eq!(calcular_minutos_restantes(420), 420); // 7h restantes
        assert_eq!(calcular_minutos_restantes(810), 30); // 30min restantes
        assert_eq!(calcular_minutos_restantes(840), 0); // justo en el límite
        assert_eq!(calcular_minutos_restantes(900), -60); // 1h excedido (negativo)
    }

    // ==========================================
    // Tests de Mensajes de Alerta
    // ==========================================

    #[test]
    fn test_mensaje_alerta_normal() {
        let mensaje = mensaje_alerta_permanencia(&EstadoPermanencia::Normal, 500);
        assert!(mensaje.is_none());
    }

    #[test]
    fn test_mensaje_alerta_temprana() {
        let mensaje = mensaje_alerta_permanencia(&EstadoPermanencia::AlertaTemprana, 25);
        assert!(mensaje.is_some());
        let msg = mensaje.unwrap();
        assert!(msg.contains("⚠️"));
        assert!(msg.contains("25"));
        assert!(msg.contains("minutos restantes"));
    }

    #[test]
    fn test_mensaje_alerta_excedido() {
        let mensaje = mensaje_alerta_permanencia(&EstadoPermanencia::TiempoExcedido, -30);
        assert!(mensaje.is_some());
        let msg = mensaje.unwrap();
        assert!(msg.contains("🚨"));
        assert!(msg.contains("30")); // debe mostrar valor absoluto
        assert!(msg.contains("EXCEDIDO"));
    }

    // ==========================================
    // Tests de Construcción de Alerta Completa
    // ==========================================

    #[test]
    fn test_construir_alerta_tiempo_normal() {
        let alerta = construir_alerta_tiempo(400);
        assert_eq!(alerta.estado, EstadoPermanencia::Normal);
        assert_eq!(alerta.minutos_transcurridos, 400);
        assert_eq!(alerta.minutos_restantes, 440);
        assert!(alerta.mensaje.is_none());
    }

    #[test]
    fn test_construir_alerta_tiempo_temprana() {
        let alerta = construir_alerta_tiempo(815);
        assert_eq!(alerta.estado, EstadoPermanencia::AlertaTemprana);
        assert_eq!(alerta.minutos_transcurridos, 815);
        assert_eq!(alerta.minutos_restantes, 25);
        assert!(alerta.mensaje.is_some());
    }

    #[test]
    fn test_construir_alerta_tiempo_excedido() {
        let alerta = construir_alerta_tiempo(900);
        assert_eq!(alerta.estado, EstadoPermanencia::TiempoExcedido);
        assert_eq!(alerta.minutos_transcurridos, 900);
        assert_eq!(alerta.minutos_restantes, -60);
        assert!(alerta.mensaje.is_some());
    }

    // ==========================================
    // Tests de Verificaciones Periódicas
    // ==========================================

    #[test]
    fn test_requiere_verificacion_lista_negra() {
        assert!(!requiere_verificacion_lista_negra(0));
        assert!(!requiere_verificacion_lista_negra(30));
        assert!(!requiere_verificacion_lista_negra(59));
        assert!(requiere_verificacion_lista_negra(60)); // exactamente 1h
        assert!(requiere_verificacion_lista_negra(120)); // 2h
        assert!(requiere_verificacion_lista_negra(180)); // 3h
    }

    #[test]
    fn test_debe_notificar_alerta() {
        // Normal → Normal: NO notificar
        assert!(!debe_notificar_alerta(
            &EstadoPermanencia::Normal,
            &EstadoPermanencia::Normal
        ));

        // Normal → AlertaTemprana: SÍ notificar (transición)
        assert!(debe_notificar_alerta(
            &EstadoPermanencia::Normal,
            &EstadoPermanencia::AlertaTemprana
        ));

        // AlertaTemprana → AlertaTemprana: NO notificar (mismo estado)
        assert!(!debe_notificar_alerta(
            &EstadoPermanencia::AlertaTemprana,
            &EstadoPermanencia::AlertaTemprana
        ));

        // AlertaTemprana → TiempoExcedido: SÍ notificar (transición)
        assert!(debe_notificar_alerta(
            &EstadoPermanencia::AlertaTemprana,
            &EstadoPermanencia::TiempoExcedido
        ));

        // TiempoExcedido → TiempoExcedido: NO notificar (mismo estado)
        assert!(!debe_notificar_alerta(
            &EstadoPermanencia::TiempoExcedido,
            &EstadoPermanencia::TiempoExcedido
        ));

        // TiempoExcedido → Normal: NO notificar (esto no debería pasar, pero validamos)
        assert!(!debe_notificar_alerta(
            &EstadoPermanencia::TiempoExcedido,
            &EstadoPermanencia::Normal
        ));
    }

    // ==========================================
    // Tests de Formateo de Tiempo
    // ==========================================

    #[test]
    fn test_formatear_tiempo_permanencia() {
        assert_eq!(formatear_tiempo_permanencia(0), "0min");
        assert_eq!(formatear_tiempo_permanencia(45), "45min");
        assert_eq!(formatear_tiempo_permanencia(60), "1h");
        assert_eq!(formatear_tiempo_permanencia(65), "1h 5min");
        assert_eq!(formatear_tiempo_permanencia(130), "2h 10min");
        assert_eq!(formatear_tiempo_permanencia(840), "14h");
        assert_eq!(formatear_tiempo_permanencia(900), "15h");
    }

    // ==========================================
    // Tests de Cálculo de Tiempo Entre Fechas
    // ==========================================

    #[test]
    fn test_calcular_tiempo_entre_fechas() {
        let fecha1 = "2024-01-15T10:00:00-06:00";
        let fecha2 = "2024-01-15T12:30:00-06:00";

        let minutos = calcular_tiempo_entre_fechas(fecha1, fecha2).unwrap();
        assert_eq!(minutos, 150); // 2h 30min
    }

    #[test]
    fn test_calcular_tiempo_entre_fechas_mismo_momento() {
        let fecha = "2024-01-15T10:00:00-06:00";
        let minutos = calcular_tiempo_entre_fechas(fecha, fecha).unwrap();
        assert_eq!(minutos, 0);
    }

    #[test]
    fn test_calcular_tiempo_entre_fechas_orden_inverso() {
        let fecha1 = "2024-01-15T12:00:00-06:00";
        let fecha2 = "2024-01-15T10:00:00-06:00";

        let minutos = calcular_tiempo_entre_fechas(fecha1, fecha2).unwrap();
        assert_eq!(minutos, -120); // negativo porque fecha2 es antes
    }

    #[test]
    fn test_calcular_tiempo_formato_invalido() {
        let fecha_valida = "2024-01-15T10:00:00-06:00";
        let fecha_invalida = "2024-01-15 10:00:00";

        assert!(calcular_tiempo_entre_fechas(fecha_valida, fecha_invalida).is_err());
        assert!(calcular_tiempo_entre_fechas(fecha_invalida, fecha_valida).is_err());
    }

    // ==========================================
    // Tests de Integración (Escenarios Reales)
    // ==========================================

    #[test]
    fn test_escenario_completo_ingreso_normal() {
        // Contratista entra a las 8am, son las 3pm (7 horas)
        let minutos = 7 * 60; // 420 minutos

        let alerta = construir_alerta_tiempo(minutos);
        
        assert_eq!(alerta.estado, EstadoPermanencia::Normal);
        assert_eq!(alerta.minutos_transcurridos, 420);
        assert_eq!(alerta.minutos_restantes, 420); // 7h restantes
        assert!(alerta.mensaje.is_none());
        
        let tiempo_fmt = formatear_tiempo_permanencia(minutos);
        assert_eq!(tiempo_fmt, "7h");
    }

    #[test]
    fn test_escenario_completo_alerta_temprana() {
        // Contratista lleva 13h 45min
        let minutos = 13 * 60 + 45; // 825 minutos

        let alerta = construir_alerta_tiempo(minutos);
        
        assert_eq!(alerta.estado, EstadoPermanencia::AlertaTemprana);
        assert_eq!(alerta.minutos_transcurridos, 825);
        assert_eq!(alerta.minutos_restantes, 15); // 15min para límite
        assert!(alerta.mensaje.is_some());
        assert!(alerta.mensaje.unwrap().contains("15"));
        
        let tiempo_fmt = formatear_tiempo_permanencia(minutos);
        assert_eq!(tiempo_fmt, "13h 45min");
    }

    #[test]
    fn test_escenario_completo_tiempo_excedido() {
        // Contratista lleva 15 horas (1 hora de exceso)
        let minutos = 15 * 60; // 900 minutos

        let alerta = construir_alerta_tiempo(minutos);
        
        assert_eq!(alerta.estado, EstadoPermanencia::TiempoExcedido);
        assert_eq!(alerta.minutos_transcurridos, 900);
        assert_eq!(alerta.minutos_restantes, -60); // 1h excedido
        assert!(alerta.mensaje.is_some());
        assert!(alerta.mensaje.unwrap().contains("60"));
        
        let tiempo_fmt = formatear_tiempo_permanencia(minutos);
        assert_eq!(tiempo_fmt, "15h");
    }
}