import { invoke } from '@tauri-apps/api/core';
import type { PreRegistroVisita, CreatePreRegistroInput } from '$lib/types/ingreso-nuevos';

const COMMAND_PREFIX = 'plugin:brisas|'; // Asumiendo estructura estándar, o sin prefijo si están en root handlers

export const preRegistroVisitaService = {
	async create(input: CreatePreRegistroInput): Promise<PreRegistroVisita> {
		return await invoke('create_pre_registro_visita', { input });
	},

	async update(id: string, input: CreatePreRegistroInput): Promise<PreRegistroVisita> {
		return await invoke('update_pre_registro_visita', { id, input });
	},

	async getPendientes(): Promise<PreRegistroVisita[]> {
		return await invoke('get_pre_registros_pendientes');
	},

	async checkByCedula(cedula: string): Promise<PreRegistroVisita | null> {
		return await invoke('check_pre_registro_by_cedula', { cedula });
	},

	async cancel(id: string): Promise<PreRegistroVisita> {
		return await invoke('cancel_pre_registro_visita', { id });
	}
};
