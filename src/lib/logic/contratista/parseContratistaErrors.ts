// src/lib/logic/contratista/parseContratistaErrors.ts

export function parseContratistaError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido.';

  if (typeof err === 'string') {
    if (/unique|cedula/i.test(err)) return 'Ya existe un contratista con esa cédula.';
    return err;
  }

  if (typeof err === 'object') {
    const msg = err.message ?? err.toString();
    if (/unique|cedula/i.test(msg)) return 'Ya existe un contratista con esa cédula.';
    return msg;
  }

  return 'Ocurrió un error inesperado.';
}
