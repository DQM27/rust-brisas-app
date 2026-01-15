// src/lib/logic/empresa/empresaService.ts
import {
	crearEmpresa,
	actualizarEmpresa,
	eliminarEmpresa,
	fetchTodasEmpresas,
	fetchEmpresasActivas
} from '$lib/api/empresa';
import type {
	EmpresaResponse,
	EmpresaListResponse,
	CreateEmpresaInput,
	UpdateEmpresaInput
} from '$lib/types/empresa';

/** Parseo de errores simple */
function parseEmpresaError(err: any): string {
	if (!err) return 'Ocurrió un error desconocido.';
	if (typeof err === 'string') return err;
	if (typeof err === 'object' && err.message) return err.message;
	return 'Ocurrió un error inesperado.';
}

/** Validación del nombre */
function validateEmpresaInput(nombre: string): { ok: true } | { ok: false; message: string } {
	const n = (nombre || '').trim();
	if (!n) return { ok: false, message: 'El nombre de la empresa no puede estar vacío.' };
	if (n.length > 100)
		return { ok: false, message: 'El nombre de la empresa no puede exceder 100 caracteres.' };
	return { ok: true };
}

/** Wrappers al API */
async function createEmpresaService(input: CreateEmpresaInput): Promise<EmpresaResponse> {
	return await crearEmpresa(input);
}

async function updateEmpresaService(
	id: string,
	input: UpdateEmpresaInput
): Promise<EmpresaResponse> {
	return await actualizarEmpresa(id, input);
}

async function deleteEmpresaService(id: string): Promise<void> {
	return await eliminarEmpresa(id);
}

async function fetchAllEmpresas(): Promise<EmpresaListResponse> {
	return await fetchTodasEmpresas();
}

async function fetchActiveEmpresas(): Promise<EmpresaResponse[]> {
	return await fetchEmpresasActivas();
}

/** Submit / Orquestadores */
export type SubmitCreateEmpresaResult =
	| { ok: true; empresa: EmpresaResponse }
	| { ok: false; error: string };
export type SubmitUpdateEmpresaResult =
	| { ok: true; empresa: EmpresaResponse }
	| { ok: false; error: string };
export type SubmitDeleteEmpresaResult = { ok: true } | { ok: false; error: string };
export type SubmitFetchAllEmpresasResult =
	| { ok: true; empresas: EmpresaListResponse }
	| { ok: false; error: string };
export type SubmitFetchActiveEmpresasResult =
	| { ok: true; empresas: EmpresaResponse[] }
	| { ok: false; error: string };

export async function submitCreateEmpresa(nombre: string): Promise<SubmitCreateEmpresaResult> {
	const validation = validateEmpresaInput(nombre);
	if (!validation.ok) return { ok: false, error: validation.message };

	try {
		const empresa = await createEmpresaService({ nombre });
		return { ok: true, empresa };
	} catch (err: any) {
		return { ok: false, error: parseEmpresaError(err) };
	}
}

export async function submitUpdateEmpresa(
	id: string,
	nombre: string
): Promise<SubmitUpdateEmpresaResult> {
	const validation = validateEmpresaInput(nombre);
	if (!validation.ok) return { ok: false, error: validation.message };

	try {
		const empresa = await updateEmpresaService(id, { nombre });
		return { ok: true, empresa };
	} catch (err: any) {
		return { ok: false, error: parseEmpresaError(err) };
	}
}

export async function submitDeleteEmpresa(id: string): Promise<SubmitDeleteEmpresaResult> {
	try {
		await deleteEmpresaService(id);
		return { ok: true };
	} catch (err: any) {
		return { ok: false, error: parseEmpresaError(err) };
	}
}

export async function submitFetchAllEmpresas(): Promise<SubmitFetchAllEmpresasResult> {
	try {
		const empresas = await fetchAllEmpresas();
		return { ok: true, empresas };
	} catch (err: any) {
		return { ok: false, error: parseEmpresaError(err) };
	}
}

export async function submitFetchActiveEmpresas(): Promise<SubmitFetchActiveEmpresasResult> {
	try {
		const empresas = await fetchActiveEmpresas();
		return { ok: true, empresas };
	} catch (err: any) {
		return { ok: false, error: parseEmpresaError(err) };
	}
}
