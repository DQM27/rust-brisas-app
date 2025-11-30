// ============================================
// src/lib/logic/listaNegra/listaNegraService.ts
// ============================================
// Servicio consolidado para lista negra - reemplaza 13 archivos fragmentados

import { listaNegra } from '$lib/api/listaNegra';
import type {
    ListaNegraResponse,
    ListaNegraListResponse,
    AddToListaNegraInput,
    UpdateListaNegraInput
} from '$lib/types/listaNegra';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> =
    | { ok: true; data: T }
    | { ok: false; error: string };

// ============================================
// VALIDATION
// ============================================

type ValidationResult =
    | { ok: true }
    | { ok: false; message: string };

function validateAddToListaNegraInput(input: {
    contratistaId?: string;
    cedula?: string;
    nombre?: string;
    apellido?: string;
    motivoBloqueo: string;
    fechaFinBloqueo?: string;
    bloqueadoPor: string;
    observaciones?: string;
}): ValidationResult {
    const { contratistaId, cedula, nombre, apellido, motivoBloqueo, bloqueadoPor } = input;

    if (contratistaId) {
        const cid = contratistaId.trim();
        if (!cid) {
            return { ok: false, message: 'Debe especificar un contratista.' };
        }
    } else {
        const c = (cedula || '').trim();
        const n = (nombre || '').trim();
        const a = (apellido || '').trim();

        if (!c) {
            return { ok: false, message: 'La cédula no puede estar vacía.' };
        }
        if (c.length < 7 || c.length > 20) {
            return { ok: false, message: 'La cédula debe tener entre 7 y 20 caracteres.' };
        }
        if (!/^[0-9-]+$/.test(c)) {
            return { ok: false, message: 'La cédula solo puede contener números y guiones.' };
        }

        if (!n) {
            return { ok: false, message: 'El nombre no puede estar vacío.' };
        }
        if (n.length > 50) {
            return { ok: false, message: 'El nombre no puede exceder 50 caracteres.' };
        }

        if (!a) {
            return { ok: false, message: 'El apellido no puede estar vacío.' };
        }
        if (a.length > 50) {
            return { ok: false, message: 'El apellido no puede exceder 50 caracteres.' };
        }
    }

    const m = motivoBloqueo.trim();
    if (!m) {
        return { ok: false, message: 'Debe especificar un motivo de bloqueo.' };
    }
    if (m.length > 500) {
        return { ok: false, message: 'El motivo no puede exceder 500 caracteres.' };
    }

    const b = bloqueadoPor.trim();
    if (!b) {
        return { ok: false, message: 'Debe especificar quién realizó el bloqueo.' };
    }
    if (b.length > 100) {
        return { ok: false, message: 'El nombre de quien bloqueó no puede exceder 100 caracteres.' };
    }

    return { ok: true };
}

function validateUpdateListaNegraInput(input: {
    motivoBloqueo?: string;
    fechaFinBloqueo?: string;
    observaciones?: string;
}): ValidationResult {
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

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    if (typeof err === 'string') {
        if (/ya está en la lista negra/i.test(err)) {
            return 'Esta persona ya está bloqueada.';
        }
        if (/no existe/i.test(err)) {
            return 'El contratista especificado no existe.';
        }
        if (/no encontrado/i.test(err)) {
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

    if (typeof err === 'object' && err.message) {
        return parseError(err.message);
    }

    return 'Ocurrió un error inesperado al procesar la solicitud.';
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Obtener todos los bloqueados
 */
export async function fetchAll(): Promise<ServiceResult<ListaNegraListResponse>> {
    try {
        const data = await listaNegra.getAll();
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al cargar lista negra:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener bloqueados activos
 */
export async function fetchActivos(): Promise<ServiceResult<ListaNegraResponse[]>> {
    try {
        const data = await listaNegra.getActivos();
        return { ok: true, data };
    } catch (err: any) {
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
    } catch (err: any) {
        console.error('Error al cargar bloqueado:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Agregar a lista negra (con validación)
 */
export async function add(input: AddToListaNegraInput): Promise<ServiceResult<ListaNegraResponse>> {
    // Validar
    const validation = validateAddToListaNegraInput(input);
    if (!validation.ok) {
        return { ok: false, error: validation.message };
    }

    // Ejecutar
    try {
        const data = await listaNegra.add(input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al agregar a lista negra:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Actualizar registro en lista negra (con validación)
 */
export async function update(
    id: string,
    input: UpdateListaNegraInput
): Promise<ServiceResult<ListaNegraResponse>> {
    // Validar
    const validation = validateUpdateListaNegraInput(input);
    if (!validation.ok) {
        return { ok: false, error: validation.message };
    }

    // Ejecutar
    try {
        const data = await listaNegra.update(id, input);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al actualizar lista negra:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Desbloquear (remover de lista negra)
 */
export async function unblock(
    id: string,
    motivoDesbloqueo: string,
    observaciones?: string
): Promise<ServiceResult<ListaNegraResponse>> {
    // Validaciones inline
    if (!id) {
        return { ok: false, error: 'El ID del registro es inválido.' };
    }

    if (!motivoDesbloqueo || motivoDesbloqueo.trim().length === 0) {
        return { ok: false, error: 'El motivo del desbloqueo es obligatorio.' };
    }

    // Ejecutar
    try {
        const data = await listaNegra.remove(id, motivoDesbloqueo, observaciones);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al desbloquear:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Re-bloquear persona (reactivar)
 */
export async function reblock(
    id: string,
    motivoBloqueo: string,
    observaciones?: string,
    bloqueadoPor: string = 'usuario_actual'
): Promise<ServiceResult<ListaNegraResponse>> {
    try {
        const data = await listaNegra.reactivate(id, motivoBloqueo, observaciones, bloqueadoPor);
        return { ok: true, data };
    } catch (err: any) {
        console.error('Error al re-bloquear:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// COMPATIBILITY LAYER (para transición suave)
// ============================================

// Aliases para compatibilidad con código existente
export const submitAddToListaNegra = add;
export const submitFetchAllListaNegra = fetchAll;
export const submitFetchActivosListaNegra = fetchActivos;
export const submitFetchListaNegraById = fetchById;
export const submitUpdateListaNegra = update;
export const submitUnblockListaNegra = unblock;

// Aliases para listaNegraActions.ts
export const addToListaNegraAction = add;
export const loadAllListaNegra = fetchAll;
export const unblockListaNegraAction = unblock;
export const reblockListaNegraAction = reblock;
