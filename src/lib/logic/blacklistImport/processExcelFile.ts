// $lib/logic/blacklistImport/processExcelFile.ts

import { blacklistImport } from '$lib/api/blacklistImport';
import type { ExcelPreviewResponse, ImportResultResponse } from '$lib/types/blacklistImport.types';

/**
 * Wrapper para preview del Excel
 * No maneja errores, los deja propagar
 */
export async function previewExcelFile(
  filePath: string,
  skipHeader: boolean = true
): Promise<ExcelPreviewResponse> {
  return await blacklistImport.preview(filePath, skipHeader);
}

/**
 * Wrapper para parse completo del Excel (sin insertar en BD)
 * No maneja errores, los deja propagar
 */
export async function parseExcelFile(
  filePath: string,
  skipHeader: boolean = true
): Promise<ImportResultResponse> {
  return await blacklistImport.parse(filePath, skipHeader);
}

/**
 * Wrapper para importación directa a BD
 * No maneja errores, los deja propagar
 */
export async function importExcelFile(
  filePath: string,
  userId: string,
  skipHeader: boolean = true
): Promise<ImportResultResponse> {
  return await blacklistImport.import(filePath, userId, skipHeader);
}

/**
 * Wrapper para importar entradas corregidas manualmente
 * No maneja errores, los deja propagar
 */
export async function importReviewedEntries(
  entries: any[],
  userId: string
): Promise<ImportResultResponse> {
  return await blacklistImport.importReviewed(entries, userId);
}