// ============================================
// src/lib/logic/contratista/contratistaService.ts
// ============================================
// Servicio para gestión de contratistas

import { contratistas } from '$lib/api/contratista';
import type {
	ContratistaResponse,
	ContratistaListResponse,
	CreateContratistaInput,
	UpdateContratistaInput,
	EstadoContratista
} from '$lib/types/contratista';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// PUBLIC API - FETCH OPERATIONS
// ============================================

/**
 * Obtener todos los contratistas
 */
export async function fetchAllContratistas(): Promise<ServiceResult<ContratistaListResponse>> {
	try {
		const result = await contratistas.list();
		return { ok: true, data: result };
	} catch (err: unknown) {
		console.error('Error al cargar contratistas:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener solo contratistas activos
 */
export async function fetchActiveContratistas(): Promise<ServiceResult<ContratistaResponse[]>> {
	try {
		const result = await contratistas.list();
		const activos = result.contratistas.filter((c) => c.estado === 'activo');
		return { ok: true, data: activos };
	} catch (err: unknown) {
		console.error('Error al cargar contratistas activos:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener un contratista por ID
 */
export async function fetchContratistaById(
	id: string
): Promise<ServiceResult<ContratistaResponse>> {
	try {
		const contratista = await contratistas.getById(id);
		return { ok: true, data: contratista };
	} catch (err: unknown) {
		console.error('Error al cargar contratista:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear nuevo contratista
 */
export async function createContratista(
	input: CreateContratistaInput
): Promise<ServiceResult<ContratistaResponse>> {
	try {
		const contratista = await contratistas.create(input);
		return { ok: true, data: contratista };
	} catch (err: unknown) {
		console.error('Error al crear contratista:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar contratista existente
 */
export async function updateContratista(
	id: string,
	input: UpdateContratistaInput
): Promise<ServiceResult<ContratistaResponse>> {
	try {
		const contratista = await contratistas.update(id, input);
		return { ok: true, data: contratista };
	} catch (err: unknown) {
		console.error('Error al actualizar contratista:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Eliminar contratista
 */
export async function deleteContratista(id: string): Promise<ServiceResult<void>> {
	try {
		await contratistas.delete(id);
		return { ok: true, data: undefined };
	} catch (err: unknown) {
		console.error('Error al eliminar contratista:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Cambiar estado de contratista
 */
export async function changeEstado(
	id: string,
	nuevoEstado: EstadoContratista
): Promise<ServiceResult<ContratistaResponse>> {
	try {
		const contratista = await contratistas.changeEstado(id, nuevoEstado);
		return { ok: true, data: contratista };
	} catch (err: unknown) {
		console.error('Error al cambiar estado:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: unknown): string {
	if (!err) return 'Ocurrió un error desconocido.';

	if (typeof err === 'string') {
		if (/unique|cedula|duplicat/i.test(err)) return 'Ya existe un contratista con esa cédula.';
		if (/empresa/i.test(err)) return 'La empresa seleccionada no es válida.';
		if (/praind|vencimiento/i.test(err)) return 'Fecha de vencimiento PRAIND inválida.';
		return err;
	}

	if (err instanceof Error) return parseError(err.message);

	if (typeof err === 'object' && err !== null) {
		const obj = err as Record<string, unknown>;
		const msg = (obj.message as string) ?? obj.toString();
		if (/unique|cedula|duplicat/i.test(msg)) return 'Ya existe un contratista con esa cédula.';
		if (/empresa/i.test(msg)) return 'La empresa seleccionada no es válida.';
		if (/failed/i.test(msg)) return 'Falló la operación en la base de datos.';
		return msg;
	}

	return 'Ocurrió un error inesperado.';
}

// ============================================
// ARCHIVED / RESTORE
// ============================================

export async function restoreContratista(id: string): Promise<ServiceResult<void>> {
	try {
		await contratistas.restore(id);
		return { ok: true, data: undefined };
	} catch (err: unknown) {
		console.error('Error al restaurar contratista:', err);
		return { ok: false, error: parseError(err) };
	}
}

export async function getArchivedContratistas(): Promise<ServiceResult<ContratistaResponse[]>> {
	try {
		const result = await contratistas.listArchived();
		return { ok: true, data: result };
	} catch (err: unknown) {
		console.error('Error al cargar contratistas archivados:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// UI HELPERS & LOGIC (Decoupled from Components)
// ============================================

/**
 * Validar unicidad de cédula
 */
export async function checkCedulaUnique(cedula: string, excludeId?: string): Promise<boolean> {
	if (!cedula || cedula.length < 4) return true;
	try {
		// El backend espera el field 'cedula'
		return await contratistas.checkUnique('cedula', cedula, excludeId);
	} catch (err) {
		console.error('Error checking unique cedula:', err);
		// Si falla, asumimos false para evitar duplicados accidentales o true para no bloquear?
		// Mejor false para obligar re-intento o mostrar error
		return false;
	}
}

/**
 * Formatear fecha para mostrar en UI (YYYY-MM-DD -> DD/MM/YYYY)
 */
export function formatDateForDisplay(isoDate?: string): string {
	if (!isoDate) return '';
	// Asume YYYY-MM-DD o ISO completo
	try {
		const [year, month, day] = isoDate.split('T')[0].split('-');
		if (!year || !month || !day) return '';
		return `${day}/${month}/${year}`;
	} catch {
		return '';
	}
}

/**
 * Formatear fecha para backend (DD/MM/YYYY -> YYYY-MM-DD)
 */
export function formatDateForBackend(displayDate: string): string {
	if (!displayDate || displayDate.length !== 10) return '';
	const [day, month, year] = displayDate.split('/');
	if (!day || !month || !year) return '';
	return `${year}-${month}-${day}`;
}

/**
 * Preparar payload para creación
 */
export function prepareCreatePayload(
	formData: any // Se puede tipar con ContratistaFormData si se importa, pero para evitar deps circulares usamos any o definimos interfaz parcial
): CreateContratistaInput {
	const payload: CreateContratistaInput = {
		cedula: formData.cedula,
		nombre: formData.nombre,
		apellido: formData.apellido,
		empresaId: formData.empresaId,
		fechaVencimientoPraind: formatDateForBackend(formData.fechaVencimientoPraind),
		tieneVehiculo: false
	};

	if (formData.segundoNombre?.trim()) payload.segundoNombre = formData.segundoNombre.trim();
	if (formData.segundoApellido?.trim()) payload.segundoApellido = formData.segundoApellido.trim();

	return payload;
}

/**
 * Preparar payload para actualización
 */
export function prepareUpdatePayload(id: string, formData: any): UpdateContratistaInput {
	const payload: UpdateContratistaInput = {
		id,
		cedula: formData.cedula,
		nombre: formData.nombre,
		apellido: formData.apellido,
		empresaId: formData.empresaId,
		fechaVencimientoPraind: formatDateForBackend(formData.fechaVencimientoPraind)
	};

	payload.segundoNombre = formData.segundoNombre?.trim() || undefined;
	payload.segundoApellido = formData.segundoApellido?.trim() || undefined;

	return payload;
}
