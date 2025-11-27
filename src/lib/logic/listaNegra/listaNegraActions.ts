// src/lib/logic/listaNegra/listaNegraActions.ts
import { listaNegra } from "$lib/api/listaNegra";
import type { 
  ListaNegraResponse, 
  ListaNegraListResponse,
  AddToListaNegraInput 
} from "$lib/types/listaNegra";

/**
 * Cargar toda la lista negra
 */
export async function loadAllListaNegra(): Promise<{
  ok: boolean;
  data?: ListaNegraListResponse;
  error?: string;
}> {
  try {
    const data = await listaNegra.getAll();
    return { ok: true, data };
  } catch (err: any) {
    console.error("Error al cargar lista negra:", err);
    return { 
      ok: false, 
      error: err?.message || "Error al cargar lista negra" 
    };
  }
}

/**
 * Agregar persona a lista negra
 */
export async function addToListaNegraAction(
  input: AddToListaNegraInput
): Promise<{
  ok: boolean;
  data?: ListaNegraResponse;
  error?: string;
}> {
  try {
    const data = await listaNegra.add(input);
    return { ok: true, data };
  } catch (err: any) {
    console.error("Error al agregar a lista negra:", err);
    return { 
      ok: false, 
      error: err?.message || "Error al agregar a lista negra" 
    };
  }
}

/**
 * Desbloquear persona (cambiar isActive de true → false)
 */
export async function unblockListaNegraAction(
  id: string,
  motivo: string,
  observaciones?: string
): Promise<{
  ok: boolean;
  data?: ListaNegraResponse;
  error?: string;
}> {
  try {
    const data = await listaNegra.remove(id, motivo, observaciones);
    return { ok: true, data };
  } catch (err: any) {
    console.error("Error al desbloquear:", err);
    return { 
      ok: false, 
      error: err?.message || "Error al desbloquear" 
    };
  }
}

/**
 * Re-bloquear persona (cambiar isActive de false → true)
 */
export async function reblockListaNegraAction(
  id: string,
  motivoBloqueo: string,
  observaciones?: string,
  bloqueadoPor: string = "usuario_actual"
): Promise<{
  ok: boolean;
  data?: ListaNegraResponse;
  error?: string;
}> {
  try {
    const data = await listaNegra.reactivate(
      id, 
      motivoBloqueo, 
      observaciones, 
      bloqueadoPor
    );
    return { ok: true, data };
  } catch (err: any) {
    console.error("Error al re-bloquear:", err);
    return { 
      ok: false, 
      error: err?.message || "Error al re-bloquear" 
    };
  }
}