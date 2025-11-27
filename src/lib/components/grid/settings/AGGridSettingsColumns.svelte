<script lang="ts">
  import type { GridId } from "$lib/types/agGrid";
  import type { GridApi, ColumnState } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { Eye, EyeOff, RotateCcw, GripVertical } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  interface ColumnInfo {
    id: string;
    name: string;
    visible: boolean;
    order: number;
  }

  let columns = $state<ColumnInfo[]>([]);
  let draggedIndex = $state<number | null>(null);

  // Cargar columnas del gridApi
  $effect(() => {
    if (gridApi) {
      const colState = gridApi.getColumnState();
      if (colState) {
        columns = colState
          .filter((col) => col.colId !== "ag-Grid-AutoColumn") // Filtrar columnas internas
          .map((col, idx) => ({
            id: col.colId,
            name: gridApi.getColumnDef(col.colId)?.headerName || col.colId,
            visible: !col.hide,
            order: idx,
          }));
      }
    }
  });

  function toggleVisibility(columnId: string) {
    const col = columns.find((c) => c.id === columnId);
    if (col && gridApi) {
      col.visible = !col.visible;
      gridApi.setColumnsVisible([columnId], col.visible);
    }
  }

  function showAll() {
    if (!gridApi) return;
    columns.forEach((col) => {
      col.visible = true;
    });
    gridApi.setColumnsVisible(
      columns.map((c) => c.id),
      true,
    );
  }

  function hideAll() {
    if (!gridApi) return;
    columns.forEach((col) => {
      col.visible = false;
    });
    gridApi.setColumnsVisible(
      columns.map((c) => c.id),
      false,
    );
  }

  function resetToDefault() {
    if (!gridApi) return;
    gridApi.resetColumnState();
    // Recargar columnas
    const colState = gridApi.getColumnState();
    if (colState) {
      columns = colState
        .filter((col) => col.colId !== "ag-Grid-AutoColumn")
        .map((col, idx) => ({
          id: col.colId,
          name: gridApi.getColumnDef(col.colId)?.headerName || col.colId,
          visible: !col.hide,
          order: idx,
        }));
    }
  }

  // Drag & Drop handlers
  function handleDragStart(index: number) {
    draggedIndex = index;
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    if (draggedIndex === null || draggedIndex === index) return;

    const newColumns = [...columns];
    const [draggedItem] = newColumns.splice(draggedIndex, 1);
    newColumns.splice(index, 0, draggedItem);

    draggedIndex = index;
    columns = newColumns;
  }

  function handleDragEnd() {
    if (gridApi && draggedIndex !== null) {
      // Aplicar nuevo orden al grid
      const columnState: ColumnState[] = columns.map((col, idx) => ({
        colId: col.id,
        hide: !col.visible,
        sort: null,
        sortIndex: null,
        aggFunc: null,
        width: undefined,
        flex: null,
        pivot: false,
        pivotIndex: null,
        pinned: null,
        rowGroup: false,
        rowGroupIndex: null,
      }));
      gridApi.applyColumnState({ state: columnState, applyOrder: true });
    }
    draggedIndex = null;
  }

  const visibleCount = $derived(columns.filter((c) => c.visible).length);
</script>

<div class="space-y-4">
  <!-- Header con contadores -->
  <div class="flex items-center justify-between">
    <div class="text-sm text-gray-400">
      <span class="text-white font-medium">{visibleCount}</span> de
      <span class="text-white font-medium">{columns.length}</span> columnas visibles
    </div>
    <div class="flex gap-2">
      <button
        onclick={showAll}
        class="px-3 py-1.5 text-xs font-medium text-green-400 bg-green-500/10 border border-green-500/20 rounded-md hover:bg-green-500/20 transition-colors"
      >
        Mostrar todas
      </button>
      <button
        onclick={hideAll}
        class="px-3 py-1.5 text-xs font-medium text-red-400 bg-red-500/10 border border-red-500/20 rounded-md hover:bg-red-500/20 transition-colors"
      >
        Ocultar todas
      </button>
    </div>
  </div>

  <!-- Lista de columnas -->
  <div class="space-y-1 max-h-96 overflow-y-auto" role="list">
    {#each columns as column, index (column.id)}
      <div
        role="listitem"
        draggable="true"
        ondragstart={() => handleDragStart(index)}
        ondragover={(e) => handleDragOver(e, index)}
        ondragend={handleDragEnd}
        class="flex items-center gap-3 p-3 bg-[#252526] border border-white/10 rounded-lg hover:border-white/20 transition-all cursor-move
          {draggedIndex === index ? 'opacity-50 scale-95' : ''}"
      >
        <!-- Drag Handle -->
        <div class="text-gray-500">
          <GripVertical size={18} />
        </div>

        <!-- Checkbox -->
        <input
          type="checkbox"
          checked={column.visible}
          onchange={() => toggleVisibility(column.id)}
          class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-blue-500 focus:ring-2"
        />

        <!-- Nombre -->
        <span class="flex-1 text-sm text-white">
          {column.name}
        </span>

        <!-- Icono de visibilidad -->
        {#if column.visible}
          <Eye size={16} class="text-green-400" />
        {:else}
          <EyeOff size={16} class="text-gray-500" />
        {/if}
      </div>
    {/each}
  </div>

  <!-- Botón de reset -->
  <button
    onclick={resetToDefault}
    class="w-full flex items-center justify-center gap-2 px-4 py-2.5 bg-[#252526] border border-white/10 rounded-lg text-sm font-medium text-white hover:bg-white/5 hover:border-white/20 transition-colors"
  >
    <RotateCcw size={16} />
    Restaurar por defecto
  </button>
</div>
