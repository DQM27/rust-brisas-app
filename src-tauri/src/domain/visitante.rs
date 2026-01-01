/// Capa de Dominio: Reglas de Negocio para Visitantes.
///
/// Este módulo centraliza las validaciones de identidad para personas particulares
/// que ingresan a las instalaciones (no son empleados ni contratistas fijos).
use crate::domain::errors::VisitanteError;
use crate::models::visitante::CreateVisitanteInput;

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula del visitante.
pub fn validar_cedula(cedula: &str) -> Result<(), VisitanteError> {
    let limpio = cedula.trim();

    if limpio.is_empty() {
        return Err(VisitanteError::Validation("La cédula no puede estar vacía".to_string()));
    }

    if limpio.len() > 20 {
        return Err(VisitanteError::Validation(
            "La cédula no puede exceder 20 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida el nombre del visitante.
pub fn validar_nombre(nombre: &str) -> Result<(), VisitanteError> {
    let limpio = nombre.trim();

    if limpio.is_empty() {
        return Err(VisitanteError::Validation("El nombre no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(VisitanteError::Validation(
            "El nombre no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
}

/// Valida el apellido del visitante.
pub fn validar_apellido(apellido: &str) -> Result<(), VisitanteError> {
    let limpio = apellido.trim();

    if limpio.is_empty() {
        return Err(VisitanteError::Validation("El apellido no puede estar vacío".to_string()));
    }

    if limpio.len() > 50 {
        return Err(VisitanteError::Validation(
            "El apellido no puede exceder 50 caracteres".to_string(),
        ));
    }

    Ok(())
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

    validar_opcional(input.segundo_nombre.as_ref(), 50, "Segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), 50, "Segundo apellido")?;
    validar_opcional(input.empresa.as_ref(), 100, "Empresa")?;

    Ok(())
}

// --------------------------------------------------------------------------
// UTILIDADES DE NORMALIZACIÓN
// --------------------------------------------------------------------------

/// Limpia espacios redundantes en nombres.
pub fn normalizar_nombre(nombre: &str) -> String {
    nombre.trim().to_uppercase()
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
        assert_eq!(normalizar_nombre("  pedro  "), "PEDRO");
        assert_eq!(normalizar_cedula("  v-8.765.432  "), "V-8.765.432");
    }
}
