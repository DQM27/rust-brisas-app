/// Capa de Dominio: Gestión de Lista Negra y Restricciones de Acceso.
///
/// Este módulo define la lógica pura para la validación de personas con acceso
/// denegado a las instalaciones por motivos de seguridad o administrativos.
use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{AddToListaNegraInput, NivelSeveridad, UpdateListaNegraInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que la cédula cumpla con los formatos numéricos y de longitud permitidos.
pub fn validar_cedula(cedula: &str) -> Result<(), ListaNegraError> {
    let limpia = cedula.trim();

    if limpia.is_empty() {
        return Err(ListaNegraError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if !limpia.chars().all(|c| c.is_numeric() || c == '-') {
        return Err(ListaNegraError::Validation(
            "La cédula solo puede contener números y guiones".to_string(),
        ));
    }

    if limpia.len() < 7 || limpia.len() > 20 {
        return Err(ListaNegraError::Validation(
            "La cédula debe tener entre 7 y 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida que el nombre o apellido sea textual y no exceda los límites de longitud.
pub fn validar_nombre(nombre: &str) -> Result<(), ListaNegraError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 100 {
        return Err(ListaNegraError::Validation(
            "El nombre no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida la obligatoriedad y extensión del motivo de bloqueo.
pub fn validar_motivo(motivo: &str) -> Result<(), ListaNegraError> {
    let limpio = motivo.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation(
            "Debe especificar un motivo de bloqueo".to_string(),
        ));
    }

    if limpio.len() > 500 {
        return Err(ListaNegraError::Validation(
            "El motivo no puede exceder 500 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida la identidad de la entidad que solicita el bloqueo.
pub fn validar_bloqueado_por(bloqueado_por: &str) -> Result<(), ListaNegraError> {
    let limpio = bloqueado_por.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation(
            "Debe especificar quién realizó el bloqueo".to_string(),
        ));
    }

    if limpio.len() > 100 {
        return Err(ListaNegraError::Validation(
            "El ID de quien bloqueó no puede exceder 100 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida que el nivel de severidad sea una opción reconocida por el sistema.
pub fn validar_nivel_severidad(nivel: &str) -> Result<NivelSeveridad, ListaNegraError> {
    nivel.parse::<NivelSeveridad>().map_err(|_| {
        ListaNegraError::Validation(
            "Nivel de severidad inválido. Debe ser ALTO, MEDIO o BAJO".to_string(),
        )
    })
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS COMPLETOS
// --------------------------------------------------------------------------

/// Realiza una validación exhaustiva de los datos antes de agregar a la lista negra.
pub fn validar_add_input(input: &AddToListaNegraInput) -> Result<(), ListaNegraError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_nombre(&input.apellido)?;
    validar_motivo(&input.motivo_bloqueo)?;
    validar_bloqueado_por(&input.bloqueado_por)?;
    validar_nivel_severidad(&input.nivel_severidad)?;

    Ok(())
}

/// Valida camapañas de actualización parcial de registros existentes.
pub fn validar_update_input(input: &UpdateListaNegraInput) -> Result<(), ListaNegraError> {
    if let Some(ref motivo) = input.motivo_bloqueo {
        validar_motivo(motivo)?;
    }

    if let Some(ref nivel) = input.nivel_severidad {
        validar_nivel_severidad(nivel)?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// COMPORTAMIENTOS DE DOMINIO
// --------------------------------------------------------------------------

/// Normaliza una cadena de texto eliminando espacios laterales.
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula() {
        assert!(validar_cedula("1234567890").is_ok());
        assert!(validar_cedula("123-456-789").is_ok());
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("123").is_err());
        assert!(validar_cedula("abc123").is_err());
    }

    #[test]
    fn test_validar_nombre() {
        assert!(validar_nombre("Juan").is_ok());
        assert!(validar_nombre("  María  ").is_ok());
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_validar_motivo() {
        assert!(validar_motivo("Comportamiento indebido").is_ok());
        assert!(validar_motivo("").is_err());
        assert!(validar_motivo(&"a".repeat(501)).is_err());
    }

    #[test]
    fn test_validar_nivel_severidad() {
        assert!(validar_nivel_severidad("ALTO").is_ok());
        assert!(validar_nivel_severidad("medio").is_ok());
        assert!(validar_nivel_severidad("Bajo").is_ok());
        assert!(validar_nivel_severidad("INVALIDO").is_err());
    }
}
