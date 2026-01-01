/// Capa de Dominio: Reglas de Negocio para Visitantes.
///
/// Este módulo centraliza las validaciones de identidad para personas particulares
/// que ingresan a las instalaciones (no son empleados ni contratistas fijos).
use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar,
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

/// Valida un nombre opcional usando el estándar (si existe).
pub fn validar_nombre_opcional(
    valor: Option<&String>,
    nombre_campo: &str,
) -> Result<(), VisitanteError> {
    if let Some(v) = valor {
        // Ignorar si está vacío después de trim (se normalizará a None luego)
        if !v.trim().is_empty() {
            validar_nombre_estandar(v, nombre_campo)
                .map_err(|e| VisitanteError::Validation(e.to_string()))?;
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

    validar_nombre_opcional(input.segundo_nombre.as_ref(), "Segundo nombre")?;
    validar_nombre_opcional(input.segundo_apellido.as_ref(), "Segundo apellido")?;

    // Strict mode: Validar que empresa_id esté presente y tenga formato válido si es necesario
    if input.empresa_id.trim().is_empty() {
        return Err(VisitanteError::Validation("Debe seleccionar una empresa válida".to_string()));
    }

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
    use crate::domain::common::MAX_LEN_NOMBRE;

    // -----------------------------------------------------------------------
    // Tests de validación de cédula
    // -----------------------------------------------------------------------

    #[test]
    fn test_validar_cedula_valida() {
        assert!(validar_cedula("12345678").is_ok());
        assert!(validar_cedula("1-234-5678").is_ok());
    }

    #[test]
    fn test_validar_cedula_vacia() {
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("   ").is_err());
    }

    #[test]
    fn test_validar_cedula_con_letras() {
        assert!(validar_cedula("ABC123").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de nombre
    // -----------------------------------------------------------------------

    #[test]
    fn test_validar_nombre_valido() {
        assert!(validar_nombre("Juan").is_ok());
        assert!(validar_nombre("María José").is_ok());
    }

    #[test]
    fn test_validar_nombre_vacio() {
        assert!(validar_nombre("").is_err());
        assert!(validar_nombre("   ").is_err());
    }

    #[test]
    fn test_validar_nombre_muy_largo() {
        let nombre_largo = "A".repeat(MAX_LEN_NOMBRE + 1);
        assert!(validar_nombre(&nombre_largo).is_err());
    }

    #[test]
    fn test_validar_nombre_con_numeros() {
        assert!(validar_nombre("Juan123").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de apellido
    // -----------------------------------------------------------------------

    #[test]
    fn test_validar_apellido_valido() {
        assert!(validar_apellido("Pérez").is_ok());
        assert!(validar_apellido("García López").is_ok());
    }

    #[test]
    fn test_validar_apellido_vacio() {
        assert!(validar_apellido("").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de validación de campo opcional
    // -----------------------------------------------------------------------

    #[test]
    fn test_validar_opcional_none() {
        assert!(validar_opcional(None, 50, "Campo").is_ok());
    }

    #[test]
    fn test_validar_opcional_valido() {
        let valor = "Empresa ABC".to_string();
        assert!(validar_opcional(Some(&valor), 50, "Empresa").is_ok());
    }

    #[test]
    fn test_validar_opcional_excede_limite() {
        let valor = "A".repeat(100);
        assert!(validar_opcional(Some(&valor), 50, "Empresa").is_err());
    }

    // -----------------------------------------------------------------------
    // Tests de normalización
    // -----------------------------------------------------------------------

    #[test]
    fn test_normalizar_nombre() {
        assert_eq!(normalizar_nombre("  pedro  "), "Pedro");
        assert_eq!(normalizar_nombre("MARÍA JOSÉ"), "María José");
        assert_eq!(normalizar_nombre("juan carlos"), "Juan Carlos");
    }

    #[test]
    fn test_normalizar_cedula() {
        assert_eq!(normalizar_cedula("  8765432  "), "8765432");
        assert_eq!(normalizar_cedula("abc-123"), "ABC-123");
    }
}
