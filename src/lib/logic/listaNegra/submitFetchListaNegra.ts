// src/lib/logic/listaNegra/submitFetchListaNegra.ts

import { fetchAllListaNegra, fetchActivosListaNegra, fetchListaNegraById } from './fetchListaNegra';
import { parseListaNegraError } from './parseListaNegraErrors';
import type { ListaNegraListResponse, ListaNegraResponse } from '$lib/types/listaNegra';

export type SubmitFetchAllListaNegraResult =
  | { ok: true; data: ListaNegraListResponse }
  | { ok: false; error: string };

export type SubmitFetchActivosListaNegraResult =
  | { ok: true; bloqueados: ListaNegraResponse[] }
  | { ok: false; error: string };

export type SubmitFetchListaNegraByIdResult =
  | { ok: true; bloqueado: ListaNegraResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso de obtener todos los bloqueados
 */
export async function submitFetchAllListaNegra(): Promise<SubmitFetchAllListaNegraResult> {
  try {
    const data = await fetchAllListaNegra();
    return { ok: true, data };
  } catch (err: any) {
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Orquesta el proceso de obtener bloqueados activos
 */
export async function submitFetchActivosListaNegra(): Promise<SubmitFetchActivosListaNegraResult> {
  try {
    const bloqueados = await fetchActivosListaNegra();
    return { ok: true, bloqueados };
  } catch (err: any) {
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Orquesta el proceso de obtener un bloqueado por ID
 */
export async function submitFetchListaNegraById(
  id: string
): Promise<SubmitFetchListaNegraByIdResult> {
  try {
    const bloqueado = await fetchListaNegraById(id);
    return { ok: true, bloqueado };
  } catch (err: any) {
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}