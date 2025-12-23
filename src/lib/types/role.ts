// ==========================================
// src/lib/types/role.ts
// ==========================================

// Respuesta de rol del backend
export interface RoleResponse {
    id: string;
    name: string;
    description: string | null;
    isSystem: boolean;
    permissions: string[];  // Lista de permission_ids
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
    id: string;      // 'users:create'
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
export const ROLE_ADMIN_ID = '11111111-1111-1111-1111-111111111111';
export const ROLE_SUPERVISOR_ID = '33333333-3333-3333-3333-333333333333';
export const ROLE_GUARDIA_ID = '22222222-2222-2222-2222-222222222222';
