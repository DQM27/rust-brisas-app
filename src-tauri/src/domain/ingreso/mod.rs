// ==========================================
// src/domain/ingreso/mod.rs
// ==========================================

pub mod validations;

// Re-exports para facilitar el uso
pub use validations::{
    IngresoValidator,
    ValidatedCreateContratistaInput,
    ValidatedCreateTemporalInput,
    ValidationError,
    validar_cedula,
    validar_nombre,
    validar_apellido,
    validar_empresa_nombre,
};