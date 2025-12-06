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
  }: Props = $props();

  // Estado
  let gridApi = $state<GridApi | null>(null);
  let selectedRows = $state<T[]>([]);
  let showSettings = $state(false);

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

  // Configuración del grid
  const gridOptions: GridOptions<T> = {
    columnDefs: columnDefs,
    localeText: AG_GRID_LOCALE_ES,
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 100,
      floatingFilter: true,
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

      setTimeout(() => {
        if (!params.api.isDestroyed()) {
          params.api.autoSizeAllColumns();
        }
      }, 150);
    },

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
