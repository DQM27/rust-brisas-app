import { invoke } from '@tauri-apps/api/core';
import type { ModuleStatus, ModuleStatusType } from '../types/modules';

/**
 * API layer for Module Management
 */
export const modulesApi = {
    /**
     * Gets all modules and their current statuses from the backend
     */
    fetchAll: () => invoke<ModuleStatus[]>('get_modules_status'),

    /**
     * Updates the status of a specific module
     */
    updateStatus: (key: string, status: ModuleStatusType) =>
        invoke<void>('update_module_status', { key, status })
};
