/// Capa de Dominio: Reglas de Negocio para la Gestión de Citas.
///
/// Este módulo centraliza la lógica de validación pura para las citas programadas.
/// Al pertenecer a la capa de dominio, no tiene dependencias de base de datos,
/// enfocándose exclusivamente en asegurar que los datos de entrada cumplan con los
/// requerimientos operativos del sistema antes de ser procesados por los servicios.
use crate::domain::errors::CitaError;
use crate::models::cita::CreateCitaInput;

// --------------------------------------------------------------------------
// CONSTANTES DE VALIDACIÓN
// --------------------------------------------------------------------------

/// Longitud máxima para el nombre del anfitrión.
pub const ANFITRION_MAX_LEN: usize = 100;

/// Longitud máxima para el área visitada.
pub const AREA_MAX_LEN: usize = 100;

/// Longitud máxima para el motivo de la visita.
pub const MOTIVO_MAX_LEN: usize = 255;

/// Validar formato de fecha ISO 8601 básico.
use chrono::DateTime;

// --------------------------------------------------------------------------
// VALIDACIONES DE NEGOCIO
// --------------------------------------------------------------------------

/// Verifica la integridad de los datos para la programación de una nueva cita.
///
/// # Reglas de Validación:
/// 1. **Identificación**: Debe existir `visitante_id` o `visitante_cedula` + `visitante_nombre`.
/// 2. **Responsabilidad**: `anfitrion` no vacío y dentro de límites.
/// 3. **Ubicación**: `area_visitada` no vacía y dentro de límites.
/// 4. **Propósito**: `motivo` explícito y dentro de límites.
/// 5. **Temporalidad**: `fecha_cita` debe ser un formato ISO 8601 válido.
pub fn validar_create_input(input: &CreateCitaInput) -> Result<(), CitaError> {
    // 1. Validación de Identidad del Visitante
    if input.visitante_id.is_none() {
        if input.visitante_cedula.as_ref().map_or(true, |s| s.trim().is_empty()) {
            return Err(CitaError::Validation(
                "Debe proporcionar un visitante registrado o la cédula para uno nuevo".to_string(),
            ));
        }
        if input.visitante_nombre.as_ref().map_or(true, |s| s.trim().is_empty()) {
            return Err(CitaError::Validation(
                "El nombre del visitante es obligatorio para registros nuevos".to_string(),
            ));
        }
    }

    // 2. Validación de Campos Operativos (Límites y Vacío)
    validar_campo_texto(&input.anfitrion, "Anfitrión", ANFITRION_MAX_LEN)?;
    validar_campo_texto(&input.area_visitada, "Área visitada", AREA_MAX_LEN)?;
    validar_campo_texto(&input.motivo, "Motivo", MOTIVO_MAX_LEN)?;

    // 3. Validación de Fecha
    if let Err(_) = DateTime::parse_from_rfc3339(&input.fecha_cita) {
        return Err(CitaError::Validation(
            "La fecha de la cita no tiene un formato válido (ISO 8601)".to_string(),
        ));
    }

    Ok(())
}

/// Helper para validar campos de texto genéricos (no vacío, longitud, caracteres prohibidos).
fn validar_campo_texto(texto: &str, nombre_campo: &str, max_len: usize) -> Result<(), CitaError> {
    let limpio = texto.trim();

    if limpio.is_empty() {
        return Err(CitaError::Validation(format!("El campo '{}' es obligatorio", nombre_campo)));
    }

    if limpio.len() > max_len {
        return Err(CitaError::Validation(format!(
            "El campo '{}' excede el límite de {} caracteres",
            nombre_campo, max_len
        )));
    }

    // Caracteres prohibidos básicos (<, >) para evitar inyección HTML/Script simple
    if limpio.contains('<') || limpio.contains('>') {
        return Err(CitaError::Validation(format!(
            "El campo '{}' contiene caracteres no permitidos",
            nombre_campo
        )));
    }

    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper para crear un input válido base
    fn input_valido() -> CreateCitaInput {
        CreateCitaInput {
            visitante_id: Some("visitante:123".to_string()),
            fecha_cita: "2024-01-01T10:00:00Z".to_string(), // ISO 8601 OK
            anfitrion: "Ing. Pedro Perez".to_string(),
            area_visitada: "Sistemas".to_string(),
            motivo: "Soporte Técnico".to_string(),
            visitante_cedula: None,
            visitante_nombre: None,
            visitante_apellido: None,
        }
    }

    #[test]
    fn test_todo_valido() {
        assert!(validar_create_input(&input_valido()).is_ok());
    }

    #[test]
    fn test_validar_fecha_invalida() {
        let mut input = input_valido();
        input.fecha_cita = "fecha-mala".to_string();
        assert!(validar_create_input(&input).is_err());

        input.fecha_cita = "2024/01/01".to_string(); // Formato incorrecto
        assert!(validar_create_input(&input).is_err());
    }

    #[test]
    fn test_validar_inyeccion_html() {
        let mut input = input_valido();
        input.motivo = "Revisión <script>alert('xss')</script>".to_string();
        assert!(validar_create_input(&input).is_err());

        input.motivo = "Hola".to_string(); // Reset
        input.anfitrion = "Juan <br>".to_string();
        assert!(validar_create_input(&input).is_err());
    }

    #[test]
    fn test_validar_limites_longitud() {
        let mut input = input_valido();
        input.motivo = "a".repeat(MOTIVO_MAX_LEN + 1);
        assert!(validar_create_input(&input).is_err());
    }

    #[test]
    fn test_validar_visitante_faltante() {
        let input = CreateCitaInput {
            visitante_id: None,
            visitante_cedula: None, // Faltan datos para crear uno nuevo
            ..input_valido()
        };
        assert!(validar_create_input(&input).is_err());
    }
}
