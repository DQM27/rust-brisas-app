// $lib/logic/blacklistImport/validateImportData.ts

export type ValidationResult =
  | { ok: true }
  | { ok: false; message: string };

export function validateFilePath(filePath: string): ValidationResult {
  const fp = (filePath || '').trim();

  if (!fp) {
    return { ok: false, message: 'Debe seleccionar un archivo.' };
  }

  // Validar extensión
  const validExtensions = ['.xlsx', '.xls', '.ods'];
  const hasValidExtension = validExtensions.some(ext => fp.toLowerCase().endsWith(ext));
  
  if (!hasValidExtension) {
    return { ok: false, message: 'El archivo debe ser un Excel (.xlsx, .xls, .ods).' };
  }

  return { ok: true };
}

export function validateManualEntry(
  cedula: string,
  primerNombre: string,
  primerApellido: string,
  empresa: string
): ValidationResult {
  const c = (cedula || '').trim();
  const pn = (primerNombre || '').trim();
  const pa = (primerApellido || '').trim();
  const e = (empresa || '').trim();

  if (!c) {
    return { ok: false, message: 'La cédula es obligatoria.' };
  }

  if (c.length < 9 || c.length > 15) {
    return { ok: false, message: 'La cédula debe tener entre 9 y 15 caracteres.' };
  }

  if (!pn) {
    return { ok: false, message: 'El primer nombre es obligatorio.' };
  }

  if (pn.length < 2) {
    return { ok: false, message: 'El primer nombre debe tener al menos 2 caracteres.' };
  }

  if (!pa) {
    return { ok: false, message: 'El primer apellido es obligatorio.' };
  }

  if (pa.length < 2) {
    return { ok: false, message: 'El primer apellido debe tener al menos 2 caracteres.' };
  }

  if (!e) {
    return { ok: false, message: 'La empresa es obligatoria.' };
  }

  if (e.length < 2) {
    return { ok: false, message: 'La empresa debe tener al menos 2 caracteres.' };
  }

  return { ok: true };
}

export function validateCorrectedEntry(
  primerNombre: string,
  segundoNombre: string | undefined,
  primerApellido: string,
  segundoApellido: string | undefined
): ValidationResult {
  const pn = (primerNombre || '').trim();
  const sn = (segundoNombre || '').trim();
  const pa = (primerApellido || '').trim();
  const sa = (segundoApellido || '').trim();

  if (!pn) {
    return { ok: false, message: 'El primer nombre es obligatorio.' };
  }

  if (pn.length < 2) {
    return { ok: false, message: 'El primer nombre debe tener al menos 2 caracteres.' };
  }

  // Segundo nombre es opcional, pero si existe debe ser válido
  if (sn && sn.length < 2) {
    return { ok: false, message: 'El segundo nombre debe tener al menos 2 caracteres si se especifica.' };
  }

  if (!pa) {
    return { ok: false, message: 'El primer apellido es obligatorio.' };
  }

  if (pa.length < 2) {
    return { ok: false, message: 'El primer apellido debe tener al menos 2 caracteres.' };
  }

  // Segundo apellido es opcional, pero si existe debe ser válido
  if (sa && sa.length < 2) {
    return { ok: false, message: 'El segundo apellido debe tener al menos 2 caracteres si se especifica.' };
  }

  return { ok: true };
}