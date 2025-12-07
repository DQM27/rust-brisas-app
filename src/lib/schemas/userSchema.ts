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
    cedula: stringRequerido(20, 'Cédula'),
    email: emailSchema,
    // Password es opcional en la creación, permitimos string vacío o undefined
    password: z.union([passwordSchema, z.literal(''), z.undefined()]).optional(),
    nombre: stringRequerido(50, 'Nombre'),
    apellido: stringRequerido(50, 'Apellido'),
    segundo_nombre: stringOpcional(50, 'Segundo nombre'),
    segundo_apellido: stringOpcional(50, 'Segundo apellido'),
    role: z.enum(['admin', 'supervisor', 'guardia']),

    // Campos adicionales opcionales
    telefono: stringOpcional(20, 'Teléfono'),
    direccion: stringOpcional(200, 'Dirección'),
    fecha_inicio_labores: z.string().optional(),
    numero_gafete: stringOpcional(50, 'Número de gafete'),
    fecha_nacimiento: z.string().optional(),
    contacto_emergencia_nombre: stringOpcional(100, 'Nombre contacto emergencia'),
    contacto_emergencia_telefono: stringOpcional(20, 'Teléfono contacto emergencia'),
});

export const UpdateUserSchema = CreateUserSchema.partial().extend({
    // En update, password también es opcional pero si viene debe ser válida
    password: z.union([passwordSchema, z.literal(''), z.undefined()]).optional(),
});

export const ChangePasswordSchema = z.object({
    current_password: z.string().optional(), // Requerido salvo admin reset
    new_password: passwordSchema,
    confirm_password: z.string()
}).refine((data) => data.new_password === data.confirm_password, {
    message: "Las contraseñas no coinciden",
    path: ["confirm_password"],
});

// Tipos inferidos
export type CreateUserForm = z.infer<typeof CreateUserSchema>;
export type UpdateUserForm = z.infer<typeof UpdateUserSchema>;
export type ChangePasswordForm = z.infer<typeof ChangePasswordSchema>;
