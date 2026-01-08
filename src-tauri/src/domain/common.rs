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
pub use crate::domain::errors::CommonError;
pub use crate::models::ingreso::DecisionReporteGafete;

// --------------------------------------------------------------------------
// CONSTANTES DE LÍMITES ESTÁNDAR
// --------------------------------------------------------------------------

/// Longitud mínima para nombres y apellidos.
pub const MIN_LEN_NOMBRE: usize = 2;

/// Longitud máxima para nombres y apellidos (primeros y segundos).
pub const MAX_LEN_NOMBRE: usize = 50;

/// Longitud máxima para direcciones postales.
pub const MAX_LEN_DIRECCION: usize = 200;

/// Longitud máxima para nombres de empresa.
pub const MAX_LEN_EMPRESA: usize = 50;

/// Longitud máxima para números de teléfono.
pub const MAX_LEN_TELEFONO: usize = 20;

/// Longitud máxima para identificador de gafete (texto).
pub const MAX_LEN_GAFETE: usize = 20;

/// Longitud mínima requerida para contraseñas de usuario.
pub const MIN_LEN_PASSWORD: usize = 6;

/// Longitud máxima para emails.
pub const MAX_LEN_EMAIL: usize = 50;

/// Longitud mínima de dígitos para cédula (sin contar guiones).
pub const MIN_DIGITOS_CEDULA: usize = 8;

/// Longitud máxima de dígitos para cédula (sin contar guiones).
pub const MAX_DIGITOS_CEDULA: usize = 14;

/// Longitud máxima para marca de vehículo.
pub const MAX_LEN_MARCA_VEHICULO: usize = 50;

/// Longitud máxima para modelo de vehículo.
pub const MAX_LEN_MODELO_VEHICULO: usize = 50;

/// Longitud máxima para color de vehículo.
pub const MAX_LEN_COLOR_VEHICULO: usize = 30;

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
            "Formato de gafete inválido: '{input}'. Debe ser un número positivo o 'S/G'."
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
                    "Discrepancia: Devolvió gafete incorrecto (Detectado: {devuelto} / Esperado: {asignado})"
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
// CONTROL DE INGRESOS: VALIDACIÓN DE ESTADO
// --------------------------------------------------------------------------

/// Valida que un registro de ingreso esté abierto (sin fecha de salida).
///
/// Retorna error si ya existe una fecha de salida, indicando que el ingreso
/// ya fue cerrado y no puede procesarse nuevamente.
pub fn validar_ingreso_abierto(fecha_salida: &Option<String>) -> Result<(), CommonError> {
    if fecha_salida.is_some() {
        return Err(CommonError::Validation("El ingreso ya fue cerrado".to_string()));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIÓN DE FECHAS: ESTÁNDARES ISO 8601 / RFC 3339
// --------------------------------------------------------------------------
// GUÍA DE USO:
// - Use `validar_fecha_rfc3339` para timestamps con hora (ingresos, salidas, created_at)
// - Use `validar_fecha_simple` para fechas sin hora (vencimiento PRAIND, cumpleaños)
// - El frontend puede mostrar DD/MM/YYYY pero debe enviar en estos formatos estándar

/// Valida que una cadena sea un `DateTime` válido en formato RFC 3339.
///
/// Formato esperado: `YYYY-MM-DDThh:mm:ssZ` o con offset `+/-hh:mm`
/// Ejemplo: `2026-01-15T08:30:00Z`
///
/// Uso: Timestamps de sistema (ingreso, salida, `created_at`, `updated_at`)
pub fn validar_fecha_rfc3339(fecha_str: &str) -> Result<(), CommonError> {
    if fecha_str.trim().is_empty() {
        return Err(CommonError::Validation("La fecha/hora es obligatoria".to_string()));
    }

    DateTime::parse_from_rfc3339(fecha_str).map_err(|_| {
        CommonError::Validation(
            "Formato de fecha/hora inválido. Use ISO 8601 (ej: 2026-01-15T08:30:00Z)".to_string(),
        )
    })?;

    Ok(())
}

/// Valida que una cadena sea una fecha válida en formato YYYY-MM-DD.
///
/// Formato esperado: `YYYY-MM-DD`
/// Ejemplo: `2026-12-31`
///
/// Uso: Fechas conceptuales sin hora (vencimiento de documentos, cumpleaños)
pub fn validar_fecha_simple(fecha_str: &str) -> Result<(), CommonError> {
    if fecha_str.trim().is_empty() {
        return Err(CommonError::Validation("La fecha es obligatoria".to_string()));
    }

    chrono::NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").map_err(|_| {
        CommonError::Validation(
            "Formato de fecha inválido. Use YYYY-MM-DD (ej: 2026-12-31)".to_string(),
        )
    })?;

    Ok(())
}

