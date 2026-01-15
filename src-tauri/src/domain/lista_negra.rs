/// Capa de Dominio: Gestión de Lista Negra y Restricciones de Acceso.
///
/// Este módulo define la lógica pura para la validación de personas con acceso
/// denegado a las instalaciones por motivos de seguridad o administrativos.
use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar,
    ENTIDAD_NOMBRE_MAX_LEN, MOTIVO_MAX_LEN,
};

use crate::domain::errors::ListaNegraError;
use crate::models::lista_negra::{AddToListaNegraInput, NivelSeveridad, UpdateListaNegraInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que la cédula sea estricta usando el estándar definido.
pub fn validar_cedula(cedula: &str) -> Result<(), ListaNegraError> {
    validar_cedula_estandar(cedula).map_err(|e| ListaNegraError::Validation(e.to_string()))
}

/// Valida que el nombre cumpla el estándar (solo letras).
pub fn validar_nombre(nombre: &str) -> Result<(), ListaNegraError> {
    validar_nombre_estandar(nombre, "nombre/apellido")
        .map_err(|e| ListaNegraError::Validation(e.to_string()))
}

/// Valida la obligatoriedad y extensión del motivo de bloqueo.
pub fn validar_motivo(motivo: &str) -> Result<(), ListaNegraError> {
    let limpio = motivo.trim();

    if limpio.is_empty() {
        return Err(ListaNegraError::Validation(
            "Debe especificar un motivo de bloqueo".to_string(),
        ));
    }

    if limpio.len() > MOTIVO_MAX_LEN {
        return Err(ListaNegraError::Validation(format!(
            "El motivo no puede exceder {MOTIVO_MAX_LEN} caracteres"
        )));
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

    if limpio.len() > ENTIDAD_NOMBRE_MAX_LEN {
        return Err(ListaNegraError::Validation(format!(
            "El ID de quien bloqueó no puede exceder {ENTIDAD_NOMBRE_MAX_LEN} caracteres"
        )));
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
    // motivo_bloqueo is now optional, only validate if present
    if let Some(ref motivo) = input.motivo_bloqueo {
        validar_motivo(motivo)?;
    }
    validar_bloqueado_por(&input.bloqueado_por)?;
    validar_nivel_severidad(&input.nivel_severidad)?;

    Ok(())
}

/// Valida campos de actualización parcial de registros existentes.
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
// COMPORTAMIENTOS DE DOMINIO ESTRUCTURAL
// --------------------------------------------------------------------------

/// Formatea un nombre a estilo "Title Case".
pub fn normalizar_nombre_titulo(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

/// Normaliza texto genérico (trim).
pub fn normalizar_texto(texto: &str) -> String {
    texto.trim().to_string()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

