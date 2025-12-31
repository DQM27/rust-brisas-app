
import { invoke } from '@tauri-apps/api/core';
import type { VisitanteResponse, CreateVisitanteInput } from '$lib/types/visitante';

export const visitante = {
    create: async (input: CreateVisitanteInput): Promise<VisitanteResponse> => {
        return await invoke<VisitanteResponse>('create_visitante', { input });
    },

    search: async (query: string): Promise<VisitanteResponse[]> => {
        return await invoke<VisitanteResponse[]>('search_visitantes_catalog', { query });
    },

    getByCedula: async (cedula: string): Promise<VisitanteResponse | null> => {
        return await invoke<VisitanteResponse | null>('get_visitante_by_cedula', { cedula });
    },

    update: async (id: string, input: CreateVisitanteInput): Promise<VisitanteResponse> => {
        return await invoke<VisitanteResponse>('update_visitante', { id, input });
    },

    delete: async (id: string): Promise<void> => {
        await invoke('delete_visitante', { id });
    },

    listArchived: async (): Promise<VisitanteResponse[]> => {
        return await invoke<VisitanteResponse[]>('get_archived_visitantes');
    },
    // Alias to satisfy TrashService interface
    getArchived: async (): Promise<{ ok: boolean; data: VisitanteResponse[]; error?: string }> => {
        try {
            const data = await invoke<VisitanteResponse[]>('get_archived_visitantes');
            return { ok: true, data };
        } catch (e: any) {
            return { ok: false, data: [], error: e.message || String(e) };
        }
    },
    restore: async (id: string): Promise<{ ok: boolean; error?: string }> => {
        try {
            await invoke<VisitanteResponse>('restore_visitante', { id });
            return { ok: true };
        } catch (e: any) {
            return { ok: false, error: e.message || String(e) };
        }
    },
    restoreWithData: async (id: string): Promise<VisitanteResponse> => {
        return await invoke<VisitanteResponse>('restore_visitante', { id });
    },

    list: async (): Promise<VisitanteResponse[]> => {
        return await invoke<VisitanteResponse[]>('list_visitantes');
    }
};
