// src/lib/logic/contratista/validateContratistaInput.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateContratistaInput(
  nombre: string,
  apellido: string,
  cedula: string,
  empresaId: string,
  fechaVencimientoPraind: string,
  segundoNombre?: string,
  segundoApellido?: string
): ValidationResult {
  const n = (nombre || '').trim();
  const a = (apellido || '').trim();
  const c = (cedula || '').trim();
  const e = (empresaId || '').trim();
  const f = (fechaVencimientoPraind || '').trim();
  const sn = (segundoNombre || '').trim();
  const sa = (segundoApellido || '').trim();

  if (!n) return { ok: false, message: 'El nombre no puede estar vacío.' };
  if (n.length > 60) return { ok: false, message: 'Nombre demasiado largo.' };

  // Validar segundo nombre solo si existe
  if (sn && sn.length > 60) {
    return { ok: false, message: 'Segundo nombre demasiado largo.' };
  }

  if (!a) return { ok: false, message: 'El apellido no puede estar vacío.' };
  if (a.length > 60) return { ok: false, message: 'Apellido demasiado largo.' };

  // Validar segundo apellido solo si existe
  if (sa && sa.length > 60) {
    return { ok: false, message: 'Segundo apellido demasiado largo.' };
  }

  if (!c) return { ok: false, message: 'La cédula no puede estar vacía.' };
  if (c.length > 20) return { ok: false, message: 'La cédula es demasiado larga.' };

  if (!e) return { ok: false, message: 'Debe seleccionar una empresa.' };

  if (!f) return { ok: false, message: 'Debe ingresar la fecha de vencimiento.' };

  return { ok: true };
}