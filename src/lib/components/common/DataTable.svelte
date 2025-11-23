<script lang="ts" generics="T extends Record<string, any>">
  import { AgGrid } from "ag-grid-svelte5-extended";
  import { ClientSideRowModelModule } from "@ag-grid-community/client-side-row-model";
  import { themeQuartz, colorSchemeDark } from "@ag-grid-community/theming";
  import type {
    GridOptions,
    ColDef,
    ICellRendererParams,
    RowSelectedEvent,
    CellContextMenuEvent,
    GridApi,
  } from "@ag-grid-community/core";
  import {
    Settings,
    Download,
    CheckSquare,
    X,
    Copy,
    Trash2,
  } from "lucide-svelte";
  import DataTableToolbar from "./DataTableToolbar.svelte";
  import DataTableColumnSelector from "./DataTableColumnSelector.svelte";
  import DataTableContextMenu from "./DataTableContextMenu.svelte";
  import {
    getTablePreferencesStore,
    getVisibleColumnCount,
    toggleColumnVisibility,
    showAllColumns,
    updatePageSize,
  } from "$lib/stores/dataTableStore";
  import type {
    DataTableColumn,
    DataTableAction,
    DataTableContextMenuItem,
    DataTableExportConfig,
    DataTableToolbarConfig,
    ColumnVisibilityConfig,
  } from "$lib/types/dataTable";

  // Props
  interface Props {
    data: T[];
    columns: DataTableColumn<T>[];
    storageKey: string;
    rowSelection?: "single" | "multiple" | false;
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

  let {
    data,
    columns,
    storageKey,
    rowSelection = false,
    pagination = true,
    paginationPageSize = 20,
    paginationPageSizeSelector = [10, 20, 30, 50, 100],
    actions = [],
    contextMenuItems = [],
    exportConfig = {},
    toolbarConfig = {
      showColumnSelector: true,
      showExport: true,
      showAutoSize: true,
    },
    onRowClick,
    onRowDoubleClick,
    onSelectionChange,
    getRowId = (row) => row.id || String(Math.random()),
    autoSizeOnLoad = true,
    height = "100%",
  }: Props = $props();

  // Estado
  let rowData = $state<T[]>([]);
  let gridApi = $state<GridApi | null>(null);
  let selectedRows = $state<T[]>([]);
  let showColumnSelector = $state(false);

  // Context menu
  let contextMenu = $state<{
    show: boolean;
    x: number;
    y: number;
    row: T | null;
  }>({
    show: false,
    x: 0,
    y: 0,
    row: null,
  });

  // Store de preferencias
  const defaultVisibility: ColumnVisibilityConfig = columns.reduce(
    (acc, col) => ({ ...acc, [String(col.field)]: !col.hide }),
    {},
  );
  const preferencesStore = getTablePreferencesStore(
    storageKey,
    defaultVisibility,
  );
  const visibleColumnCount = getVisibleColumnCount(preferencesStore);

  // Actualizar datos
  $effect(() => {
    rowData = data;
  });

  // Tema oscuro
  const myTheme = themeQuartz.withPart(colorSchemeDark).withParams({
    backgroundColor: "#1e1e1e",
    foregroundColor: "#ffffff",
    browserColorScheme: "dark",
    headerBackgroundColor: "#252526",
    headerTextColor: "#cccccc",
    oddRowBackgroundColor: "#1e1e1e",
    chromeBackgroundColor: "#252526",
    rowHoverColor: "rgba(255, 255, 255, 0.05)",
    columnBorder: true,
    borderColor: "rgba(255, 255, 255, 0.1)",
    fontSize: 13,
    headerFontSize: 12,
    spacing: 4,
    cellHorizontalPadding: 16,
  });

  // Convertir columnas a definiciones de AG Grid
  let columnDefs = $derived.by((): ColDef<T>[] => {
    const cols: ColDef<T>[] = [];

    // Agregar columna de selección si está habilitada
    if (rowSelection) {
      cols.push({
        headerCheckboxSelection: rowSelection === "multiple",
        checkboxSelection: true,
        width: 50,
        pinned: "left",
        lockPinned: true,
        sortable: false,
        filter: false,
        resizable: false,
        suppressMovable: true,
      });
    }

    // Agregar columnas de datos
    columns.forEach((col) => {
      const colDef: ColDef<T> = {
        field: String(col.field) as any,
        headerName: col.headerName,
        width: col.width,
        minWidth: col.minWidth,
        maxWidth: col.maxWidth,
        flex: col.flex,
        hide: !$preferencesStore.columnVisibility[String(col.field)],
        sortable: col.sortable !== false,
        filter: col.filter !== false,
        resizable: col.resizable !== false,
        pinned: col.pinned || null,
        cellRenderer: col.cellRenderer,
        valueFormatter: col.valueFormatter,
        cellStyle: col.cellStyle,
        tooltipField: col.tooltipField as any,
        wrapText: col.wrapText,
        autoHeight: col.autoHeight,
        suppressMovable: col.suppressMovable,
        suppressSizeToFit: col.suppressSizeToFit,
      };

      cols.push(colDef);
    });

    // Agregar columna de acciones si hay acciones
    if (actions.length > 0) {
      cols.push({
        headerName: "Acciones",
        width: 130 + actions.length * 40,
        sortable: false,
        filter: false,
        pinned: "right",
        lockPinned: true,
        suppressMovable: true,
        cellRenderer: (params: ICellRendererParams<T>) => {
          const row = params.data;
          if (!row) return "";

          const buttons = actions
            .filter((action) => !action.show || action.show(row))
            .map((action) => {
              const variant = action.variant || "default";
              const colors = {
                default: {
                  bg: "rgba(59, 130, 246, 0.1)",
                  text: "#60a5fa",
                  border: "rgba(59, 130, 246, 0.2)",
                },
                danger: {
                  bg: "rgba(239, 68, 68, 0.1)",
                  text: "#f87171",
                  border: "rgba(239, 68, 68, 0.2)",
                },
                success: {
                  bg: "rgba(34, 197, 94, 0.1)",
                  text: "#4ade80",
                  border: "rgba(34, 197, 94, 0.2)",
                },
                warning: {
                  bg: "rgba(234, 179, 8, 0.1)",
                  text: "#fde047",
                  border: "rgba(234, 179, 8, 0.2)",
                },
              };
              const color = colors[variant];

              return `
                <button
                  data-action="${action.id}"
                  data-row-id="${getRowId(row)}"
                  style="
                    display: inline-flex;
                    align-items: center;
                    gap: 4px;
                    padding: 4px 10px;
                    margin-right: 4px;
                    border-radius: 6px;
                    background-color: ${color.bg};
                    color: ${color.text};
                    border: 1px solid ${color.border};
                    font-size: 10px;
                    font-weight: 500;
                    cursor: pointer;
                    transition: all 0.2s;
                    white-space: nowrap;
                  "
                  onmouseover="this.style.opacity='0.8'"
                  onmouseout="this.style.opacity='1'"
                >
                  ${action.label}
                </button>
              `;
            })
            .join("");

          return `<div style="display: flex; gap: 4px;">${buttons}</div>`;
        },
      });
    }

    return cols;
  });

  // Configuración del grid
  const gridOptions: GridOptions<T> = {
    get columnDefs() {
      return columnDefs;
    },
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 100,
    },
    rowSelection: rowSelection || undefined,
    suppressRowClickSelection: true,
    pagination,
    paginationPageSize: $preferencesStore.pageSize || paginationPageSize,
    paginationPageSizeSelector,
    getRowId: (params) => getRowId(params.data),
    theme: myTheme,
    tooltipShowDelay: 500,
    enableCellTextSelection: true,
    ensureDomOrder: true,
    enableCellChangeFlash: true,
    suppressMovableColumns: false,
    onGridReady: (params) => {
      gridApi = params.api;
    },
    onFirstDataRendered: (params) => {
      if (autoSizeOnLoad) {
        params.api.autoSizeAllColumns();
      }
    },
    onCellClicked: (event) => {
      const target = event.event?.target as HTMLElement;

      // Manejar clicks en botones de acción
      if (target?.dataset?.action) {
        const actionId = target.dataset.action;
        const rowId = target.dataset.rowId;
        const row = rowData.find((r) => getRowId(r) === rowId);

        if (row) {
          const action = actions.find((a) => a.id === actionId);
          action?.onClick(row);
        }
        return;
      }

      // Callback de click en fila
      if (event.data && onRowClick) {
        onRowClick(event.data);
      }
    },
    onRowDoubleClicked: (event) => {
      if (event.data && onRowDoubleClick) {
        onRowDoubleClick(event.data);
      }
    },
    onRowSelected: (event: RowSelectedEvent) => {
      if (gridApi) {
        const selected = gridApi.getSelectedRows();
        selectedRows = selected;
        onSelectionChange?.(selected);
      }
    },
    onCellContextMenu: (event: CellContextMenuEvent) => {
      if (contextMenuItems.length === 0) return;

      event.event?.preventDefault();
      const mouseEvent = event.event as MouseEvent;

      contextMenu = {
        show: true,
        x: mouseEvent.clientX,
        y: mouseEvent.clientY,
        row: event.data || null,
      };
    },
  };

  const modules = [ClientSideRowModelModule];

  // Funciones del toolbar
  function handleExport() {
    if (!gridApi) return;

    const fileName =
      exportConfig.fileName ||
      `export-${storageKey}-${new Date().toISOString().split("T")[0]}.csv`;
    const columnKeys =
      exportConfig.columnKeys || columns.map((col) => String(col.field));

    gridApi.exportDataAsCsv({
      fileName,
      columnKeys,
      onlySelected: exportConfig.onlySelected,
    });
  }

  function handleAutoSize() {
    if (!gridApi) return;
    gridApi.autoSizeAllColumns();
  }

  function handleSizeToFit() {
    if (!gridApi) return;
    gridApi.sizeColumnsToFit();
  }

  function handleSelectAll() {
    if (!gridApi) return;
    gridApi.selectAllFiltered();
  }

  function handleDeselectAll() {
    if (!gridApi) return;
    gridApi.deselectAll();
  }

  function handleCopySelected() {
    if (!gridApi || selectedRows.length === 0) return;

    const headers = columns.map((col) => col.headerName).join("\t");
    const rows = selectedRows
      .map((row) =>
        columns
          .map((col) => {
            const value = row[col.field as keyof T];
            return value !== null && value !== undefined ? String(value) : "";
          })
          .join("\t"),
      )
      .join("\n");

    const text = `${headers}\n${rows}`;
    navigator.clipboard.writeText(text);
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, row: null };
  }

  function handleContextMenuClick(item: DataTableContextMenuItem<T>) {
    if (contextMenu.row) {
      item.onClick(contextMenu.row);
    }
    closeContextMenu();
  }
