import type { ColDef, ICellRendererParams } from '@ag-grid-community/core';

/**
 * Configuración de una columna para DataTable
 */
export interface DataTableColumn<T = any> {
    field: keyof T | string;
    headerName: string;
    width?: number;
    minWidth?: number;
    maxWidth?: number;
    flex?: number;
    hide?: boolean;
    sortable?: boolean;
    filter?: boolean;
    resizable?: boolean;
    pinned?: 'left' | 'right' | null;
    cellRenderer?: (params: ICellRendererParams<T>) => string | HTMLElement;
    valueFormatter?: (params: any) => string;
    cellStyle?: any;
    tooltipField?: string;
    wrapText?: boolean;
    autoHeight?: boolean;
    suppressMovable?: boolean;
    suppressSizeToFit?: boolean;
}

/**
 * Acción disponible para una fila
 */
export interface DataTableAction<T = any> {
    id: string;
    label: string;
    icon?: any; // Lucide icon component
    variant?: 'default' | 'danger' | 'success' | 'warning';
    show?: (row: T) => boolean;
    onClick: (row: T) => void | Promise<void>;
}

/**
 * Item del context menu
 */
export interface DataTableContextMenuItem<T = any> {
    id: string;
    label: string;
    icon?: any;
    variant?: 'default' | 'danger';
    show?: (row: T) => boolean;
    onClick: (row: T) => void | Promise<void>;
    dividerAfter?: boolean;
}

/**
 * Configuración de exportación
 */
export interface DataTableExportConfig {
    fileName?: string;
    columnKeys?: string[];
    onlySelected?: boolean;
}

/**
 * Configuración del toolbar
 */
export interface DataTableToolbarConfig {
    showColumnSelector?: boolean;
    showExport?: boolean;
    showAutoSize?: boolean;
    customButtons?: DataTableToolbarButton[];
}

export interface DataTableToolbarButton {
    id: string;
    label: string;
    icon?: any;
    onClick: () => void;
    disabled?: boolean;
}

/**
 * Props principales del DataTable
 */
export interface DataTableProps<T = any> {
    data: T[];
    columns: DataTableColumn<T>[];
    storageKey: string;
    rowSelection?: 'single' | 'multiple' | false;
    pagination?: boolean;
    paginationPageSize?: number;
    paginationPageSizeSelector?: number[];
    actions?: DataTableAction<T>[];
    contextMenuItems?: DataTableContextMenuItem<T>[];
    exportConfig?: DataTableExportConfig;
    toolbarConfig?: DataTableToolbarConfig;
    onRowClick?: (row: T) => void;
    onRowDoubleClick?: (row: T) => void;
    onSelectionChange?: (rows: T[]) => void;
    getRowId?: (row: T) => string;
    autoSizeOnLoad?: boolean;
    height?: string;
}

/**
 * Store de configuración de columnas (genérico)
 */
export interface ColumnVisibilityConfig {
    [key: string]: boolean;
}

export interface TablePreferences {
    columnVisibility: ColumnVisibilityConfig;
    columnOrder?: string[];
    columnWidths?: { [key: string]: number };
    pageSize: number;
}