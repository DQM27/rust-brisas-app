// ==========================================
// src/lib/logic/permissions.ts
// ==========================================
// Sistema de permisos simplificado.
// La fuente de verdad es el backend (user.permissions).
// El formato de permisos es "module:action" (ej: "users:create").

import type { UserResponse } from '$lib/types/user';
import { ROLE_ADMIN_ID } from '$lib/types/role';

// ==========================================
// PERMISOS (Formato Backend: module:action)
// ==========================================

/**
 * Tipo de permiso en formato backend "module:action".
 * Ejemplos: 'users:read', 'contratistas:create', 'ingresos:view'
 */
export type Permission = string;

// Alias para compatibilidad con código existente
export type Action = Permission;

// ==========================================
// HELPERS
// ==========================================

function isAdmin(user: UserResponse): boolean {
	return user.roleId === ROLE_ADMIN_ID || user.roleId === `role:${ROLE_ADMIN_ID}`;
}

function isSuperuser(user: UserResponse): boolean {
	return user.isSuperuser === true;
}

// ==========================================
// FUNCIÓN PRINCIPAL DE VERIFICACIÓN
// ==========================================

/**
 * Verifica si el usuario tiene un permiso específico.
 * 
 * La lógica es simple:
 * 1. Superusuario (God Mode) tiene acceso total
 * 2. Admin tiene acceso total (excepto cambiar contraseñas de otros)
 * 3. Verificar si el permiso está en user.permissions
 *
 * @param actor El usuario que intenta la acción
 * @param permission El permiso en formato "module:action" (ej: "users:create")
 * @param target Recurso objetivo (opcional, para verificaciones de "self-edit")
 * @returns true si tiene permiso, false en caso contrario
 */
export function can(
	actor: UserResponse | null | undefined,
	permission: Permission,
	target?: UserResponse | null
): boolean {
	if (!actor) return false;

	// 1. SUPERUSER OVERRIDE - God Mode tiene acceso total
	if (isSuperuser(actor)) {
		return true;
	}

	// 2. ADMIN OVERRIDE - Admin tiene acceso casi total
	// Excepción: No puede cambiar contraseñas de otros usuarios directamente
	if (isAdmin(actor) && permission !== 'users:change_password') {
		return true;
	}

	// 3. VERIFICAR SELF-EDIT (usuario editando su propio perfil)
	if (target && actor.id === target.id) {
		if (permission === 'users:update' || permission === 'users:change_password') {
			return true;
		}
	}

	// 4. VERIFICAR PERMISOS DEL BACKEND
	if (actor.permissions && Array.isArray(actor.permissions)) {
		if (actor.permissions.includes(permission)) {
			return true;
		}

		// Verificar acceso por vista del módulo
		// Si tiene "module:view" puede ver el módulo, pero necesita permisos específicos para acciones
		const [module] = permission.split(':');
		const hasModuleView = actor.permissions.includes(`${module}:view`);
		const hasModuleRead = actor.permissions.includes(`${module}:read`);

		// Si pide "module:view" o "module:read" y tiene cualquiera de los dos
		if (permission.endsWith(':view') && hasModuleRead) return true;
		if (permission.endsWith(':read') && hasModuleView) return true;
	}

	return false;
}

/**
 * Verifica si tiene cualquiera de varios permisos
 */
export function canAny(
	actor: UserResponse | null | undefined,
	permissions: Permission[]
): boolean {
	return permissions.some(p => can(actor, p));
}

/**
 * Verifica si tiene todos los permisos
 */
export function canAll(
	actor: UserResponse | null | undefined,
	permissions: Permission[]
): boolean {
	return permissions.every(p => can(actor, p));
}

// ==========================================
// HELPERS PARA UI
// ==========================================

export type UserPermissions = {
	canEditBasic: boolean;
	canEditSensitive: boolean;
	canChangePassword: boolean;
	canResetPassword: boolean;
	canDelete: boolean;
};

export function getPermissionsForUser(
	actor: UserResponse | null,
	target: UserResponse
): UserPermissions {
	return {
		canEditBasic: can(actor, 'users:update', target),
		canEditSensitive: can(actor, 'users:update'),
		canChangePassword: can(actor, 'users:update', target), // Self-edit
		canResetPassword: can(actor, 'users:update'), // Admin can reset others
		canDelete: can(actor, 'users:delete')
	};
}
