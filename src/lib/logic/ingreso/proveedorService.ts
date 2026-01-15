// ==========================================
// src/lib/logic/ingreso/proveedorService.ts
// ==========================================
// Lógica de negocio para ingresos de PROVEEDORES

import type { IngresoResponse } from '$lib/types/ingreso';
import type { CreateIngresoProveedorInput } from '$lib/types/ingreso-nuevos';

// ==========================================
// TIPOS INTERNOS
// ==========================================

export interface ProveedorFormData {
	cedula: string;
	nombre: string;
	apellido: string;
	empresaId: string;
	areaVisitada: string;
	motivo: string;
	modoIngreso: 'caminando' | 'vehiculo';
	vehiculoTipo?: string;
	vehiculoPlaca?: string;
	vehiculoMarca?: string;
	vehiculoModelo?: string;
	vehiculoColor?: string;
	gafeteNumero?: string;
	observaciones?: string;
}

export interface ValidacionProveedorResult {
	isValid: boolean;
	errors: Record<string, string>;
}

// ==========================================
// VALIDACIONES DE DOMINIO
// ==========================================

/**
 * Valida los datos del formulario de proveedor
 */
export function validarDatosProveedor(data: ProveedorFormData): ValidacionProveedorResult {
	const errors: Record<string, string> = {};

	// Validar cédula
	if (!data.cedula?.trim()) {
		errors.cedula = 'Cédula es requerida';
	}

	// Validar nombre
	if (!data.nombre?.trim()) {
		errors.nombre = 'Nombre es requerido';
	}

	// Validar apellido
	if (!data.apellido?.trim()) {
		errors.apellido = 'Apellido es requerido';
	}

	// Validar empresa
	if (!data.empresaId?.trim()) {
		errors.empresaId = 'Empresa proveedora es requerida';
	}

	// Validar área visitada
	if (!data.areaVisitada?.trim()) {
		errors.areaVisitada = 'Área visitada es requerida';
	}

	// Validar motivo
	if (!data.motivo?.trim()) {
		errors.motivo = 'Motivo es requerido';
	}

	// Validar modo y vehículo
	if (data.modoIngreso === 'vehiculo' && !data.vehiculoPlaca?.trim()) {
		errors.vehiculoPlaca = 'Placa de vehículo es requerida cuando ingresa en vehículo';
	}

	return {
		isValid: Object.keys(errors).length === 0,
		errors
	};
}

/**
 * Normaliza los datos antes de enviar al backend
 */
export function normalizarDatosProveedor(data: ProveedorFormData): ProveedorFormData {
	return {
		...data,
		cedula: data.cedula.trim(),
		nombre: data.nombre.trim(),
		apellido: data.apellido.trim(),
		empresaId: data.empresaId.trim(),
		areaVisitada: data.areaVisitada.trim(),
		motivo: data.motivo.trim(),
		vehiculoPlaca: data.vehiculoPlaca?.trim() || undefined,
		gafeteNumero: data.gafeteNumero?.trim().toUpperCase() || undefined,
		observaciones: data.observaciones?.trim() || undefined
	};
}

// ==========================================
// OPERACIONES PRINCIPALES
// ==========================================

/**
 * Crea un ingreso de proveedor
 */
export async function crearIngresoProveedor(
	data: ProveedorFormData,
	usuarioId: string
): Promise<IngresoResponse> {
	// 1. Validar datos
	const validacion = validarDatosProveedor(data);
	if (!validacion.isValid) {
		const primerError = Object.values(validacion.errors)[0];
		throw new Error(primerError);
	}

	// 2. Normalizar datos
	const datosNormalizados = normalizarDatosProveedor(data);

	// 3. Preparar input para el backend
	const gafeteNum = data.gafeteNumero ? parseInt(data.gafeteNumero) : undefined;
	const finalGafete = gafeteNum !== undefined && !isNaN(gafeteNum) ? gafeteNum : undefined;

	const input: CreateIngresoProveedorInput = {
		cedula: datosNormalizados.cedula,
		nombre: datosNormalizados.nombre,
		apellido: datosNormalizados.apellido,
		proveedorId: datosNormalizados.empresaId, // Aquí empresaId en el formulario es el ID del proveedor
		areaVisitada: datosNormalizados.areaVisitada,
		motivo: datosNormalizados.motivo,
		modoIngreso: datosNormalizados.modoIngreso,
		placaVehiculo: datosNormalizados.vehiculoPlaca || undefined,
		gafeteNumero: finalGafete,
		observaciones: datosNormalizados.observaciones || undefined
	};

	// 4. Invocar API
	const { ingresoProveedorService } = await import('$lib/services/ingresoProveedorService');

	const result = await ingresoProveedorService.createIngreso(input, usuarioId);
	return result as any;
}

// ==========================================
// UTILIDADES
// ==========================================

/**
 * Verifica si un modo requiere placa de vehículo
 */
export function requierePlaca(modoIngreso: 'caminando' | 'vehiculo'): boolean {
	return modoIngreso === 'vehiculo';
}

// ==========================================
// VALIDACIÓN Y AUTO-SELECCIÓN (Mimic Contratista)
// ==========================================

import type { AutoSelectionResult } from '$lib/types/ingreso-form.types';
import type { ValidacionIngresoProveedorResponse } from '$lib/types/ingreso-nuevos';

export interface PrepararProveedorOutput {
	validacion: ValidacionIngresoProveedorResponse;
	autoSeleccion: AutoSelectionResult;
}

/**
 * Valida al proveedor y calcula autoselección
 */
export async function validarProveedor(proveedorId: string): Promise<PrepararProveedorOutput> {
	const { ingresoProveedorService } = await import('$lib/services/ingresoProveedorService');

	// 1. Llamar al backend
	const validacion = await ingresoProveedorService.validarIngreso(proveedorId);

	// 2. Calcular autoselección
	// NOTA: El backend ya no devuelve vehículos en la validación por seguridad.
	// Se asume que el llamador ya tiene los datos del proveedor.
	const autoSeleccion: AutoSelectionResult = {
		suggestedMode: 'caminando',
		suggestedVehicleId: null,
		reason: 'multiple_vehicles'
	};

	return {
		validacion,
		autoSeleccion
	};
}
