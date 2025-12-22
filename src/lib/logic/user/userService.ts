// ============================================
// src/lib/logic/user/userService.ts
// ============================================
// Servicio para gestión de usuarios

import { users } from '$lib/api/users';
import type { UserResponse, UpdateUserInput, CreateUserInput } from '$lib/types/user';

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
 * Obtener todos los usuarios
 */
export async function fetchAllUsers(): Promise<ServiceResult<UserResponse[]>> {
    try {
        const result = await users.list();
        return { ok: true, data: result.users };
    } catch (err: any) {
        console.error('Error al cargar usuarios:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener solo usuarios activos
 */
export async function fetchActiveUsers(): Promise<ServiceResult<UserResponse[]>> {
    try {
        const result = await fetchAllUsers();
        if (result.ok) {
            const activos = result.data.filter(u => u.isActive);
            return { ok: true, data: activos };
        }
        return result;
    } catch (err: any) {
        console.error('Error al cargar usuarios activos:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Obtener un usuario por ID
 */
export async function fetchUserById(id: string): Promise<ServiceResult<UserResponse>> {
    try {
        const user = await users.getById(id);
        return { ok: true, data: user };
    } catch (err: any) {
        console.error('Error al cargar usuario:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear nuevo usuario
 */
export async function createUser(input: CreateUserInput): Promise<ServiceResult<UserResponse>> {
    try {
        const user = await users.create(input);
        return { ok: true, data: user };
    } catch (err: any) {
        console.error('Error al crear usuario:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Cambiar estado de usuario (Activo/Inactivo)
 */
export async function changeStatus(id: string, isActive: boolean): Promise<ServiceResult<UserResponse>> {
    try {
        const input: Partial<UpdateUserInput> = {
            isActive: isActive
        };

        const user = await users.update(id, input as UpdateUserInput);
        return { ok: true, data: user };
    } catch (err: any) {
        console.error('Error al cambiar estado:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Actualizar usuario
 */
export async function updateUser(id: string, input: UpdateUserInput): Promise<ServiceResult<UserResponse>> {
    try {
        const user = await users.update(id, input);
        return { ok: true, data: user };
    } catch (err: any) {
        console.error('Error al actualizar usuario:', err);
        return { ok: false, error: parseError(err) };
    }
}

/**
 * Eliminar usuario
 */
export async function deleteUser(id: string): Promise<ServiceResult<void>> {
    try {
        await users.delete(id);
        return { ok: true, data: undefined };
    } catch (err: any) {
        console.error('Error al eliminar usuario:', err);
        return { ok: false, error: parseError(err) };
    }
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
    if (!err) return 'Ocurrió un error desconocido.';

    if (typeof err === 'string') {
        if (/unique|email/i.test(err)) return 'Ya existe un usuario con ese email.';
        return err;
    }

    if (typeof err === 'object') {
        const msg = err.message ?? err.toString();
        // Errores de SurrealDB o Tauri
        if (/unique|email/i.test(msg)) return 'Ya existe un usuario con ese email.';
        if (/failed/i.test(msg)) return 'Falló la operación en la base de datos.';
        return msg;
    }

    return 'Ocurrió un error inesperado.';
}
