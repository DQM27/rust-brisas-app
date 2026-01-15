import { z } from 'zod';
import { Operacion } from '$lib/types/user'; // Import Enum
import { CEDULA_MIN_LEN, CEDULA_MAX_LEN, NOMBRE_MAX_LEN, EMAIL_MAX_LEN } from './domainConstants';

// ==========================================
// VALIDACIONES BÁSICAS (Alineadas con backend domain/user.rs y common.rs)
// ==========================================

const stringOpcional = (max: number, nombre: string) =>
	z
		.string()
		.trim()
		.max(max, `${nombre} no puede exceder ${max} caracteres`)
		.optional()
		.or(z.literal('')); // Permitir string vacío como opcional

/**
 * Email: Contiene @, max EMAIL_MAX_LEN chars
 * Backend: validar_email_estandar()
 */
const emailSchema = z
	.string()
	.trim()
	.min(1, 'Correo requerido')
	.pipe(z.string().email('Correo inválido').max(EMAIL_MAX_LEN, 'Muy largo'));

/**
 * Password: min 6 chars
 * Backend: validar_password() - min 6 chars
 */
const passwordSchema = z
	.string()
	.min(6, 'La contraseña debe tener al menos 6 caracteres')
	.max(100, 'La contraseña no puede exceder 100 caracteres');

/**
 * Cédula: Solo números y guiones, sin letras
 * Backend: validar_cedula_estandar() - CEDULA_MIN_LEN-CEDULA_MAX_LEN chars
 */
const cedulaSchema = z
	.string()
	.trim()
	.min(1, 'Cédula requerida')
	.pipe(
		z
			.string()
			.min(CEDULA_MIN_LEN, 'Muy corta')
			.max(CEDULA_MAX_LEN, 'Muy larga')
			.regex(/^[0-9-]+$/, 'Formato inválido')
			.refine((val) => /\d/.test(val), 'Formato inválido')
	);

/**
 * Nombre/Apellido: Solo letras con acentos, espacios permitidos
 * Backend: validar_nombre_estandar() - max NOMBRE_MAX_LEN chars
 */
const nombreSchema = z
	.string()
	.trim()
	.min(1, 'Nombre requerido')
	.pipe(
		z
			.string()
			.max(NOMBRE_MAX_LEN, 'Muy largo')
			.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, 'Solo letras')
	);

const apellidoSchema = z
	.string()
	.trim()
	.min(1, 'Apellido requerido')
	.pipe(
		z
			.string()
			.max(NOMBRE_MAX_LEN, 'Muy largo')
			.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, 'Solo letras')
	);

/**
 * Campos opcionales según domain/user.rs:
 * - segundo_nombre/apellido: max 50
 * - telefono: max 20
 * - direccion: max 200
 * - numero_gafete: max 20
 */
const segundoNombreSchema = z
	.string()
	.max(50, 'Segundo nombre no puede exceder 50 caracteres')
	.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]*$/, 'Segundo nombre solo puede contener letras')
	.optional()
	.or(z.literal(''));

const segundoApellidoSchema = z
	.string()
	.max(50, 'Segundo apellido no puede exceder 50 caracteres')
	.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]*$/, 'Segundo apellido solo puede contener letras')
	.optional()
	.or(z.literal(''));

/**
 * Número de Gafete: max 20 chars (según backend domain/user.rs línea 83)
 * El formato K-#### es convención UI, backend solo valida longitud
 */
const numeroGafeteSchema = z
	.string()
	.max(20, 'Número de gafete no puede exceder 20 caracteres')
	.regex(/^(K-\d+)?$/, 'El número de gafete debe tener el formato K-1234 o estar vacío')
	.optional()
	.or(z.literal(''));

// ==========================================
// SCHEMAS PRINCIPALES
// ==========================================

export const CreateUserSchema = z.object({
	cedula: cedulaSchema,
	email: emailSchema,
	// Password es opcional en la creación, permitimos string vacío o undefined
	password: z.union([passwordSchema, z.literal(''), z.undefined()]).optional(),
	nombre: nombreSchema,
	apellido: apellidoSchema,
	segundoNombre: segundoNombreSchema,
	segundoApellido: segundoApellidoSchema,
	roleId: z.string().min(1, 'Rol requerido'),
	operacion: z.nativeEnum(Operacion, { message: 'CDI requerido' }),

	// Campos adicionales opcionales
	telefono: stringOpcional(20, 'Teléfono'),
	direccion: stringOpcional(200, 'Dirección'),
	fechaInicioLabores: z.string().optional(),
	numeroGafete: numeroGafeteSchema,
	fechaNacimiento: z.string().optional(),
	contactoEmergenciaNombre: stringOpcional(100, 'Nombre contacto emergencia'),
	contactoEmergenciaTelefono: stringOpcional(20, 'Teléfono contacto emergencia'),
	vencimientoPortacion: z.string().min(1, 'Vencimiento requerido'),
	mustChangePassword: z.boolean().optional()
});

// Update schema - NO incluye password ya que el cambio de contraseña
// se maneja por separado con ChangePasswordPanel
export const UpdateUserSchema = CreateUserSchema.omit({ password: true }).partial().extend({
	mustChangePassword: z.boolean().optional()
});

export const ChangePasswordSchema = z
	.object({
		currentPassword: z.string().optional(), // Requerido salvo admin reset
		newPassword: passwordSchema,
		confirmPassword: z.string()
	})
	.refine((data) => data.newPassword === data.confirmPassword, {
		message: 'Las contraseñas no coinciden',
		path: ['confirmPassword']
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
