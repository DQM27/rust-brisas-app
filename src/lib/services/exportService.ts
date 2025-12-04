// src/lib/services/exportService.ts
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import type { GridApi } from '@ag-grid-community/core';

// ==========================================
// TIPOS
// ==========================================

export interface ExportRequest {
  format: 'pdf' | 'excel' | 'csv';
  headers: string[];
  rows: Record<string, any>[];

  // Opcionales para PDF
  title?: string;
  orientation?: 'portrait' | 'landscape';
  showPreview?: boolean;

  // Opcionales para CSV
  delimiter?: 'comma' | 'semicolon' | 'tab' | 'pipe';
  includeBom?: boolean;

  // Opcionales generales
  targetPath?: string;
}

export interface ExportResponse {
  success: boolean;
  format: string;
  bytes?: number[];
  filePath?: string;
  message: string;
}

export interface ExportOptions {
  title?: string;
  orientation?: 'portrait' | 'landscape';
  delimiter?: 'comma' | 'semicolon' | 'tab' | 'pipe';
  includeBom?: boolean;
  showPreview?: boolean;
}

// ==========================================
// EXTRACCIÓN DE DATOS
// ==========================================

/**
 * Extrae datos del AG-Grid para exportación
 */
export function extractGridData(gridApi: GridApi): {
  headers: string[];
  rows: Record<string, any>[];
} {
  // Obtener columnas visibles
  const columns = gridApi.getColumns() || [];
  const headers = columns
    .filter(col => !col.getColDef().hide)
    .map(col => col.getColDef().headerName || col.getColId());

  // Obtener todas las filas (respetando filtros)
  const rows: Record<string, any>[] = [];
  gridApi.forEachNodeAfterFilterAndSort(node => {
    if (node.data) {
      const row: Record<string, any> = {};

      columns.forEach(col => {
        const colId = col.getColId();
        const headerName = col.getColDef().headerName || colId;
        const value = node.data[colId];

        // Convertir valores a string de manera segura
        row[headerName] = value != null ? String(value) : '';
      });

      rows.push(row);
    }
  });

  return { headers, rows };
}

/**
 * Extrae solo filas seleccionadas
 */
export function extractSelectedRows(gridApi: GridApi): {
  headers: string[];
  rows: Record<string, any>[];
} {
  const columns = gridApi.getColumns() || [];
  const headers = columns
    .filter(col => !col.getColDef().hide)
    .map(col => col.getColDef().headerName || col.getColId());

  const selectedNodes = gridApi.getSelectedNodes();
  const rows: Record<string, any>[] = [];

  selectedNodes.forEach(node => {
    if (node.data) {
      const row: Record<string, any> = {};

      columns.forEach(col => {
        const colId = col.getColId();
        const headerName = col.getColDef().headerName || colId;
        const value = node.data[colId];
        row[headerName] = value != null ? String(value) : '';
      });

      rows.push(row);
    }
  });

  return { headers, rows };
}

// ==========================================
// COMANDOS TAURI
// ==========================================

/**
 * Exporta datos usando el backend
 */
export async function exportData(
  gridApi: GridApi,
  format: 'pdf' | 'excel' | 'csv',
  options: ExportOptions = {},
  onlySelected: boolean = false
): Promise<ExportResponse> {
  try {
    // Extraer datos
    const { headers, rows } = onlySelected
      ? extractSelectedRows(gridApi)
      : extractGridData(gridApi);

    if (rows.length === 0) {
      throw new Error('No hay datos para exportar');
    }

    let targetPath: string | null = null;

    // Si NO es preview, pedir path de guardado
    if (!options.showPreview) {
      const defaultName = options.title
        ? `${options.title.replace(/[^a-z0-9]/gi, '_')}.${format}`
        : `export.${format}`;

      targetPath = await save({
        defaultPath: defaultName,
        filters: [{
          name: format.toUpperCase(),
          extensions: [format]
        }]
      });

      if (!targetPath) {
        throw new Error('Exportación cancelada por el usuario');
      }
    }

    // Construir request
    const request: ExportRequest = {
      format,
      headers,
      rows,
      title: options.title || `Reporte ${new Date().toLocaleDateString()}`,
      orientation: options.orientation || 'landscape',
      delimiter: options.delimiter || 'comma',
      includeBom: options.includeBom ?? true,
      showPreview: options.showPreview || false,
      targetPath: targetPath || undefined,
    };

    // Invocar comando Tauri
    const response = await invoke<ExportResponse>('export_data', { request });

    return response;
  } catch (error) {
    console.error('[ExportService] Error:', error);
    throw error;
  }
}

/**
 * Verifica si la exportación está disponible
 */
export async function checkExportAvailable(): Promise<boolean> {
  try {
    return await invoke<boolean>('check_export_available');
  } catch {
    return false;
  }
}

/**
 * Obtiene formatos disponibles
 */
export async function getAvailableFormats(): Promise<string[]> {
  try {
    return await invoke<string[]>('get_available_export_formats');
  } catch {
    return ['csv']; // CSV siempre disponible
  }
}

// ==========================================
// HELPERS
// ==========================================

/**
 * Descarga bytes como archivo
 */
export function downloadBytes(bytes: number[], filename: string) {
  const blob = new Blob([new Uint8Array(bytes)]);
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

/**
 * Abre archivo PDF en nueva pestaña (preview)
 */
export function previewPDF(bytes: number[]) {
  const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
  const url = URL.createObjectURL(blob);
  window.open(url, '_blank');
}