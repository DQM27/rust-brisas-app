import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import type { GridApi } from '@ag-grid-community/core';
import type { ExportOptions, ExportRequest, ExportResponse } from './types';
import { extractGridData, extractSelectedRows } from './grid';

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
            templateId: options.templateId,
            targetPath: targetPath || undefined,
            // PDF specific options
            fontSize: options.fontSize,
            fontFamily: options.fontFamily,
            marginTop: options.marginTop,
            marginBottom: options.marginBottom,
            marginLeft: options.marginLeft,
            marginRight: options.marginRight,
            bannerColor: options.bannerColor,
            generatedBy: options.generatedBy,
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
