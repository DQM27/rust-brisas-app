import { invoke } from '@tauri-apps/api/core';
import type { IngresoVisita } from '$lib/types/ingreso-nuevos';

export interface CreateIngresoVisitaFullInput {
	// Datos Visitante
	cedula: string;
	nombre: string;
	segundo_nombre?: string;
	apellido: string;
	segundo_apellido?: string;
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
	async getHistorial(range?: { start: string; end: string }): Promise<IngresoVisita[]> {
		if (!range) {
			const now = new Date();
			const startStr = new Date(now.getFullYear(), now.getMonth(), now.getDate()).toISOString();
			const endStr = new Date().toISOString();
			range = { start: startStr, end: endStr };
		}
		return await invoke<IngresoVisita[]>('get_ingresos_visita_historial', {
			fechaInicio: range.start, // Ojo: los nombres de argumentos deben coincidir con Rust
			fechaFin: range.end
		});
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
