// ============================================
// src/lib/logic/gafete/gafeteService.ts
// ============================================

import { gafete } from '$lib/api/gafete';
import type {
	GafeteResponse,
	GafeteListResponse,
	CreateGafeteInput,
	UpdateGafeteInput
} from '$lib/types/gafete';
import { ZodError } from 'zod';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: unknown): string {
	if (!err) return 'Ocurrió un error desconocido.';

	// Manejo de errores de Zod
	if (err instanceof ZodError) {
		return err.issues.map((e) => e.message).join(', ');
	}

	if (typeof err === 'string') {
		if (/ya existe/i.test(err)) {
			return 'Ya existe un gafete con ese número.';
		}
		if (/no encontrado/i.test(err)) {
			return 'Gafete no encontrado.';
		}
		return err;
	}

	if (err instanceof Error) return parseError(err.message);

	if (typeof err === 'object' && err !== null) {
		const obj = err as Record<string, unknown>;
		if (obj.message && typeof obj.message === 'string') {
			return parseError(obj.message);
		}
	}

	return 'Ocurrió un error inesperado al procesar la solicitud.';
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear un nuevo gafete
 */
export async function create(input: CreateGafeteInput): Promise<ServiceResult<GafeteResponse>> {
	try {
		const data = await gafete.create(input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al crear gafete:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener todos los gafetes
 */
export async function fetchAll(): Promise<ServiceResult<GafeteListResponse>> {
	try {
		const data = await gafete.getAll();
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar gafetes:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener gafetes disponibles
 */
export async function fetchDisponibles(
	tipo: string = 'contratista'
): Promise<ServiceResult<GafeteResponse[]>> {
	try {
		const data = await gafete.getDisponibles(tipo);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar gafetes disponibles:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener gafete por Número y Tipo
 */
export async function fetchByNumero(
	numero: string,
	tipo: string
): Promise<ServiceResult<GafeteResponse>> {
	try {
		const data = await gafete.get(numero, tipo);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar gafete:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar gafete
 */
export async function update(
	numero: string,
	tipo: string,
	input: UpdateGafeteInput
): Promise<ServiceResult<GafeteResponse>> {
	try {
		const data = await gafete.update(numero, tipo, input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al actualizar gafete:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Eliminar gafete
 */
export async function remove(
	numero: string,
	tipo: string,
	userId?: string
): Promise<ServiceResult<null>> {
	try {
		await gafete.delete(numero, tipo, userId);
		return { ok: true, data: null };
	} catch (err: unknown) {
		console.error('Error al eliminar gafete:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Crear rango de gafetes
 */
export async function createRange(
	input: import('$lib/types/gafete').CreateGafeteRangeInput
): Promise<ServiceResult<string[]>> {
	try {
		const data = await gafete.createRange(input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al crear rango de gafetes:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Verificar disponibilidad de un gafete específico
 */
export async function isDisponible(numero: string, tipo: string): Promise<ServiceResult<boolean>> {
	try {
		const data = await gafete.isDisponible(numero, tipo);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al verificar disponibilidad:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar estado del gafete
 */
export async function updateStatus(
	id: string,
	estado: string,
	usuarioId?: string,
	motivo?: string
): Promise<ServiceResult<GafeteResponse>> {
	try {
		const data = await gafete.updateStatus(id, estado, usuarioId, motivo);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al actualizar estado del gafete:', err);
		return { ok: false, error: parseError(err) };
	}
}
