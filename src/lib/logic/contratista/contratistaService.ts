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

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

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
    } catch (err: any) {
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
        const activos = result.contratistas.filter(c => c.estado === 'activo');
        return { ok: true, data: activos };
    } catch (err: any) {
        console.error('Error al cargar contratistas activos:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener un contratista por ID
 */
export async function fetchContratistaById(id: string): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await contratistas.getById(id);
        return { ok: true, data: contratista };
    } catch (err: any) {
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
export async function createContratista(input: CreateContratistaInput): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await contratistas.create(input);
        return { ok: true, data: contratista };
    } catch (err: any) {
        console.error('Error al crear contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Actualizar contratista existente
 */
export async function updateContratista(id: string, input: UpdateContratistaInput): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await contratistas.update(id, input);
        return { ok: true, data: contratista };
    } catch (err: any) {
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
    } catch (err: any) {
        console.error('Error al eliminar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Cambiar estado de contratista
 */
export async function changeEstado(id: string, nuevoEstado: EstadoContratista): Promise<ServiceResult<ContratistaResponse>> {
    try {
        const contratista = await contratistas.changeEstado(id, nuevoEstado);
        return { ok: true, data: contratista };
    } catch (err: any) {
        console.error('Error al cambiar estado:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    if (typeof err === 'string') {
        if (/unique|cedula|duplicat/i.test(err)) return 'Ya existe un contratista con esa cédula.';
        if (/empresa/i.test(err)) return 'La empresa seleccionada no es válida.';
        if (/praind|vencimiento/i.test(err)) return 'Fecha de vencimiento PRAIND inválida.';
        return err;
    }

    if (typeof err === 'object') {
        const msg = err.message ?? err.toString();
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
    } catch (err: any) {
        console.error('Error al restaurar contratista:', err);
        return { ok: false, error: parseError(err) };
    }
}

export async function getArchivedContratistas(): Promise<ServiceResult<ContratistaResponse[]>> {
    try {
        const result = await contratistas.listArchived();
        return { ok: true, data: result };
    } catch (err: any) {
        console.error('Error al cargar contratistas archivados:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Reindexar búsqueda de contratistas
 */
export async function reindexContratistas(): Promise<ServiceResult<void>> {
    try {
        await contratistas.reindex();
        return { ok: true, data: undefined };
    } catch (err: any) {
        console.error('Error al reindexar contratistas:', err);
        return { ok: false, error: parseError(err) };
    }
}