</script>

<div class="data-table-wrapper" style="height: {height};">
  <!-- Toolbar -->
  <DataTableToolbar
    {toolbarConfig}
    {selectedRows}
    visibleCount={$visibleColumnCount}
    totalColumns={columns.length}
    gridReady={!!gridApi}
    onOpenColumnSelector={() => (showColumnSelector = true)}
    onExport={handleExport}
    onAutoSize={handleAutoSize}
    onSizeToFit={handleSizeToFit}
    onSelectAll={handleSelectAll}
    onDeselectAll={handleDeselectAll}
    onCopySelected={handleCopySelected}
  />

  <!-- Column Selector Modal -->
  {#if showColumnSelector}
    <DataTableColumnSelector
      {columns}
      {preferencesStore}
      onClose={() => (showColumnSelector = false)}
    />
  {/if}

  <!-- Context Menu -->
  {#if contextMenu.show && contextMenu.row && contextMenuItems.length > 0}
    <DataTableContextMenu
      {contextMenuItems}
      row={contextMenu.row}
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={closeContextMenu}
      onItemClick={handleContextMenuClick}
    />
  {/if}

  <!-- AG Grid -->
  <div class="ag-grid-container">
    <AgGrid {gridOptions} {rowData} {modules} />
  </div>
</div>

<style>
  .data-table-wrapper {
    width: 100%;
    display: flex;
    flex-direction: column;
    background-color: #1e1e1e;
  }

  .ag-grid-container {
    flex: 1;
    overflow: hidden;
  }

  :global(.ag-root-wrapper) {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 0 0 8px 8px;
    overflow: hidden;
  }

  :global(.ag-header-cell-text) {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-size: 11px;
  }

  :global(.ag-cell) {
    display: flex;
    align-items: center;
    line-height: 1.4;
    padding-top: 8px;
    padding-bottom: 8px;
  }

  :global(.ag-paging-panel) {
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background-color: #252526;
    padding: 16px 24px;
  }

  :global(.ag-paging-button),
  :global(.ag-paging-page-size) {
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #ffffff;
    border-radius: 6px;
  }

  :global(.ag-paging-button:hover:not(:disabled)) {
    background-color: rgba(255, 255, 255, 0.05);
  }

  :global(.ag-cell-wrap-text) {
    word-break: break-word;
    white-space: normal;
  }

  :global(.ag-row-selected) {
    background-color: rgba(59, 130, 246, 0.15) !important;
  }

  :global(.ag-row-selected:hover) {
    background-color: rgba(59, 130, 246, 0.2) !important;
  }

  :global(.ag-checkbox-input-wrapper) {
    accent-color: #3b82f6;
  }
</style>
