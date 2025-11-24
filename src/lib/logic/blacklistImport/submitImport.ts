// $lib/logic/blacklistImport/submitImport.ts

import { validateFilePath } from './validateImportData';
import { parseExcelFile, importExcelFile } from './processExcelFile';
import { parseImportError } from './parseImportErrors';
import type { ImportResultResponse } from '$lib/types/blacklistImport.types';

export type SubmitImportResult =
  | { ok: true; result: ImportResultResponse }
  | { ok: false; error: string };

/**
 * Orquesta el proceso completo de importación:
 * 1. Valida el archivo
 * 2. Parsea el Excel
 * 3. Importa a la BD
 * 4. Parsea errores si falla
 */
export async function submitImport(
  filePath: string,
  userId: string,
  skipHeader: boolean = true
): Promise<SubmitImportResult> {
  // 1. Validar archivo
  const validation = validateFilePath(filePath);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Intentar parsear
  try {
    const parseResult = await parseExcelFile(filePath, skipHeader);
    
    // Si hay errores críticos (todas las filas fallaron), no continuar
    if (parseResult.failed === parseResult.totalRows && parseResult.totalRows > 0) {
      return { 
        ok: false, 
        error: 'No se pudo procesar ninguna fila del Excel. Verifique el formato.' 
      };
    }

    // 3. Importar a la BD (solo válidos)
    const importResult = await importExcelFile(filePath, userId, skipHeader);
    return { ok: true, result: importResult };
    
  } catch (err: any) {
    // 4. Parsear error
    const errorMessage = parseImportError(err);
    return { ok: false, error: errorMessage };
  }
}

/**
 * Orquesta el proceso de preview (sin importar):
 * 1. Valida el archivo
 * 2. Solo parsea el Excel
 * 3. Retorna resultado para preview
 */
export async function submitPreview(
  filePath: string,
  skipHeader: boolean = true
): Promise<SubmitImportResult> {
  // 1. Validar archivo
  const validation = validateFilePath(filePath);
  if (!validation.ok) {
    return { ok: false, error: validation.message };
  }

  // 2. Parsear para preview
  try {
    const parseResult = await parseExcelFile(filePath, skipHeader);
    return { ok: true, result: parseResult };
  } catch (err: any) {
    const errorMessage = parseImportError(err);
    return { ok: false, error: errorMessage };
  }
}