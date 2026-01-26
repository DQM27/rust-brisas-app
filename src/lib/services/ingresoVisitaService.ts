import { invoke } from '@tauri-apps/api/core';
import type { IngresoVisita } from '$lib/types/ingreso-nuevos';

export interface CreateIngresoVisitaFullInput {
	cedula: string;
	preRegistroId?: string;
	nombre: string;
	apellido: string;
	empresaNombre?: string;
	empresaId?: string;
	segundoNombre?: string;
	segundoApellido?: string;
	anfitrion: string;
	areaVisitada: string;
	motivo: string;
	modoIngreso: string;
	placaVehiculo?: string;
	gafeteNumero?: number;
	observaciones?: string;
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
