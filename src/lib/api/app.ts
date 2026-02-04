import { invoke } from '@tauri-apps/api/core';

/**
 * API layer for general Application commands
 */
export const appApi = {
    /**
     * Shows the main window once the frontend is ready
     */
    showMainWindow: () => invoke<void>('show_main_window'),

    /**
     * Gets the local IP address of the terminal
     */
    getLocalIp: () => invoke<string>('get_local_ip')
};
