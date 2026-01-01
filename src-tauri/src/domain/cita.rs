/// Capa de Dominio: Reglas de Negocio para la Gestión de Citas.
///
/// Este módulo centraliza la lógica de validación pura para las citas programadas.
/// Al pertenecer a la capa de dominio, no tiene dependencias de base de datos,
/// enfocándose exclusivamente en asegurar que los datos de entrada cumplan con los
/// requerimientos operativos del sistema antes de ser procesados por los servicios.
use crate::domain::errors::CitaError;
use crate::models::cita::CreateCitaInput;

// --------------------------------------------------------------------------
// VALIDACIONES DE NEGOCIO
// --------------------------------------------------------------------------

/// Verifica la integridad de los datos para la programación de una nueva cita.
///
/// # Reglas de Validación:
/// 1. **Identificación**: Debe existir un `visitante_id` (visitante recurrente) o,
///    en su defecto, la cédula y nombre para crear uno nuevo.
/// 2. **Responsabilidad**: El campo `anfitrion` es obligatorio para saber quién recibe.
/// 3. **Ubicación**: El `area_visitada` debe estar definida para el control de flujo.
/// 4. **Propósito**: El `motivo` de la visita debe ser explícito.
pub fn validar_create_input(input: &CreateCitaInput) -> Result<(), CitaError> {
    // Validación de Identidad del Visitante
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

    // Validación de Campos Operativos
    if input.anfitrion.trim().is_empty() {
        return Err(CitaError::Validation("El nombre del anfitrión es obligatorio".to_string()));
    }
    if input.area_visitada.trim().is_empty() {
        return Err(CitaError::Validation("El área de destino es obligatoria".to_string()));
    }
    if input.motivo.trim().is_empty() {
        return Err(CitaError::Validation("El motivo de la cita no puede estar vacío".to_string()));
    }

    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Escenario: Validación exitosa para un visitante que ya existe en el sistema.
    #[test]
    fn test_validar_create_input_existente() {
        let input = CreateCitaInput {
            visitante_id: Some("visitante:123".to_string()),
            fecha_cita: "2024-01-01T10:00:00Z".to_string(),
            anfitrion: "Ing. Pedro Perez".to_string(),
            area_visitada: "Sistemas".to_string(),
            motivo: "Soporte Técnico".to_string(),
            visitante_cedula: None,
            visitante_nombre: None,
            visitante_apellido: None,
        };
        assert!(validar_create_input(&input).is_ok());
    }

    /// Escenario: Validación exitosa para un visitante nuevo proporcionando sus datos mínimos.
    #[test]
    fn test_validar_create_input_nuevo_visitante() {
        let input = CreateCitaInput {
            visitante_id: None,
            fecha_cita: "2024-01-01T10:00:00Z".to_string(),
            anfitrion: "Ing. Pedro Perez".to_string(),
            area_visitada: "Sistemas".to_string(),
            motivo: "Soporte Técnico".to_string(),
            visitante_cedula: Some("V-12345678".to_string()),
            visitante_nombre: Some("Juan".to_string()),
            visitante_apellido: Some("Pueblo".to_string()),
        };
        assert!(validar_create_input(&input).is_ok());
    }

    /// Escenario: Error de validación cuando faltan campos mandatorios.
    #[test]
    fn test_validar_create_input_incompleto() {
        let input = CreateCitaInput {
            visitante_id: None,
            fecha_cita: "2024-01-01T10:00:00Z".to_string(),
            anfitrion: "".to_string(), // Campo vacío
            area_visitada: "".to_string(),
            motivo: "".to_string(),
            visitante_cedula: None,
            visitante_nombre: None,
            visitante_apellido: None,
        };
        let result = validar_create_input(&input);
        assert!(result.is_err());
    }
}
