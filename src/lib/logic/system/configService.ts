import { configApi } from '$lib/api/config';

export type ServiceResult<T> = { ok: true; data: T } | { ok: false; error: string };

/**
 * Service to manage application configuration
 */
export const configService = {
    async getAppConfig(): Promise<ServiceResult<any>> {
        try {
            const config = await configApi.getAppConfig();
            return { ok: true, data: config };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    },

    async updateAudioConfig(alertSound: string): Promise<ServiceResult<void>> {
        try {
            await configApi.updateAudioConfig(alertSound);
            return { ok: true, data: undefined };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    },

    async setUseCustomSound(useCustom: boolean): Promise<ServiceResult<void>> {
        try {
            await configApi.setUseCustomSound(useCustom);
            return { ok: true, data: undefined };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    },

    async uploadCustomSound(filePath: string): Promise<ServiceResult<string>> {
        try {
            const newPath = await configApi.uploadCustomSound(filePath);
            return { ok: true, data: newPath };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    },

    async updateTerminalConfig(nombre: string, ubicacion: string): Promise<ServiceResult<void>> {
        try {
            await configApi.updateTerminalConfig(nombre, ubicacion);
            return { ok: true, data: undefined };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    },

    async playAlertSound(): Promise<ServiceResult<void>> {
        try {
            await configApi.playAlertSound();
            return { ok: true, data: undefined };
        } catch (e) {
            return { ok: false, error: String(e) };
        }
    }
};
