// ============================================
// src/lib/schemas/contratistaSchema.ts
// ============================================
// Validaciones Zod para contratistas

import { z } from 'zod';

// ============================================
// VALIDACIÓN DE CÉDULA
// ============================================

const cedulaRegex = /^\d{1,2}-?\d{4}-?\d{4}$/;

// ============================================
// SCHEMAS
// ============================================

export const CreateContratistaSchema = z.object({
    cedula: z.string()
        .min(1, 'Cédula es requerida')
        .regex(cedulaRegex, 'Formato de cédula inválido (ej: 1-1234-5678)'),
    nombre: z.string()
        .min(2, 'Nombre debe tener al menos 2 caracteres')
        .max(50, 'Nombre no puede exceder 50 caracteres'),
    apellido: z.string()
        .min(2, 'Apellido debe tener al menos 2 caracteres')
        .max(50, 'Apellido no puede exceder 50 caracteres'),
    empresaId: z.string()
        .min(1, 'Empresa es requerida'),
    fechaVencimientoPraind: z.string()
        .min(1, 'Fecha de vencimiento PRAIND es requerida')
        .regex(/^\d{4}-\d{2}-\d{2}$/, 'Formato de fecha inválido (YYYY-MM-DD)'),
    // Campos de vehículo opcionales
    tieneVehiculo: z.boolean().default(false),
    tipoVehiculo: z.string().optional(),
    placa: z.string().optional(),
    marca: z.string().optional(),
    modelo: z.string().optional(),
    color: z.string().optional(),
}).refine((data) => {
    // Si tiene vehículo, placa es requerida
    if (data.tieneVehiculo && !data.placa) {
        return false;
    }
    return true;
}, {
    message: 'Placa es requerida cuando tiene vehículo',
    path: ['placa'],
});

export const UpdateContratistaSchema = z.object({
    cedula: z.string()
        .regex(cedulaRegex, 'Formato de cédula inválido')
        .optional(),
    nombre: z.string()
        .min(2, 'Nombre debe tener al menos 2 caracteres')
        .max(50, 'Nombre no puede exceder 50 caracteres')
        .optional(),
    apellido: z.string()
        .min(2, 'Apellido debe tener al menos 2 caracteres')
        .max(50, 'Apellido no puede exceder 50 caracteres')
        .optional(),
    empresaId: z.string().optional(),
    fechaVencimientoPraind: z.string()
        .regex(/^\d{4}-\d{2}-\d{2}$/, 'Formato de fecha inválido')
        .optional(),
    isActive: z.boolean().optional(),
    // Campos de vehículo
    tieneVehiculo: z.boolean().optional(),
    tipoVehiculo: z.string().optional(),
    placa: z.string().optional(),
    marca: z.string().optional(),
    modelo: z.string().optional(),
    color: z.string().optional(),
});

export const CambiarEstadoSchema = z.object({
    estado: z.enum(['activo', 'inactivo', 'suspendido']),
});

// ============================================
// TIPOS INFERIDOS
// ============================================

export type CreateContratistaForm = z.infer<typeof CreateContratistaSchema>;
export type UpdateContratistaForm = z.infer<typeof UpdateContratistaSchema>;
export type CambiarEstadoForm = z.infer<typeof CambiarEstadoSchema>;
