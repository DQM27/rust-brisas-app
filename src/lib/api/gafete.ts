import { invoke } from "@tauri-apps/api/core";
import {
    CreateGafeteSchema,
    UpdateGafeteSchema,
    type GafeteResponse,
    type CreateGafeteInput,
    type UpdateGafeteInput,
    type GafeteListResponse
} from '$lib/types/gafete';

export const gafete = {
    /**
     * Crear un nuevo gafete
     */
    create: async (input: CreateGafeteInput): Promise<GafeteResponse> => {
        const validated = CreateGafeteSchema.parse(input);
        return await invoke('create_gafete', { input: validated });
    },

    /**
     * Obtener gafete por Número (ID)
     */
    get: async (numero: string): Promise<GafeteResponse> => {
        return await invoke('get_gafete', { numero });
    },

    /**
     * Obtener todos los gafetes
     */
    getAll: async (): Promise<GafeteListResponse> => {
        return await invoke('get_all_gafetes');
    },

    /**
     * Obtener gafetes disponibles (que no están en uso)
     */
    getDisponibles: async (): Promise<GafeteResponse[]> => {
        return await invoke('get_gafetes_disponibles');
    },

    /**
     * Verificar si un gafete está disponible
     */
    isDisponible: async (numero: string): Promise<boolean> => {
        return await invoke('is_gafete_disponible', { numero });
    },

    /**
     * Actualizar un gafete existente
     */
    update: async (numero: string, input: UpdateGafeteInput): Promise<GafeteResponse> => {
        const validated = UpdateGafeteSchema.parse(input);
        return await invoke('update_gafete', { numero, input: validated });
    },

    /**
     * Eliminar un gafete
     */
    delete: async (numero: string): Promise<void> => {
        return await invoke('delete_gafete', { numero });
    }
};
