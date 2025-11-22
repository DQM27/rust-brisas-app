// src/lib/logic/listaNegra/addToListaNegra.ts

import { listaNegra } from '$lib/api/listaNegra';
import type { ListaNegraResponse, AddToListaNegraInput } from '$lib/types/listaNegra';

/**
 * Wrapper simple del service listaNegra.add()
 * Lanza excepción si falla (no maneja errores)
 */
export async function addToListaNegra(input: AddToListaNegraInput): Promise<ListaNegraResponse> {
  return await listaNegra.add(input);
}