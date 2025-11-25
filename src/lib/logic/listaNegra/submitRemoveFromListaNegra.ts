// src/lib/logic/listaNegra/submitRemoveFromListaNegra.ts

import { removeFromListaNegra } from './removeFromListaNegra';
import { parseListaNegraError } from './parseListaNegraErrors';
import type { ListaNegraResponse } from '$lib/types/listaNegra';

export type SubmitRemoveFromListaNegraResult =
  | { ok: true; bloqueado: ListaNegraResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de remover de lista negra:
 * 1. Llama al service
 * 2. Parsea errores si falló
 */
export async function submitRemoveFromListaNegra(
  id: string,
  motivo: string,
  observaciones?: string
): Promise<SubmitRemoveFromListaNegraResult> {
  // Intentar remover
  try {
    const bloqueado = await removeFromListaNegra(id, motivo, observaciones);
    return { ok: true, bloqueado };
  } catch (err: any) {
    // Parsear error
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}