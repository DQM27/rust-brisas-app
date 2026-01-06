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
// CONSTANTES DE VALIDACIÓN COMPARTIDAS (Fuente de Verdad Única)
// --------------------------------------------------------------------------

// === Identificación Personal ===
/// Longitud mínima de cédula de identidad.
pub const CEDULA_MIN_LEN: usize = 5;
/// Longitud máxima de cédula de identidad.
pub const CEDULA_MAX_LEN: usize = 20;

// === Nombres y Apellidos ===
/// Longitud mínima de nombre/apellido principal (requerido).
pub const NOMBRE_MIN_LEN: usize = 1;
/// Longitud máxima de nombre/apellido principal.
pub const NOMBRE_MAX_LEN: usize = 100;
/// Longitud máxima de segundo nombre/apellido (opcional, min=0).
pub const SEGUNDO_NOMBRE_MAX_LEN: usize = 50;

// === Contacto ===
/// Longitud mínima de teléfono (si se proporciona).
pub const TELEFONO_MIN_LEN: usize = 7;
/// Longitud máxima de teléfono.
pub const TELEFONO_MAX_LEN: usize = 20;
/// Longitud mínima de dirección (si se proporciona).
pub const DIRECCION_MIN_LEN: usize = 5;
/// Longitud máxima de dirección.
pub const DIRECCION_MAX_LEN: usize = 200;
/// Longitud mínima de email.
pub const EMAIL_MIN_LEN: usize = 5;
/// Longitud máxima de email.
pub const EMAIL_MAX_LEN: usize = 100;

// === Seguridad ===
/// Longitud mínima de contraseña.
pub const PASSWORD_MIN_LEN: usize = 6;
/// Longitud máxima de contraseña.
pub const PASSWORD_MAX_LEN: usize = 100;
/// Longitud máxima de número de gafete.
pub const GAFETE_MAX_LEN: usize = 20;

// === Entidades ===
/// Longitud mínima de nombre de entidad (empresa, institución).
pub const ENTIDAD_NOMBRE_MIN_LEN: usize = 1;
/// Longitud máxima de nombre de entidad (empresa, institución).
pub const ENTIDAD_NOMBRE_MAX_LEN: usize = 100;

// === Vehículos ===
/// Longitud mínima de placa de vehículo.
pub const PLACA_MIN_LEN: usize = 2;
/// Longitud máxima de placa de vehículo.
pub const PLACA_MAX_LEN: usize = 15;
/// Longitud mínima de marca/modelo de vehículo (si se proporciona).
pub const MARCA_MODELO_MIN_LEN: usize = 2;
/// Longitud máxima de marca/modelo de vehículo.
pub const MARCA_MODELO_MAX_LEN: usize = 50;
/// Longitud máxima de color de vehículo.
pub const COLOR_MAX_LEN: usize = 30;

// === Lista Negra ===
/// Longitud mínima de motivo de bloqueo (requerido).
pub const MOTIVO_MIN_LEN: usize = 1;
/// Longitud máxima de motivo de bloqueo.
pub const MOTIVO_MAX_LEN: usize = 500;
/// Longitud máxima de observaciones (opcional, min=0).
pub const OBSERVACIONES_MAX_LEN: usize = 1000;

// === Roles ===
/// Longitud mínima de nombre de rol.
pub const ROLE_NAME_MIN_LEN: usize = 1;
/// Longitud máxima de nombre de rol.
pub const ROLE_NAME_MAX_LEN: usize = 50;
/// Longitud máxima de descripción de rol.
pub const ROLE_DESC_MAX_LEN: usize = 200;

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
// ESTÁNDARES DE VALIDACIÓN Y NORMALIZACIÓN (NOMBRES Y CÉDULAS)
// --------------------------------------------------------------------------

/// Valida un nombre o apellido bajo el estándar estricto:
/// - Solo letras (Unicode, incluye acentos).
/// - Espacios permitidos.
/// - Sin números ni caracteres especiales.
pub fn validar_nombre_estandar(texto: &str, campo: &str) -> Result<(), CommonError> {
    let limpio = texto.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation(format!("El {} es obligatorio", campo)));
    }

    if limpio.len() > NOMBRE_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "El {} no puede exceder {} caracteres",
            campo, NOMBRE_MAX_LEN
        )));
    }

    // Permitimos alfabéticos (incluye áéí...) y espacios.
    // Rechazamos todo lo demás (números, símbolos).
    if !limpio.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        return Err(CommonError::Validation(format!(
            "El {} solo puede contener letras (sin números ni símbolos)",
            campo
        )));
    }

    Ok(())
}

/// Valida una cédula bajo el estándar estricto:
/// - Solo números y guiones.
/// - Sin letras (V/E prohibidas).
/// - Al menos un dígito.
pub fn validar_cedula_estandar(cedula: &str) -> Result<(), CommonError> {
    let limpio = cedula.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation("La cédula es obligatoria".to_string()));
    }

    // Solo dígitos y guiones
    if !limpio.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(CommonError::Validation(
            "La cédula solo puede contener números y guiones (sin letras)".to_string(),
        ));
    }

    // Al menos un dígito
    if !limpio.chars().any(|c| c.is_ascii_digit()) {
        return Err(CommonError::Validation("La cédula debe contener números".to_string()));
    }

    if limpio.len() < CEDULA_MIN_LEN || limpio.len() > CEDULA_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "Longitud de cédula inválida ({}-{} caracteres)",
            CEDULA_MIN_LEN, CEDULA_MAX_LEN
        )));
    }

    Ok(())
}

