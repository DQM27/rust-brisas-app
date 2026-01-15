// src/lib/api/listaNegra.ts
import { invoke } from '@tauri-apps/api/core';
import type {
	ListaNegraResponse,
	ListaNegraListResponse,
	BlockCheckResponse,
	AddToListaNegraInput,
	UpdateListaNegraInput,
	PersonaSearchResult
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
	 * Desbloquear (desactivar) - soft delete, isActive: true → false
	 * Usa delete_from_lista_negra que hace soft delete
	 */
	remove: async (id: string): Promise<void> => {
		return await invoke('delete_from_lista_negra', { id });
	},

	/**
	 * Re-bloquear (reactivar) - isActive: false → true
	 * Usa restore_lista_negra para restaurar un registro desactivado
	 */
	reactivate: async (id: string): Promise<ListaNegraResponse> => {
		return await invoke('restore_lista_negra', { id });
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
	 * Verificar si cédula está bloqueada (para guardias - sin motivo)
	 */
	checkIsBlocked: async (cedula: string): Promise<BlockCheckResponse> => {
		return await invoke('check_is_blocked', { cedula });
	},

	/**
	 * Buscar personas para el formulario de bloqueo
	 * Busca en contratistas, proveedores y visitantes
	 */
	searchPersonas: async (query: string): Promise<PersonaSearchResult[]> => {
		return await invoke('search_personas_for_block', { query });
	}
};
