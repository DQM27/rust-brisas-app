// src/lib/logic/vehiculo/submitRegisterVehiculo.ts

import type { VehiculoResponse } from '$lib/types/vehiculo';
import { vehiculos } from '$lib/api/vehiculos'; // wrapper Tauri commands

export type SubmitVehiculoResult =
  | { ok: true; vehiculo: VehiculoResponse }
  | { ok: false; error: string };

export function parseVehiculoError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido.';
  if (typeof err === 'string') return err;
  if (typeof err === 'object' && err.message) return err.message;
  return 'Ocurrió un error inesperado.';
}

/**
 * Crea un vehículo
 */
export async function registerVehiculo(input: {
  contratista_id: string;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
}): Promise<VehiculoResponse> {
  return await vehiculos.create(input);
}

/**
 * Orquesta el registro de vehículo y parsea errores
 */
export async function submitRegisterVehiculo(input: {
  contratista_id: string;
  placa: string;
  marca?: string;
  modelo?: string;
  color?: string;
}): Promise<SubmitVehiculoResult> {
  try {
    const vehiculo = await registerVehiculo(input);
    return { ok: true, vehiculo };
  } catch (err: any) {
    const errorMessage = parseVehiculoError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Actualiza un vehículo
 */
export async function submitUpdateVehiculo(
  id: string,
  input: { marca?: string; modelo?: string; color?: string; is_active?: number }
): Promise<SubmitVehiculoResult> {
  try {
    const vehiculo = await vehiculos.update(id, input);
    return { ok: true, vehiculo };
  } catch (err: any) {
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
  } catch (err: any) {
    const errorMessage = parseVehiculoError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Obtener todos los vehículos
 */
export async function fetchAllVehiculos(): Promise<VehiculoResponse[]> {
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
export async function fetchVehiculosByContratista(contratista_id: string): Promise<VehiculoResponse[]> {
  return await vehiculos.getByContratista(contratista_id);
}

/**
 * Obtener vehículo por ID
 */
export async function fetchVehiculoById(id: string): Promise<VehiculoResponse> {
  return await vehiculos.getById(id);
}
