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
    SelectionChangedEvent,
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
    // NUEVO: Opciones de animaciones
    enableAnimations?: boolean;
    animateRows?: boolean;
    // NUEVO: Opciones de filtros avanzados
    enableAdvancedFilters?: boolean;
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
    enableAnimations = true,
    animateRows = true,
    enableAdvancedFilters = true,
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

  // Tema oscuro para AG Grid v32 con animaciones mejoradas
  const myTheme = themeQuartz.withPart(colorSchemeDark).withParams({
    backgroundColor: "rgb(30 30 30)",
    foregroundColor: "rgb(255 255 255)", 
    browserColorScheme: "dark",
    headerBackgroundColor: "rgb(37 37 38)",
    headerTextColor: "rgb(209 213 219)",
    oddRowBackgroundColor: "rgb(30 30 30)",
    chromeBackgroundColor: "rgb(37 37 38)",
    rowHoverColor: "rgba(255, 255, 255, 0.05)", 
    columnBorder: true,
    borderColor: "rgba(255, 255, 255, 0.1)",
    fontSize: 13,
    headerFontSize: 12,
    spacing: 4,
    cellHorizontalPadding: 16,
    // NUEVO: Configuración de animaciones
    
  });

  /**
   * Determina el tipo de filtro según el tipo de dato de la columna
   * Solo usa filtros disponibles en AG Grid Community (gratuito)
   */
  function getFilterType(column: DataTableColumn<T>): string | boolean {
    if (!enableAdvancedFilters) return true;
    
    // Si la columna especifica un filtro custom, usarlo
    if (column.filter === false) return false;
    if (typeof column.filter === 'string') return column.filter;
    
    // Inferir tipo de filtro basado en el field
    const field = String(column.field);
    
    // Filtros de fecha (Community)
    if (field.includes('fecha') || field.includes('date') || field.includes('_at') || field.includes('vencimiento')) {
      return 'agDateColumnFilter';
    }
    
    // Filtros numéricos (Community)
    if (field.includes('id') || field.includes('cantidad') || field.includes('monto') || field.includes('precio') || field.includes('numero')) {
      return 'agNumberColumnFilter';
    }
    
    // Por defecto: filtro de texto (Community)
    return 'agTextColumnFilter';
  }

  /**
   * Configuración de filtros avanzados por tipo
   * Solo características de AG Grid Community
   */
  function getFilterParams(column: DataTableColumn<T>) {
    const filterType = getFilterType(column);
    
    if (filterType === 'agDateColumnFilter') {
      return {
        buttons: ['reset', 'apply'],
        closeOnApply: true,
        comparator: (filterDate: Date, cellValue: string) => {
          if (!cellValue) return -1;
          const cellDate = new Date(cellValue);
          if (filterDate.getTime() === cellDate.getTime()) return 0;
          return cellDate < filterDate ? -1 : 1;
        },
      };
    }
    
    if (filterType === 'agNumberColumnFilter') {
      return {
        buttons: ['reset', 'apply'],
        closeOnApply: true,
        allowedCharPattern: '\\d\\-\\,\\.', // números, guiones, comas y puntos
      };
    }
    
    // Text filter (por defecto) - Community
    return {
      buttons: ['reset', 'apply'],
      closeOnApply: true,
      // Opciones de búsqueda de texto
      filterOptions: [
        'contains',
        'notContains',
        'equals',
        'notEqual',
        'startsWith',
        'endsWith',
      ],
      defaultOption: 'contains',
      // Case insensitive por defecto
      caseSensitive: false,
      // Trim espacios
      trimInput: true,
    };
  }

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
        showDisabledCheckboxes: true,
      });
    }

    // Agregar columnas de datos
    columns.forEach((col) => {
      const filterType = getFilterType(col);
      const filterParams = getFilterParams(col);
      
      const colDef: ColDef<T> = {
        field: String(col.field) as any,
        headerName: col.headerName,
        width: col.width,
        minWidth: col.minWidth,
        maxWidth: col.maxWidth,
        flex: col.flex,
        hide: !$preferencesStore.columnVisibility[String(col.field)],
        sortable: col.sortable !== false,
        filter: filterType,
        filterParams: filterParams,
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
        enableCellChangeFlash: enableAnimations,
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
              const colorClasses = {
                default: "bg-blue-500/10 text-blue-400 border-blue-500/20 hover:bg-blue-500/20",
                danger: "bg-red-500/10 text-red-400 border-red-500/20 hover:bg-red-500/20",
                success: "bg-green-500/10 text-green-400 border-green-500/20 hover:bg-green-500/20",
                warning: "bg-amber-500/10 text-amber-400 border-amber-500/20 hover:bg-amber-500/20",
              };
              const colorClass = colorClasses[variant];

              return `
                <button
                  data-action="${action.id}"
                  data-row-id="${getRowId(row)}"
                  class="inline-flex items-center gap-1 px-2.5 py-1 mr-1 rounded-md border text-xs font-medium transition-colors ${colorClass}"
                >
                  ${action.label}
                </button>
              `;
            })
            .join("");

          return `<div class="flex gap-1">${buttons}</div>`;
        },
      });
    }

    return cols;
  });

  // Effect para actualizar el grid cuando cambian las columnas
  $effect(() => {
    if (gridApi && columnDefs) {
      gridApi.setGridOption('columnDefs', columnDefs);
    }
  });

  // Configuración del grid para v32.3.9
  const gridOptions: GridOptions<T> = {
    columnDefs: columnDefs,
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 100,
      enableCellChangeFlash: enableAnimations,
      // NUEVO: Floating filter (mini filtros bajo los headers)
      floatingFilter: enableAdvancedFilters,
    },
    rowSelection: rowSelection ? (rowSelection === "multiple" ? "multiple" : "single") : undefined,
    suppressRowClickSelection: true,
    pagination,
    paginationPageSize: $preferencesStore.pageSize || paginationPageSize,
    paginationPageSizeSelector,
    getRowId: (params) => getRowId(params.data),
    theme: myTheme,
    tooltipShowDelay: 500,
    enableCellTextSelection: true,
    ensureDomOrder: true,
    enableCellChangeFlash: enableAnimations,
    suppressMovableColumns: false,
    
    // NUEVO: Configuración de animaciones
    animateRows: animateRows,
    
    // NUEVO: Configuración de filtros avanzados
    suppressMenuHide: false,
    
    // Eventos actualizados para v32
    onGridReady: (params) => {
      gridApi = params.api;
      
      // Auto-size después de que el grid esté listo
      if (autoSizeOnLoad) {
        setTimeout(() => {
          params.api.autoSizeAllColumns();
        }, 150);
      }
    },
    
    onFirstDataRendered: (params) => {
      // Opcional: puedes agregar lógica adicional aquí
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
    
    onSelectionChanged: (event: SelectionChangedEvent) => {
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
      onlySelected: exportConfig.onlySelected ?? false,
      processCellCallback: (params) => {
        return params.value ?? '';
      }
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
    navigator.clipboard.writeText(text).catch(console.error);
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

<div class="flex flex-col w-full bg-gray-950" style="height: {height};">
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

  <!-- AG Grid Container -->
  <div class="flex-1 overflow-hidden rounded-b-lg border border-white/10">
    <AgGrid {gridOptions} {rowData} {modules} />
  </div>
</div>