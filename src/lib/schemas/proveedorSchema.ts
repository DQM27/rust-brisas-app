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
    empresaId: z.string().min(1, 'Seleccione una empresa válida'),

    // Vehículo (opcional)
    tieneVehiculo: z.boolean().default(false),
    tipoVehiculo: z.string().optional(),
    placa: placaSchema,
    marca: z.string().trim().max(50).optional().or(z.literal('')),
    modelo: z.string().trim().max(50).optional().or(z.literal('')),
    color: z.string().trim().max(30).optional().or(z.literal('')),
}).superRefine((data, ctx) => {
    if (data.tieneVehiculo) {
        if (!data.placa || data.placa.trim() === '') {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: 'La placa es requerida si tiene vehículo',
                path: ['placa'],
            });
        }
        if (!data.tipoVehiculo || data.tipoVehiculo.trim() === '') {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: 'El tipo de vehículo es requerido',
                path: ['tipoVehiculo'],
            });
        }
    }
});

export const UpdateProveedorSchema = z.object({
    nombre: nombreSchema.optional(),
    segundoNombre: nombreOpcionalSchema,
    apellido: apellidoSchema.optional(),
    segundoApellido: nombreOpcionalSchema,
    empresaId: z.string().optional(),
    estado: z.enum(['ACTIVO', 'INACTIVO', 'SUSPENDIDO']).optional(),

    // Vehículo update
    tieneVehiculo: z.boolean().optional(),
    tipoVehiculo: z.string().optional(),
    placa: placaSchema,
    marca: z.string().trim().max(50).optional().or(z.literal('')),
    modelo: z.string().trim().max(50).optional().or(z.literal('')),
    color: z.string().trim().max(30).optional().or(z.literal('')),
}).superRefine((data, ctx) => {
    // Si se activa tieneVehiculo explícitamente, validar
    if (data.tieneVehiculo === true) {
        if (!data.placa || data.placa.trim() === '') {
            ctx.addIssue({
                code: z.ZodIssueCode.custom,
                message: 'La placa es requerida',
                path: ['placa'],
            });
        }
    }
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type CreateProveedorForm = z.infer<typeof CreateProveedorSchema>;
export type UpdateProveedorForm = z.infer<typeof UpdateProveedorSchema>;
