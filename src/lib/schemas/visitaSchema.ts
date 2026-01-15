import { z } from 'zod';
import {
	CEDULA_MIN_LEN,
	CEDULA_MAX_LEN,
	NOMBRE_MAX_LEN,
	SEGUNDO_NOMBRE_MAX_LEN,
	PLACA_MAX_LEN,
	ENTIDAD_NOMBRE_MAX_LEN,
	OBSERVACIONES_MAX_LEN
} from './domainConstants';

// ==========================================
// VALIDACIONES BÁSICAS (Alineadas con backend domain/common.rs)
// ==========================================

/**
 * Cédula para visitantes: más flexible para incluir pasaportes
 * Backend: validar_cedula_estandar() - 5-20 chars
 */
const cedulaSchema = z
	.string()
	.trim()
	.min(CEDULA_MIN_LEN, `Cédula debe tener al menos ${CEDULA_MIN_LEN} caracteres`)
	.max(CEDULA_MAX_LEN, `Cédula no puede exceder ${CEDULA_MAX_LEN} caracteres`);

/**
 * Nombre/Apellido: Solo letras con acentos, espacios permitidos
 * Backend: validar_nombre_estandar() - max NOMBRE_MAX_LEN chars
 */
const nombreSchema = z
	.string()
	.trim()
	.min(1, 'Nombre es requerido')
	.max(NOMBRE_MAX_LEN, `Nombre no puede exceder ${NOMBRE_MAX_LEN} caracteres`)
	.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, 'Solo puede contener letras');

const apellidoSchema = z
	.string()
	.trim()
	.min(1, 'Apellido es requerido')
	.max(NOMBRE_MAX_LEN, `Apellido no puede exceder ${NOMBRE_MAX_LEN} caracteres`)
	.regex(/^[a-zA-ZáéíóúÁÉÍÓÚñÑüÜ\s]+$/, 'Solo puede contener letras');

const opcionalTextoSchema = z
	.string()
	.trim()
	.max(SEGUNDO_NOMBRE_MAX_LEN, `No puede exceder ${SEGUNDO_NOMBRE_MAX_LEN} caracteres`)
	.optional()
	.or(z.literal(''));

// ==========================================
// SCHEMA PRINCIPAL
// ==========================================

export const VisitaSchema = z
	.object({
		// Datos de Cita
		fecha: z.string().min(1, 'Fecha es requerida'), // YYYY-MM-DD
		hora: z.string().min(1, 'Hora es requerida'), // HH:MM
		anfitrion: z.string().min(1, 'Anfitrión es requerido').max(NOMBRE_MAX_LEN),
		areaVisitada: z.string().min(1, 'Área es requerida').max(NOMBRE_MAX_LEN),
		motivo: z.string().max(200).optional().or(z.literal('')),

		// Datos del Visitante
		cedula: cedulaSchema,
		nombre: nombreSchema,
		segundoNombre: opcionalTextoSchema,
		apellido: apellidoSchema,
		segundoApellido: opcionalTextoSchema,
		empresa: z.string().max(ENTIDAD_NOMBRE_MAX_LEN).optional().or(z.literal('')),

		// Vehículo (Opcional)
		tieneVehiculo: z.boolean().default(false),
		placa: z.string().max(PLACA_MAX_LEN).optional().or(z.literal(''))
	})
	.superRefine((data, ctx) => {
		if (data.tieneVehiculo && (!data.placa || data.placa.trim() === '')) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				message: 'La placa es requerida si tiene vehículo',
				path: ['placa']
			});
		}
	});

export type VisitaForm = z.infer<typeof VisitaSchema>;

// ==========================================
// INGRESO DIRECTO SCHEMA (SUPERFORMS)
// ==========================================

export const ingresoVisitaSchemaBase = z.object({
	cedula: z.string().default(''),
	nombre: z.string().default(''),
	segundoNombre: z.string().default(''),
	apellido: z.string().default(''),
	segundoApellido: z.string().default(''),
	empresaId: z.string().default(''),
	anfitrion: z.string().default(''),
	areaVisitada: z.string().default(''),
	motivo: z.string().default(''),
	gafete: z.string().default(''),
	observaciones: z.string().default('')
});

export const ingresoVisitaSchema = z.object({
	cedula: cedulaSchema,
	nombre: nombreSchema,
	segundoNombre: opcionalTextoSchema,
	apellido: apellidoSchema,
	segundoApellido: opcionalTextoSchema,
	empresaId: z.string().min(1, 'Empresa requerida'),
	anfitrion: z.string().trim().min(1, 'Anfitrión requerido').max(NOMBRE_MAX_LEN),
	areaVisitada: z.string().trim().min(1, 'Área requerida').max(NOMBRE_MAX_LEN),
	motivo: z.string().trim().min(1, 'Motivo requerido').max(200),
	gafete: z.string().trim().max(20).optional().or(z.literal('')),
	observaciones: z.string().trim().max(OBSERVACIONES_MAX_LEN).optional().or(z.literal(''))
});

export type IngresoVisitaFormData = z.infer<typeof ingresoVisitaSchemaBase>;
