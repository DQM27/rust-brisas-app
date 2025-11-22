// src/lib/logic/contratista/validateContratistaInput.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateContratistaInput(
  nombre: string,
  apellido: string,
  cedula: string,
  empresaId: string,
  fechaVencimientoPraind: string
): ValidationResult {
  const n = (nombre || '').trim();
  const a = (apellido || '').trim();
  const c = (cedula || '').trim();
  const e = (empresaId || '').trim();
  const f = (fechaVencimientoPraind || '').trim();

  if (!n) return { ok: false, message: 'El nombre no puede estar vacío.' };
  if (n.length > 60) return { ok: false, message: 'Nombre demasiado largo.' };

  if (!a) return { ok: false, message: 'El apellido no puede estar vacío.' };
  if (a.length > 60) return { ok: false, message: 'Apellido demasiado largo.' };

  if (!c) return { ok: false, message: 'La cédula no puede estar vacía.' };
  if (c.length > 20) return { ok: false, message: 'La cédula es demasiado larga.' };

  if (!e) return { ok: false, message: 'Debe seleccionar una empresa.' };

  if (!f) return { ok: false, message: 'Debe ingresar la fecha de vencimiento.' };

  return { ok: true };
}
