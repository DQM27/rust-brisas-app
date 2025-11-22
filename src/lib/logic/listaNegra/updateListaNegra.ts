// src/lib/logic/listaNegra/updateListaNegra.ts

import { listaNegra } from '$lib/api/listaNegra';
import type { ListaNegraResponse, UpdateListaNegraInput } from '$lib/types/listaNegra';

/**
 * Wrapper simple del service listaNegra.update()
 * Lanza excepción si falla (no maneja errores)
 */
export async function updateListaNegra(
  id: string,
  input: UpdateListaNegraInput
): Promise<ListaNegraResponse> {
  return await listaNegra.update(id, input);
}