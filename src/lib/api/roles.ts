// ==========================================
// src/lib/api/roles.ts
// ==========================================

import { invoke } from '@tauri-apps/api/core';
import type {
	RoleResponse,
	RoleListResponse,
	CreateRoleInput,
	UpdateRoleInput,
	Permission,
	VisibleModule
} from '$lib/types/role';

// ==========================================
// CONSULTAS
// ==========================================

export async function getAllRoles(): Promise<RoleListResponse> {
	return invoke<RoleListResponse>('get_all_roles');
}

export async function getRoleById(id: string): Promise<RoleResponse> {
	return invoke<RoleResponse>('get_role_by_id', { id });
}

export async function getAllPermissions(): Promise<Permission[]> {
	return invoke<Permission[]>('get_all_permissions');
}

export async function getVisibleModules(): Promise<VisibleModule[]> {
	return invoke<VisibleModule[]>('get_visible_modules');
}

// ==========================================
// MUTACIONES
// ==========================================

export async function createRole(input: CreateRoleInput): Promise<RoleResponse> {
	return invoke<RoleResponse>('create_role', { input });
}

export async function updateRole(id: string, input: UpdateRoleInput): Promise<RoleResponse> {
	return invoke<RoleResponse>('update_role', { id, input });
}

export async function deleteRole(id: string): Promise<void> {
	return invoke<void>('delete_role', { id });
}
