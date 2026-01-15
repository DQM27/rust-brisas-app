import { invoke } from '@tauri-apps/api/core';
import type { CitaPopulated, CreateCitaInput, CreateVisitanteInput, Cita } from '$lib/types/cita';

export const visitaService = {
	async createCita(cita: CreateCitaInput, visitante: CreateVisitanteInput | null): Promise<Cita> {
		return await invoke<Cita>('create_cita', { cita, visitante });
	},

	async getCitasHoy(): Promise<CitaPopulated[]> {
		return await invoke<CitaPopulated[]>('get_citas_hoy');
	},

	/** Obtiene todas las citas pendientes (hoy y futuras) */
	async getCitasPendientes(): Promise<CitaPopulated[]> {
		return await invoke<CitaPopulated[]>('get_citas_pendientes');
	},

	async procesarIngresoCita(citaId: string, gafete: string, usuarioId: string): Promise<string> {
		return await invoke<string>('procesar_ingreso_cita', {
			cita_id: citaId,
			gafete,
			usuario_id: usuarioId
		});
	},

	/** Cancela una cita pendiente */
	async cancelarCita(citaId: string): Promise<void> {
		return await invoke<void>('cancelar_cita', { id: citaId });
	},

	/** Actualiza los detalles de una cita pendiente */
	async actualizarCita(
		id: string,
		data: {
			fecha_cita: string;
			anfitrion: string;
			area_visitada: string;
			motivo?: string;
		}
	): Promise<void> {
		return await invoke<void>('update_cita', {
			id,
			fecha_cita: data.fecha_cita,
			anfitrion: data.anfitrion,
			area_visitada: data.area_visitada,
			motivo: data.motivo || null
		});
	}
};
