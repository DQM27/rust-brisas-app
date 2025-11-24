// $lib/logic/blacklistImport/parseImportErrors.ts

export function parseImportError(err: any): string {
  if (!err) return 'Ocurrió un error desconocido al importar.';

  if (typeof err === 'string') {
    // Errores comunes del backend
    if (/no contiene datos/i.test(err)) {
      return 'El archivo Excel no contiene datos válidos.';
    }
    if (/no contiene hojas/i.test(err)) {
      return 'El archivo Excel está vacío o corrupto.';
    }
    if (/Error abriendo archivo/i.test(err)) {
      return 'No se pudo abrir el archivo Excel. Verifique que no esté corrupto.';
    }
    if (/ya existe/i.test(err)) {
      return 'Ya existe una entrada con esa cédula en la tabla de prueba.';
    }
    return err;
  }

  if (typeof err === 'object' && err.message) {
    return parseImportError(err.message);
  }

  return 'Ocurrió un error inesperado durante la importación.';
}

export function parseValidationError(err: any): string {
  if (!err) return 'Error de validación desconocido.';

  if (typeof err === 'string') {
    return err;
  }

  if (typeof err === 'object' && err.message) {
    return err.message;
  }

  return 'Error de validación inesperado.';
}