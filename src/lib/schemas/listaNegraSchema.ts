// src/lib/schemas/listaNegraSchema.ts
import { z } from 'zod';
import {
    CEDULA_MIN_LEN,
    CEDULA_MAX_LEN,
    NOMBRE_MAX_LEN,
    SEGUNDO_NOMBRE_MAX_LEN,
    ENTIDAD_NOMBRE_MAX_LEN,
    MOTIVO_MAX_LEN,
    OBSERVACIONES_MAX_LEN
} from './domainConstants';

// ==========================================
// VALIDACIONES BÁSICAS (Alineadas con backend domain/common.rs)
// ==========================================

/**
 * Niveles de severidad - Sincronizado con backend models/lista_negra.rs
 */
export const NIVELES_SEVERIDAD = ['ALTO', 'MEDIO', 'BAJO'] as const;

/**
 * Cédula: Solo números y guiones
 * Backend: validar_cedula_estandar() - CEDULA_MIN_LEN-CEDULA_MAX_LEN chars
 */
const cedulaSchema = z.string()
    .trim()
    .min(CEDULA_MIN_LEN, `Cédula debe tener al menos ${CEDULA_MIN_LEN} caracteres`)
    .max(CEDULA_MAX_LEN, `Cédula no puede exceder ${CEDULA_MAX_LEN} caracteres`)
    .regex(/^[0-9-]+$/, 'Cédula solo puede contener números y guiones')
    .refine(val => /\d/.test(val), 'La cédula debe contener al menos un número');

/**
 * Nombre/Apellido: Solo letras con acentos
 * Backend: validar_nombre_estandar() - max NOMBRE_MAX_LEN chars
 */
const nombreSchema = z.string()
    .trim()
    .min(1, 'Nombre es requerido')
    .max(NOMBRE_MAX_LEN, `Nombre no puede exceder ${NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, 'Nombre solo puede contener letras');

const nombreOpcionalSchema = z.string()
    .trim()
    .max(SEGUNDO_NOMBRE_MAX_LEN, `No puede exceder ${SEGUNDO_NOMBRE_MAX_LEN} caracteres`)
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]*$/, 'Solo puede contener letras')
    .optional()
    .or(z.literal(''));

/**
 * Motivo de bloqueo
 * Backend: MOTIVO_MAX_LEN = 500
 */
const motivoSchema = z.string()
    .trim()
    .min(1, 'El motivo es requerido')
    .max(MOTIVO_MAX_LEN, `El motivo no puede exceder ${MOTIVO_MAX_LEN} caracteres`);

/**
 * Observaciones
 * Backend: OBSERVACIONES_MAX_LEN = 1000
 */
const observacionesSchema = z.string()
    .trim()
    .max(OBSERVACIONES_MAX_LEN, `Las observaciones no pueden exceder ${OBSERVACIONES_MAX_LEN} caracteres`)
    .optional()
    .or(z.literal(''));

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

export const AddToListaNegraSchema = z.object({
    cedula: cedulaSchema,
    nombre: nombreSchema,
    segundoNombre: nombreOpcionalSchema,
    apellido: nombreSchema,
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().optional().or(z.literal('')),
    empresaNombre: z.string()
        .trim()
        .max(ENTIDAD_NOMBRE_MAX_LEN, `Nombre de empresa no puede exceder ${ENTIDAD_NOMBRE_MAX_LEN} caracteres`)
        .optional()
        .or(z.literal('')),
    nivelSeveridad: z.enum(NIVELES_SEVERIDAD),
    motivoBloqueo: motivoSchema,
    observaciones: observacionesSchema,
});

export const UpdateListaNegraSchema = z.object({
    nivelSeveridad: z.enum(NIVELES_SEVERIDAD).optional(),
    motivoBloqueo: z.string()
        .trim()
        .max(MOTIVO_MAX_LEN, `El motivo no puede exceder ${MOTIVO_MAX_LEN} caracteres`)
        .optional(),
    observaciones: observacionesSchema,
});

export const ReactivateListaNegraSchema = z.object({
    nivelSeveridad: z.enum(NIVELES_SEVERIDAD),
    motivoBloqueo: motivoSchema,
});

// Tipos inferidos
export type AddToListaNegraForm = z.infer<typeof AddToListaNegraSchema>;
export type UpdateListaNegraForm = z.infer<typeof UpdateListaNegraSchema>;
export type ReactivateListaNegraForm = z.infer<typeof ReactivateListaNegraSchema>;
export type NivelSeveridad = typeof NIVELES_SEVERIDAD[number];
