/**
 * Servicio de Atajos Personalizados
 * 
 * Comunica con el backend de Rust para CRUD de atajos personalizados
 */

import { invoke } from '@tauri-apps/api/core';
import type { UserShortcutCustomization } from '$lib/shortcuts';

// ============================================
// TIPOS DE RESPUESTA
// ============================================

interface UserShortcutResponse {
    id: string;
    user_id: string;
    shortcut_id: string;
    custom_keys: string;
    enabled: boolean;
}

interface UserShortcutsListResponse {
    shortcuts: UserShortcutResponse[];
}

interface UserShortcutInput {
    shortcut_id: string;
    custom_keys: string;
    enabled: boolean;
}

// ============================================
// FUNCIONES DEL SERVICIO
// ============================================

/**
 * Obtiene todos los atajos personalizados del usuario actual
 */
export async function getUserShortcuts(): Promise<UserShortcutCustomization[]> {
    try {
        const response = await invoke<UserShortcutsListResponse>('get_user_shortcuts');
        return response.shortcuts.map(s => ({
            userId: s.user_id,
            shortcutId: s.shortcut_id,
            customKeys: s.custom_keys,
            enabled: s.enabled
        }));
    } catch (error) {
        console.error('[Shortcuts] Error al obtener atajos:', error);
        return [];
    }
}

/**
 * Guarda o actualiza un atajo personalizado
 */
export async function saveUserShortcut(
    shortcutId: string,
    customKeys: string,
    enabled: boolean = true
): Promise<boolean> {
    try {
        const input: UserShortcutInput = {
            shortcut_id: shortcutId,
            custom_keys: customKeys,
            enabled
        };
        await invoke('save_user_shortcut', { input });
        return true;
    } catch (error) {
        console.error('[Shortcuts] Error al guardar atajo:', error);
        return false;
    }
}

/**
 * Elimina un atajo personalizado (vuelve al default)
 */
export async function deleteUserShortcut(shortcutId: string): Promise<boolean> {
    try {
        await invoke('delete_user_shortcut', { shortcutId });
        return true;
    } catch (error) {
        console.error('[Shortcuts] Error al eliminar atajo:', error);
        return false;
    }
}

/**
 * Resetea todos los atajos a sus valores default
 */
export async function resetUserShortcuts(): Promise<boolean> {
    try {
        await invoke('reset_user_shortcuts');
        return true;
    } catch (error) {
        console.error('[Shortcuts] Error al resetear atajos:', error);
        return false;
    }
}
