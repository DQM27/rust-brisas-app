// src/lib/logic/contratista/submitFetchContratistas.ts

import { fetchActivosContratistas, fetchAllContratistas } from './fetchContratistas';
import { parseContratistaError } from './parseContratistaErrors';
import type { ContratistaResponse } from '$lib/types/contratista';

export type SubmitFetchActivosContratistasResult =
  | { ok: true; contratistas: ContratistaResponse[] }
  | { ok: false; error: string };

export type SubmitFetchAllContratistasResult =
  | { ok: true; contratistas: ContratistaResponse[] }
  | { ok: false; error: string };

/**
 * Orquesta el proceso de obtener contratistas activos
 */
export async function submitFetchActiveContratistas(): Promise<SubmitFetchActivosContratistasResult> {
  try {
    const contratistas = await fetchActivosContratistas();
    return { ok: true, contratistas };
  } catch (err: any) {
    const errorMessage = parseContratistaError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Orquesta el proceso de obtener todos los contratistas
 */
export async function submitFetchAllContratistas(): Promise<SubmitFetchAllContratistasResult> {
  try {
    const contratistas = await fetchAllContratistas();
    return { ok: true, contratistas };
  } catch (err: any) {
    const errorMessage = parseContratistaError(err);
    return { ok: false, error: errorMessage };
  }
}