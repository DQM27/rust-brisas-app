// ============================================
// src/lib/logic/proveedor/proveedorService.ts
// ============================================
// High-level business logic service for Proveedor

import { proveedor } from '$lib/api/proveedor';
import type { ProveedorResponse, CreateProveedorInput, UpdateProveedorInput } from '$lib/types/proveedor';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

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
    } catch (err: any) {
        console.error('Error al cargar proveedores:', err);
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
            const activos = result.data.filter(p =>
                p.estado?.toLowerCase() === 'activo'
            );
            return { ok: true, data: activos };
        }
        return result;
    } catch (err: any) {
        console.error('Error al cargar proveedores activos:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Buscar proveedores
 */
export async function searchProveedores(query: string): Promise<ServiceResult<ProveedorResponse[]>> {
    try {
        const data = await proveedor.search(query);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al buscar proveedores:', err);
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
    } catch (err: any) {
        console.error('Error al cargar proveedor:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener proveedor por cédula
 */
export async function fetchProveedorByCedula(cedula: string): Promise<ServiceResult<ProveedorResponse>> {
    try {
        const data = await proveedor.getByCedula(cedula);
        if (!data) {
            return { ok: false, error: 'Proveedor no encontrado' };
        }
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al cargar proveedor:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear nuevo proveedor
 */
export async function createProveedor(input: CreateProveedorInput): Promise<ServiceResult<ProveedorResponse>> {
    try {
        const data = await proveedor.create(input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al crear proveedor:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Actualizar proveedor
 */
export async function updateProveedor(id: string, input: UpdateProveedorInput): Promise<ServiceResult<ProveedorResponse>> {
    try {
        const data = await proveedor.update(id, input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al actualizar proveedor:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Cambiar estado de proveedor (Activo/Inactivo)
 */
export async function changeStatus(id: string, newStatus: string): Promise<ServiceResult<ProveedorResponse>> {
    try {
        const data = await proveedor.changeStatus(id, newStatus);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al cambiar estado:', err);
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
    } catch (err: any) {
        console.error('Error al eliminar proveedor:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    if (typeof err === 'string') {
        if (/unique|cedula/i.test(err)) return 'Ya existe un proveedor con esa cédula.';
        if (/empresa/i.test(err)) return 'La empresa especificada no existe.';
        return err;
    }

    if (typeof err === 'object') {
        const msg = err.message ?? err.toString();
        if (/unique|cedula/i.test(msg)) return 'Ya existe un proveedor con esa cédula.';
        if (/empresa/i.test(msg)) return 'La empresa especificada no existe.';
        return msg;
    }

    return 'Ocurrió un error inesperado.';
}
