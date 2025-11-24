// $lib/api/blacklistImport.ts

import { invoke } from '@tauri-apps/api/core';
import type {
  BlacklistImportEntry,
  ImportResultResponse,
  ExcelPreviewResponse,
  CreateBlacklistImportInput
} from '$lib/types/blacklistImport.types';

export const blacklistImport = {
  // Preview del Excel sin importar
  preview: async (filePath: string, skipHeader: boolean = true): Promise<ExcelPreviewResponse> => {
    return await invoke<ExcelPreviewResponse>('preview_excel_import', { 
      filePath, 
      skipHeader 
    });
  },

  // Parse completo del Excel (sin insertar en BD)
  parse: async (filePath: string, skipHeader: boolean = true): Promise<ImportResultResponse> => {
    return await invoke<ImportResultResponse>('parse_excel_file', { 
      filePath, 
      skipHeader 
    });
  },

  // Importar Excel a la BD (solo entradas válidas)
  import: async (
    filePath: string, 
    userId: string, 
    skipHeader: boolean = true
  ): Promise<ImportResultResponse> => {
    return await invoke<ImportResultResponse>('import_excel_to_database', {
      filePath,
      userId,
      skipHeader
    });
  },

  // Importar entradas corregidas manualmente
  importReviewed: async (
    entries: CreateBlacklistImportInput[], 
    userId: string
  ): Promise<ImportResultResponse> => {
    return await invoke<ImportResultResponse>('import_reviewed_entries', {
      entries,
      userId
    });
  },

  // CRUD básico
  getAll: async (): Promise<BlacklistImportEntry[]> => {
    return await invoke<BlacklistImportEntry[]>('get_all_blacklist_imports');
  },

  getById: async (id: string): Promise<BlacklistImportEntry> => {
    return await invoke<BlacklistImportEntry>('get_blacklist_import_by_id', { id });
  },

  getByCedula: async (cedula: string): Promise<BlacklistImportEntry> => {
    return await invoke<BlacklistImportEntry>('get_blacklist_import_by_cedula', { cedula });
  },

  deleteAll: async (): Promise<number> => {
    return await invoke<number>('delete_all_blacklist_imports');
  },

  // Utilidades
  validateName: async (nombreCompleto: string): Promise<{
    primerNombre: string;
    segundoNombre?: string;
    primerApellido: string;
    segundoApellido?: string;
  }> => {
    return await invoke('validate_and_split_name', { nombreCompleto });
  },

  checkNameRequiresValidation: async (nombreCompleto: string): Promise<boolean> => {
    return await invoke<boolean>('check_name_requires_validation', { nombreCompleto });
  },

  normalizeCedula: async (cedula: string): Promise<string> => {
    return await invoke<string>('normalize_cedula', { cedula });
  },

  capitalizeName: async (nombre: string): Promise<string> => {
    return await invoke<string>('capitalize_name', { nombre });
  }
};