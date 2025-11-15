// domain/contratista/validations.rs
use crate::models::contratista::*;
use crate::domain::errors::ContratistaError;
use sqlx::SqlitePool;
use chrono::NaiveDate;

// ==========================================
// ERRORES DE VALIDACIÓN
// ==========================================
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("La cédula no puede estar vacía")]
    CedulaVacia,
    // ... otros errores
}

// ==========================================
// CONSTANTES
// ==========================================
const CEDULA_MIN_LENGTH: usize = 7;
const CEDULA_MAX_LENGTH: usize = 20;

// ==========================================
// VALIDACIONES DE FORMATO
// ==========================================
pub fn validar_cedula(cedula: &str) -> Result<String, ValidationError> {
    let limpia = cedula.trim();
    // Validaciones...
    Ok(limpia.to_string())
}

pub fn validar_nombre(nombre: &str) -> Result<String, ValidationError> { }
pub fn validar_apellido(apellido: &str) -> Result<String, ValidationError> { }

// ==========================================
// VALIDADOR PRINCIPAL
// ==========================================
pub struct ContratistaValidator;

impl ContratistaValidator {
    pub async fn validar_creacion(
        pool: &SqlitePool,
        input: &CreateContratistaInput,
    ) -> Result<ValidatedCreateInput, ContratistaError> {
        // 1. Validar formato
        let validated = Self::validar_formato(input)?;
        
        // 2. Validaciones de base de datos
        Self::verificar_lista_negra(pool, &validated.cedula).await?;
        Self::verificar_no_existe(pool, &validated.cedula).await?;
        Self::verificar_empresa_existe(pool, &validated.empresa_id).await?;
        
        Ok(validated)
    }
    
    fn validar_formato(
        input: &CreateContratistaInput
    ) -> Result<ValidatedCreateInput, ValidationError> {
        Ok(ValidatedCreateInput {
            cedula: validar_cedula(&input.cedula)?,
            nombre: validar_nombre(&input.nombre)?,
            apellido: validar_apellido(&input.apellido)?,
            empresa_id: input.empresa_id.clone(),
            fecha_vencimiento_praind: validar_fecha(&input.fecha_vencimiento_praind)?,
        })
    }
    
    async fn verificar_lista_negra(
        pool: &SqlitePool,
        cedula: &str,
    ) -> Result<(), ContratistaError> {
        // Lógica de verificación
    }
    
    async fn verificar_no_existe(
        pool: &SqlitePool,
        cedula: &str,
    ) -> Result<(), ContratistaError> {
        // Lógica de verificación
    }
    
    async fn verificar_empresa_existe(
        pool: &SqlitePool,
        empresa_id: &str,
    ) -> Result<(), ContratistaError> {
        // Lógica de verificación
    }
}

// ==========================================
// INPUT VALIDADO
// ==========================================
#[derive(Debug, Clone)]
pub struct ValidatedCreateInput {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub empresa_id: String,
    pub fecha_vencimiento_praind: NaiveDate,
}

// ==========================================
// TESTS
// ==========================================
#[cfg(test)]
mod tests { }