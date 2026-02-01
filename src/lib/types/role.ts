// ==========================================
// src/lib/types/role.ts
// ==========================================

// Respuesta de rol del backend
export interface RoleResponse {
	id: string;
	name: string;
	description: string | null;
	isSystem: boolean;
	permissions: string[]; // Lista de permission_ids
	createdAt: string;
	updatedAt: string;
}

export interface RoleListResponse {
	roles: RoleResponse[];
	total: number;
	systemRoles: number;
	customRoles: number;
}

export interface CreateRoleInput {
	name: string;
	description?: string;
	permissions: string[];
}

export interface UpdateRoleInput {
	name?: string;
	description?: string;
	permissions?: string[];
}

// Permiso disponible
export interface Permission {
	id: string; // 'users:create'
	module: string;
	action: string;
	description: string | null;
}

// Módulo visible para el usuario
export interface VisibleModule {
	module: string;
	displayName: string;
	canCreate: boolean;
	canRead: boolean;
	canUpdate: boolean;
	canDelete: boolean;
	canExport: boolean;
}

// Constantes de roles del sistema
// Constantes de roles del sistema (Coinciden con backend/domain/role.rs)
export const ROLE_ADMIN_ID = '019b93da-9d13-71ad-903f-ee0dae187f6b';
export const ROLE_GUARDIA_ID = '019b93da-9d17-751b-8e6a-0a09529a6982';
