use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar,
};
/// Capa de Dominio: Reglas para Proveedores.
///
/// Este módulo define la lógica pura para la gestión de proveedores,
/// incluyendo validaciones de identidad y de integridad de datos de entrada.
use crate::domain::errors::ProveedorError;
use crate::models::proveedor::{CreateProveedorInput, UpdateProveedorInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que la cédula cumpla el estándar.
pub fn validar_cedula(cedula: &str) -> Result<(), ProveedorError> {
    validar_cedula_estandar(cedula).map_err(|e| ProveedorError::Validation(e.to_string()))
}

/// Valida que el nombre de la persona cumpla el estándar.
pub fn validar_nombre_persona(nombre: &str) -> Result<(), ProveedorError> {
    validar_nombre_estandar(nombre, "nombre/apellido")
        .map_err(|e| ProveedorError::Validation(e.to_string()))
}

/// Valida que el ID de la empresa vinculada sea un identificador válido (no vacío).
pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ProveedorError> {
    if empresa_id.trim().is_empty() {
        return Err(ProveedorError::Validation("El ID de la empresa es obligatorio".to_string()));
    }
    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS COMPLETOS
// --------------------------------------------------------------------------

/// Realiza una validación integral de los datos para el registro de un nuevo proveedor.
pub fn validar_create_input(input: &CreateProveedorInput) -> Result<(), ProveedorError> {
    validar_cedula(&input.cedula)?;
    validar_nombre_persona(&input.nombre)?;
    validar_nombre_persona(&input.apellido)?;
    validar_empresa_id(&input.empresa_id)?;
    Ok(())
}

/// Valida selectivamente los campos presentes en una solicitud de actualización.
pub fn validar_update_input(input: &UpdateProveedorInput) -> Result<(), ProveedorError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre_persona(nombre)?;
    }

    if let Some(ref apellido) = input.apellido {
        validar_nombre_persona(apellido)?;
    }

    if let Some(ref empresa_id) = input.empresa_id {
        validar_empresa_id(empresa_id)?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// COMPORTAMIENTOS DE DOMINIO
// --------------------------------------------------------------------------

pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_string()
}

/// Normaliza un nombre a Title Case.
pub fn normalizar_nombre(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

pub fn normalizar_segundo_nombre(segundo_nombre: Option<&String>) -> Option<String> {
    segundo_nombre
        .map(|n| {
            let trimmed = n.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .flatten()
}

pub fn normalizar_apellido(apellido: &str) -> String {
    normalizar_nombre_propio(apellido)
}

pub fn normalizar_segundo_apellido(segundo_apellido: Option<&String>) -> Option<String> {
    segundo_apellido
        .map(|a| {
            let trimmed = a.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula_valida() {
        assert!(validar_cedula("1234567").is_ok());
    }

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre_persona("Juan").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre_persona("   ").is_err());
    }
}
