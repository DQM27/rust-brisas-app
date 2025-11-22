// src/lib/logic/listaNegra/removeFromListaNegra.ts

import { listaNegra } from '$lib/api/listaNegra';
import type { ListaNegraResponse } from '$lib/types/listaNegra';

/**
 * Wrapper simple del service listaNegra.remove()
 * Lanza excepción si falla (no maneja errores)
 */
export async function removeFromListaNegra(id: string): Promise<ListaNegraResponse> {
  return await listaNegra.remove(id);
}