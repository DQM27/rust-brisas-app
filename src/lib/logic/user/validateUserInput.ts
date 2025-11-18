// src/lib/logic/user/validateUserInput.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateUserInput(
  email: string,
  password: string,
  nombre: string,
  apellido: string
): ValidationResult {
  const e = (email || '').trim();
  const p = password || '';
  const n = (nombre || '').trim();
  const a = (apellido || '').trim();

  if (!e) return { ok: false, message: 'El email no puede estar vacío.' };
  if (!e.includes('@')) return { ok: false, message: 'Email inválido.' };
  if (e.length > 100) return { ok: false, message: 'El email no puede exceder 100 caracteres.' };

  if (!p) return { ok: false, message: 'La contraseña no puede estar vacía.' };
  if (p.length < 6) return { ok: false, message: 'La contraseña debe tener al menos 6 caracteres.' };
  if (p.length > 100) return { ok: false, message: 'La contraseña no puede exceder 100 caracteres.' };

  if (!n) return { ok: false, message: 'El nombre no puede estar vacío.' };
  if (n.length > 50) return { ok: false, message: 'El nombre no puede exceder 50 caracteres.' };

  if (!a) return { ok: false, message: 'El apellido no puede estar vacío.' };
  if (a.length > 50) return { ok: false, message: 'El apellido no puede exceder 50 caracteres.' };

  return { ok: true };
}
