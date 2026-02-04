// src/lib/logic/system/windowService.ts
import { setWindowDecorations, setWindowSize } from '$lib/logic/keyring/keyringService';

/**
 * Servicio para centralizar la gestión de ventanas de Tauri
 * y asegurar que las transiciones entre estados (Setup, Login, App)
 * sean suaves y consistentes.
 */
export const windowService = {
    /**
     * Configura la ventana para el modo Launcher (Setup o Login)
     * Ajusta tamaño, quita decoraciones y centra.
     */
    async setLauncherMode(isSetup = false): Promise<void> {
        try {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            const appWindow = getCurrentWindow();

            // Unmaximize if needed
            if (await appWindow.isMaximized()) await appWindow.unmaximize();

            // Sizing based on mode
            const width = isSetup ? 500 : 450;
            const height = isSetup ? 550 : 500;

            await setWindowDecorations(false);
            await setWindowSize(width, height);
            await appWindow.center();

            console.log(`[Window] Set to Launcher Mode (Setup: ${isSetup})`);
        } catch (e) {
            console.error('[Window] Error setting launcher mode:', e);
        }
    },

    /**
     * Configura la ventana para el modo Aplicación Completa
     * Pone decoraciones, tamaño estándar y centra.
     */
    async setAppMode(): Promise<void> {
        try {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            const appWindow = getCurrentWindow();

            await setWindowDecorations(true);
            await setWindowSize(1200, 800);
            await appWindow.center();

            console.log('[Window] Set to App Mode');
        } catch (e) {
            console.error('[Window] Error setting app mode:', e);
        }
    },

    /**
     * Proxy para setear tamaño directamente si es necesario
     */
    async setSize(width: number, height: number): Promise<void> {
        const res = await setWindowSize(width, height);
        if (!res.ok) console.error('[Window] Error setting size:', res.error);
    }
};
