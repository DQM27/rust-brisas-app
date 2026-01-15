import { z } from 'zod';
import { PLACA_MIN_LEN, PLACA_MAX_LEN } from './domainConstants';

// ==========================================
// BASE SCHEMA (Para defaults de Superforms)
// ==========================================

export const vehiculoSchemaBase = z.object({
	tipoVehiculo: z.string().default(''),
	placa: z.string().default(''),
	marca: z.string().default(''),
	modelo: z.string().default(''),
	color: z.string().default('')
});

// ==========================================
// SCHEMA CON VALIDACIONES (Alineado con backend models/vehiculo.rs)
// ==========================================

/**
 * TipoVehiculo enum - Sincronizado con backend:
 * Backend: TipoVehiculo { Motocicleta, Automovil, Camioneta, Camion, Otro }
 */
export const TIPO_VEHICULO_OPTIONS = [
	'motocicleta',
	'automovil',
	'camioneta',
	'camion',
	'otro'
] as const;

export const vehiculoSchema = z.object({
	tipoVehiculo: z.enum(TIPO_VEHICULO_OPTIONS, { message: 'Tipo de vehículo requerido' }),
	placa: z
		.string()
		.trim()
		.min(1, 'Placa requerida')
		.pipe(
			z
				.string()
				.min(PLACA_MIN_LEN, `Mínimo ${PLACA_MIN_LEN} caracteres`)
				.max(PLACA_MAX_LEN, `Máximo ${PLACA_MAX_LEN} caracteres`)
				.regex(/^[A-Za-z0-9\s-]+$/, 'Solo letras, números, guiones y espacios')
				.transform((val) => val.toUpperCase().trim())
		),
	marca: z.string().max(50, 'Máximo 50 caracteres').optional().or(z.literal('')),
	modelo: z.string().max(50, 'Máximo 50 caracteres').optional().or(z.literal('')),
	color: z.string().max(30, 'Máximo 30 caracteres').optional().or(z.literal(''))
});

// ==========================================
// TIPOS INFERIDOS
// ==========================================

export type VehiculoFormData = z.infer<typeof vehiculoSchemaBase>;
export type VehiculoSchema = typeof vehiculoSchema;
export type TipoVehiculo = (typeof TIPO_VEHICULO_OPTIONS)[number];
