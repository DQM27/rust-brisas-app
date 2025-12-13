// ==========================================
// src/domain/ingreso/mod.rs
// ==========================================

pub mod tipos;
pub mod validaciones_comunes;
pub mod validaciones_entrada;
pub mod validaciones_permanencia;
pub mod validaciones_salida;

// Strategy pattern
pub mod strategy;

// Re-exports de tipos
pub use tipos::*;

// Re-exports de validaciones comunes
pub use validaciones_comunes::*;

// Re-exports de funciones de validación entrada
pub use validaciones_entrada::*;

// Re-exports de funciones de validación permanencia
pub use validaciones_permanencia::*;

// Re-exports de funciones de validación salida
pub use validaciones_salida::*;

// Re-exports de strategy
pub use strategy::*;