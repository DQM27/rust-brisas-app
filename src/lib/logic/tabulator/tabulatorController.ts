import type { Options, Tabulator } from 'tabulator-tables';

/**
 * Interface defining the minimal API required by a Tabulator Wrapper
 */
export interface TabulatorWrapperAPI {
    replaceData: (newData: any[]) => void;
    updateRow: (id: any, rowData: any) => void;
    deleteRow: (id: any) => void;
    addData: (newData: any[]) => void;
    redraw: (force?: boolean) => void;
    getTable: () => Tabulator | undefined;
    setTable: (instance: Tabulator) => void;
    deselectAll: () => void;
}

/**
 * Handles the manual update logic for Tabulator to avoid Svelte proxy issues.
 * Returns a set of safe, manual control functions.
 */
export function createTabulatorController(): TabulatorWrapperAPI {
    let tableInstance: Tabulator | undefined;

    // Deep clone helper to strip Svelte proxies
    const safeClone = (data: any) => JSON.parse(JSON.stringify(data));

    const getTable = () => tableInstance;
    const setTable = (instance: Tabulator) => { tableInstance = instance; };

    const replaceData = (newData: any[]) => {
        const table = getTable();
        // Check if table is fully initialized to avoid internal Tabulator warnings/errors
        // @ts-ignore - 'initialized' property exists on runtime instance
        if (table && table.initialized) {
            table.replaceData(safeClone(newData))
                .catch(err => console.warn("Tabulator: replaceData failed", err));
        }
    };

    const updateRow = (id: any, rowData: any) => {
        const table = getTable();
        if (table) table.updateRow(id, safeClone(rowData));
    };

    const deleteRow = (id: any) => {
        const table = getTable();
        if (table) table.deleteRow(id);
    };

    const deselectAll = () => {
        const table = getTable();
        if (table) table.deselectRow(); // Tabulator deselectRow() without args deselects all? Verify docs. 
        // Docs: "If no argument is passed to this function it will deselect all rows." -> Yes.
    };

    const addData = (newData: any[]) => {
        const table = getTable();
        if (table) table.addData(safeClone(newData));
    };

    const redraw = (force = true) => {
        const table = getTable();
        // @ts-ignore - 'initialized' property exists on runtime instance
        if (table && table.initialized) table.redraw(force);
    }

    return {
        replaceData,
        updateRow,
        deleteRow,
        addData,
        redraw,
        getTable,
        setTable,
        deselectAll
    };
}

/**
 * Default options for the Brisas App Tabulator Theme
 */
export const defaultTabulatorOptions: any = {
    layout: "fitData",
    validationMode: "highlight",
    pagination: true,
    paginationSize: 20,
    paginationSizeSelector: [10, 20, 50, 100],
    movableColumns: true,
    resizableRows: true,
    persistence: true,
    index: "id",
    selectable: true,
    headerWordWrap: true,
    placeholder: "No se encontraron datos",
    // Pro Arsenal Features
    clipboard: "copy", // Permite Ctrl+C para copiar datos (tipo Excel)
    clipboardCopyStyled: false,
    clipboardCopyConfig: {
        columnHeaders: true,
    },
    columnHeaderVertAlign: "middle",
    // Configuración estética de grupos
    groupHeader: (value: any, count: number) => {
        return `<span class='text-blue-400 font-bold'>${value}</span> <span class='text-gray-500 font-normal ml-2'>(${count} registros)</span>`;
    },
    locale: "es",
    langs: {
        "es": {
            "pagination": {
                "page_size": "Por Página",
                "page_title": "Ver Página",
                "first": "Primero",
                "first_title": "Primera Página",
                "last": "Último",
                "last_title": "Última Página",
                "prev": "Anterior",
                "prev_title": "Página Anterior",
                "next": "Siguiente",
                "next_title": "Página Siguiente",
                "all": "Todos",
            },
            "groups": {
                "item": "registro",
                "items": "registros",
            },
            "ajax": {
                "loading": "Cargando...",
                "error": "Error al cargar datos",
            }
        }
    }
};
