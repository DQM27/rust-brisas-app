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
  templateId?: string; // ✅ Add templateId

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
  templateId?: string; // ✅ Add templateId
  columnIds?: string[]; // ✅ Add columnIds for explicit column selection
}

// ==========================================
// EXTRACCIÓN DE DATOS
// ==========================================

/**
 * Extrae datos del AG-Grid para exportación
 */
export function extractGridData(gridApi: GridApi, columnIds?: string[]): {
  headers: string[];
  rows: Record<string, any>[];
} {
  // Obtener columnas
  const allColumns = gridApi.getColumns() || [];

  // Si se especifican columnas, filtrar por ID. Si no, usar visibles.
  const targetColumns = columnIds
    ? allColumns.filter(col => columnIds.includes(col.getColId()))
    : allColumns.filter(col => col.isVisible());

  // Ordenar columnas según el orden en columnIds si existe, para respetar la selección del usuario
  if (columnIds) {
    targetColumns.sort((a, b) => {
      return columnIds.indexOf(a.getColId()) - columnIds.indexOf(b.getColId());
    });
  }

  const headers = targetColumns.map(col => col.getColDef().headerName || col.getColId());

  // Obtener todas las filas (respetando filtros)
  const rows: Record<string, any>[] = [];
  gridApi.forEachNodeAfterFilterAndSort(node => {
    if (node.data) {
      const row: Record<string, any> = {};

      targetColumns.forEach(col => {
        const colDef = col.getColDef();
        const colId = col.getColId();
        const headerName = colDef.headerName || colId;

        // ✅ Obtener valor usando valueGetter si existe, sino del field (NO colId)
        let value;
        if (colDef.valueGetter && typeof colDef.valueGetter === 'function') {
          // Usar valueGetter personalizado
          value = colDef.valueGetter({ data: node.data, node, colDef, column: col, api: gridApi } as any);
        } else {
          // ✅ CRÍTICO: Usar 'field' en lugar de 'colId' para obtener el valor
          // Cuando múltiples columnas usan el mismo field (ej: fecha y hora),
          // AG Grid asigna diferentes colIds, pero el valor debe venir del field
          const fieldName = (colDef as any).field || colId;
          value = node.data[fieldName];
        }

        // ✅ Usar valueFormatter si existe (para fechas separadas, etc.)
        if (colDef.valueFormatter && typeof colDef.valueFormatter === 'function') {
          try {
            const formattedValue = colDef.valueFormatter({ value, data: node.data, node, colDef, column: col, api: gridApi } as any);
            row[headerName] = formattedValue || '';
          } catch (e) {
            console.error(`Error formatting column ${headerName}:`, e);
            // Fallback si valueFormatter falla
            row[headerName] = value != null ? String(value) : '';
          }
        } else {
          // Convertir valores a string de manera segura
          row[headerName] = value != null ? String(value) : '';
        }
      });

      rows.push(row);
    }
  });

  return { headers, rows };
}

/**
 * Extrae solo filas seleccionadas
 */
export function extractSelectedRows(gridApi: GridApi, columnIds?: string[]): {
  headers: string[];
  rows: Record<string, any>[];
} {
  const allColumns = gridApi.getColumns() || [];

  // Si se especifican columnas, filtrar por ID. Si no, usar visibles.
  const targetColumns = columnIds
    ? allColumns.filter(col => columnIds.includes(col.getColId()))
    : allColumns.filter(col => col.isVisible());

  // Ordenar columnas según el orden en columnIds si existe
  if (columnIds) {
    targetColumns.sort((a, b) => {
      return columnIds.indexOf(a.getColId()) - columnIds.indexOf(b.getColId());
    });
  }

  const headers = targetColumns.map(col => col.getColDef().headerName || col.getColId());

  const selectedNodes = gridApi.getSelectedNodes();
  const rows: Record<string, any>[] = [];

  selectedNodes.forEach(node => {
    if (node.data) {
      const row: Record<string, any> = {};

      targetColumns.forEach(col => {
        const colDef = col.getColDef();
        const colId = col.getColId();
        const headerName = colDef.headerName || colId;

        // ✅ Obtener valor usando valueGetter si existe, sino del field (NO colId)
        let value;
        if (colDef.valueGetter && typeof colDef.valueGetter === 'function') {
          // Usar valueGetter personalizado
          value = colDef.valueGetter({ data: node.data, node, colDef, column: col, api: gridApi } as any);
        } else {
          // ✅ CRÍTICO: Usar 'field' en lugar de 'colId' para obtener el valor
          const fieldName = (colDef as any).field || colId;
          value = node.data[fieldName];
        }

        // ✅ Usar valueFormatter si existe (para fechas separadas, etc.)
        if (colDef.valueFormatter && typeof colDef.valueFormatter === 'function') {
          try {
            const formattedValue = colDef.valueFormatter({ value, data: node.data, node, colDef, column: col, api: gridApi } as any);
            row[headerName] = formattedValue || '';
          } catch (e) {
            console.error(`Error formatting column ${headerName}:`, e);
            // Fallback si valueFormatter falla
            row[headerName] = value != null ? String(value) : '';
          }
        } else {
          // Convertir valores a string de manera segura
          row[headerName] = value != null ? String(value) : '';
        }
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
      ? extractSelectedRows(gridApi, options.columnIds)
      : extractGridData(gridApi, options.columnIds);

    if (rows.length === 0) {
      throw new Error('No hay datos para exportar');
    }

    let targetPath: string | null = null;

    // Si NO es preview, pedir path de guardado
    if (!options.showPreview) {
      const defaultName = options.title
        ? `${options.title.replace(/[^a-z0-9]/gi, '_')}.${format === 'excel' ? 'xlsx' : format}`
        : `export.${format === 'excel' ? 'xlsx' : format}`;

      // ✅ Mapear extensión correcta para el filtro del diálogo
      const fileExtension = format === 'excel' ? 'xlsx' : format;

      targetPath = await save({
        defaultPath: defaultName,
        filters: [{
          name: format.toUpperCase(),
          extensions: [fileExtension]  // ✅ Usar 'xlsx' no 'excel'
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
      templateId: options.templateId, // ✅ Pass templateId
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
  try {
    const blob = new Blob([new Uint8Array(bytes)], { type: 'application/pdf' });
    const url = URL.createObjectURL(blob);

    // Intentar abrir en nueva pestaña
    const newWindow = window.open(url, '_blank');

    // Verificar si fue bloqueado por popup blocker
    if (!newWindow || newWindow.closed || typeof newWindow.closed === 'undefined') {
      // Alternativa: crear link de descarga temporal
      const link = document.createElement('a');
      link.href = url;
      link.download = `preview-${Date.now()}.pdf`;
      link.target = '_blank';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);

      throw new Error('El navegador bloqueó la ventana emergente. Descargando PDF en su lugar.');
    }

    // Limpiar URL después de un tiempo
    setTimeout(() => URL.revokeObjectURL(url), 60000);
  } catch (error) {
    throw error;
  }
}