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

  remove: async (id: string): Promise<ListaNegraResponse> => {
    return await invoke('remove_from_lista_negra', { id });
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