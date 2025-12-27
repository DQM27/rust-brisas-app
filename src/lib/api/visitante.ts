
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

    restore: async (id: string): Promise<VisitanteResponse> => {
        return await invoke<VisitanteResponse>('restore_visitante', { id });
    },

    listArchived: async (): Promise<VisitanteResponse[]> => {
        return await invoke<VisitanteResponse[]>('get_archived_visitantes');
    },

    list: async (): Promise<VisitanteResponse[]> => {
        return await invoke<VisitanteResponse[]>('list_visitantes');
    }
};
