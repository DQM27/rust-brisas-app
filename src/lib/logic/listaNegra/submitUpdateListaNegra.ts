// src/lib/logic/listaNegra/submitUpdateListaNegra.ts

import { validateUpdateListaNegraInput } from './validateListaNegraInput';
import { updateListaNegra } from './updateListaNegra';
import { parseListaNegraError } from './parseListaNegraErrors';
import type { ListaNegraResponse, UpdateListaNegraInput } from '$lib/types/listaNegra';

export type SubmitUpdateListaNegraResult =
  | { ok: true; bloqueado: ListaNegraResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de actualizar lista negra:
 * 1. Valida input
 * 2. Llama al service
 * 3. Parsea errores si falló
 */
export async function submitUpdateListaNegra(
  id: string,
  input: UpdateListaNegraInput
): Promise<SubmitUpdateListaNegraResult> {
  // 1. Validar
  const validation = validateUpdateListaNegraInput(input);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar actualizar
  try {
    const bloqueado = await updateListaNegra(id, input);
    return { ok: true, bloqueado };
  } catch (err: any) {
    // 3. Parsear error
    const errorMessage = parseListaNegraError(err);
    return { ok: false, error: errorMessage };
  }
}