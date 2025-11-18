// ==========================================
// parseAuthErrors.ts
// Normaliza los errores devueltos por invoke() de Tauri.
// Puede llegar como string o como objeto con { message, code }.
// ==========================================

export function parseAuthError(err: any): { message: string; code?: string } {
  // ==========================================
  // Sin error
  // ==========================================
  if (!err) {
    return { message: 'Ocurrió un error desconocido.' };
  }

  // ==========================================
  // Caso: objeto (Tauri suele mandar { message, code })
  // ==========================================
  if (typeof err === 'object') {
    const raw = typeof err.message === 'string' ? err.message : null;
    const code = typeof err.code === 'string' ? err.code : undefined;

    if (raw) {
      // ---- Errores comunes / autenticación ----
      if (/invalid|credencial|password/i.test(raw)) {
        return {
          message: 'Credenciales inválidas. Verifica tu email y contraseña.',
          code,
        };
      }

      if (/not found|no existe/i.test(raw)) {
        return {
          message: 'Usuario no encontrado.',
          code,
        };
      }

      if (/inactive|disabled|suspend/i.test(raw)) {
        return {
          message: 'El usuario está inactivo. Contacta al administrador.',
          code,
        };
      }

      // Sin coincidencias: mensaje tal cual
      return { message: raw, code };
    }

    // Objeto extraño → intentar serializar
    try {
      return { message: `Error del servidor: ${JSON.stringify(err)}` };
    } catch {
      return { message: 'Ocurrió un error inesperado.' };
    }
  }

  // ==========================================
  // Caso: string
  // ==========================================
  if (typeof err === 'string') {
    const raw = err;

    if (/invalid|credencial|password/i.test(raw)) {
      return {
        message: 'Credenciales inválidas. Verifica tu email y contraseña.',
      };
    }

    if (/not found|no existe/i.test(raw)) {
      return { message: 'Usuario no encontrado.' };
    }

    return { message: raw };
  }

  // ==========================================
  // Fallback
  // ==========================================
  return { message: 'Ocurrió un error inesperado.' };
}
