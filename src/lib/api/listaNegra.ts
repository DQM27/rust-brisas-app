// src/lib/api/listaNegra.ts
import { invoke } from "@tauri-apps/api/core";
import type {
  ListaNegraResponse,
  ListaNegraListResponse,
  BlockCheckResponse,
  AddToListaNegraInput,
  UpdateListaNegraInput,
  PersonaSearchResult,
  NivelSeveridad
} from '$lib/types/listaNegra';

export const listaNegra = {
  /**
   * Agregar persona a lista negra
   */
  add: async (input: AddToListaNegraInput): Promise<ListaNegraResponse> => {
    return await invoke('add_to_lista_negra', { input });
  },

  /**
   * Actualizar información de bloqueo
   */
  update: async (id: string, input: UpdateListaNegraInput): Promise<ListaNegraResponse> => {
    return await invoke('update_lista_negra', { id, input });
  },

  /**
   * Eliminar registro permanentemente
   */
  delete: async (id: string): Promise<void> => {
    return await invoke('delete_lista_negra', { id });
  },

  /**
   * Desbloquear (desactivar) - isActive: true → false
   */
  remove: async (id: string): Promise<ListaNegraResponse> => {
    return await invoke('remove_from_lista_negra', { id });
  },

  /**
   * Re-bloquear (reactivar) - isActive: false → true
   */
  reactivate: async (
    id: string,
    nivelSeveridad: NivelSeveridad,
    motivoBloqueo: string,
    bloqueadoPor: string
  ): Promise<ListaNegraResponse> => {
    return await invoke('reactivate_lista_negra', {
      id,
      nivelSeveridad,
      motivoBloqueo,
      bloqueadoPor
    });
  },

  /**
   * Obtener por ID
   */
  getById: async (id: string): Promise<ListaNegraResponse> => {
    return await invoke('get_lista_negra_by_id', { id });
  },

  /**
   * Obtener todos (historial completo)
   */
  getAll: async (): Promise<ListaNegraListResponse> => {
    return await invoke('get_all_lista_negra');
  },

  /**
   * Obtener solo activos (bloqueados actualmente)
   */
  getActivos: async (): Promise<ListaNegraResponse[]> => {
    return await invoke('get_lista_negra_activos');
  },

  /**
   * Verificar si cédula está bloqueada (para guardias - sin motivo)
   */
  checkIsBlocked: async (cedula: string): Promise<BlockCheckResponse> => {
    return await invoke('check_is_blocked', { cedula });
  },

  /**
   * Obtener bloqueo por cédula (para admins - con motivo)
   */
  getByCedula: async (cedula: string): Promise<ListaNegraResponse | null> => {
    return await invoke('get_blocked_by_cedula', { cedula });
  },

  /**
   * Buscar personas para el formulario de bloqueo
   * Busca en contratistas, proveedores y visitantes
   */
  searchPersonas: async (query: string): Promise<PersonaSearchResult[]> => {
    return await invoke('search_personas_for_block', { query });
  },
};