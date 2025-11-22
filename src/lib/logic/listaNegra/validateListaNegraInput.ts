// src/lib/logic/listaNegra/validateListaNegraInput.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateAddToListaNegraInput(input: {
  contratistaId?: string;
  cedula?: string;
  nombre?: string;
  apellido?: string;
  motivoBloqueo: string;
  fechaFinBloqueo?: string;
  bloqueadoPor: string;
  observaciones?: string;
}): ValidationResult {
  const { contratistaId, cedula, nombre, apellido, motivoBloqueo, bloqueadoPor } = input;

  // Si tiene contratista_id, no necesita validar cedula/nombre/apellido
  if (contratistaId) {
    const cid = contratistaId.trim();
    if (!cid) {
      return { ok: false, message: 'Debe especificar un contratista.' };
    }
  } else {
    // Si NO tiene contratista_id, requiere cedula + nombre + apellido
    const c = (cedula || '').trim();
    const n = (nombre || '').trim();
    const a = (apellido || '').trim();

    if (!c) {
      return { ok: false, message: 'La cédula no puede estar vacía.' };
    }
    if (c.length < 7 || c.length > 20) {
      return { ok: false, message: 'La cédula debe tener entre 7 y 20 caracteres.' };
    }
    if (!/^[0-9-]+$/.test(c)) {
      return { ok: false, message: 'La cédula solo puede contener números y guiones.' };
    }

    if (!n) {
      return { ok: false, message: 'El nombre no puede estar vacío.' };
    }
    if (n.length > 50) {
      return { ok: false, message: 'El nombre no puede exceder 50 caracteres.' };
    }

    if (!a) {
      return { ok: false, message: 'El apellido no puede estar vacío.' };
    }
    if (a.length > 50) {
      return { ok: false, message: 'El apellido no puede exceder 50 caracteres.' };
    }
  }

  // Validar motivo de bloqueo
  const m = motivoBloqueo.trim();
  if (!m) {
    return { ok: false, message: 'Debe especificar un motivo de bloqueo.' };
  }
  if (m.length > 500) {
    return { ok: false, message: 'El motivo no puede exceder 500 caracteres.' };
  }

  // Validar quien bloqueó
  const b = bloqueadoPor.trim();
  if (!b) {
    return { ok: false, message: 'Debe especificar quién realizó el bloqueo.' };
  }
  if (b.length > 100) {
    return { ok: false, message: 'El nombre de quien bloqueó no puede exceder 100 caracteres.' };
  }

  return { ok: true };
}

export function validateUpdateListaNegraInput(input: {
  motivoBloqueo?: string;
  fechaFinBloqueo?: string;
  observaciones?: string;
}): ValidationResult {
  // Validar motivo si viene
  if (input.motivoBloqueo !== undefined) {
    const m = input.motivoBloqueo.trim();
    if (!m) {
      return { ok: false, message: 'El motivo de bloqueo no puede estar vacío.' };
    }
    if (m.length > 500) {
      return { ok: false, message: 'El motivo no puede exceder 500 caracteres.' };
    }
  }

  return { ok: true };
}