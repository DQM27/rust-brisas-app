// src/lib/schemas/listaNegraSchema.ts
import { z } from 'zod';

// ==========================================
// VALIDACIONES BÁSICAS
// ==========================================

const NIVELES_SEVERIDAD = ['ALTO', 'MEDIO', 'BAJO'] as const;

const cedulaSchema = z.string()
    .trim()
    .min(7, 'Cédula debe tener al menos 7 caracteres')
    .max(20, 'Cédula no puede exceder 20 caracteres')
    .regex(/^[0-9-]+$/, 'Cédula solo puede contener números y guiones');

const nombreSchema = z.string()
    .trim()
    .min(1, 'Nombre es requerido')
    .max(50, 'Nombre no puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Nombre solo puede contener letras');

const nombreOpcionalSchema = z.string()
    .trim()
    .max(50, 'No puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]*$/, 'Solo puede contener letras')
    .optional()
    .or(z.literal(''));

const motivoSchema = z.string()
    .trim()
    .min(1, 'El motivo es requerido')
    .max(500, 'El motivo no puede exceder 500 caracteres');

const observacionesSchema = z.string()
    .trim()
    .max(1000, 'Las observaciones no pueden exceder 1000 caracteres')
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
        .max(100, 'Nombre de empresa no puede exceder 100 caracteres')
        .optional()
        .or(z.literal('')),
    nivelSeveridad: z.enum(['ALTO', 'MEDIO', 'BAJO']),
    motivoBloqueo: motivoSchema,
    observaciones: observacionesSchema,
});

export const UpdateListaNegraSchema = z.object({
    nivelSeveridad: z.enum(['ALTO', 'MEDIO', 'BAJO']).optional(),
    motivoBloqueo: z.string()
        .trim()
        .max(500, 'El motivo no puede exceder 500 caracteres')
        .optional(),
    observaciones: observacionesSchema,
});

export const ReactivateListaNegraSchema = z.object({
    nivelSeveridad: z.enum(['ALTO', 'MEDIO', 'BAJO']),
    motivoBloqueo: motivoSchema,
});



// Tipos inferidos
export type AddToListaNegraForm = z.infer<typeof AddToListaNegraSchema>;
export type UpdateListaNegraForm = z.infer<typeof UpdateListaNegraSchema>;
export type ReactivateListaNegraForm = z.infer<typeof ReactivateListaNegraSchema>;
