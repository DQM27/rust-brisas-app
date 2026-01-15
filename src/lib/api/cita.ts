import { invoke } from '@tauri-apps/api/core';
import type {
	Cita,
	CitaPopulated,
	CreateCitaInput,
	CreateVisitanteInput,
	Visitante
} from '$lib/types/cita';

export const cita = {
	create: async (cita: CreateCitaInput, visitante?: CreateVisitanteInput): Promise<Cita> => {
		return await invoke<Cita>('create_cita', { cita, visitante });
	},

	getHoy: async (): Promise<CitaPopulated[]> => {
		return await invoke<CitaPopulated[]>('get_citas_hoy');
	},

	procesarIngreso: async (citaId: string, gafete: string, usuarioId: string): Promise<string> => {
		return await invoke<string>('procesar_ingreso_cita', { citaId, gafete, usuarioId });
	},

	getVisitanteByCedula: async (cedula: string): Promise<Visitante | null> => {
		return await invoke<Visitante | null>('get_visitante_by_cedula', { cedula });
	}
};
