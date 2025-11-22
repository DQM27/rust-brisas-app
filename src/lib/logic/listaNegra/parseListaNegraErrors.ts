// src/lib/logic/listaNegra/parseListaNegraErrors.ts

export function parseListaNegraError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido.';

  if (typeof err === 'string') {
    // Errores comunes
    if (/ya está en la lista negra/i.test(err)) {
      return 'Esta persona ya está bloqueada.';
    }
    if (/no existe/i.test(err)) {
      return 'El contratista especificado no existe.';
    }
    if (/no encontrado/i.test(err)) {
      return 'Registro no encontrado.';
    }
    if (/cédula/i.test(err) && /vacía/i.test(err)) {
      return 'La cédula no puede estar vacía.';
    }
    if (/motivo/i.test(err)) {
      return 'Debe especificar un motivo de bloqueo válido.';
    }
    return err;
  }

  if (typeof err === 'object' && err.message) {
    // Recursión para manejar objetos con message
    return parseListaNegraError(err.message);
  }

  return 'Ocurrió un error inesperado al procesar la solicitud.';
}