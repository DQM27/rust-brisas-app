/// Capa de Dominio: Reglas de Negocio para Contratistas.
///
/// Este módulo define las validaciones y lógicas puras aplicables a los
/// contratistas externos. Estas reglas aseguran la integridad de los datos
/// de filiación y laborales antes de su almacenamiento en la base de datos.
use crate::domain::common::{
    normalizar_nombre_propio, validar_cedula_estandar, validar_nombre_estandar,
};
use crate::domain::errors::ContratistaError;
use crate::models::contratista::{
    CreateContratistaInput, EstadoContratista, UpdateContratistaInput,
};
use chrono::NaiveDate;

// --------------------------------------------------------------------------
// VALIDACIONES DE CAMPOS INDIVIDUALES
// --------------------------------------------------------------------------

/// Valida el formato y longitud de la cédula de identidad.
pub fn validar_cedula(cedula: &str) -> Result<(), ContratistaError> {
    validar_cedula_estandar(cedula).map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del nombre.
pub fn validar_nombre(nombre: &str) -> Result<(), ContratistaError> {
    validar_nombre_estandar(nombre, "nombre")
        .map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida los requisitos mínimos del apellido.
pub fn validar_apellido(apellido: &str) -> Result<(), ContratistaError> {
    validar_nombre_estandar(apellido, "apellido")
        .map_err(|e| ContratistaError::Validation(e.to_string()))
}

/// Valida que el ID de la empresa vinculada sea válido.
pub fn validar_empresa_id(empresa_id: &str) -> Result<(), ContratistaError> {
    let limpia = empresa_id.trim();

    if limpia.is_empty() {
        return Err(ContratistaError::Validation(
            "Debe seleccionar una empresa válida".to_string(),
        ));
    }

    Ok(())
}

/// Parsea y valida una fecha en formato estándar (YYYY-MM-DD).
pub fn validar_fecha(fecha_str: &str) -> Result<NaiveDate, ContratistaError> {
    NaiveDate::parse_from_str(fecha_str, "%Y-%m-%d").map_err(|_| {
        ContratistaError::Validation("Formato de fecha inválido. Use YYYY-MM-DD".to_string())
    })
}

// --------------------------------------------------------------------------
// VALIDACIONES DE INPUTS (DTOs)
// --------------------------------------------------------------------------

/// Valida el conjunto completo de datos para la creación de un contratista.
pub fn validar_create_input(input: &CreateContratistaInput) -> Result<(), ContratistaError> {
    validar_cedula(&input.cedula)?;
    validar_nombre(&input.nombre)?;
    validar_apellido(&input.apellido)?;
    validar_empresa_id(&input.empresa_id)?;
    validar_fecha(&input.fecha_vencimiento_praind)?;
    Ok(())
}

/// Valida los cambios parciales solicitados en una actualización.
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

/// Limpia y normaliza el texto para su persistencia.
pub fn normalizar_texto(texto: &str) -> String {
    normalizar_nombre_propio(texto)
}

/// Normaliza una cédula eliminando espacios y convirtiéndola a mayúsculas.
pub fn normalizar_cedula(cedula: &str) -> String {
    cedula.trim().to_uppercase()
}

/// Valida que el estado del contratista sea uno de los valores permitidos.
pub fn validar_estado(estado: &str) -> Result<(), ContratistaError> {
    estado
        .parse::<EstadoContratista>()
        .map_err(|_| ContratistaError::Validation(format!("Estado inválido: {}", estado)))?;
    Ok(())
}

// --------------------------------------------------------------------------
// PRUEBAS UNITARIAS
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::common::MAX_LEN_NOMBRE;

    #[test]
    fn test_validar_cedula_valida() {
        assert!(validar_cedula("12345678").is_ok());
        assert!(validar_cedula("12-345-678").is_ok());
    }

    #[test]
    fn test_validar_cedula_vacia() {
        assert!(validar_cedula("").is_err());
        assert!(validar_cedula("   ").is_err());
    }

    #[test]
    fn test_validar_cedula_con_letras() {
        // Ahora las letras están prohibidas en cédulas
        assert!(validar_cedula("V-12345678").is_err());
        assert!(validar_cedula("E-1234567").is_err());
    }

    #[test]
    fn test_validar_cedula_muy_corta() {
        assert!(validar_cedula("123").is_err());
    }

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
    fn test_validar_nombre_chars_prohibidos() {
        assert!(validar_nombre("Juan<script>").is_err());
        assert!(validar_nombre("Pedro{json}").is_err());
        assert!(validar_nombre("Carlos|pipe").is_err());
    }

    #[test]
    fn test_validar_apellido_valido() {
        assert!(validar_apellido("Pérez").is_ok());
        assert!(validar_apellido("De La Cruz").is_ok());
    }

    #[test]
    fn test_validar_apellido_vacio() {
        assert!(validar_apellido("").is_err());
    }

    #[test]
    fn test_validar_apellido_chars_prohibidos() {
        assert!(validar_apellido("García<>").is_err());
        assert!(validar_apellido("López{malware}").is_err());
    }

    #[test]
    fn test_validar_empresa_id_valida() {
        assert!(validar_empresa_id("empresa:abc123").is_ok());
        assert!(validar_empresa_id("empresa:1").is_ok());
    }

    #[test]
    fn test_validar_empresa_id_vacia() {
        assert!(validar_empresa_id("").is_err());
        assert!(validar_empresa_id("   ").is_err());
    }

    #[test]
    fn test_validar_fecha_valida() {
        assert!(validar_fecha("2025-12-31").is_ok());
        assert!(validar_fecha("2024-01-01").is_ok());
    }

    #[test]
    fn test_validar_fecha_formato_invalido() {
        assert!(validar_fecha("31-12-2025").is_err());
        assert!(validar_fecha("2025/12/31").is_err());
        assert!(validar_fecha("").is_err());
    }

    #[test]
    fn test_normalizar_cedula() {
        assert_eq!(normalizar_cedula("  12345678  "), "12345678");
        assert_eq!(normalizar_cedula("1234567"), "1234567");
    }

    #[test]
    fn test_validar_estado_activo() {
        assert!(validar_estado("activo").is_ok());
    }

    #[test]
    fn test_validar_estado_suspendido() {
        assert!(validar_estado("suspendido").is_ok());
    }

    #[test]
    fn test_validar_estado_invalido() {
        assert!(validar_estado("invalido").is_err());
        assert!(validar_estado("").is_err());
    }

    #[test]
    fn test_validar_create_input_completo() {
        let input = CreateContratistaInput {
            cedula: "12345678".to_string(),
            nombre: "Juan".to_string(),
            segundo_nombre: None,
            apellido: "Pérez".to_string(),
            segundo_apellido: None,
            empresa_id: "empresa:1".to_string(),
            fecha_vencimiento_praind: "2025-12-31".to_string(),
            tiene_vehiculo: None,
            tipo_vehiculo: None,
            placa: None,
            marca: None,
            modelo: None,
            color: None,
        };
        assert!(validar_create_input(&input).is_ok());
    }

    #[test]
    fn test_validar_create_input_cedula_invalida() {
        let input = CreateContratistaInput {
            cedula: "".to_string(),
            nombre: "Juan".to_string(),
            segundo_nombre: None,
            apellido: "Pérez".to_string(),
            segundo_apellido: None,
            empresa_id: "empresa:1".to_string(),
            fecha_vencimiento_praind: "2025-12-31".to_string(),
            tiene_vehiculo: None,
            tipo_vehiculo: None,
            placa: None,
            marca: None,
            modelo: None,
            color: None,
        };
        assert!(validar_create_input(&input).is_err());
    }

    #[test]
    fn test_validar_update_input_parcial() {
        let input = UpdateContratistaInput {
            nombre: Some("Nuevo Nombre".to_string()),
            segundo_nombre: None,
            apellido: None,
            segundo_apellido: None,
            empresa_id: None,
            fecha_vencimiento_praind: None,
            tiene_vehiculo: None,
            tipo_vehiculo: None,
            placa: None,
            marca: None,
            modelo: None,
            color: None,
        };
        assert!(validar_update_input(&input).is_ok());
    }

    #[test]
    fn test_validar_update_input_fecha_invalida() {
        let input = UpdateContratistaInput {
            nombre: None,
            segundo_nombre: None,
            apellido: None,
            segundo_apellido: None,
            empresa_id: None,
            fecha_vencimiento_praind: Some("fecha-invalida".to_string()),
            tiene_vehiculo: None,
            tipo_vehiculo: None,
            placa: None,
            marca: None,
            modelo: None,
            color: None,
        };
        assert!(validar_update_input(&input).is_err());
    }
}
