import { z } from 'zod';
import type { ValidacionIngresoResponse } from './ingreso';
import type { GafeteResponse } from './gafete';

// ==========================================
// TIPOS DE DOMINIO - FORMULARIO
// ==========================================

/**
 * Estado de validación del contratista
 * Representa el resultado de validar si puede ingresar
 */
export interface ContratistaValidationState {
	/** Indica si ya se ejecutó la validación */
	isValidated: boolean;
	/** Indica si el contratista puede ingresar */
	canEnter: boolean;
	/** Datos completos de la validación desde el backend */
	data: ValidacionIngresoResponse | null;
	/** Mensaje de error si no puede ingresar */
	errorMessage: string | null;
}

/**
 * Resultado de la lógica de auto-selección de vehículo
 */
export interface AutoSelectionResult {
	/** Modo sugerido basado en la cantidad de vehículos */
	suggestedMode: 'caminando' | 'vehiculo';
	/** ID del vehículo sugerido (solo si tiene exactamente 1) */
	suggestedVehicleId: string | null;
	/** Razón de la sugerencia */
	reason: 'single_vehicle' | 'no_vehicles' | 'multiple_vehicles';
}

/**
 * Resultado de validación de gafete
 */
export interface GafeteValidationResult {
	/** Indica si el gafete es válido */
	isValid: boolean;
	/** Lista de sugerencias si el gafete no es válido */
	suggestions: string[];
}

/**
 * Estado completo del formulario de ingreso
 */
export interface IngresoFormState {
	// Datos del contratista
	contratistaId: string;
	contratistaNombre: string;
	contratistaData: any | null;
	puedeIngresar: boolean;
	mensajeValidacion: string;

	// Datos del ingreso
	modoIngreso: 'caminando' | 'vehiculo';
	vehiculoId: string | null;
	gafeteNumero: string;
	tipoAutorizacion: string;
	observaciones: string;
}

// ==========================================
// ESQUEMAS DE VALIDACIÓN CON ZOD
// ==========================================

/**
 * Schema para validar gafete contra lista de disponibles
 */
export const GafeteValidationSchema = z
	.object({
		numero: z.string().trim().toUpperCase(),
		gafetesDisponibles: z.array(
			z.object({
				numero: z.string(),
				estaDisponible: z.boolean(),
				tipo: z.string().optional()
			})
		)
	})
	.refine(
		(data) => {
			// Si no hay número, es válido (opcional)
			if (!data.numero) return true;

			// Verificar que existe y está disponible
			return data.gafetesDisponibles.some((g) => g.numero === data.numero && g.estaDisponible);
		},
		{
			message: 'Gafete no disponible o inválido',
			path: ['numero']
		}
	);

/**
 * Schema para validar modo de ingreso con vehículo
 */
export const ModoVehiculoSchema = z
	.object({
		modoIngreso: z.enum(['caminando', 'vehiculo']),
		vehiculoId: z.string().uuid().nullable(),
		tieneVehiculos: z.boolean()
	})
	.refine(
		(data) => {
			// Si modo es vehículo, debe tener vehiculoId
			if (data.modoIngreso === 'vehiculo') {
				return data.vehiculoId !== null;
			}
			return true;
		},
		{
			message: 'Debe seleccionar un vehículo',
			path: ['vehiculoId']
		}
	)
	.refine(
		(data) => {
			// Si modo es vehículo, debe tener vehículos disponibles
			if (data.modoIngreso === 'vehiculo') {
				return data.tieneVehiculos;
			}
			return true;
		},
		{
			message: 'El contratista no tiene vehículos registrados',
			path: ['modoIngreso']
		}
	);

/**
 * Schema para validación completa del formulario antes de submit
 */
export const IngresoFormValidationSchema = z.object({
	contratistaValidated: z.boolean().refine((val) => val === true, {
		message: 'Debe validar el contratista primero'
	}),
	canEnter: z.boolean().refine((val) => val === true, {
		message: 'El contratista no puede ingresar'
	}),
	contratistaId: z.string().uuid('ID de contratista inválido'),
	modoIngreso: z.enum(['caminando', 'vehiculo']),
	vehiculoId: z.string().uuid().nullable(),
	gafeteNumero: z.string().optional(),
	tipoAutorizacion: z.enum(['praind', 'correo'])
});

// ==========================================
// TIPOS INFERIDOS DE LOS SCHEMAS
// ==========================================

export type GafeteValidationInput = z.infer<typeof GafeteValidationSchema>;
export type ModoVehiculoValidation = z.infer<typeof ModoVehiculoSchema>;
export type IngresoFormValidation = z.infer<typeof IngresoFormValidationSchema>;

// ==========================================
// TIPOS PARA INPUTS DE VALIDACIÓN
// ==========================================

/**
 * Input para preparar formulario de ingreso
 */
export interface PrepararFormularioInput {
	contratistaId: string;
}

/**
 * Output de preparar formulario de ingreso
 */
export interface PrepararFormularioOutput {
	validacion: ValidacionIngresoResponse;
	autoSeleccion: AutoSelectionResult;
}

/**
 * Input para validar gafete
 */
export interface ValidarGafeteInput {
	gafeteNumero: string;
	gafetesDisponibles: GafeteResponse[];
}

/**
 * Input para validar modo vehículo
 */
export interface ValidarModoVehiculoInput {
	modoIngreso: 'caminando' | 'vehiculo';
	vehiculoId: string | null;
	tieneVehiculos: boolean;
}

/**
 * Input para validación completa del formulario
 */
export interface ValidarFormularioCompletoInput {
	contratistaValidated: boolean;
	canEnter: boolean;
	contratistaId: string;
	modoIngreso: 'caminando' | 'vehiculo';
	vehiculoId: string | null;
	gafeteNumero?: string;
	tipoAutorizacion: 'praind' | 'correo';
}
