// ==========================================
// src/domain/contratista/mod.rs
// ==========================================

pub mod validations;

// Re-exports para facilitar el uso
pub use validations::{
    ContratistaValidator,
    ValidatedCreateInput,
    ValidationError,
    validar_cedula,
    validar_nombre,
    validar_apellido,
    validar_empresa_id,
    validar_fecha,
};