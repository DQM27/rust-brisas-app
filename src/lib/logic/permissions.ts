
import type { UserResponse } from '$lib/types/user';
import { ROLE_ADMIN_ID, ROLE_SUPERVISOR_ID } from '$lib/types/role';

// ==========================================
// CAPABILITIES (ACTIONS)
// ==========================================

export type Action =
    // Core Actions
    | 'VIEW_ADMIN_DASHBOARD'

    // User Module
    | 'VIEW_USER_LIST'
    | 'CREATE_USER'
    | 'UPDATE_USER_PROFILE'
    | 'UPDATE_USER_SENSITIVE'
    | 'CHANGE_USER_PASSWORD'
    | 'RESET_USER_PASSWORD'
    | 'DELETE_USER'

    // Contractor Module
    | 'VIEW_CONTRACTOR_LIST'
    | 'CREATE_CONTRACTOR'
    | 'UPDATE_CONTRACTOR'
    | 'DELETE_CONTRACTOR'

    // Provider Module
    | 'VIEW_PROVIDER_LIST'
    | 'CREATE_PROVIDER'
    | 'UPDATE_PROVIDER'
    | 'DELETE_PROVIDER'

    // Visitor Module
    | 'VIEW_VISITOR_LIST'
    | 'CREATE_VISITOR'
    | 'UPDATE_VISITOR'
    | 'DELETE_VISITOR'

    // Vehicle Module
    | 'VIEW_VEHICLE_LIST'
    | 'CREATE_VEHICLE'
    | 'UPDATE_VEHICLE'
    | 'DELETE_VEHICLE'

    // Appointment (Citas) Module
    | 'VIEW_APPOINTMENT_LIST'
    | 'CREATE_APPOINTMENT'
    | 'UPDATE_APPOINTMENT'
    | 'DELETE_APPOINTMENT'

    // Entry (Ingresos) Module
    | 'VIEW_ENTRY_LIST'
    | 'CREATE_ENTRY'
    | 'UPDATE_ENTRY'

    // Gafete Module
    | 'VIEW_GAFETE_LIST'
    | 'CREATE_GAFETE'
    | 'UPDATE_GAFETE'
    | 'DELETE_GAFETE'

    // Settings Modules
    | 'VIEW_SETTINGS_GENERAL'
    | 'UPDATE_SETTINGS_GENERAL'
    | 'VIEW_SETTINGS_VISUAL'
    | 'UPDATE_SETTINGS_VISUAL'
    | 'VIEW_SETTINGS_SECURITY'
    | 'UPDATE_SETTINGS_SECURITY'
    | 'VIEW_SETTINGS_SESSIONS'
    | 'UPDATE_SETTINGS_SESSIONS'
    | 'VIEW_SETTINGS_BACKUP'
    | 'UPDATE_SETTINGS_BACKUP'

    // Role Module
    | 'VIEW_ROLE_LIST'
    | 'CREATE_ROLE'
    | 'UPDATE_ROLE'
    | 'DELETE_ROLE'

    // Blacklist
    | 'VIEW_BLACKLIST'
    | 'MANAGE_BLACKLIST'
    | 'VIEW_BLACKLIST_REASON';

// ==========================================
// HELPERS
// ==========================================

function isAdmin(user: UserResponse): boolean {
    return user.roleId === ROLE_ADMIN_ID;
}

function isSupervisor(user: UserResponse): boolean {
    return user.roleId === ROLE_SUPERVISOR_ID;
}

function isAdminOrSupervisor(user: UserResponse): boolean {
    return isAdmin(user) || isSupervisor(user);
}

// ==========================================
// LOGIC
// ==========================================

/**
 * Checks if an actor (user) can perform an action on a target resource.
 * 
 * @param actor The user attempting the action
 * @param action The action being attempted
 * @param target The resource being acted upon (optional)
 * @returns true if allowed, false otherwise
 */
