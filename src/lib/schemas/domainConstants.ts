/**
 * Constantes de Validación del Dominio
 * 
 * Estas constantes DEBEN estar sincronizadas con:
 * Backend: src-tauri/src/domain/common.rs
 * 
 * FUENTE DE VERDAD: domain/common.rs (Rust)
 */

// ==========================================
// CÉDULA
// ==========================================
export const CEDULA_MIN_LEN = 5;
export const CEDULA_MAX_LEN = 20;

// ==========================================
// NOMBRES Y APELLIDOS
// ==========================================
export const NOMBRE_MAX_LEN = 100;

// ==========================================
// ENTIDADES (Empresa, Institución)
// ==========================================
export const ENTIDAD_NOMBRE_MAX_LEN = 100;

// ==========================================
// EMAIL
// ==========================================
export const EMAIL_MAX_LEN = 100;

// ==========================================
// PLACA DE VEHÍCULO
// ==========================================
export const PLACA_MIN_LEN = 2;
export const PLACA_MAX_LEN = 15;
