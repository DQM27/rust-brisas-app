import { invoke } from '@tauri-apps/api/core';
import type {
	EmpresaResponse,
	EmpresaListResponse,
	CreateEmpresaInput,
	UpdateEmpresaInput
} from '$lib/types/empresa';

// Obtener todas las empresas activas
export async function fetchEmpresasActivas(): Promise<EmpresaResponse[]> {
	return await invoke<EmpresaResponse[]>('get_empresas_activas');
}

// Obtener todas las empresas
export async function fetchTodasEmpresas(): Promise<EmpresaListResponse> {
	return await invoke<EmpresaListResponse>('get_all_empresas');
}

// Crear una nueva empresa
export async function crearEmpresa(input: CreateEmpresaInput): Promise<EmpresaResponse> {
	return await invoke<EmpresaResponse>('create_empresa', { input });
}

// Actualizar empresa existente
export async function actualizarEmpresa(
	id: string,
	input: UpdateEmpresaInput
): Promise<EmpresaResponse> {
	return await invoke<EmpresaResponse>('update_empresa', { id, input });
}

// Obtener empresa por ID
export async function fetchEmpresaPorId(id: string): Promise<EmpresaResponse> {
	return await invoke<EmpresaResponse>('get_empresa_by_id', { id });
}

// Eliminar empresa
export async function eliminarEmpresa(id: string): Promise<void> {
	await invoke('delete_empresa', { id });
}
