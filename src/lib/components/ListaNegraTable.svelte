<script lang="ts">
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
    X,
    Eye,
    EyeOff,
    Download,
    Copy,
    Trash2,
    CheckSquare,
    Square,
  } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";
  import ListaNegraContextMenu from "./ListaNegraContextMenu.svelte";
  import {
    columnVisibilityStore,
    COLUMN_LABELS,
    toggleColumnVisibility,
    resetColumnVisibility,
    showMinimalColumns,
    visibleColumnCount,
    type ColumnVisibility,
  } from "$lib/stores/columnConfigStore";

  interface Props {
    data: ListaNegraResponse[];
    onUnblock: (bloqueado: ListaNegraResponse) => void;
  }

  let { data, onUnblock }: Props = $props();

  // Estado reactivo para AG Grid
  let rowData = $state<ListaNegraResponse[]>([]);
  let gridApi = $state<GridApi | null>(null);

  // Actualizar cuando cambien los props
  $effect(() => {
    rowData = data;
  });

  // Estado para el modal de columnas
  let showColumnSelector = $state(false);

  // Estado para row selection
  let selectedRows = $state<ListaNegraResponse[]>([]);

  // Estado para context menu
  let contextMenu = $state<{
    show: boolean;
    x: number;
    y: number;
    row: ListaNegraResponse | null;
  }>({
    show: false,
    x: 0,
    y: 0,
    row: null,
  });

  // Suscribirse al store de visibilidad
  let columnVisibility = $state<ColumnVisibility>($columnVisibilityStore);

  $effect(() => {
    columnVisibility = $columnVisibilityStore;
  });

  // Tema oscuro personalizado
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

  // Definición de columnas con visibilidad dinámica
  let columnDefs = $derived.by((): ColDef<ListaNegraResponse>[] => [
    {
      headerCheckboxSelection: true,
      checkboxSelection: true,
      width: 50,
      pinned: "left",
      lockPinned: true,
      sortable: false,
      filter: false,
      resizable: false,
      suppressMovable: true,
    },
    {
      field: "cedula",
      headerName: "Cédula",
      width: 130,
      hide: !columnVisibility.cedula,
      cellStyle: { fontFamily: "monospace", color: "#d4d4d4" },
    },
    {
      field: "nombreCompleto",
      headerName: "Nombre Completo",
      width: 220,
      hide: !columnVisibility.nombreCompleto,
      cellStyle: { fontWeight: 500, color: "#ffffff" },
    },
    {
      field: "empresaNombre",
      headerName: "Empresa",
      width: 180,
      hide: !columnVisibility.empresaNombre,
      valueFormatter: (params) => params.value || "N/A",
      cellStyle: { color: "#a0a0a0" },
    },
    {
      field: "motivoBloqueo",
      headerName: "Motivo",
      flex: 1,
      minWidth: 200,
      hide: !columnVisibility.motivoBloqueo,
      cellStyle: { color: "#d4d4d4" },
      tooltipField: "motivoBloqueo",
      wrapText: true,
      autoHeight: true,
    },
    {
      field: "isActive",
      headerName: "Estado",
      width: 130,
      hide: !columnVisibility.isActive,
      autoHeight: true,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        const isActive = params.value;
        const bgColor = isActive
          ? "rgba(239, 68, 68, 0.1)"
          : "rgba(107, 114, 128, 0.1)";
        const textColor = isActive ? "#fca5a5" : "#9ca3af";
        const borderColor = isActive
          ? "rgba(239, 68, 68, 0.2)"
          : "rgba(107, 114, 128, 0.2)";
        const icon = isActive ? "🚫" : "⏰";
        const text = isActive ? "Bloqueado" : "Desbloqueado";

        return `
          <div style="
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 2px 8px;
            border-radius: 9999px;
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            font-size: 10px;
            font-weight: 500;
            white-space: nowrap;
          ">
            <span style="font-size: 9px;">${icon}</span>
            ${text}
          </div>
        `;
      },
    },
    {
      field: "esBloqueoPermanente",
      headerName: "Tipo",
      width: 120,
      hide: !columnVisibility.esBloqueoPermanente,
      autoHeight: true,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        const isPermanente = params.value;
        const bgColor = isPermanente
          ? "rgba(168, 85, 247, 0.1)"
          : "rgba(234, 179, 8, 0.1)";
        const textColor = isPermanente ? "#c084fc" : "#fde047";
        const borderColor = isPermanente
          ? "rgba(168, 85, 247, 0.2)"
          : "rgba(234, 179, 8, 0.2)";
        const text = isPermanente ? "Permanente" : "Temporal";

        return `
          <div style="
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 2px 8px;
            border-radius: 9999px;
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            font-size: 10px;
            font-weight: 500;
            white-space: nowrap;
          ">
            ${text}
          </div>
        `;
      },
    },
    {
      field: "bloqueadoPor",
      headerName: "Bloqueado por",
      width: 160,
      hide: !columnVisibility.bloqueadoPor,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        return `
          <div style="display: flex; align-items: center; gap: 6px; color: #a0a0a0;">
            <span style="font-size: 12px;">👤</span>
            ${params.value}
          </div>
        `;
      },
    },
    {
      field: "fechaInicioBloqueo",
      headerName: "Desde",
      width: 120,
      hide: !columnVisibility.fechaInicioBloqueo,
      valueFormatter: (params) => {
        if (!params.value) return "";
        const date = new Date(params.value);
        return date.toLocaleDateString("es-PA", {
          year: "numeric",
          month: "short",
          day: "numeric",
        });
      },
      cellStyle: { color: "#a0a0a0", fontSize: "11px" },
    },
    {
      field: "diasTranscurridos",
      headerName: "Días",
      width: 90,
      hide: !columnVisibility.diasTranscurridos,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        return `
          <div style="color: #6b7280; font-size: 11px;">
            ${params.value} días
          </div>
        `;
      },
    },
    {
      field: "id",
      headerName: "Acciones",
      width: 130,
      hide: !columnVisibility.actions,
      sortable: false,
      filter: false,
      pinned: "right",
      lockPinned: true,
      suppressMovable: true,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        const bloqueado = params.data;
        if (!bloqueado) return "";

        if (bloqueado.isActive) {
          return `
            <button
              data-action="unblock"
              data-id="${bloqueado.id}"
              style="
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 4px 10px;
                border-radius: 6px;
                background-color: rgba(34, 197, 94, 0.1);
                color: #4ade80;
                border: 1px solid rgba(34, 197, 94, 0.2);
                font-size: 10px;
                font-weight: 500;
                cursor: pointer;
                transition: all 0.2s;
                white-space: nowrap;
              "
              onmouseover="this.style.backgroundColor='rgba(34, 197, 94, 0.2)'"
              onmouseout="this.style.backgroundColor='rgba(34, 197, 94, 0.1)'"
            >
              Desbloquear
            </button>
          `;
        }
        return `<span style="color: #6b7280; font-size: 10px;">Desbloqueado</span>`;
      },
    },
  ]);

  // Configuración del grid
  const gridOptions: GridOptions<ListaNegraResponse> = {
    get columnDefs() {
      return columnDefs;
    },
    defaultColDef: {
      sortable: true,
      filter: true,
      resizable: true,
      minWidth: 100,
    },
    rowSelection: "multiple",
    suppressRowClickSelection: true,
    pagination: true,
    paginationPageSize: 20,
    paginationPageSizeSelector: [10, 20, 30, 50, 100],
    getRowId: (params) => params.data.id,
    theme: myTheme,
    tooltipShowDelay: 500,
    enableCellTextSelection: true,
    ensureDomOrder: true,
    enableCellChangeFlash: true,
    onGridReady: (params) => {
      gridApi = params.api;
    },
    onFirstDataRendered: (params) => {
      // Auto-ajustar columnas al contenido cuando los datos se rendericen
      params.api.autoSizeAllColumns();
    },
    onCellClicked: (event) => {
      const target = event.event?.target as HTMLElement;
      if (target?.dataset?.action === "unblock") {
        const id = target.dataset.id;
        const bloqueado = rowData.find((b) => b.id === id);
        if (bloqueado) {
          onUnblock(bloqueado);
        }
      }
    },
    onRowSelected: (event: RowSelectedEvent) => {
      if (gridApi) {
        selectedRows = gridApi.getSelectedRows();
      }
    },
    onCellContextMenu: (event: CellContextMenuEvent) => {
      event.event?.preventDefault();
      const mouseEvent = event.event as MouseEvent;

      contextMenu = {
        show: true,
        x: mouseEvent.clientX,
        y: mouseEvent.clientY,
        row: event.data || null,
      };
    },
    suppressMovableColumns: false,
  };

  const modules = [ClientSideRowModelModule];

  // Función para reajustar columnas manualmente
  function autoSizeColumns() {
    if (!gridApi) return;
    gridApi.autoSizeAllColumns();
  }

  // Función para ajustar al ancho disponible
  function sizeColumnsToFit() {
    if (!gridApi) return;
    gridApi.sizeColumnsToFit();
  }

  // Exportar a CSV
  function exportToCSV() {
    if (!gridApi) return;

    gridApi.exportDataAsCsv({
      fileName: `lista-negra-${new Date().toISOString().split("T")[0]}.csv`,
      columnKeys: [
        "cedula",
        "nombreCompleto",
        "empresaNombre",
        "motivoBloqueo",
        "isActive",
        "esBloqueoPermanente",
        "bloqueadoPor",
        "fechaInicioBloqueo",
        "diasTranscurridos",
      ],
    });
  }

  // Copiar seleccionados
  function copySelectedRows() {
    if (!gridApi) return;

    const selectedData = gridApi.getSelectedRows();
    if (selectedData.length === 0) {
      alert("No hay filas seleccionadas");
      return;
    }

    const text = selectedData
      .map(
        (row: ListaNegraResponse) =>
          `${row.cedula}\t${row.nombreCompleto}\t${row.empresaNombre || "N/A"}`,
      )
      .join("\n");

    navigator.clipboard.writeText(text).then(() => {
      alert(`${selectedData.length} fila(s) copiadas al portapapeles`);
    });
  }

  // Desbloquear seleccionados
  function unblockSelected() {
    if (selectedRows.length === 0) {
      alert("No hay filas seleccionadas");
      return;
    }

    const activeRows = selectedRows.filter((row) => row.isActive);
    if (activeRows.length === 0) {
      alert("Ninguna de las filas seleccionadas está bloqueada");
      return;
    }

    const confirmed = confirm(
      `¿Desbloquear ${activeRows.length} persona(s)?\n\n` +
        activeRows.map((r) => `• ${r.nombreCompleto} (${r.cedula})`).join("\n"),
    );

    if (confirmed) {
      // Aquí llamarías a una función batch que desbloqueara todas
      activeRows.forEach((row) => onUnblock(row));
    }
  }

  // Seleccionar todas las filas visibles
  function selectAllFiltered() {
    if (!gridApi) return;
    gridApi.selectAllFiltered();
  }

  // Deseleccionar todas
  function deselectAll() {
    if (!gridApi) return;
    gridApi.deselectAll();
  }

  // Context menu handlers
  function handleCopyRow(row: ListaNegraResponse) {
    const text = `${row.cedula}\t${row.nombreCompleto}\t${row.empresaNombre || "N/A"}`;
    navigator.clipboard.writeText(text);
  }

  function handleViewDetails(row: ListaNegraResponse) {
    alert(
      `Detalles de ${row.nombreCompleto}:\n\n` +
        `Cédula: ${row.cedula}\n` +
        `Empresa: ${row.empresaNombre || "N/A"}\n` +
        `Motivo: ${row.motivoBloqueo}\n` +
        `Estado: ${row.isActive ? "Bloqueado" : "Desbloqueado"}\n` +
        `Tipo: ${row.esBloqueoPermanente ? "Permanente" : "Temporal"}\n` +
        `Bloqueado por: ${row.bloqueadoPor}\n` +
        `Fecha: ${new Date(row.fechaInicioBloqueo).toLocaleDateString("es-PA")}\n` +
        `Días: ${row.diasTranscurridos}`,
    );
  }

  function closeContextMenu() {
    contextMenu = { show: false, x: 0, y: 0, row: null };
  }
