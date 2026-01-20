import type { TabulatorFull as Tabulator } from 'tabulator-tables';

/**
 * Extrae datos de Tabulator para exportación compatible con el backend
 */
export function extractTabulatorData(
    table: Tabulator,
    columnIds?: string[]
): {
    headers: string[];
    rows: Record<string, any>[];
} {
    // Obtener columnas
    const allColumns = table.getColumns();

    // Si se especifican columnas, filtrar por field. Si no, usar las que tienen title y field.
    let targetColumns = allColumns.filter((col) => {
        const def = col.getDefinition();
        // Evitar columnas de selección o acciones sin field
        if (!def.field || !def.title || def.field === 'acciones') return false;

        if (columnIds) {
            return columnIds.includes(def.field);
        }

        // Por defecto solo columnas visibles
        return col.isVisible();
    });

    // Ordenar columnas según el orden en columnIds si existe
    if (columnIds) {
        targetColumns.sort((a, b) => {
            const fieldA = a.getDefinition().field || '';
            const fieldB = b.getDefinition().field || '';
            return columnIds.indexOf(fieldA) - columnIds.indexOf(fieldB);
        });
    }

    const headers = targetColumns.map((col) => (col.getDefinition().title as string) || '');

    // Obtener datos respetando filtros y orden actual
    const data = table.getData('active'); // 'active' obtiene datos filtrados y ordenados
    const rows: Record<string, any>[] = [];

    data.forEach((rowData: any) => {
        const row: Record<string, any> = {};

        targetColumns.forEach((col) => {
            const def = col.getDefinition();
            const field = def.field as string;
            const headerName = (def.title as string) || field;

            let value = rowData[field];

            // Nota: Tabulator aplica formatters en el DOM, no en getData.
            // Si el backend espera el valor formateado (como lo hacía AG Grid via valueFormatter),
            // tendríamos que aplicar lógica similar aquí si es necesario.
            // Por ahora enviamos el valor crudo o convertido a string.

            row[headerName] = value != null ? String(value) : '';
        });

        rows.push(row);
    });

    return { headers, rows };
}

/**
 * Extrae solo filas seleccionadas de Tabulator
 */
export function extractTabulatorSelectedRows(
    table: Tabulator,
    columnIds?: string[]
): {
    headers: string[];
    rows: Record<string, any>[];
} {
    const allColumns = table.getColumns();

    // Si se especifican columnas, filtrar por field
    let targetColumns = allColumns.filter((col) => {
        const def = col.getDefinition();
        if (!def.field || !def.title || def.field === 'acciones') return false;

        if (columnIds) {
            return columnIds.includes(def.field);
        }
        return col.isVisible();
    });

    if (columnIds) {
        targetColumns.sort((a, b) => {
            const fieldA = a.getDefinition().field || '';
            const fieldB = b.getDefinition().field || '';
            return columnIds.indexOf(fieldA) - columnIds.indexOf(fieldB);
        });
    }

    const headers = targetColumns.map((col) => (col.getDefinition().title as string) || '');

    const selectedData = table.getSelectedData();
    const rows: Record<string, any>[] = [];

    selectedData.forEach((rowData: any) => {
        const row: Record<string, any> = {};
        targetColumns.forEach((col) => {
            const def = col.getDefinition();
            const field = def.field as string;
            const headerName = (def.title as string) || field;
            const value = rowData[field];
            row[headerName] = value != null ? String(value) : '';
        });
        rows.push(row);
    });

    return { headers, rows };
}
