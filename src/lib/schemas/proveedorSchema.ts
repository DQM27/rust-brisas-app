// ==========================================
// src/lib/schemas/proveedorSchema.ts
// ==========================================
// Zod validation schemas for Proveedor forms

import { z } from 'zod';

// ==========================================
// VALIDACIONES BÁSICAS
// ==========================================

const cedulaSchema = z.string()
    .trim()
    .min(7, 'Cédula debe tener al menos 7 caracteres')
    .max(15, 'Cédula no puede exceder 15 caracteres')
    .regex(/^[0-9-]+$/, 'Cédula solo puede contener números y guiones');

const nombreSchema = z.string()
    .trim()
    .min(1, 'Nombre es requerido')
    .max(50, 'Nombre no puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Nombre solo puede contener letras');

const apellidoSchema = z.string()
    .trim()
    .min(1, 'Apellido es requerido')
    .max(50, 'Apellido no puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Apellido solo puede contener letras');

const nombreOpcionalSchema = z.string()
    .trim()
    .max(50, 'No puede exceder 50 caracteres')
    .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]*$/, 'Solo puede contener letras')
    .optional()
    .or(z.literal(''));

const placaSchema = z.string()
    .trim()
    .max(10, 'Placa no puede exceder 10 caracteres')
    .regex(/^[A-Z0-9-]*$/i, 'Placa solo puede contener letras, números y guiones')
    .optional()
    .or(z.literal(''));

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

export const CreateProveedorSchema = z.object({
    cedula: cedulaSchema,
    nombre: nombreSchema,
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema,
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().min(1, 'Empresa es requerida'),

    // Vehículo (opcional)
    tieneVehiculo: z.boolean().optional().default(false),
    tipoVehiculo: z.string().optional(),
    placa: placaSchema,
    marca: z.string().trim().max(50).optional().or(z.literal('')),
    modelo: z.string().trim().max(50).optional().or(z.literal('')),
    color: z.string().trim().max(30).optional().or(z.literal('')),
}).refine(
    (data) => {
        // Si tiene vehículo, placa y tipo son requeridos
        if (data.tieneVehiculo) {
            return data.placa && data.placa.trim() !== '' && data.tipoVehiculo;
        }
        return true;
    },
    {
        message: 'Placa y tipo de vehículo son requeridos cuando se registra un vehículo',
        path: ['placa'],
    }
);

export const UpdateProveedorSchema = z.object({
    nombre: nombreSchema.optional(),
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema.optional(),
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().optional(),
    estado: z.enum(['ACTIVO', 'INACTIVO', 'SUSPENDIDO']).optional(),
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type CreateProveedorForm = z.infer<typeof CreateProveedorSchema>;
export type UpdateProveedorForm = z.infer<typeof UpdateProveedorSchema>;
