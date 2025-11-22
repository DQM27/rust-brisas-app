// src/lib/logic/listaNegra/removeFromListaNegra.ts

import { listaNegra } from '$lib/api/listaNegra';
import type { ListaNegraResponse } from '$lib/types/listaNegra';

/**
 * Wrapper simple del service listaNegra.remove()
 * @param id ID del registro de Lista Negra a desactivar (desbloquear).
 * @param motivo Motivo por el cual se realiza el desbloqueo.
 * @param observaciones Observaciones adicionales sobre el desbloqueo (opcional).
 * Lanza excepción si falla (no maneja errores)
 */
export async function removeFromListaNegra(
    id: string, 
    motivo: string, 
    observaciones?: string // Puede ser opcional, asumiendo que el tercer argumento lo es
): Promise<ListaNegraResponse> {
    // Pasar todos los argumentos requeridos a la función de la API
    return await listaNegra.remove(id, motivo, observaciones); 
}