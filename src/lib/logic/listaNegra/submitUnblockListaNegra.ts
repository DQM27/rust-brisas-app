import { listaNegra } from '$lib/api/listaNegra';

/**
 * Lógica para desbloquear.
 * Recibe el motivo de desbloqueo y lo envía como el nuevo "motivo" 
 * para que el backend actualice el registro.
 */
export async function submitUnblockListaNegra(
  id: string,
  motivoDesbloqueo: string,
  observaciones?: string
) {
  try {
    // 1. Validaciones
    if (!id) {
      return { ok: false, error: 'El ID del registro es inválido.' };
    }

    if (!motivoDesbloqueo || motivoDesbloqueo.trim().length === 0) {
      return { ok: false, error: 'El motivo del desbloqueo es obligatorio.' };
    }

    // 2. Llamada a la API actualizada
    // Pasamos 'motivoDesbloqueo' como el segundo argumento, que el back usará como 'motivo'
    await listaNegra.remove(id, motivoDesbloqueo, observaciones);

    // 3. Retorno exitoso
    return { ok: true };

  } catch (e) {
    console.error('Error en submitUnblockListaNegra:', e);
    
    const errorMessage = e instanceof Error ? e.message : String(e);

    return { 
      ok: false, 
      error: `No se pudo desbloquear: ${errorMessage}` 
    };
  }
}