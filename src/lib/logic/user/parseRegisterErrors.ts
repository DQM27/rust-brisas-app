// src/lib/logic/user/parseRegisterErrors.ts

export function parseRegisterError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido.';

  if (typeof err === 'string') {
    if (/unique/i.test(err)) return 'El email ya está registrado.';
    return err;
  }

  if (typeof err === 'object' && err.message) {
    if (/unique/i.test(err.message)) return 'El email ya está registrado.';
    return err.message;
  }

  return 'Ocurrió un error inesperado.';
}
