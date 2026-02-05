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
     * Updates the audio alert settings
     */
    updateAudioConfig: (alertSound: string) =>
        invoke<void>('update_audio_config', { alertSound }),

    /**
     * Toggles between custom and native sounds
     */
    setUseCustomSound: (useCustom: boolean) =>
        invoke<void>('set_use_custom_sound', { useCustom }),

    /**
     * Uploads and registers a custom alert sound file
     */
    uploadCustomSound: (filePath: string) =>
        invoke<string>('upload_custom_sound', { filePath }),

    /**
     * Updates the terminal configuration (name and location)
     */
    updateTerminalConfig: (nombre: string, ubicacion: string) =>
        invoke<void>('update_terminal_config', { nombre, ubicacion }),

    /**
     * Plays the current alert sound for testing
     */
    playAlertSound: (soundType?: string) => invoke<void>('play_alert_sound', { soundType })
};
