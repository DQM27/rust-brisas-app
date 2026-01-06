/**
 * Constantes de Validación del Dominio
 * 
 * Estas constantes DEBEN estar sincronizadas con:
 * Backend: src-tauri/src/domain/common.rs
 * 
 * FUENTE DE VERDAD: domain/common.rs (Rust)
 */

// ==========================================
// Identificación Personal
// ==========================================
export const CEDULA_MIN_LEN = 5;
export const CEDULA_MAX_LEN = 20;

// ==========================================
// Nombres y Apellidos
// ==========================================
export const NOMBRE_MAX_LEN = 100;
export const SEGUNDO_NOMBRE_MAX_LEN = 50;

// ==========================================
// Contacto
// ==========================================
export const TELEFONO_MAX_LEN = 20;
export const DIRECCION_MAX_LEN = 200;
export const EMAIL_MAX_LEN = 100;

// ==========================================
// Seguridad
// ==========================================
export const PASSWORD_MIN_LEN = 6;
export const GAFETE_MAX_LEN = 20;

// ==========================================
// Entidades (Empresa, Institución)
// ==========================================
export const ENTIDAD_NOMBRE_MAX_LEN = 100;

// ==========================================
// Vehículos
// ==========================================
export const PLACA_MIN_LEN = 2;
export const PLACA_MAX_LEN = 15;
export const MARCA_MODELO_MAX_LEN = 50;
export const COLOR_MAX_LEN = 30;

// ==========================================
// Lista Negra
// ==========================================
export const MOTIVO_MAX_LEN = 500;
export const OBSERVACIONES_MAX_LEN = 1000;
