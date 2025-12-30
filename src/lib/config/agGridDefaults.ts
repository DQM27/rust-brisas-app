// ============================================
// src/lib/config/agGridDefaults.ts
// ============================================
// Default configuration for AG-Grid instances

import type { ColDef } from "@ag-grid-community/core";

/**
 * Default column definition applied to all grids
 */
export const DEFAULT_COL_DEF: ColDef = {
    sortable: true,
    filter: true,
    resizable: true,
    minWidth: 80,
    floatingFilter: false,
};

/**
 * Default row selection configuration
 */
export const DEFAULT_ROW_SELECTION = {
    mode: "multiRow" as const,
    enableClickSelection: false,
    checkboxes: true,
    headerCheckbox: true,
    copySelectedRows: true,
};

/**
 * Default pagination configuration
 */
export const DEFAULT_PAGINATION = {
    enabled: true,
    pageSize: 50,
    pageSizeSelector: [10, 20, 50, 100, 200, 500],
};

/**
 * Default performance settings
 */
export const PERFORMANCE_DEFAULTS = {
    animateRows: true,
    rowBuffer: 10,
    debounceVerticalScrollbar: true,
    suppressColumnVirtualisation: false,
    suppressRowHoverHighlight: false,
    suppressAnimationFrame: false,
};
