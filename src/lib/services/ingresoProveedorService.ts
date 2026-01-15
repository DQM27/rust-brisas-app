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

	async getHistorial(): Promise<IngresoProveedor[]> {
		return await invoke<IngresoProveedor[]>('get_ingresos_proveedores_historial');
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
