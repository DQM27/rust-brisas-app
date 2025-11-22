// src/lib/logic/listaNegra/submitAddToListaNegra.ts

import { validateAddToListaNegraInput } from './validateListaNegraInput';
import { addToListaNegra } from './addToListaNegra';
import { parseListaNegraError } from './parseListaNegraErrors';
import type { ListaNegraResponse, AddToListaNegraInput } from '$lib/types/listaNegra';

export type SubmitAddToListaNegraResult =
  | { ok: true; bloqueado: ListaNegraResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de agregar a lista negra:
 * 1. Valida input
 * 2. Llama al service
 * 3. Parsea errores si falló
 */
export async function submitAddToListaNegra(
  input: AddToListaNegraInput
): Promise<SubmitAddToListaNegraResult> {
  // 1. Validar
  const validation = validateAddToListaNegraInput(input);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar agregar
  try {
    const bloqueado = await addToListaNegra(input);
    return { ok: true, bloqueado };
  } catch (err: any) {
    // 3. Parsear error
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}