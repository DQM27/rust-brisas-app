//! # Dominio: Contratista
//!
//! Contiene las reglas de negocio puras y validaciones para contratistas externos.
//!
//! ## Responsabilidades
//! - Validar formatos de entrada (cédulas, nombres, fechas PRAIND)
//! - Normalizar datos (mayúsculas, espacios, Title Case)
//! - Validar estados permitidos (activo, suspendido, etc.)
//! - Validar DTOs de creación y actualización
//!
//! ## Principios
//! - **Sin efectos secundarios**: Todas las funciones son puras
//! - **Sin dependencias de infraestructura**: No accede a DB ni servicios
//! - **Testing obligatorio**: Cada función tiene al menos un test
//!
//! ## Estándares de Fechas
//! - Fechas de vencimiento PRAIND: YYYY-MM-DD (ej: "2026-12-31")
//!
//! Ver [`common`](crate::domain::common) para funciones centralizadas de validación.

use crate::domain::common::{validar_cedula_estandar, validar_nombre_estandar};
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CreateContratistaInput, EstadoContratista, EstadoPraind, UpdateContratistaInput,
};
use chrono::{DateTime, NaiveDate};

/// Días previos al vencimiento para considerar que requiere atención (alerta amarilla).
pub const DIAS_ALERTA_VENCIMIENTO: i64 = 30;

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula de identidad.
///
/// # Argumentos
/// * `cedula` - Número de documento de identidad.
///
/// # Retorno
/// `Ok(())` si la cédula cumple los requisitos de formato.
pub fn validar_cedula(cedula: &str) -> Result<(), ContratistaError> {
    validar_cedula_estandar(cedula).map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del nombre.
///
/// # Argumentos
/// * `nombre` - Nombre del contratista.
///
/// # Retorno
/// `Ok(())` si el nombre es válido.
pub fn validar_nombre(nombre: &str) -> Result<(), ContratistaError> {
    validar_nombre_estandar(nombre, "nombre")
        .map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del apellido.
///
/// # Argumentos
/// * `apellido` - Apellido del contratista.
///
/// # Retorno
/// `Ok(())` si el apellido es válido.
pub fn validar_apellido(apellido: &str) -> Result<(), ContratistaError> {
    validar_nombre_estandar(apellido, "apellido")
        .map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida que el ID de la empresa vinculada sea válido.
///
/// El ID debe ser un identificador no vacío en formato `SurrealDB`.
///
/// # Argumentos
/// * `empresa_id` - Identificador de empresa (ej: "empresa:abc123")
///
/// # Errores
/// * [`ContratistaError::Validation`] - Si el ID está vacío o solo espacios
///
/// # Ejemplo
/// ```rust,ignore
/// use brisas_app_lib::domain::contratista::validar_empresa_id;
///
/// assert!(validar_empresa_id("empresa:abc123").is_ok());
/// assert!(validar_empresa_id("").is_err());
/// assert!(validar_empresa_id("   ").is_err());
/// ```
pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ContratistaError> {
    let limpia = empresa_id.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation(
            "Debe seleccionar una empresa válida".to_string(),
        ));
    }

    if !limpia.contains(':') {
        return Err(ContratistaError::Validation(
            "ID de empresa con formato inválido (debe ser tabla:id)".to_string(),
        ));
    }

    Ok(())
}

/// Parsea y valida una fecha en formato estándar (YYYY-MM-DD).
/// Devuelve `DateTime`<Utc> para compatibilidad directa con `surrealdb::Datetime`.
pub fn validar_fecha(fecha_str: &str) -> Result<chrono::DateTime<chrono::Utc>, ContratistaError> {
    use chrono::{NaiveDate, TimeZone, Utc};

    let naive = NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").map_err(|_| {
        ContratistaError::Validation("Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    })?;

    Ok(Utc.from_utc_datetime(&naive.and_hms_opt(0, 0, 0).unwrap()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para la creación de un contratista.
///
/// # Argumentos
/// * `input` - DTO con los datos de creación.
///
/// # Retorno
/// `Ok(())` si todos los campos son válidos.
pub fn validar_create_input(input: &CreateContratistaInput) -> Result<(), ContratistaError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;
    validar_empresa_id(&input.empresa_id)?;
    validar_fecha(&input.fecha_vencimiento_praind)?;
    Ok(())
}

/// Valida los cambios parciales solicitados en una actualización.
///
/// # Argumentos
/// * `input` - DTO con los campos opcionales a actualizar.
///
/// # Retorno
/// `Ok(())` si los cambios propuestos son válidos.
pub fn validar_update_input(input: &UpdateContratistaInput) -> Result<(), ContratistaError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre(nombre)?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_apellido(apellido)?;
    }

    if let Some(ref empresa_id) = input.empresa_id {
        validar_empresa_id(empresa_id)?;
    }

    if let Some(ref fecha) = input.fecha_vencimiento_praind {
        validar_fecha(fecha)?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Normaliza una cédula eliminando espacios y convirtiéndola a mayúsculas.
///
/// # Argumentos
/// * `cedula` - Cédula en cualquier formato
///
/// # Ejemplo
/// ```rust,ignore
/// use brisas_app_lib::domain::contratista::normalizar_cedula;
///
/// assert_eq!(normalizar_cedula("  12345678  "), "12345678");
/// assert_eq!(normalizar_cedula("12-345-678"), "12-345-678");
/// ```
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

/// Valida que el estado del contratista sea uno de los valores permitidos.
pub fn validar_estado(estado: &str) -> Result<(), ContratistaError> {
    estado
        .parse::<EstadoContratista>()
        .map_err(|_| ContratistaError::Validation(format!("Estado inválido: {estado}")))?;
    Ok(())
}

// --------------------------------------------------------------------------
// REGLAS DE NEGOCIO: PRAIND Y ACCESO
// --------------------------------------------------------------------------

/// Calcula el estado del PRAIND basado en la fecha de vencimiento.
///
/// # Argumentos
/// * `fecha_vencimiento` - Fecha de vencimiento en formato RFC 3339 o YYYY-MM-DD
///
/// # Retorno
/// Estructura `EstadoPraind` con los cálculos de vencimiento
///
/// # Ejemplo
/// ```rust,ignore
/// let estado = calcular_estado_praind("2026-12-31");
/// if estado.vencido {
///     println!("PRAIND vencido hace {} días", -estado.dias_hasta_vencimiento);
/// }
/// ```
pub fn calcular_estado_praind(fecha_vencimiento_str: &str) -> EstadoPraind {
    use chrono::Utc;

    let hoy = Utc::now();
    let hoy_date = hoy.date_naive();

    // NOTA: Se asume que fecha_vencimiento_str ya viene limpio (sin d'...')
    // La capa de infraestructura (Services/Models) debe encargarse de limpiar formatos de DB.

    // Intentar parsear como RFC 3339 primero, luego como fecha simple
    let fecha_venc: Option<NaiveDate> = DateTime::parse_from_rfc3339(fecha_vencimiento_str)
        .map(|dt| dt.date_naive())
        .ok()
        .or_else(|| NaiveDate::parse_from_str(fecha_vencimiento_str, "%Y-%m-%d").ok());

    match fecha_venc {
        Some(venc_date) => {
            let dias_hasta_vencimiento = (venc_date - hoy_date).num_days();
            let vencido = venc_date < hoy_date;
            let requiere_atencion = (0..=DIAS_ALERTA_VENCIMIENTO).contains(&dias_hasta_vencimiento);

            EstadoPraind { dias_hasta_vencimiento, vencido, requiere_atencion }
        }
        None => {
            // Fecha inválida: tratar como vencido para seguridad
            // Usamos un Result idealmente, pero para no romper compatibilidad mantenemos el log
            // como fail-safe, pero eliminamos el warn directo si queremos ser puristas.
            // Por ahora mantenemos el println/eprintln como debug si es necesario, pero
            // al ser dominio puro, retornamos estado vencido silenciosamente (fail-secure).
            EstadoPraind { dias_hasta_vencimiento: -1, vencido: true, requiere_atencion: false }
        }
    }
}

/// Construye el nombre completo de un contratista.
///
/// # Argumentos
/// * `nombre` - Primer nombre (obligatorio)
/// * `segundo_nombre` - Segundo nombre (opcional)
/// * `apellido` - Primer apellido (obligatorio)
/// * `segundo_apellido` - Segundo apellido (opcional)
///
/// # Ejemplo
/// ```rust,ignore
/// let nombre = construir_nombre_completo("Juan", Some("Carlos"), "Pérez", None);
/// assert_eq!(nombre, "Juan Carlos Pérez");
/// ```
pub fn construir_nombre_completo(
    nombre: &str,
    segundo_nombre: Option<&str>,
    apellido: &str,
    segundo_apellido: Option<&str>,
) -> String {
    let mut completo = nombre.to_string();

    if let Some(segundo) = segundo_nombre {
        if !segundo.is_empty() {
            completo.push(' ');
            completo.push_str(segundo);
        }
    }

    completo.push(' ');
    completo.push_str(apellido);

    if let Some(segundo) = segundo_apellido {
        if !segundo.is_empty() {
            completo.push(' ');
            completo.push_str(segundo);
        }
    }

    completo
}

/// Determina si un contratista puede ingresar a las instalaciones.
///
/// # Reglas de Negocio
/// - El contratista debe estar en estado "Activo"
/// - Su PRAIND no debe estar vencido
///
/// # Argumentos
/// * `estado` - Estado actual del contratista
/// * `praind_vencido` - Si el PRAIND está vencido
///
/// # Retorno
/// `true` si el contratista puede ingresar
pub fn puede_ingresar(
    estado: &crate::models::contratista::EstadoContratista,
    praind_vencido: bool,
) -> bool {
    *estado == crate::models::contratista::EstadoContratista::Activo && !praind_vencido
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

