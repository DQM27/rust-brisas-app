import type { GridApi } from '@ag-grid-community/core';

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
