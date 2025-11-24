// $lib/types/blacklistImport.types.ts

export type ValidationStatus = 'valid' | 'needs_review' | 'invalid';

export interface ExcelRowRaw {
  cedula?: string;
  nombreCompleto?: string;
  empresa?: string;
  motivo?: string;
  fechaInicio?: string;
  observaciones?: string;
}

export interface BlacklistImportEntry {
  id: string;
  cedula: string;
  primerNombre: string;
  segundoNombre?: string;
  primerApellido: string;
  segundoApellido?: string;
  nombreCompleto: string;
  empresa: string;
  motivoBloqueo: string;
  fechaInicioBloqueo: string;
  observaciones?: string;
  validationStatus: ValidationStatus;
  validationMessage?: string;
  importedAt: string;
  importedBy: string;
}

export interface ImportError {
  rowNumber: number;
  cedula?: string;
  errorType: string;
  message: string;
}

export interface ImportResultResponse {
  totalRows: number;
  successful: number;
  needsReview: number;
  failed: number;
  entries: BlacklistImportEntry[];
  errors: ImportError[];
}

export interface ExcelPreviewResponse {
  totalRows: number;
  detectedColumns: string[];
  sampleRows: ExcelRowRaw[];
  validationSummary: {
    validRows: number;
    needsReviewRows: number;
    invalidRows: number;
  };
}

export interface CreateBlacklistImportInput {
  cedula: string;
  primerNombre: string;
  segundoNombre?: string;
  primerApellido: string;
  segundoApellido?: string;
  empresa: string;
  motivoBloqueo?: string;
  fechaInicioBloqueo?: string;
  observaciones?: string;
}