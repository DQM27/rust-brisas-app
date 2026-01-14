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
    if (gridApi) {
      const currentDefaultColDef = gridApi.getGridOption("defaultColDef");
      gridApi.setGridOption("defaultColDef", {
        ...currentDefaultColDef,
        floatingFilter: showFloatingFilters,
      });
      gridApi.refreshHeader();
    }
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

  // Reactivity for columnDefs
  $effect(() => {
    if (gridApi && columnDefs) {
      gridApi.setGridOption("columnDefs", columnDefs);
      // Optional: autosize columns after change to ensure fit
      // gridApi.autoSizeAllColumns();
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
      if (params.finished) debouncedSaveColumnState(params.api);
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
