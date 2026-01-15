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
export const CEDULA_MIN_LEN = 6;
export const CEDULA_MAX_LEN = 16;

// ==========================================
// Nombres y Apellidos
// ==========================================
/** Mínimo para nombre/apellido requerido */
export const NOMBRE_MIN_LEN = 1;
export const NOMBRE_MAX_LEN = 100;
/** Máximo para segundo nombre/apellido (opcional, min=0) */
export const SEGUNDO_NOMBRE_MAX_LEN = 50;

// ==========================================
// Contacto
// ==========================================
/** Mínimo de teléfono si se proporciona */
export const TELEFONO_MIN_LEN = 7;
export const TELEFONO_MAX_LEN = 20;
/** Mínimo de dirección si se proporciona */
export const DIRECCION_MIN_LEN = 5;
export const DIRECCION_MAX_LEN = 200;
export const EMAIL_MIN_LEN = 5;
export const EMAIL_MAX_LEN = 100;

// ==========================================
// Seguridad
// ==========================================
export const PASSWORD_MIN_LEN = 6;
export const PASSWORD_MAX_LEN = 100;
export const GAFETE_MAX_LEN = 20;

// ==========================================
// Entidades (Empresa, Institución)
// ==========================================
export const ENTIDAD_NOMBRE_MIN_LEN = 1;
export const ENTIDAD_NOMBRE_MAX_LEN = 100;

// ==========================================
// Vehículos
// ==========================================
export const PLACA_MIN_LEN = 2;
export const PLACA_MAX_LEN = 15;
/** Mínimo de marca/modelo si se proporciona */
export const MARCA_MODELO_MIN_LEN = 2;
export const MARCA_MODELO_MAX_LEN = 50;
export const COLOR_MAX_LEN = 30;

// ==========================================
// Lista Negra
// ==========================================
/** Mínimo de motivo (requerido) */
export const MOTIVO_MIN_LEN = 1;
export const MOTIVO_MAX_LEN = 500;
/** Observaciones (opcional, min=0) */
export const OBSERVACIONES_MAX_LEN = 1000;

// ==========================================
// Roles
// ==========================================
export const ROLE_NAME_MIN_LEN = 1;
export const ROLE_NAME_MAX_LEN = 50;
export const ROLE_DESC_MAX_LEN = 200;
