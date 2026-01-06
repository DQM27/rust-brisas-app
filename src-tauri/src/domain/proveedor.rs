/// Capa de Dominio: Reglas para Proveedores.
///
/// Este módulo define la lógica pura para la gestión de proveedores,
/// incluyendo validaciones de identidad, integridad de datos y coherencia vehicular.
use crate::domain::common::{
    normalizar_nombre_opcional_estandar, normalizar_nombre_propio, normalizar_opcional_estandar,
    validar_cedula_estandar, validar_nombre_estandar, validar_opcional_estandar, COLOR_MAX_LEN,
    MARCA_MODELO_MAX_LEN, SEGUNDO_NOMBRE_MAX_LEN,
};
use crate::domain::errors::ProveedorError;
use crate::domain::vehiculo as vehiculo_domain;
use crate::models::proveedor::{CreateProveedorInput, UpdateProveedorInput};

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida que la cédula cumpla el estándar institucional.
pub fn validar_cedula(cedula: &str) -> Result<(), ProveedorError> {
    validar_cedula_estandar(cedula).map_err(|e| ProveedorError::Validation(e.to_string()))
}

/// Valida que el nombre de la persona cumpla el estándar.
pub fn validar_nombre_persona(nombre: &str, campo: &str) -> Result<(), ProveedorError> {
    validar_nombre_estandar(nombre, campo).map_err(|e| ProveedorError::Validation(e.to_string()))
}

/// Valida que el ID de la empresa vinculada sea un identificador válido.
pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ProveedorError> {
    if empresa_id.trim().is_empty() {
        return Err(ProveedorError::Validation("El ID de la empresa es obligatorio".to_string()));
    }
    Ok(())
}

/// Valida campos opcionales con un límite de caracteres.
pub fn validar_opcional(
    valor: Option<&String>,
    max_len: usize,
    campo: &str,
) -> Result<(), ProveedorError> {
    validar_opcional_estandar(valor, max_len, campo)
        .map_err(|e| ProveedorError::Validation(e.to_string()))
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS COMPLETOS
// --------------------------------------------------------------------------

/// Realiza una validación integral de los datos para el registro de un nuevo proveedor.
pub fn validar_create_input(input: &CreateProveedorInput) -> Result<(), ProveedorError> {
    validar_cedula(&input.cedula)?;
    validar_nombre_persona(&input.nombre, "nombre")?;
    validar_nombre_persona(&input.apellido, "apellido")?;
    validar_empresa_id(&input.empresa_id)?;

    // Validaciones de nombres opcionales
    validar_opcional(input.segundo_nombre.as_ref(), SEGUNDO_NOMBRE_MAX_LEN, "segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), SEGUNDO_NOMBRE_MAX_LEN, "segundo apellido")?;

    // Validación vehicular si se declara
    if let Some(true) = input.tiene_vehiculo {
        if let Some(ref tipo) = input.tipo_vehiculo {
            vehiculo_domain::validar_tipo_vehiculo(tipo)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        if let Some(ref placa) = input.placa {
            vehiculo_domain::validar_placa(placa)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        if let Some(ref marca) = input.marca {
            vehiculo_domain::validar_marca(marca)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        validar_opcional(input.modelo.as_ref(), MARCA_MODELO_MAX_LEN, "modelo de vehículo")?;
        validar_opcional(input.color.as_ref(), COLOR_MAX_LEN, "color de vehículo")?;
    }

    Ok(())
}

/// Valida selectivamente los campos presentes en una solicitud de actualización.
pub fn validar_update_input(input: &UpdateProveedorInput) -> Result<(), ProveedorError> {
    if let Some(ref nombre) = input.nombre {
        validar_nombre_persona(nombre, "nombre")?;
    }
    if let Some(ref apellido) = input.apellido {
        validar_nombre_persona(apellido, "apellido")?;
    }
    if let Some(ref empresa_id) = input.empresa_id {
        validar_empresa_id(empresa_id)?;
    }

    // Nombres opcionales
    validar_opcional(input.segundo_nombre.as_ref(), SEGUNDO_NOMBRE_MAX_LEN, "segundo nombre")?;
    validar_opcional(input.segundo_apellido.as_ref(), SEGUNDO_NOMBRE_MAX_LEN, "segundo apellido")?;

    // Datos vehiculares reactivos
    if let Some(true) = input.tiene_vehiculo {
        if let Some(ref tipo) = input.tipo_vehiculo {
            vehiculo_domain::validar_tipo_vehiculo(tipo)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        if let Some(ref placa) = input.placa {
            vehiculo_domain::validar_placa(placa)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        if let Some(ref marca) = input.marca {
            vehiculo_domain::validar_marca(marca)
                .map_err(|e| ProveedorError::Validation(e.to_string()))?;
        }
        validar_opcional(input.modelo.as_ref(), MARCA_MODELO_MAX_LEN, "modelo de vehículo")?;
        validar_opcional(input.color.as_ref(), COLOR_MAX_LEN, "color de vehículo")?;
    }

    Ok(())
}

// --------------------------------------------------------------------------
// COMPORTAMIENTOS DE DOMINIO (NORMALIZACIÓN)
// --------------------------------------------------------------------------

pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

pub fn normalizar_nombre(nombre: &str) -> String {
    normalizar_nombre_propio(nombre)
}

pub fn normalizar_apellido(apellido: &str) -> String {
    normalizar_nombre_propio(apellido)
}

pub fn normalizar_segundo_nombre(segundo_nombre: Option<&String>) -> Option<String> {
    normalizar_nombre_opcional_estandar(segundo_nombre)
}

pub fn normalizar_segundo_apellido(segundo_apellido: Option<&String>) -> Option<String> {
    normalizar_nombre_opcional_estandar(segundo_apellido)
}

pub fn normalizar_texto_opcional(texto: Option<&String>) -> Option<String> {
    normalizar_opcional_estandar(texto)
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
        assert!(validar_nombre_persona("Juan", "nombre").is_ok());
    }

    #[test]
    fn test_normalizaciones() {
        assert_eq!(normalizar_nombre("  pedro  "), "Pedro");
        assert_eq!(
            normalizar_segundo_nombre(Some(&"  JOSÉ  ".to_string())),
            Some("José".to_string())
        );
        assert_eq!(normalizar_segundo_nombre(Some(&"  ".to_string())), None);
    }
}
