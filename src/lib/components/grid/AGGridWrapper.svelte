<script lang="ts" generics="T extends Record<string, any>">
  import { AgGrid } from "ag-grid-svelte5-extended";
  import { ClientSideRowModelModule } from "@ag-grid-community/client-side-row-model";
  import { CsvExportModule } from "@ag-grid-community/csv-export";
  import {
    themeQuartz,
    colorSchemeDark,
    colorSchemeLight,
  } from "@ag-grid-community/theming";
  import type {
    GridOptions,
    ColDef,
    GridApi,
    SelectionChangedEvent,
  } from "@ag-grid-community/core";
  import type {
    GridId,
    CustomToolbarButton,
    ToolbarContext,
    AGGridWrapperProps,
  } from "$lib/types/agGrid";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { gridState } from "$lib/stores/gridStateStore.svelte";
  import { AG_GRID_LOCALE_ES } from "$lib/config/agGridLocale";
  import AGGridToolbar from "./AGGridToolbar.svelte";
  import AGGridSettingsModal from "./AGGridSettingsModal.svelte";

  interface Props extends AGGridWrapperProps<T> {
    customToolbarSlot?: import("svelte").Snippet;
    onRefresh?: () => void | Promise<void>;
  }

  let {
    gridId,
    columnDefs,
    rowData,
    customButtons = {},
    customToolbarSlot,
    onGridReady,
    onSelectionChanged,
    onRowClicked,
    onRowDoubleClicked,
    enableGrouping = false,
    getRowId,
    persistenceKey,
    onRefresh,
  }: Props = $props();

  // Estado
  let gridApi = $state<GridApi | null>(null);
  let selectedRows = $state<T[]>([]);
  let showSettings = $state(false);

  // Contexto de toolbar
  const context = $derived.by((): ToolbarContext => {
    if (selectedRows.length === 0) return "default";
    if (selectedRows.length === 1) return "singleSelect";
    return "multiSelect";
  });

  // Configuraciones reactivas del store
  const currentTheme = $derived(agGridSettings.getTheme(gridId));
  const themeClass = $derived(agGridSettings.getThemeClass(gridId));
  const fontClass = $derived(agGridSettings.getFontClass(gridId));
  const rowHeightPx = $derived(agGridSettings.getRowHeightPx(gridId));
  const paginationSize = $derived(agGridSettings.getPaginationSize(gridId));
  const showFloatingFilters = $derived(
    agGridSettings.getShowFloatingFilters(gridId),
  );
  const animateRows = $derived(agGridSettings.getAnimateRows(gridId));
  const cellTextSelection = $derived(
    agGridSettings.getCellTextSelection(gridId),
  );
  const enableUndoRedo = $derived(agGridSettings.getEnableUndoRedo(gridId));
  const rowBuffer = $derived(agGridSettings.getRowBuffer(gridId));
  const debounceScroll = $derived(agGridSettings.getDebounceScroll(gridId));
  const enableQuickFilter = $derived(
    agGridSettings.getEnableQuickFilter(gridId),
  );

  const toolbarPosition = $derived(agGridSettings.getToolbarPosition(gridId));
  const headerHeight = $derived(agGridSettings.getHeaderHeight(gridId));
  const rowSelectionMode = $derived(agGridSettings.getRowSelectionMode(gridId));
  const suppressRowClickSelection = $derived(
    agGridSettings.getSuppressRowClickSelection(gridId),
  );
  const enterNavigation = $derived(agGridSettings.getEnterNavigation(gridId));
  const tabNavigation = $derived(agGridSettings.getTabNavigation(gridId));
  const copyWithHeaders = $derived(agGridSettings.getCopyWithHeaders(gridId));
  const suppressContextMenu = $derived(
    agGridSettings.getSuppressContextMenu(gridId),
  );
  const autoSizeOnLoad = $derived(
    agGridSettings.getAutoSizeColumnsOnLoad(gridId),
  );

  // Tema personalizado reactivo
  const myTheme = $derived.by(() => {
    const isDark = currentTheme.includes("dark");
    const baseTheme = themeQuartz.withPart(
      isDark ? colorSchemeDark : colorSchemeLight,
    );

    const fontFamilies: Record<string, string> = {
      system: "system-ui, sans-serif",
      inter: "'Inter', sans-serif",
      roboto: "'Roboto', sans-serif",
      "source-sans": "'Source Sans 3', sans-serif",
    };

    const currentFont = agGridSettings.getFont(gridId);

    return baseTheme.withParams({
      backgroundColor: isDark ? "rgb(26 26 27)" : "rgb(255 255 255)",
      foregroundColor: isDark ? "rgb(255 255 255)" : "rgb(0 0 0)",
      browserColorScheme: isDark ? "dark" : "light",
      headerBackgroundColor: isDark ? "rgb(37 37 38)" : "rgb(243 244 246)",
      headerTextColor: isDark ? "rgb(209 213 219)" : "rgb(17 24 39)",
      oddRowBackgroundColor: isDark ? "rgb(26 26 27)" : "rgb(255 255 255)",
      chromeBackgroundColor: isDark ? "rgb(37 37 38)" : "rgb(249 250 251)",
      rowHoverColor: isDark
        ? "rgba(255, 255, 255, 0.05)"
        : "rgba(0, 0, 0, 0.05)",
      selectedRowBackgroundColor: isDark
        ? "rgba(59, 130, 246, 0.15)"
        : "rgba(59, 130, 246, 0.1)",
      columnBorder: true,
      borderColor: isDark ? "rgba(255, 255, 255, 0.1)" : "rgba(0, 0, 0, 0.1)",
      fontSize: 13,
      headerFontSize: 12,
      spacing: 4,
      cellHorizontalPadding: 12,
      fontFamily: fontFamilies[currentFont] || "system-ui, sans-serif",
    });
  });

  // Effects para actualizar opciones cuando cambian
  $effect(() => {
    if (gridApi) {
      gridState.registerGrid(gridId, gridApi);
      return () => gridState.unregisterGrid(gridId);
    }
  });

  $effect(() => {
    if (gridApi && myTheme) {
      gridApi.setGridOption("theme", myTheme as any);
    }
  });

  $effect(() => {
    if (gridApi && rowHeightPx) {
      gridApi.setGridOption("rowHeight", rowHeightPx);
      gridApi.resetRowHeights();
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("paginationPageSize", paginationSize);
    }
  });

  $effect(() => {
    if (!gridApi) return;

    // Save current column state BEFORE changing defaultColDef
    const savedState = gridApi.getColumnState();

    // Suppress animation during restore
    gridApi.setGridOption("suppressColumnMoveAnimation", true);

    const currentDefaultColDef = gridApi.getGridOption("defaultColDef");
    gridApi.setGridOption("defaultColDef", {
      ...currentDefaultColDef,
      floatingFilter: showFloatingFilters,
    });
    gridApi.refreshHeader();

    // Restore column state immediately (synchronously) to prevent reset
    if (savedState && savedState.length > 0) {
      gridApi.applyColumnState({
        state: savedState,
        applyOrder: true,
      });
    }

    // Re-enable animation after a short delay
    setTimeout(() => {
      if (gridApi) {
        gridApi.setGridOption("suppressColumnMoveAnimation", false);
      }
    }, 100);
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("headerHeight", headerHeight);
    }
  });

  $effect(() => {
    if (gridApi) {
      if (rowSelectionMode === "none") {
        gridApi.setGridOption("rowSelection", undefined);
      } else {
        gridApi.setGridOption("rowSelection", {
          mode: rowSelectionMode === "single" ? "singleRow" : "multiRow",
          enableClickSelection: !suppressRowClickSelection,
          checkboxes: true,
          headerCheckbox: rowSelectionMode === "multiple",
          copySelectedRows: true,
        });
      }
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption(
        "enterNavigatesVertically",
        enterNavigation === "down",
      );
      gridApi.setGridOption(
        "enterNavigatesVerticallyAfterEdit",
        enterNavigation === "down",
      );
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("suppressContextMenu", suppressContextMenu);
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("copyHeadersToClipboard", copyWithHeaders);
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("animateRows", animateRows);
    }
  });

  $effect(() => {
    if (gridApi) {
      gridApi.setGridOption("enableCellTextSelection", cellTextSelection);
    }
  });

  // Reactivity for columnDefs - Preserve column state when columnDefs change
  // Only runs AFTER initial restore is complete (when gridState.isReady is true)
  $effect(() => {
    if (!gridApi || !columnDefs) return;

    // Skip if grid is not ready yet (initial restore hasn't completed)
    if (persistenceKey && !gridState.isReady(persistenceKey)) {
      console.log(
        "[AGGridWrapper] Skipping columnDefs update - grid not ready yet",
      );
      return;
    }

    console.log("[AGGridWrapper] columnDefs updated - preserving state");

    // Save current column state BEFORE applying new columnDefs
    const savedState = gridApi.getColumnState();

    // Apply new column definitions
    gridApi.setGridOption("columnDefs", columnDefs);

    // Restore column state AFTER applying columnDefs
    if (savedState && savedState.length > 0) {
      setTimeout(() => {
        if (!gridApi) return;
        gridApi.applyColumnState({
          state: savedState,
          applyOrder: true,
        });
      }, 50);
    }
  });

  // Debounce para eventos de columna - usa gridState store
  let columnEventTimeout: ReturnType<typeof setTimeout>;
  function debouncedSaveColumnState(api: GridApi) {
    if (!persistenceKey) return;
    clearTimeout(columnEventTimeout);
    columnEventTimeout = setTimeout(
      () => gridState.saveColumnState(api, persistenceKey),
      300,
    );
  }

  // Configuración del grid
  // svelte-ignore state_referenced_locally
  const gridOptions: GridOptions<T> = {
    columnDefs: columnDefs,
    localeText: AG_GRID_LOCALE_ES,
    loadThemeGoogleFonts: false,

    // Overlay personalizado cuando no hay datos
    overlayNoRowsTemplate: `
      <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; color: #9ca3af;">
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="opacity: 0.5; margin-bottom: 16px;">
          <path d="M21 15V19A2 2 0 0 1 19 21H5A2 2 0 0 1 3 19V15"></path>
          <polyline points="17 8 12 3 7 8"></polyline>
          <line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
        <span style="font-size: 16px; font-weight: 500; color: #d1d5db;">No hay registros para mostrar</span>
        <span style="font-size: 13px; margin-top: 8px; opacity: 0.8;">Usa el botón "Nuevo" para agregar un registro</span>
      </div>
    `,

    // Preserve column order when columnDefs are updated
    maintainColumnOrder: true,

    // Default Column Definition
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 80,
      floatingFilter: showFloatingFilters,
    },

    // Header
    headerHeight: headerHeight,

    // Selection
    rowSelection:
      rowSelectionMode === "none"
        ? undefined
        : {
            mode: rowSelectionMode === "single" ? "singleRow" : "multiRow",
            enableClickSelection: !suppressRowClickSelection,
            checkboxes: true,
            headerCheckbox: rowSelectionMode === "multiple",
            copySelectedRows: true,
          },

    // Navigation
    enterNavigatesVertically: enterNavigation === "down",
    enterNavigatesVerticallyAfterEdit: enterNavigation === "down",
    tabToNextCell: tabNavigation ? undefined : () => null, // null disables tab nav

    // Clipboard
    copyHeadersToClipboard: copyWithHeaders,

    // UI
    suppressContextMenu: suppressContextMenu,

    // Pagination
    pagination: true,
    paginationPageSize: paginationSize,
    paginationPageSizeSelector: [10, 20, 50, 100, 200, 500],

    // Performance
    animateRows: animateRows,
    rowBuffer: rowBuffer,
    debounceVerticalScrollbar: debounceScroll,
    suppressColumnVirtualisation: false,
    suppressRowHoverHighlight: false,
    suppressAnimationFrame: false,

    // Features
    enableCellTextSelection: cellTextSelection,
    ensureDomOrder: true,
    undoRedoCellEditing: enableUndoRedo,
    undoRedoCellEditingLimit: 20,

    // Clipboard
    cellSelection: false, // Replaces enableRangeSelection

    // Quick Filter
    cacheQuickFilter: true,

    // Suppress initial animation
    suppressColumnMoveAnimation: true,

    // Events
    onGridReady: (params) => {
      gridApi = params.api;
      onGridReady?.(params.api);

      // IMMEDIATELY block effects while we restore state
      if (persistenceKey) {
        gridState.prepareForRestore(persistenceKey);
      }

      setTimeout(async () => {
        if (params.api.isDestroyed()) return;

        let restored = false;
        if (persistenceKey) {
          restored = await gridState.restoreColumnState(
            params.api,
            persistenceKey,
          );
        }

        if (!restored) {
          if (autoSizeOnLoad) {
            params.api.autoSizeAllColumns();
          } else {
            params.api.sizeColumnsToFit();
          }
        }

        setTimeout(() => {
          if (persistenceKey) gridState.enableSaving(persistenceKey);
          params.api.setGridOption("suppressColumnMoveAnimation", false);
        }, 500);
      }, 200);
    },

    onColumnMoved: (params) => debouncedSaveColumnState(params.api),
    onColumnPinned: (params) => debouncedSaveColumnState(params.api),
    onColumnResized: (params) => {
      if (!persistenceKey) return;
      if (
        params.source === "autosizeColumns" ||
        params.source === "sizeColumnsToFit"
      ) {
        gridState.saveColumnState(params.api, persistenceKey);
      } else {
        debouncedSaveColumnState(params.api);
      }
    },
    onColumnVisible: (params) => debouncedSaveColumnState(params.api),
    onSortChanged: (params) => debouncedSaveColumnState(params.api),

    onCellClicked: (event) => {
      if (event.colDef && (event.colDef as any).onCellClicked) {
        (event.colDef as any).onCellClicked(event);
        return;
      }

      if (event.data && onRowClicked) {
        onRowClicked(event.data);
      }
    },

    onRowDoubleClicked: (event) => {
      if (event.data && onRowDoubleClicked) {
        onRowDoubleClicked(event.data);
      }
    },

    onSelectionChanged: (event: SelectionChangedEvent) => {
      if (gridApi) {
        const selected = gridApi.getSelectedRows();
        selectedRows = selected;
        onSelectionChanged?.(selected);
      }
    },
  };

  const modules = [ClientSideRowModelModule, CsvExportModule];
</script>

<div class="flex flex-col w-full h-full bg-[#0d0d0d] {fontClass}">
  {#if toolbarPosition === "top"}
    <AGGridToolbar
      {gridId}
      {context}
      {selectedRows}
      {gridApi}
      {customButtons}
      {customToolbarSlot}
      {onRefresh}
      onOpenSettings={() => (showSettings = true)}
    />
  {/if}

  {#if showSettings}
    <AGGridSettingsModal
      {gridId}
      {gridApi}
      {customButtons}
      onClose={() => (showSettings = false)}
    />
  {/if}

  <div
    class="flex-1 overflow-hidden border-x border-white/10 {themeClass}
      {toolbarPosition === 'top' ? 'border-b' : 'border-t'}"
  >
    <AgGrid {gridOptions} {rowData} {modules} />
  </div>

  {#if toolbarPosition === "bottom"}
    <AGGridToolbar
      {gridId}
      {context}
      {selectedRows}
      {gridApi}
      {customButtons}
      {customToolbarSlot}
      onOpenSettings={() => (showSettings = true)}
    />
  {/if}
</div>