</script>

<div class="ag-grid-wrapper">
  <!-- Toolbar -->
  <div class="toolbar">
    <div class="toolbar-section">
      <button
        onclick={() => (showColumnSelector = !showColumnSelector)}
        class="toolbar-button"
        title="Configurar columnas"
      >
        <Settings size={16} />
        Columnas ({$visibleColumnCount}/{Object.keys(COLUMN_LABELS).length})
      </button>

      <button
        onclick={exportToCSV}
        class="toolbar-button"
        title="Exportar a CSV"
        disabled={!gridApi}
      >
        <Download size={16} />
        Exportar CSV
      </button>

      <div class="toolbar-divider"></div>

      <button
        onclick={autoSizeColumns}
        class="toolbar-button compact"
        title="Ajustar columnas al contenido"
        disabled={!gridApi}
      >
        Ajustar columnas
      </button>

      <button
        onclick={sizeColumnsToFit}
        class="toolbar-button compact"
        title="Ajustar al ancho disponible"
        disabled={!gridApi}
      >
        Ajustar ancho
      </button>
    </div>

    {#if selectedRows.length > 0}
      <div class="toolbar-section selection-actions">
        <span class="selection-count">
          {selectedRows.length} seleccionada(s)
        </span>

        <button
          onclick={copySelectedRows}
          class="toolbar-button compact"
          title="Copiar seleccionadas"
        >
          <Copy size={14} />
        </button>

        <button
          onclick={unblockSelected}
          class="toolbar-button compact danger"
          title="Desbloquear seleccionadas"
        >
          <Trash2 size={14} />
        </button>

        <button
          onclick={deselectAll}
          class="toolbar-button compact"
          title="Deseleccionar todas"
        >
          <X size={14} />
        </button>
      </div>
    {:else}
      <div class="toolbar-section">
        <button
          onclick={selectAllFiltered}
          class="toolbar-button compact"
          title="Seleccionar todas las visibles"
        >
          <CheckSquare size={14} />
          Seleccionar todas
        </button>
      </div>
    {/if}
  </div>

  <!-- Modal selector de columnas -->
  {#if showColumnSelector}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="column-selector-overlay"
      onclick={() => (showColumnSelector = false)}
    >
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="column-selector" onclick={(e) => e.stopPropagation()}>
        <div class="column-selector-header">
          <h3>Configurar Columnas</h3>
          <button
            onclick={() => (showColumnSelector = false)}
            class="close-button"
            title="Cerrar"
          >
            <X size={18} />
          </button>
        </div>

        <div class="column-selector-body">
          {#each Object.entries(COLUMN_LABELS) as [field, label]}
            {@const isVisible =
              columnVisibility[field as keyof ColumnVisibility]}
            <label class="column-option">
              <input
                type="checkbox"
                checked={isVisible}
                onchange={() =>
                  toggleColumnVisibility(field as keyof ColumnVisibility)}
              />
              <span class="column-label">
                <svelte:component
                  this={isVisible ? Eye : EyeOff}
                  size={14}
                  class={isVisible ? "text-blue-400" : "text-gray-500"}
                />
                {label}
              </span>
            </label>
          {/each}
        </div>

        <div class="column-selector-footer">
          <button onclick={showMinimalColumns} class="action-button">
            Vista Mínima
          </button>
          <button onclick={resetColumnVisibility} class="action-button">
            Restablecer
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Context Menu -->
  {#if contextMenu.show && contextMenu.row}
    <ListaNegraContextMenu
      row={contextMenu.row}
      x={contextMenu.x}
      y={contextMenu.y}
      onClose={closeContextMenu}
      onCopyRow={handleCopyRow}
      onViewDetails={handleViewDetails}
      onUnblock={contextMenu.row.isActive ? onUnblock : undefined}
    />
  {/if}

  <!-- AG Grid -->
  <div class="ag-grid-container">
    <AgGrid {gridOptions} {rowData} {modules} />
  </div>
</div>

<style>
  .ag-grid-wrapper {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background-color: #252526;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .toolbar-section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toolbar-divider {
    width: 1px;
    height: 24px;
    background-color: rgba(255, 255, 255, 0.1);
  }

  .selection-actions {
    background-color: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 6px;
    padding: 6px 12px;
  }

  .selection-count {
    color: #60a5fa;
    font-size: 12px;
    font-weight: 500;
    margin-right: 8px;
  }

  .toolbar-button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ffffff;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .toolbar-button:hover:not(:disabled) {
    background-color: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .toolbar-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .toolbar-button.compact {
    padding: 6px 8px;
    font-size: 11px;
  }

  .toolbar-button.danger {
    color: #f87171;
    border-color: rgba(248, 113, 113, 0.2);
  }

  .toolbar-button.danger:hover {
    background-color: rgba(248, 113, 113, 0.1);
  }

  .column-selector-overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s;
  }

  .column-selector {
    background-color: #252526;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    width: 420px;
    max-width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
    animation: slideUp 0.3s;
  }

  .column-selector-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .column-selector-header h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }

  .close-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    color: #a0a0a0;
    cursor: pointer;
    transition: all 0.2s;
  }

  .close-button:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }

  .column-selector-body {
    padding: 16px 20px;
    overflow-y: auto;
    flex: 1;
  }

  .column-option {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 0;
    cursor: pointer;
    user-select: none;
  }

  .column-option input[type="checkbox"] {
    width: 16px;
    height: 16px;
    cursor: pointer;
    accent-color: #007acc;
  }

  .column-label {
    display: flex;
    align-items: center;
    gap: 8px;
    color: #d4d4d4;
    font-size: 13px;
  }

  .column-option:hover .column-label {
    color: #ffffff;
  }

  .column-selector-footer {
    padding: 16px 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    gap: 8px;
  }

  .action-button {
    flex: 1;
    padding: 8px 16px;
    background-color: #1e1e1e;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ffffff;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-button:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .ag-grid-container {
    flex: 1;
    overflow: hidden;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideUp {
    from {
      transform: translateY(20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
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