/// Parsea una fecha simple (YYYY-MM-DD) y retorna un `NaiveDate`.
pub fn parsear_fecha_simple(fecha_str: &str) -> Result<chrono::NaiveDate, CommonError> {
    chrono::NaiveDate::parse_from_str(fecha_str.trim(), "%Y-%m-%d").map_err(|_| {
        CommonError::Validation("Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    })
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
/// - Entre `MIN_LEN_NOMBRE` y `MAX_LEN_NOMBRE` caracteres.
pub fn validar_nombre_estandar(texto: &str, campo: &str) -> Result<(), CommonError> {
    let limpio = texto.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation(format!("El {campo} es obligatorio")));
    }

    if limpio.len() < MIN_LEN_NOMBRE {
        return Err(CommonError::Validation(format!(
            "El {campo} debe tener al menos {MIN_LEN_NOMBRE} caracteres"
        )));
    }

    if limpio.len() > MAX_LEN_NOMBRE {
        return Err(CommonError::Validation(format!(
            "El {campo} no puede exceder {MAX_LEN_NOMBRE} caracteres"
        )));
    }

    // Permitimos alfabéticos (incluye áéí...), espacios y puntos (para Ing., Dr., etc.).
    // Rechazamos todo lo demás (números, símbolos).
    if !limpio.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '.') {
        return Err(CommonError::Validation(format!(
            "El {campo} solo puede contener letras (sin números ni símbolos)"
        )));
    }

    Ok(())
}

/// Valida una cédula bajo el estándar estricto:
/// - Solo números y guiones.
/// - Sin letras (V/E prohibidas).
/// - Entre `MIN_DIGITOS_CEDULA` y `MAX_DIGITOS_CEDULA` dígitos (sin contar guiones).
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

    // Contar solo los dígitos (ignorando guiones)
    let cantidad_digitos = limpio.chars().filter(char::is_ascii_digit).count();

    if !(MIN_DIGITOS_CEDULA..=MAX_DIGITOS_CEDULA).contains(&cantidad_digitos) {
        return Err(CommonError::Validation(format!(
            "La cédula debe tener entre {MIN_DIGITOS_CEDULA} y {MAX_DIGITOS_CEDULA} dígitos (actualmente tiene {cantidad_digitos})"
        )));
    }

    Ok(())
}

/// Normaliza un nombre propio a formato "Title Case".
/// Ej: "JUAN pérez" -> "Juan Pérez"
pub fn normalizar_nombre_propio(texto: &str) -> String {
    texto
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

    if limpio.len() > EMAIL_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "El email no puede exceder {EMAIL_MAX_LEN} caracteres"
        )));
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
            "La placa debe tener entre {PLACA_MIN_LEN} y {PLACA_MAX_LEN} caracteres"
        )));
    }

    Ok(())
}

/// Valida el nombre de una entidad (Empresa, Institución, etc).
/// Permite números y caracteres especiales básicos pero rechaza inyecciones.
pub fn validar_nombre_entidad_estandar(nombre: &str, campo: &str) -> Result<(), CommonError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(CommonError::Validation(format!("El {campo} es obligatorio")));
    }

    if limpio.len() > ENTIDAD_NOMBRE_MAX_LEN {
        return Err(CommonError::Validation(format!(
            "El {campo} no puede exceder {ENTIDAD_NOMBRE_MAX_LEN} caracteres"
        )));
    }

    // Rechazamos caracteres sospechosos de inyección/formato roto
    let prohibidos = ['<', '>', '{', '}', '|', '\\', '^', '`'];
    if limpio.chars().any(|c| prohibidos.contains(&c)) {
        return Err(CommonError::Validation(format!(
            "El {campo} contiene caracteres no permitidos"
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
                "El {campo} no puede exceder {max_len} caracteres"
            )));
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------
// PARSEO DE IDENTIFICADORES (IDs de SurrealDB)
// --------------------------------------------------------------------------

/// Parsea un string de ID a un `RecordId` de `SurrealDB`.
///
/// Acepta dos formatos:
/// - Compuesto: `"tabla:id123"` → `RecordId` { tabla, "id123" }
/// - Simple: `"id123"` → `RecordId` { `tabla_default`, "id123" }
///
/// # Argumentos
/// * `id_str` - El string con el ID (puede incluir o no el nombre de tabla)
/// * `default_table` - Tabla a usar si el ID no incluye prefijo de tabla
///
/// # Ejemplo
/// ```rust
/// # use brisas_app_lib::domain::common::parse_record_id;
/// let id = parse_record_id("visitante:abc123", "visitante"); // tabla: visitante, key: abc123
/// let id = parse_record_id("abc123", "visitante"); // tabla: visitante, key: abc123
/// ```
pub fn parse_record_id(id_str: &str, default_table: &str) -> surrealdb::RecordId {
    if id_str.contains(':') {
        let parts: Vec<&str> = id_str.split(':').collect();
        surrealdb::RecordId::from_table_key(parts[0], parts[1])
    } else {
        surrealdb::RecordId::from_table_key(default_table, id_str)
    }
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
        assert!(validar_nombre_estandar("Juan Pérez", "nombre").is_ok());
        assert!(validar_nombre_estandar("María José", "nombre").is_ok());
        assert!(validar_nombre_estandar("Juan123", "nombre").is_err());
        assert!(validar_nombre_estandar("Juan!", "nombre").is_err());
    }

    #[test]
    fn test_validar_cedula_estandar() {
        assert!(validar_cedula_estandar("12345678").is_ok());
        assert!(validar_cedula_estandar("12-345-678").is_ok());
        assert!(validar_cedula_estandar("V-123456").is_err());
    }

    #[test]
    fn test_normalizar_nombre_propio() {
        assert_eq!(normalizar_nombre_propio("juan pérez"), "Juan Pérez");
        assert_eq!(normalizar_nombre_propio("MARÍA JOSÉ"), "María José");
    }
}
