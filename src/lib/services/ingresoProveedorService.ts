import { invoke } from '@tauri-apps/api/core';
import type { IngresoProveedor, CreateIngresoProveedorInput } from '$lib/types/ingreso-nuevos';

export const ingresoProveedorService = {
	async createIngreso(
		input: CreateIngresoProveedorInput,
		usuarioId: string
	): Promise<IngresoProveedor> {
		return await invoke<IngresoProveedor>('crear_ingreso_proveedor_v2', { input, usuarioId });
	},

	async getActivos(): Promise<IngresoProveedor[]> {
		return await invoke<IngresoProveedor[]>('get_ingresos_proveedores_activos');
	},

	async getHistorial(range?: {
		fechaInicio: string;
		fechaFin: string;
	}): Promise<IngresoProveedor[]> {
		if (!range) {
			// Fallback seguro si no se pasan fechas (por ejemplo: hoy)
			const today = new Date().toISOString().split('T')[0];
			range = {
				fechaInicio: `${today}T00:00:00`,
				fechaFin: `${today}T23:59:59`
			};
		}
		return await invoke<IngresoProveedor[]>('get_ingresos_proveedores_historial', {
			fechaInicio: range.fechaInicio,
			fechaFin: range.fechaFin
		});
	},

	async registrarSalida(
		id: string,
		usuarioId: string,
		observaciones?: string,
		devolvioGafete: boolean = true
	): Promise<void> {
		return await invoke('registrar_salida_proveedor', {
			id,
			usuarioId,
			observaciones,
			devolvioGafete
		});
	},

	async searchProveedores(
		query: string
	): Promise<import('$lib/types/ingreso-nuevos').ProveedorCatalogItem[]> {
		return await invoke('search_proveedores_catalog', { query });
	},

	async validarIngreso(
		proveedorId: string
	): Promise<import('$lib/types/ingreso-nuevos').ValidacionIngresoProveedorResponse> {
		return await invoke('validar_ingreso_proveedor', { proveedorId });
	}
};
