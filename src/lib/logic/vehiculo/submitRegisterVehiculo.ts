// src/lib/logic/vehiculo/submitRegisterVehiculo.ts

import type {
	VehiculoResponse,
	CreateVehiculoInput,
	UpdateVehiculoInput,
	VehiculoListResponse
} from '$lib/types/vehiculo';
import { vehiculos } from '$lib/api/vehiculos';

export type SubmitVehiculoResult =
	| { ok: true; vehiculo: VehiculoResponse }
	| { ok: false; error: string };

export function parseVehiculoError(err: unknown): string {
	if (!err) return 'Ocurrió un error desconocido.';
	if (typeof err === 'string') return err;
	if (err instanceof Error) return err.message;
	if (typeof err === 'object' && err !== null && 'message' in err) {
		return String((err as { message: unknown }).message);
	}
	return 'Ocurrió un error inesperado.';
}

/**
 * Crea un vehículo
 */
export async function registerVehiculo(input: CreateVehiculoInput): Promise<VehiculoResponse> {
	return await vehiculos.create(input);
}

/**
 * Orquesta el registro de vehículo y parsea errores
 */
export async function submitRegisterVehiculo(
	input: CreateVehiculoInput
): Promise<SubmitVehiculoResult> {
	try {
		const vehiculo = await registerVehiculo(input);
		return { ok: true, vehiculo };
	} catch (err: unknown) {
		const errorMessage = parseVehiculoError(err);
		return { ok: false, error: errorMessage };
	}
}

/**
 * Actualiza un vehículo
 */
export async function submitUpdateVehiculo(
	id: string,
	input: UpdateVehiculoInput
): Promise<SubmitVehiculoResult> {
	try {
		const vehiculo = await vehiculos.update(id, input);
		return { ok: true, vehiculo };
	} catch (err: unknown) {
		const errorMessage = parseVehiculoError(err);
		return { ok: false, error: errorMessage };
	}
}

/**
 * Elimina un vehículo
 */
export async function submitDeleteVehiculo(id: string): Promise<SubmitVehiculoResult> {
	try {
		await vehiculos.delete(id);
		return { ok: true, vehiculo: {} as VehiculoResponse };
	} catch (err: unknown) {
		const errorMessage = parseVehiculoError(err);
		return { ok: false, error: errorMessage };
	}
}

/**
 * Obtener todos los vehículos
 */
export async function fetchAllVehiculos(): Promise<VehiculoListResponse> {
	return await vehiculos.getAll();
}

/**
 * Obtener vehículos activos
 */
export async function fetchVehiculosActivos(): Promise<VehiculoResponse[]> {
	return await vehiculos.getActivos();
}

/**
 * Obtener vehículos por contratista
 */
export async function fetchVehiculosByContratista(
	contratistaId: string
): Promise<VehiculoResponse[]> {
	return await vehiculos.getByPropietario(contratistaId);
}

/**
 * Obtener vehículo por ID
 */
export async function fetchVehiculoById(id: string): Promise<VehiculoResponse> {
	return await vehiculos.getById(id);
}
