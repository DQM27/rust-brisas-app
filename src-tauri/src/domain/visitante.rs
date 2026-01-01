/// Capa de Dominio: Reglas de Negocio para Visitantes.
///
/// Este módulo centraliza las validaciones de identidad para personas particulares
/// que ingresan a las instalaciones (no son empleados ni contratistas fijos).
use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar, MAX_LEN_EMPRESA,
    MAX_LEN_NOMBRE,
};
use crate::domain::errors::VisitanteError;
use crate::models::visitante::CreateVisitanteInput;

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula del visitante.
pub fn validar_cedula(cedula: &str) -> Result<(), VisitanteError> {
    validar_cedula_estandar(cedula).map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida el nombre del visitante.
pub fn validar_nombre(nombre: &str) -> Result<(), VisitanteError> {
    validar_nombre_estandar(nombre, "nombre").map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida el apellido del visitante.
pub fn validar_apellido(apellido: &str) -> Result<(), VisitanteError> {
    validar_nombre_estandar(apellido, "apellido")
        .map_err(|e| VisitanteError::Validation(e.to_string()))
}

/// Valida campos opcionales con un límite de caracteres.
pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    nombre_campo: &str,
) -> Result<(), VisitanteError> {
    if let Some(v) = valor {
        if v.trim().len() > max_len {
            return Err(VisitanteError::Validation(format!(
                "{} no puede exceder {} caracteres",
                nombre_campo, max_len
            )));
        }
    }
    Ok(())
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para un nuevo visitante.
pub fn validar_create_input(input: &CreateVisitanteInput) -> Result<(), VisitanteError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;

    validar_opcional(input.segundo_nombre.as_ref(), MAX_LEN_NOMBRE, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), MAX_LEN_NOMBRE, "Segundo apellido")?;
    validar_opcional(input.empresa.as_ref(), MAX_LEN_EMPRESA, "Empresa")?;

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Limpia espacios redundantes en nombres.
pub fn normalizar_nombre(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

/// Normaliza la cédula a mayúsculas para comparaciones consistentes.
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cedula_vacia() {
        assert!(validar_cedula("   ").is_err());
    }

    #[test]
    fn test_normalizar_documentacion() {
        assert_eq!(normalizar_nombre("  pedro  "), "Pedro");
        assert_eq!(normalizar_cedula("  8765432  "), "8765432");
    }
}
