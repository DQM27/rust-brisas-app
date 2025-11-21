// src/lib/logic/contratista/parseContratistaErrors.ts

export function parseContratistaError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido.';

  if (typeof err === 'string') {
    if (/unique/i.test(err)) return 'Ya existe un contratista con esa cédula.';
    return err;
  }

  if (typeof err === 'object' && err.message) {
    if (/unique/i.test(err.message)) return 'Ya existe un contratista con esa cédula.';
    return err.message;
  }

  return 'Ocurrió un error inesperado.';
}