/// Normaliza un nombre propio a formato "Title Case".
/// Ej: "JUAN pérez" -> "Juan Pérez"
pub fn normalizar_nombre_propio(texto: &str) -> String {
    texto
        .trim()
        .split_whitespace()
        .map(|palabra| {
            let mut chars = palabra.chars();
            match chars.next() {
                None => String::new(),
                Some(primera) => {
                    primera.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Valida una dirección de correo electrónico bajo un estándar básico pero estricto.
pub fn validar_email_estandar(email: &str) -> Result<(), CommonError> {
    let limpio = email.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation("El email es obligatorio".to_string()));
    }

    if !limpio.contains('@') || limpio.starts_with('@') || limpio.ends_with('@') {
        return Err(CommonError::Validation("Formato de email inválido".to_string()));
    }

    if limpio.len() > 100 {
        return Err(CommonError::Validation(
            "El email no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida una placa de vehículo (matrícula).
/// - Alfanumérico, guiones y espacios.
/// - Longitud 2-15.
pub fn validar_placa_estandar(placa: &str) -> Result<(), CommonError> {
    let limpia = placa.trim().to_uppercase();

    if limpia.is_empty() {
        return Err(CommonError::Validation("La placa es obligatoria".to_string()));
    }

    if !limpia.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ' ') {
        return Err(CommonError::Validation(
            "La placa solo puede contener letras, números, guiones y espacios".to_string(),
        ));
    }

    if limpia.len() < PLACA_MIN_LEN || limpia.len() > PLACA_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "La placa debe tener entre {} y {} caracteres",
            PLACA_MIN_LEN, PLACA_MAX_LEN
        )));
    }

    Ok(())
}

/// Valida el nombre de una entidad (Empresa, Institución, etc).
/// Permite números y caracteres especiales básicos pero rechaza inyecciones.
pub fn validar_nombre_entidad_estandar(nombre: &str, campo: &str) -> Result<(), CommonError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation(format!("El {} es obligatorio", campo)));
    }

    if limpio.len() > ENTIDAD_NOMBRE_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "El {} no puede exceder {} caracteres",
            campo, ENTIDAD_NOMBRE_MAX_LEN
        )));
    }

    // Rechazamos caracteres sospechosos de inyección/formato roto
    let prohibidos = ['<', '>', '{', '}', '|', '\\', '^', '`'];
    if limpio.chars().any(|c| prohibidos.contains(&c)) {
        return Err(CommonError::Validation(format!(
            "El {} contiene caracteres no permitidos",
            campo
        )));
    }

    Ok(())
}

/// Valida un campo de texto opcional contra una longitud máxima.
pub fn validar_opcional_estandar(
    valor: Option<&String>,
    max_len: usize,
    campo: &str,
) -> Result<(), CommonError> {
    if let Some(v) = valor {
        let limpio = v.trim();
        if limpio.len() > max_len {
            return Err(CommonError::Validation(format!(
                "El {} no puede exceder {} caracteres",
                campo, max_len
            )));
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN TRANSVERSAL
// --------------------------------------------------------------------------

/// Normaliza un campo opcional: trim y convierte strings vacíos en None.
pub fn normalizar_opcional_estandar(valor: Option<&String>) -> Option<String> {
    valor.and_then(|v| {
        let limpio = v.trim();
        if limpio.is_empty() {
            None
        } else {
            Some(limpio.to_string())
        }
    })
}

/// Normaliza un nombre propio opcional a Title Case si existe.
pub fn normalizar_nombre_opcional_estandar(valor: Option<&String>) -> Option<String> {
    valor.and_then(|v| {
        let limpio = v.trim();
        if limpio.is_empty() {
            None
        } else {
            Some(normalizar_nombre_propio(limpio))
        }
    })
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

    #[test]
    fn test_validar_nombre_estandar() {
        assert!(validar_nombre_estandar("Juan Pérez", "nombre").is_ok()); // Acentos OK
        assert!(validar_nombre_estandar("María José", "nombre").is_ok()); // Espacios OK
        assert!(validar_nombre_estandar("Juan123", "nombre").is_err()); // Números fail
        assert!(validar_nombre_estandar("Juan!", "nombre").is_err()); // Símbolos fail
    }

    #[test]
    fn test_validar_cedula_estandar() {
        assert!(validar_cedula_estandar("12345678").is_ok());
        assert!(validar_cedula_estandar("12-345-678").is_ok());
        assert!(validar_cedula_estandar("V-123456").is_err()); // Letras fail
    }

    #[test]
    fn test_normalizar_nombre_propio() {
        assert_eq!(normalizar_nombre_propio("juan pérez"), "Juan Pérez");
        assert_eq!(normalizar_nombre_propio("MARÍA JOSÉ"), "María José");
    }
}
