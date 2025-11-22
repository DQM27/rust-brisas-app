// src/lib/logic/listaNegra/fetchListaNegra.ts

import { listaNegra } from '$lib/api/listaNegra';
import type { ListaNegraListResponse, ListaNegraResponse } from '$lib/types/listaNegra';

/**
 * Wrapper simple del service listaNegra.getAll()
 * Lanza excepción si falla (no maneja errores)
 */
export async function fetchAllListaNegra(): Promise<ListaNegraListResponse> {
  return await listaNegra.getAll();
}

/**
 * Wrapper simple del service listaNegra.getActivos()
 * Lanza excepción si falla (no maneja errores)
 */
export async function fetchActivosListaNegra(): Promise<ListaNegraResponse[]> {
  return await listaNegra.getActivos();
}

/**
 * Wrapper simple del service listaNegra.getById()
 * Lanza excepción si falla (no maneja errores)
 */
export async function fetchListaNegraById(id: string): Promise<ListaNegraResponse> {
  return await listaNegra.getById(id);
}