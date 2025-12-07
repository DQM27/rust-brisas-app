import { z } from 'zod';

// ==========================================
// VALIDACIONES BÁSICAS
// ==========================================

const stringRequerido = (max: number, nombre: string) =>
    z.string()
        .trim()
        .min(1, `${nombre} es requerido`)
        .max(max, `${nombre} no puede exceder ${max} caracteres`);

const stringOpcional = (max: number, nombre: string) =>
    z.string()
        .trim()
        .max(max, `${nombre} no puede exceder ${max} caracteres`)
        .optional()
        .or(z.literal('')); // Permitir string vacío como opcional

const emailSchema = z.string()
    .trim()
    .email('Email inválido')
    .max(100, 'Email no puede exceder 100 caracteres');

const passwordSchema = z.string()
    .min(6, 'La contraseña debe tener al menos 6 caracteres')
    .max(100, 'La contraseña no puede exceder 100 caracteres');

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

export const CreateUserSchema = z.object({
    cedula: z.string()
        .trim()
        .min(7, 'Cédula debe tener al menos 7 caracteres')
        .max(15, 'Cédula no puede exceder 15 caracteres')
        .regex(/^[0-9-]+$/, 'Cédula solo puede contener números y guiones'),
    email: emailSchema,
    // Password es opcional en la creación, permitimos string vacío o undefined
    password: z.union([passwordSchema, z.literal(''), z.undefined()]).optional(),

    nombre: z.string()
        .trim()
        .min(1, 'Nombre es requerido')
        .max(50, 'Nombre no puede exceder 50 caracteres')
        .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Nombre solo puede contener letras'),

    apellido: z.string()
        .trim()
        .min(1, 'Apellido es requerido')
        .max(50, 'Apellido no puede exceder 50 caracteres')
        .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]+$/, 'Apellido solo puede contener letras'),

    segundoNombre: z.string()
        .trim()
        .max(50, 'Segundo nombre no puede exceder 50 caracteres')
        .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]*$/, 'Segundo nombre solo puede contener letras')
        .optional()
        .or(z.literal('')),

    segundoApellido: z.string()
        .trim()
        .max(50, 'Segundo apellido no puede exceder 50 caracteres')
        .regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑ\s]*$/, 'Segundo apellido solo puede contener letras')
        .optional()
        .or(z.literal('')),
    role: z.enum(['admin', 'supervisor', 'guardia']),

    // Campos adicionales opcionales
    telefono: stringOpcional(20, 'Teléfono'),
    direccion: stringOpcional(200, 'Dirección'),
    fechaInicioLabores: z.string().optional(),
    numeroGafete: z.string()
        .max(50, 'Número de gafete no puede exceder 50 caracteres')
        .regex(/^K-\d+$/, 'El número de gafete debe tener el formato K-1234')
        .optional()
        .or(z.literal('')),
    fechaNacimiento: z.string().optional(),
    contactoEmergenciaNombre: stringOpcional(100, 'Nombre contacto emergencia'),
    contactoEmergenciaTelefono: stringOpcional(20, 'Teléfono contacto emergencia'),
});

export const UpdateUserSchema = CreateUserSchema.partial().extend({
    // En update, password también es opcional pero si viene debe ser válida
    password: z.union([passwordSchema, z.literal(''), z.undefined()]).optional(),
});

export const ChangePasswordSchema = z.object({
    currentPassword: z.string().optional(), // Requerido salvo admin reset
    newPassword: passwordSchema,
    confirmPassword: z.string()
}).refine((data) => data.newPassword === data.confirmPassword, {
    message: "Las contraseñas no coinciden",
    path: ["confirmPassword"],
});

export const LoginSchema = z.object({
    email: emailSchema,
    password: z.string().min(1, 'La contraseña es requerida') // Para login solo validamos que exista
});

// Tipos inferidos
export type CreateUserForm = z.infer<typeof CreateUserSchema>;
export type UpdateUserForm = z.infer<typeof UpdateUserSchema>;
export type ChangePasswordForm = z.infer<typeof ChangePasswordSchema>;
export type LoginForm = z.infer<typeof LoginSchema>;
