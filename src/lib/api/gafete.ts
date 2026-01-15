import { invoke } from '@tauri-apps/api/core';
import {
	CreateGafeteSchema,
	UpdateGafeteSchema,
	type GafeteResponse,
	type CreateGafeteInput,
	type UpdateGafeteInput,
	type GafeteListResponse
} from '$lib/types/gafete';

export const gafete = {
	create: async (input: CreateGafeteInput): Promise<GafeteResponse> => {
		// Validar con Zod (esto ya normaliza el número con .trim())
		const validated = CreateGafeteSchema.parse(input);
		return await invoke('create_gafete', { input: validated });
	},

	get: async (numero: string, tipo: string): Promise<GafeteResponse> => {
		return await invoke('get_gafete', { numero, tipo });
	},

	getAll: async (): Promise<GafeteListResponse> => {
		return await invoke('get_all_gafetes');
	},

	getDisponibles: async (tipo: string = ''): Promise<GafeteResponse[]> => {
		return await invoke('get_gafetes_disponibles', { tipo });
	},

	isDisponible: async (numero: string, tipo: string): Promise<boolean> => {
		return await invoke('is_gafete_disponible', { numero, tipo });
	},

	update: async (
		numero: string,
		tipo: string,
		input: UpdateGafeteInput
	): Promise<GafeteResponse> => {
		const validated = UpdateGafeteSchema.parse(input);
		return await invoke('update_gafete', { numero, tipo, input: validated });
	},

	createRange: async (
		input: import('$lib/types/gafete').CreateGafeteRangeInput
	): Promise<string[]> => {
		return await invoke('create_gafete_range', { input });
	},

	updateStatus: (id: string, estado: string, usuarioId?: string, motivo?: string) =>
		invoke<GafeteResponse>('update_gafete_status', {
			id,
			input: { estado },
			usuarioId,
			motivo
		}),

	delete: async (numero: string, tipo: string, usuarioId?: string): Promise<void> => {
		return await invoke('delete_gafete', { numero, tipo, usuarioId });
	}
};
