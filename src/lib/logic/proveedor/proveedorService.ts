// ============================================
// src/lib/logic/proveedor/proveedorService.ts
// ============================================
// High-level business logic service for Proveedor

import { proveedor } from '$lib/api/proveedor';
import type {
	ProveedorResponse,
	CreateProveedorInput,
	UpdateProveedorInput
} from '$lib/types/proveedor';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// PUBLIC API - FETCH OPERATIONS
// ============================================

/**
 * Obtener todos los proveedores
 */
export async function fetchAllProveedores(): Promise<ServiceResult<ProveedorResponse[]>> {
	try {
		const data = await proveedor.getAll();
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar proveedores:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener solo proveedores activos
 */
export async function fetchActiveProveedores(): Promise<ServiceResult<ProveedorResponse[]>> {
	try {
		const result = await fetchAllProveedores();
		if (result.ok) {
			const activos = result.data.filter((p) => p.estado?.toLowerCase() === 'activo');
			return { ok: true, data: activos };
		}
		return result;
	} catch (err: unknown) {
		console.error('Error al cargar proveedores activos:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Buscar proveedores
 */
export async function searchProveedores(
	query: string
): Promise<ServiceResult<ProveedorResponse[]>> {
	try {
		const data = await proveedor.search(query);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al buscar proveedores:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener proveedor por ID
 */
export async function fetchProveedorById(id: string): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const data = await proveedor.getById(id);
		if (!data) {
			return { ok: false, error: 'Proveedor no encontrado' };
		}
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener proveedor por cédula
 */
export async function fetchProveedorByCedula(
	cedula: string
): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const data = await proveedor.getByCedula(cedula);
		if (!data) {
			return { ok: false, error: 'Proveedor no encontrado' };
		}
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear nuevo proveedor
 */
export async function createProveedor(
	input: CreateProveedorInput
): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const data = await proveedor.create(input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al crear proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar proveedor
 */
export async function updateProveedor(
	id: string,
	input: UpdateProveedorInput
): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const data = await proveedor.update(id, input);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al actualizar proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Cambiar estado de proveedor (Activo/Inactivo)
 */
export async function changeStatus(
	id: string,
	newStatus: string
): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const data = await proveedor.changeStatus(id, newStatus);
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cambiar estado:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Eliminar proveedor
 */
export async function deleteProveedor(id: string): Promise<ServiceResult<void>> {
	try {
		await proveedor.delete(id);
		return { ok: true, data: undefined };
	} catch (err: unknown) {
		console.error('Error al eliminar proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// ERROR PARSING
// ============================================

// ============================================
// SOFT DELETE OPERATIONS
// ============================================

/**
 * Restaurar proveedor eliminado
 */
export async function restoreProveedor(id: string): Promise<ServiceResult<ProveedorResponse>> {
	try {
		const result = await proveedor.restore(id);
		if (!result.ok || !result.data) {
			return { ok: false, error: result.error || 'No se pudo restaurar el proveedor.' };
		}
		return { ok: true, data: result.data };
	} catch (err: unknown) {
		console.error('Error al restaurar proveedor:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener proveedores archivados (eliminados)
 */
export async function getArchivedProveedores(): Promise<ServiceResult<ProveedorResponse[]>> {
	try {
		const data = await proveedor.listArchived();
		return { ok: true, data };
	} catch (err: unknown) {
		console.error('Error al cargar proveedores archivados:', JSON.stringify(err));
		return { ok: false, error: parseError(err) };
	}
}

function parseError(err: unknown): string {
	if (!err) return 'Ocurrió un error desconocido.';

	if (typeof err === 'string') {
		if (/unique|cedula/i.test(err)) return 'Ya existe un proveedor con esa cédula.';
		if (/empresa/i.test(err)) return 'La empresa especificada no existe.';
		return err;
	}

	if (err instanceof Error) {
		const msg = err.message;
		if (/unique|cedula/i.test(msg)) return 'Ya existe un proveedor con esa cédula.';
		if (/empresa/i.test(msg)) return 'La empresa especificada no existe.';
		return msg;
	}

	if (typeof err === 'object' && err !== null) {
		const obj = err as Record<string, any>;
		// Handle Rust Serde Enum errors
		if (obj.type) {
			const type = obj.type as string;
			switch (type) {
				case 'CedulaExists':
					return 'Ya existe un proveedor con esa cédula.';
				case 'EmpresaNotFound':
					return 'La empresa especificada no existe.';
				case 'NotFound':
					return 'Proveedor no encontrado.';
				case 'AlreadyInside':
					return 'El proveedor ya tiene un ingreso activo.';
			}
			if (obj.message) return obj.message as string;
		}

		const msg = (obj.message as string) ?? JSON.stringify(err);
		if (/unique|cedula/i.test(msg)) return 'Ya existe un proveedor con esa cédula.';
		if (/empresa/i.test(msg)) return 'La empresa especificada no existe.';
		return msg;
	}

	return 'Ocurrió un error inesperado.';
}
