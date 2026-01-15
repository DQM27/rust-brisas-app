// ============================================
// src/lib/logic/role/roleService.ts
// ============================================
// Servicio para gestión de roles

import * as rolesApi from '$lib/api/roles';
import type {
	RoleResponse,
	RoleListResponse,
	CreateRoleInput,
	UpdateRoleInput,
	Permission,
	VisibleModule
} from '$lib/types/role';

// ============================================
// TYPES FOR RESULTS
// ============================================

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

// ============================================
// PUBLIC API - FETCH OPERATIONS
// ============================================

/**
 * Obtener todos los roles
 */
export async function fetchAllRoles(): Promise<ServiceResult<RoleListResponse>> {
	try {
		const result = await rolesApi.getAllRoles();
		return { ok: true, data: result };
	} catch (err: any) {
		console.error('Error al cargar roles:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener un rol por ID
 */
export async function fetchRoleById(id: string): Promise<ServiceResult<RoleResponse>> {
	try {
		const role = await rolesApi.getRoleById(id);
		return { ok: true, data: role };
	} catch (err: any) {
		console.error('Error al cargar rol:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener todos los permisos disponibles
 */
export async function fetchAllPermissions(): Promise<ServiceResult<Permission[]>> {
	try {
		const permissions = await rolesApi.getAllPermissions();
		return { ok: true, data: permissions };
	} catch (err: any) {
		console.error('Error al cargar permisos:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Obtener módulos visibles para el usuario actual
 */
export async function fetchVisibleModules(): Promise<ServiceResult<VisibleModule[]>> {
	try {
		const modules = await rolesApi.getVisibleModules();
		return { ok: true, data: modules };
	} catch (err: any) {
		console.error('Error al cargar módulos visibles:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// PUBLIC API - CRUD OPERATIONS
// ============================================

/**
 * Crear nuevo rol
 */
export async function createRole(input: CreateRoleInput): Promise<ServiceResult<RoleResponse>> {
	try {
		const role = await rolesApi.createRole(input);
		return { ok: true, data: role };
	} catch (err: any) {
		console.error('Error al crear rol:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Actualizar rol
 */
export async function updateRole(
	id: string,
	input: UpdateRoleInput
): Promise<ServiceResult<RoleResponse>> {
	try {
		const role = await rolesApi.updateRole(id, input);
		return { ok: true, data: role };
	} catch (err: any) {
		console.error('Error al actualizar rol:', err);
		return { ok: false, error: parseError(err) };
	}
}

/**
 * Eliminar rol
 */
export async function deleteRole(id: string): Promise<ServiceResult<void>> {
	try {
		await rolesApi.deleteRole(id);
		return { ok: true, data: undefined };
	} catch (err: any) {
		console.error('Error al eliminar rol:', err);
		return { ok: false, error: parseError(err) };
	}
}

// ============================================
// ERROR PARSING
// ============================================

function parseError(err: any): string {
	if (!err) return 'Ocurrió un error desconocido.';

	if (typeof err === 'string') {
		if (/CannotDeleteSystemRole/i.test(err)) return 'No se puede eliminar un rol del sistema.';
		if (/CannotModifySystemRole/i.test(err))
			return 'Solo el superusuario puede modificar roles del sistema.';
		if (/NameExists/i.test(err)) return 'Ya existe un rol con ese nombre.';
		if (/NotFound/i.test(err)) return 'Rol no encontrado.';
		if (/SessionRequired/i.test(err)) return 'Sesión requerida.';
		return err;
	}

	if (typeof err === 'object') {
		// Si tiene message, usarlo
		if (err.message) return err.message;

		// Si es un error de Tauri/Backend serializado
		// Intentar stringify para ver qué tiene dentro
		try {
			return JSON.stringify(err);
		} catch {
			return err.toString();
		}
	}

	return 'Ocurrió un error inesperado.';
}
