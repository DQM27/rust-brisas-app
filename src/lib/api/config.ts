import { invoke } from '@tauri-apps/api/core';

/**
 * API layer for Application Configuration
 */
export const configApi = {
    /**
     * Gets the full application configuration
     */
    getAppConfig: () => invoke<any>('get_app_config'),

    /**
     * Updates the terminal configuration (name and location)
     */
    updateTerminalConfig: (nombre: string, ubicacion: string) =>
        invoke<void>('update_terminal_config', { nombre, ubicacion })
};