export function can(actor: UserResponse | null | undefined, action: Action, target?: UserResponse | null): boolean {
    if (!actor) return false;

    // ADMIN OVERRIDE
    // Admin has full access to almost everything
    // Supervisor temporarily has same privileges as Admin
    // ADMIN OVERRIDE
    // Admin has full access to almost everything, EXCEPT changing other people's passwords directly (they use reset)
    if (isAdminOrSupervisor(actor) && action !== 'CHANGE_USER_PASSWORD') {
        // Special case: RESET_USER_PASSWORD is allowed for admins via this override (or explicit below)
        return true;
    }

    // CHECK BACKEND PERMISSIONS
    if (actor.permissions && Array.isArray(actor.permissions)) {
        if (actor.permissions.includes(action)) {
            return true;
        }
        // Try with module:action format if needed, but action provided here is usually unique enough or mapped.
        // The backend returns "module:action" strings usually.
        // Frontend Action type is like "VIEW_USER_LIST".
        // We need to map Frontend Action -> Backend Permission String?
        // OR, the backend returns permissions like "users:view".

        // Let's assume for now we need a mapping or the permissions are just strings.
        // Wait, the backend returns "users:view", "users:create", etc.
        // The frontend uses "VIEW_USER_LIST".

        // MAPPING:
        const permissionMap: Record<Action, string> = {
            // User
            'VIEW_USER_LIST': 'users:read',
            'CREATE_USER': 'users:create',
            'UPDATE_USER_PROFILE': 'users:update',
            'UPDATE_USER_SENSITIVE': 'users:update',
            'DELETE_USER': 'users:delete',
            'CHANGE_USER_PASSWORD': 'users:update',
            'RESET_USER_PASSWORD': 'users:update',

            // Contractor
            'VIEW_CONTRACTOR_LIST': 'contratistas:read',
            'CREATE_CONTRACTOR': 'contratistas:create',
            'UPDATE_CONTRACTOR': 'contratistas:update',
            'DELETE_CONTRACTOR': 'contratistas:delete',

            // Provider
            'VIEW_PROVIDER_LIST': 'proveedores:read',
            'CREATE_PROVIDER': 'proveedores:create',
            'UPDATE_PROVIDER': 'proveedores:update',
            'DELETE_PROVIDER': 'proveedores:delete',

            // Visitor
            'VIEW_VISITOR_LIST': 'visitantes:read',
            'CREATE_VISITOR': 'visitantes:create',
            'UPDATE_VISITOR': 'visitantes:update',
            'DELETE_VISITOR': 'visitantes:delete',

            // Vehicle
            'VIEW_VEHICLE_LIST': 'vehiculos:read',
            'CREATE_VEHICLE': 'vehiculos:create',
            'UPDATE_VEHICLE': 'vehiculos:update',
            'DELETE_VEHICLE': 'vehiculos:delete',

            // Appointment (Citas)
            'VIEW_APPOINTMENT_LIST': 'citas:read',
            'CREATE_APPOINTMENT': 'citas:create',
            'UPDATE_APPOINTMENT': 'citas:update',
            'DELETE_APPOINTMENT': 'citas:delete',

            // Entry (Ingresos)
            'VIEW_ENTRY_LIST': 'ingresos:read',
            'CREATE_ENTRY': 'ingresos:create',
            'UPDATE_ENTRY': 'ingresos:update',

            // Gafete
            'VIEW_GAFETE_LIST': 'gafetes:read',
            'CREATE_GAFETE': 'gafetes:create',
            'UPDATE_GAFETE': 'gafetes:update',
            'DELETE_GAFETE': 'gafetes:delete',

            // Settings
            'VIEW_SETTINGS_GENERAL': 'settings_general:view',
            'UPDATE_SETTINGS_GENERAL': 'settings_general:update',
            'VIEW_SETTINGS_VISUAL': 'settings_visual:view',
            'UPDATE_SETTINGS_VISUAL': 'settings_visual:update',
            'VIEW_SETTINGS_SECURITY': 'settings_security:view',
            'UPDATE_SETTINGS_SECURITY': 'settings_security:update',
            'VIEW_SETTINGS_SESSIONS': 'settings_sessions:view',
            'UPDATE_SETTINGS_SESSIONS': 'settings_sessions:update',
            'VIEW_SETTINGS_BACKUP': 'backup:view',
            'UPDATE_SETTINGS_BACKUP': 'backup:create', // Backup creation implies update/create

            // Roles
            'VIEW_ROLE_LIST': 'roles:read',
            'CREATE_ROLE': 'roles:create',
            'UPDATE_ROLE': 'roles:update',
            'DELETE_ROLE': 'roles:delete',

            // Blacklist
            'VIEW_BLACKLIST': 'lista_negra:read',
            'MANAGE_BLACKLIST': 'lista_negra:create',
            'VIEW_BLACKLIST_REASON': 'lista_negra:read',
            'VIEW_ADMIN_DASHBOARD': 'dashboard:view'
        };

        const backendPerm = permissionMap[action];
        if (backendPerm && actor.permissions.includes(backendPerm)) {
            return true;
        }
    }

    switch (action) {
        case 'DELETE_USER':
            // Only admins can do these (already handled by top check, but explicit here for clarity)
            return false;

        case 'UPDATE_USER_SENSITIVE':
            // Supervisors might eventually need this, but for now only Admin
            return false;

        case 'UPDATE_USER_PROFILE':
            // Users can edit their own profile
            if (target && actor.id === target.id) return true;
            // Supervisors/Guardias cannot edit others
            return false;

        case 'CHANGE_USER_PASSWORD':
            // Users can only change their OWN password
            if (target && actor.id === target.id) return true;
            return false;

        case 'RESET_USER_PASSWORD':
            // Only admins/supervisors can reset OTHER users' passwords
            // We already returned true for them at the top, so this is just fallback for safety
            return false;

        case 'MANAGE_BLACKLIST':
            // Admins/Supervisors true via override. Guards false.
            return false;

        case 'VIEW_BLACKLIST_REASON':
            // Admins/Supervisors true via override. Guards false.
            return false;

        default:
            return false;
    }
}

/**
 * Helper to check capability for UI rendering
 */
export type UserPermissions = {
    canEditBasic: boolean;
    canEditSensitive: boolean;
    canChangePassword: boolean;
    canResetPassword: boolean;
    canDelete: boolean;
};

export function getPermissionsForUser(actor: UserResponse | null, target: UserResponse): UserPermissions {
    return {
        canEditBasic: can(actor, 'UPDATE_USER_PROFILE', target),
        canEditSensitive: can(actor, 'UPDATE_USER_SENSITIVE', target),
        canChangePassword: can(actor, 'CHANGE_USER_PASSWORD', target),
        canResetPassword: can(actor, 'RESET_USER_PASSWORD', target),
        canDelete: can(actor, 'DELETE_USER', target)
    };
}
