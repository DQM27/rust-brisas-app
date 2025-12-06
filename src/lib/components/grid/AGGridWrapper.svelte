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
    persistenceKey, // Nuevo prop
  }: Props = $props();

  // Estado
  let gridApi = $state<GridApi | null>(null);
  let selectedRows = $state<T[]>([]);
  let showSettings = $state(false);
  let isRestoring = false; // Prevents saving during restore
  let canSaveState = false; // Prevents saving during initialization

  // Determinar contexto actual basado en selección
  const context = $derived.by((): ToolbarContext => {
    if (selectedRows.length === 0) return "default";
    if (selectedRows.length === 1) return "singleSelect";
    return "multiSelect";
  });

  // Obtener configuración de tema y fuente (REACTIVO)
  const currentTheme = $derived(agGridSettings.getTheme(gridId));
  const themeClass = $derived(agGridSettings.getThemeClass(gridId));
  const fontClass = $derived(agGridSettings.getFontClass(gridId));
  const rowHeight = $derived(agGridSettings.getRowHeight(gridId));

  const paginationSize = $derived(agGridSettings.getPaginationSize(gridId));
  const showFloatingFilters = $derived(
    agGridSettings.getShowFloatingFilters(gridId),
  );

  // Calcular altura de fila según configuración
  const rowHeightPx = $derived.by(() => {
    switch (rowHeight) {
      case "compact":
        return 32;
      case "comfortable":
        return 48;
      default:
        return 40;
    }
  });

  // Tema personalizado REACTIVO - se recrea cuando cambia currentTheme
  const myTheme = $derived.by(() => {
    const isDark = currentTheme.includes("dark");
    const baseTheme = themeQuartz.withPart(
      isDark ? colorSchemeDark : colorSchemeLight,
    );

    return baseTheme.withParams({
      backgroundColor: isDark ? "rgb(30 30 30)" : "rgb(255 255 255)",
      foregroundColor: isDark ? "rgb(255 255 255)" : "rgb(0 0 0)",
      browserColorScheme: isDark ? "dark" : "light",
      headerBackgroundColor: isDark ? "rgb(37 37 38)" : "rgb(243 244 246)",
      headerTextColor: isDark ? "rgb(209 213 219)" : "rgb(17 24 39)",
      oddRowBackgroundColor: isDark ? "rgb(30 30 30)" : "rgb(255 255 255)",
      chromeBackgroundColor: isDark ? "rgb(37 37 38)" : "rgb(249 250 251)",
      rowHoverColor: isDark
        ? "rgba(255, 255, 255, 0.05)"
        : "rgba(0, 0, 0, 0.05)",
      columnBorder: true,
      borderColor: isDark ? "rgba(255, 255, 255, 0.1)" : "rgba(0, 0, 0, 0.1)",
      fontSize: 13,
      headerFontSize: 12,
      spacing: 4,
      cellHorizontalPadding: 16,
    });
  });

  // Effect para actualizar el tema cuando cambia
  $effect(() => {
    if (gridApi && myTheme) {
      gridApi.setGridOption("theme", myTheme);
    }
  });

  // Effect para actualizar rowHeight cuando cambia
  $effect(() => {
    if (gridApi && rowHeightPx) {
      gridApi.setGridOption("rowHeight", rowHeightPx);
      gridApi.resetRowHeights();
    }
  });

  // Effect para actualizar paginationSize cuando cambia
  $effect(() => {
    if (gridApi && paginationSize) {
      gridApi.setGridOption("paginationPageSize", paginationSize);
    }
  });

  // Effect para actualizar floatingFilters cuando cambia
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

  // Gestionar persistencia de columnas
  function saveColumnState(api: GridApi = gridApi!) {
    // Solo guardar si ya estamos listos y no estamos restaurando
    if (!api || !persistenceKey || isRestoring || !canSaveState) return;
    try {
      const state = api.getColumnState();
      localStorage.setItem(
        `ag-grid-state-${persistenceKey}`,
        JSON.stringify(state),
      );
    } catch (e) {
      console.warn("Error saving grid state:", e);
    }
  }

  function restoreColumnState(api: GridApi) {
    if (!api || !persistenceKey) return false;
    const savedState = localStorage.getItem(`ag-grid-state-${persistenceKey}`);
    if (savedState) {
      try {
        isRestoring = true;
        api.applyColumnState({
          state: JSON.parse(savedState),
          applyOrder: true,
        });
        // Unlock after a short delay to allow events to settle
        setTimeout(() => {
          isRestoring = false;
        }, 500);
        return true;
      } catch (e) {
        console.warn("Error restoring grid state:", e);
        isRestoring = false;
        return false;
      }
    }
    return false;
  }

  // Configuración del grid
  const gridOptions: GridOptions<T> = {
    columnDefs: columnDefs,
    localeText: AG_GRID_LOCALE_ES,
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 100,

      floatingFilter: showFloatingFilters,
    },
    rowSelection: {
      mode: "multiRow",
      enableClickSelection: false,
      checkboxes: true,
      headerCheckbox: true,
    },
    onGridReady: (params) => {
      gridApi = params.api;
      onGridReady?.(params.api);

      // Restaurar estado con un pequeño delay para asegurar que el grid esté listo
      setTimeout(() => {
        if (params.api.isDestroyed()) return;

        let restored = false;
        if (persistenceKey) {
          restored = restoreColumnState(params.api);
        }

        // Solo auto-size si NO se restauró estado
        if (!restored) {
          params.api.autoSizeAllColumns();
        }

        // Habilitar el guardado DESPUÉS de restaurar o inicializar
        // Damos un poco más de tiempo para que pasen los eventos iniciales de autoSize
        setTimeout(() => {
          canSaveState = true;
          // Reactivar animaciones para interacción del usuario
          params.api.setGridOption("suppressColumnMoveAnimation", false);
        }, 500);
      }, 200);
    },

    // Eventos para guardar estado
    onColumnMoved: (params) => saveColumnState(params.api),
    onColumnPinned: (params) => saveColumnState(params.api),
    onColumnResized: (params) => {
      // Solo guardar al terminar el resize
      if (params.finished) saveColumnState(params.api);
    },
    onColumnVisible: (params) => saveColumnState(params.api),
    onSortChanged: (params) => saveColumnState(params.api),

    // Suprimir animación inicial para evitar el "baile" de columnas al restaurar
    suppressColumnMoveAnimation: true,

    onCellClicked: (event) => {
      // Si la columna tiene su propio handler (como botones de acción), usarlo
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

<div class="flex flex-col w-full h-full bg-gray-950 {fontClass}">
  <AGGridToolbar
    {gridId}
    {context}
    {selectedRows}
    {gridApi}
    {customButtons}
    {customToolbarSlot}
    onOpenSettings={() => (showSettings = true)}
  />

  {#if showSettings}
    <AGGridSettingsModal
      {gridId}
      {gridApi}
      {customButtons}
      onClose={() => (showSettings = false)}
    />
  {/if}

  <div
    class="flex-1 overflow-hidden rounded-b-lg border border-white/10 {themeClass}"
  >
    <AgGrid {gridOptions} {rowData} {modules} />
  </div>
</div>
