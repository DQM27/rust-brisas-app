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
export const ROLE_ADMIN_ID = 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11';
export const ROLE_SUPERVISOR_ID = '75438848-185d-400e-953a-7a54a01c801e';
export const ROLE_GUARDIA_ID = '27221d6e-9818-430c-99c3-5694a971216b';
