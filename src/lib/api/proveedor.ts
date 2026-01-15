// ==========================================
// src/lib/api/proveedor.ts
// ==========================================
// Low-level Tauri command wrappers for Proveedor

import { invoke } from '@tauri-apps/api/core';
import type {
	ProveedorResponse,
	CreateProveedorInput,
	UpdateProveedorInput
} from '$lib/types/proveedor';

export const proveedor = {
	create: async (input: CreateProveedorInput): Promise<ProveedorResponse> => {
		return await invoke<ProveedorResponse>('create_proveedor', { input });
	},

	search: async (query: string): Promise<ProveedorResponse[]> => {
		return await invoke<ProveedorResponse[]>('search_proveedores_catalog', { query });
	},

	getAll: async (): Promise<ProveedorResponse[]> => {
		return await invoke<ProveedorResponse[]>('search_proveedores_catalog', { query: '' });
	},

	getById: async (id: string): Promise<ProveedorResponse | null> => {
		return await invoke<ProveedorResponse | null>('get_proveedor_by_id', { id });
	},

	getByCedula: async (cedula: string): Promise<ProveedorResponse | null> => {
		return await invoke<ProveedorResponse | null>('get_proveedor_by_cedula', { cedula });
	},

	changeStatus: async (id: string, newStatus: string): Promise<ProveedorResponse> => {
		return await invoke<ProveedorResponse>('change_proveedor_status', { id, newStatus });
	},

	update: async (id: string, input: UpdateProveedorInput): Promise<ProveedorResponse> => {
		return await invoke<ProveedorResponse>('update_proveedor', { id, input });
	},

	delete: async (id: string): Promise<void> => {
		await invoke('delete_proveedor', { id });
	},

	listArchived: async (): Promise<ProveedorResponse[]> => {
		return await invoke<ProveedorResponse[]>('get_archived_proveedores');
	},
	// Alias to satisfy TrashService interface
	getArchived: async (): Promise<{ ok: boolean; data: ProveedorResponse[]; error?: string }> => {
		try {
			const data = await invoke<ProveedorResponse[]>('get_archived_proveedores');
			return { ok: true, data };
		} catch (e: unknown) {
			const errMsg = e instanceof Error ? e.message : String(e);
			return { ok: false, data: [], error: errMsg };
		}
	},
	restore: async (
		id: string
	): Promise<{ ok: boolean; data?: ProveedorResponse; error?: string }> => {
		try {
			const data = await invoke<ProveedorResponse>('restore_proveedor', { id });
			return { ok: true, data };
		} catch (e: unknown) {
			const errMsg = e instanceof Error ? e.message : String(e);
			return { ok: false, error: errMsg };
		}
	}
};
