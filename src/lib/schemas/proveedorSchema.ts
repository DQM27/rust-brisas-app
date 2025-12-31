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

    // Vehículo (opcional)
    tieneVehiculo: z.boolean().default(false),
    tipoVehiculo: z.string().default(''),
    placa: z.string().default(''),
    marca: z.string().default(''),
    modelo: z.string().default(''),
    color: z.string().default(''),
});

export const UpdateProveedorSchemaBase = z.object({
    nombre: z.string().default(''),
    segundoNombre: z.string().default(''),
    apellido: z.string().default(''),
    segundoApellido: z.string().default(''),
    empresaId: z.string().default(''),
    estado: z.enum(['ACTIVO', 'INACTIVO', 'SUSPENDIDO']).default('ACTIVO'),

    // Vehículo update
    tieneVehiculo: z.boolean().default(false),
    tipoVehiculo: z.string().default(''),
    placa: z.string().default(''),
    marca: z.string().default(''),
    modelo: z.string().default(''),
    color: z.string().default(''),
});

// 2. Schemas Refinados (Lógica condicional)
// Estos son los que se usan para la validación final

export const CreateProveedorSchema = CreateProveedorSchemaBase.superRefine((data, ctx) => {
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

export const UpdateProveedorSchema = UpdateProveedorSchemaBase.superRefine((data, ctx) => {
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

// Usamos los Schemas Base para los tipos de formulario estructural
export type CreateProveedorForm = z.infer<typeof CreateProveedorSchemaBase>;
export type UpdateProveedorForm = z.infer<typeof UpdateProveedorSchemaBase>;
