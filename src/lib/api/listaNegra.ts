// src/lib/api/listaNegra.ts
import { invoke } from "@tauri-apps/api/core";
import type { 
  ListaNegraResponse, 
  ListaNegraListResponse,
  BlockCheckResponse,
  AddToListaNegraInput,
  UpdateListaNegraInput
} from '$lib/types/listaNegra';

export const listaNegra = {
  add: async (input: AddToListaNegraInput): Promise<ListaNegraResponse> => {
    return await invoke('add_to_lista_negra', { input });
  },

  update: async (id: string, input: UpdateListaNegraInput): Promise<ListaNegraResponse> => {
    return await invoke('update_lista_negra', { id, input });
  },

  delete: async (id: string): Promise<void> => {
    return await invoke('delete_lista_negra', { id });
  },

  /**
   * Desbloquear (desactivar) una persona de la lista negra
   * Cambia isActive de true → false
   */
  remove: async (id: string, motivo: string, observacion?: string): Promise<ListaNegraResponse> => {
    return await invoke('remove_from_lista_negra', { id, motivo, observacion });
  },

  /**
   * Re-bloquear (reactivar) una persona previamente desbloqueada
   * Cambia isActive de false → true
   */
  reactivate: async (
    id: string, 
    motivoBloqueo: string, 
    observaciones?: string,
    bloqueadoPor: string = "usuario_actual"
  ): Promise<ListaNegraResponse> => {
    return await invoke('reactivate_lista_negra', { 
      id, 
      motivoBloqueo, 
      observaciones, 
      bloqueadoPor 
    });
  },

  getById: async (id: string): Promise<ListaNegraResponse> => {
    return await invoke('get_lista_negra_by_id', { id });
  },

  getAll: async (): Promise<ListaNegraListResponse> => {
    return await invoke('get_all_lista_negra');
  },

  getActivos: async (): Promise<ListaNegraResponse[]> => {
    return await invoke('get_lista_negra_activos');
  },

  checkIsBlocked: async (cedula: string): Promise<BlockCheckResponse> => {
    return await invoke('check_is_blocked', { cedula });
  },

  getByCedula: async (cedula: string): Promise<ListaNegraResponse | null> => {
    return await invoke('get_blocked_by_cedula', { cedula });
  },
};