import { z } from 'zod';
import {
    CEDULA_MIN_LEN,
    CEDULA_MAX_LEN,
    NOMBRE_MAX_LEN,
    PLACA_MIN_LEN,
    PLACA_MAX_LEN
} from './domainConstants';

// ==========================================
// VALIDACIONES BÁSICAS (Alineadas con backend domain/common.rs)
// ==========================================

/**
 * Cédula: Solo números y guiones, sin letras.
 * Backend: validar_cedula_estandar() - CEDULA_MIN_LEN-CEDULA_MAX_LEN chars
 */
const cedulaSchema = z.string()
    .trim()
    .min(1, "Cédula requerida")
    .pipe(
        z.string()
            .min(CEDULA_MIN_LEN, `Mínimo ${CEDULA_MIN_LEN} caracteres`)
            .max(CEDULA_MAX_LEN, `Máximo ${CEDULA_MAX_LEN} caracteres`)
            .regex(/^[0-9-]+$/, "Solo números y guiones")
            .refine(val => /\d/.test(val), "Debe contener números")
    );

/**
 * Nombre/Apellido: Solo letras (incluye acentos), espacios permitidos.
 * Backend: validar_nombre_estandar() - max NOMBRE_MAX_LEN chars
 */
const nombreSchema = z.string()
    .trim()
    .min(1, "Nombre requerido")
    .pipe(
        z.string()
            .max(NOMBRE_MAX_LEN, `Máximo ${NOMBRE_MAX_LEN} caracteres`)
            .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, "Solo letras")
    );

const apellidoSchema = z.string()
    .trim()
    .min(1, "Apellido requerido")
    .pipe(
        z.string()
            .max(NOMBRE_MAX_LEN, `Máximo ${NOMBRE_MAX_LEN} caracteres`)
            .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, "Solo letras")
    );

const nombreOpcionalSchema = z.string()
    .trim()
    .max(NOMBRE_MAX_LEN, `No puede exceder ${NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]*$/, 'Solo puede contener letras')
    .optional()
    .or(z.literal(''));

/**
 * Placa: Alfanumérico y guiones.
 * Backend: validar_placa_estandar() - PLACA_MIN_LEN-PLACA_MAX_LEN chars
 */
const placaSchema = z.string()
    .trim()
    .max(PLACA_MAX_LEN, `Placa no puede exceder ${PLACA_MAX_LEN} caracteres`)
    .regex(/^[A-Z0-9-]*$/i, 'Placa solo puede contener letras, números y guiones')
    .optional()
    .or(z.literal(''));

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

// 1. Base Schemas (Estructurales, sin refinamiento)
// Exportamos estos para que Superforms pueda inferir los valores por defecto (shape)

// 1. Base Schemas (Estructurales, sin refinamiento)
// Exportamos estos para que Superforms pueda inferir los valores por defecto (shape)
// Usamos .default('') para asegurar que el formulario inicie con strings vacíos y no undefined/unions complejos

export const CreateProveedorSchemaBase = z.object({
    cedula: z.string().default(''), // Simplified for defaults
    nombre: z.string().default(''),
    segundoNombre: z.string().default(''),
    apellido: z.string().default(''),
    segundoApellido: z.string().default(''),
    empresaId: z.string().default(''),
});

export const UpdateProveedorSchemaBase = z.object({
    nombre: z.string().default(''),
    segundoNombre: z.string().default(''),
    apellido: z.string().default(''),
    segundoApellido: z.string().default(''),
    empresaId: z.string().default(''),
    estado: z.enum(['ACTIVO', 'INACTIVO', 'SUSPENDIDO']).default('ACTIVO'),
});

// 2. Schemas Refinados (Lógica condicional)
// Estos son los que se usan para la validación final

export const CreateProveedorSchema = z.object({
    cedula: cedulaSchema,
    nombre: nombreSchema,
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema,
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().min(1, "Empresa requerida"),
});

export const UpdateProveedorSchema = z.object({
    nombre: nombreSchema,
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema,
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().min(1, "Empresa requerida"),
    estado: z.enum(['ACTIVO', 'INACTIVO', 'SUSPENDIDO']).default('ACTIVO'),
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

// Usamos los Schemas Base para los tipos de formulario estructural
export type CreateProveedorForm = z.infer<typeof CreateProveedorSchemaBase>;
export type UpdateProveedorForm = z.infer<typeof UpdateProveedorSchemaBase>;
