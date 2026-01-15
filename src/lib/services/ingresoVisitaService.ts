import { invoke } from '@tauri-apps/api/core';
import type { IngresoVisita } from '$lib/types/ingreso-nuevos';

export interface CreateIngresoVisitaFullInput {
	// Datos Visitante
	cedula: string;
	nombre: string;
	apellido: string;
	empresa_nombre?: string;

	// Datos Ingreso
	anfitrion: string;
	area_visitada: string;
	motivo: string;
	gafete?: string;
	observaciones?: string;
	usuario_ingreso_id: string;

	// Opcional
	cita_id?: string;
}

export const ingresoVisitaService = {
	async createIngreso(input: CreateIngresoVisitaFullInput): Promise<IngresoVisita> {
		return await invoke<IngresoVisita>('crear_ingreso_visita', { input });
	},

	async validarIngreso(visitanteId: string): Promise<any> {
		return await invoke('validar_ingreso_visita', { visitanteId });
	},

	async getActivos(): Promise<IngresoVisita[]> {
		return await invoke<IngresoVisita[]>('get_ingresos_visita_activos');
	},

	/** Obtiene historial de visitas completadas */
	async getHistorial(): Promise<IngresoVisita[]> {
		return await invoke<IngresoVisita[]>('get_ingresos_visita_historial');
	},

	async registrarSalida(
		ingresoId: string,
		devolvioGafete: boolean,
		observaciones?: string
	): Promise<void> {
		return await invoke('registrar_salida_visita', {
			ingresoId,
			devolvioGafete,
			observaciones
		});
	}
};
