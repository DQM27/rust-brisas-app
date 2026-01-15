// ============================================
// src/lib/logic/listaNegra/listaNegraService.ts
// ============================================
// Servicio frontend para Lista Negra - Simplificado

import { listaNegra } from '$lib/api/listaNegra';
import type {
	ListaNegraResponse,
	ListaNegraListResponse,
	AddToListaNegraInput,
	UpdateListaNegraInput,
	PersonaSearchResult,
	NivelSeveridad
} from '$lib/types/listaNegra';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// VALIDATION
// ============================================

type ValidationResult = { ok: true } | { ok: false; message: string };

const NIVELES_VALIDOS: NivelSeveridad[] = ['ALTO', 'MEDIO', 'BAJO'];

function validateAddInput(input: AddToListaNegraInput): ValidationResult {
	const { cedula, nombre, apellido, nivelSeveridad, motivoBloqueo, bloqueadoPor } = input;

	// Validar cédula
	const c = (cedula || '').trim();
	if (!c) {
		return { ok: false, message: 'La cédula no puede estar vacía.' };
	}
	if (c.length < 7 || c.length > 20) {
		return { ok: false, message: 'La cédula debe tener entre 7 y 20 caracteres.' };
	}
	if (!/^[0-9-]+$/.test(c)) {
		return { ok: false, message: 'La cédula solo puede contener números y guiones.' };
	}

	// Validar nombre
	const n = (nombre || '').trim();
	if (!n) {
		return { ok: false, message: 'El nombre no puede estar vacío.' };
	}
	if (n.length > 50) {
		return { ok: false, message: 'El nombre no puede exceder 50 caracteres.' };
	}

	// Validar apellido
	const a = (apellido || '').trim();
	if (!a) {
		return { ok: false, message: 'El apellido no puede estar vacío.' };
	}
	if (a.length > 50) {
		return { ok: false, message: 'El apellido no puede exceder 50 caracteres.' };
	}

	// Validar nivel de severidad
	if (!NIVELES_VALIDOS.includes(nivelSeveridad)) {
		return { ok: false, message: 'Debe seleccionar un nivel de severidad válido.' };
	}

	// Validar motivo (opcional, solo validar longitud si se proporciona)
	if (motivoBloqueo) {
		const m = motivoBloqueo.trim();
		if (m.length > 500) {
			return { ok: false, message: 'El motivo no puede exceder 500 caracteres.' };
		}
	}

	// Validar bloqueadoPor
	const b = bloqueadoPor.trim();
	if (!b) {
		return { ok: false, message: 'Debe especificar quién realizó el bloqueo.' };
	}
	if (b.length > 100) {
		return { ok: false, message: 'El nombre de quien bloqueó no puede exceder 100 caracteres.' };
	}

	return { ok: true };
}

function validateUpdateInput(input: UpdateListaNegraInput): ValidationResult {
	if (input.nivelSeveridad !== undefined && !NIVELES_VALIDOS.includes(input.nivelSeveridad)) {
		return { ok: false, message: 'Nivel de severidad inválido.' };
	}

	if (input.motivoBloqueo !== undefined) {
		const m = input.motivoBloqueo.trim();
		if (!m) {
			return { ok: false, message: 'El motivo de bloqueo no puede estar vacío.' };
		}
		if (m.length > 500) {
			return { ok: false, message: 'El motivo no puede exceder 500 caracteres.' };
		}
	}

	return { ok: true };
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: unknown): string {
	if (!err) return 'Ocurrió un error desconocido.';

	if (typeof err === 'string') {
		if (/ya está en la lista negra|AlreadyExists/i.test(err)) {
			return 'Esta persona ya está bloqueada.';
		}
		if (/no existe|NotFound/i.test(err)) {
			return 'Registro no encontrado.';
		}
		if (/cédula/i.test(err) && /vacía/i.test(err)) {
			return 'La cédula no puede estar vacía.';
		}
		if (/motivo/i.test(err)) {
			return 'Debe especificar un motivo de bloqueo válido.';
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
 * Obtener todos los bloqueados (historial)
 */
export async function fetchAll(): Promise<ServiceResult<ListaNegraListResponse>> {
	try {
		const data = await listaNegra.getAll();
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar lista negra:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener bloqueados activos (filtrar de todos)
 */
export async function fetchActivos(): Promise<ServiceResult<ListaNegraResponse[]>> {
	try {
		const result = await listaNegra.getAll();
		const activos = result.bloqueados.filter((b) => b.isActive);
		return { ok: true, data: activos };
	} catch (err: unknown) {
		console.error('Error al cargar bloqueados activos:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener bloqueado por ID
 */
export async function fetchById(id: string): Promise<ServiceResult<ListaNegraResponse>> {
	try {
		const data = await listaNegra.getById(id);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar bloqueado:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Agregar a lista negra (con validación)
 */
export async function add(input: AddToListaNegraInput): Promise<ServiceResult<ListaNegraResponse>> {
	const validation = validateAddInput(input);
	if (!validation.ok) {
		return { ok: false, error: validation.message };
	}

	try {
		const data = await listaNegra.add(input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al agregar a lista negra:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar registro en lista negra
 */
export async function update(
	id: string,
	input: UpdateListaNegraInput
): Promise<ServiceResult<ListaNegraResponse>> {
	const validation = validateUpdateInput(input);
	if (!validation.ok) {
		return { ok: false, error: validation.message };
	}

	try {
		const data = await listaNegra.update(id, input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al actualizar lista negra:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Desbloquear (desactivar isActive)
 * Nota: La operación no retorna el registro actualizado, solo confirma el éxito
 */
export async function unblock(id: string): Promise<ServiceResult<void>> {
	if (!id) {
		return { ok: false, error: 'El ID del registro es inválido.' };
	}

	try {
		await listaNegra.remove(id);
		return { ok: true, data: undefined };
	} catch (err: unknown) {
		console.error('Error al desbloquear:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Re-bloquear persona (reactivar)
 * Nota: Los parámetros adicionales ya no son necesarios,
 * el registro se restaura con sus valores originales
 */
export async function reblock(id: string): Promise<ServiceResult<ListaNegraResponse>> {
	if (!id) {
		return { ok: false, error: 'El ID del registro es inválido.' };
	}

	try {
		const data = await listaNegra.reactivate(id);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al re-bloquear:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Buscar personas para formulario de bloqueo
 */
export async function searchPersonas(query: string): Promise<ServiceResult<PersonaSearchResult[]>> {
	if (!query || query.trim().length < 2) {
		return { ok: true, data: [] };
	}

	try {
		const data = await listaNegra.searchPersonas(query.trim());
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al buscar personas:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// COMPATIBILITY LAYER
// ============================================

export const submitAddToListaNegra = add;
export const submitFetchAllListaNegra = fetchAll;
export const submitFetchActivosListaNegra = fetchActivos;
export const submitFetchListaNegraById = fetchById;
export const submitUpdateListaNegra = update;
export const submitUnblockListaNegra = unblock;
export const addToListaNegraAction = add;
export const loadAllListaNegra = fetchAll;
export const unblockListaNegraAction = unblock;
export const reblockListaNegraAction = reblock;
