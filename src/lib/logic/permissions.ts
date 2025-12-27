
import type { UserResponse } from '$lib/types/user';
import { ROLE_ADMIN_ID, ROLE_SUPERVISOR_ID } from '$lib/types/role';

// ==========================================
// CAPABILITIES (ACTIONS)
// ==========================================

export type Action =
    | 'VIEW_ADMIN_DASHBOARD'
    | 'VIEW_USER_LIST'
    | 'CREATE_USER'
    | 'UPDATE_USER_PROFILE'    // Basic info: name, email, phone
    | 'UPDATE_USER_SENSITIVE'  // Sensitive info: role, status, active/inactive
    | 'CHANGE_USER_PASSWORD'   // Change own password
    | 'RESET_USER_PASSWORD'    // Admin reset password (optional future)
    | 'DELETE_USER'
    | 'MANAGE_BLACKLIST'       // Add/Remove from blacklist
    | 'VIEW_BLACKLIST_REASON'; // View sensitive reason column

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
            'VIEW_USER_LIST': 'users:view',
            'CREATE_USER': 'users:create',
            'UPDATE_USER_PROFILE': 'users:update', // Partial mapping
            'UPDATE_USER_SENSITIVE': 'users:update', // This separation is semantic in frontend
            'DELETE_USER': 'users:delete',
            'CHANGE_USER_PASSWORD': 'users:update', // Own password
            'RESET_USER_PASSWORD': 'users:update', // Admin reset
            'MANAGE_BLACKLIST': 'lista_negra:create', // Approximation
            'VIEW_BLACKLIST_REASON': 'lista_negra:view', // Approximation
            'VIEW_ADMIN_DASHBOARD': 'dashboard:view' // Not a real permission yet?
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
