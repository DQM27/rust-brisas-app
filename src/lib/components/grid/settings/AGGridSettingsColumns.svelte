<script lang="ts">
  import type { GridId } from "$lib/types/agGrid";
  import type { GridApi, ColumnState } from "@ag-grid-community/core";
  import {
    Eye,
    EyeOff,
    GripVertical,
    ArrowLeftToLine,
    ArrowRightToLine,
    Columns,
    Search,
    RotateCcw,
  } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  interface ColumnInfo {
    id: string;
    name: string;
    visible: boolean;
    pinned: "left" | "right" | null;
    width: number;
  }

  let columns = $state<ColumnInfo[]>([]);
  let searchQuery = $state("");
  let draggedIndex = $state<number | null>(null);

  $effect(() => {
    if (gridApi) {
      loadColumns();
    }
  });

  function loadColumns() {
    if (!gridApi) return;
    const colState = gridApi.getColumnState();
    if (colState) {
      columns = colState
        .filter((col) => col.colId !== "ag-Grid-AutoColumn")
        .map((col) => ({
          id: col.colId,
          name: gridApi.getColumnDef(col.colId)?.headerName || col.colId,
          visible: !col.hide,
          pinned: col.pinned as "left" | "right" | null,
          width: col.width || 100,
        }));
    }
  }

  const filteredColumns = $derived(
    columns.filter((col) =>
      col.name.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  const visibleCount = $derived(columns.filter((c) => c.visible).length);
  const pinnedLeftCount = $derived(
    columns.filter((c) => c.pinned === "left").length,
  );
  const pinnedRightCount = $derived(
    columns.filter((c) => c.pinned === "right").length,
  );

  function toggleVisibility(columnId: string) {
    const col = columns.find((c) => c.id === columnId);
    if (!col || !gridApi) return;
    col.visible = !col.visible;
    gridApi.setColumnsVisible([columnId], col.visible);
  }

  function setPinned(columnId: string, pinned: "left" | "right" | null) {
    const col = columns.find((c) => c.id === columnId);
    if (!col || !gridApi) return;
    col.pinned = pinned;
    gridApi.setColumnsPinned([columnId], pinned);
  }

  function showAll() {
    if (!gridApi) return;
    columns.forEach((col) => (col.visible = true));
    gridApi.setColumnsVisible(
      columns.map((c) => c.id),
      true,
    );
  }

  function hideAll() {
    if (!gridApi) return;
    columns.forEach((col) => (col.visible = false));
    gridApi.setColumnsVisible(
      columns.map((c) => c.id),
      false,
    );
  }

  function unpinAll() {
    if (!gridApi) return;
    columns.forEach((col) => (col.pinned = null));
    gridApi.setColumnsPinned(
      columns.map((c) => c.id),
      null,
    );
  }

  function autosizeAll() {
    if (!gridApi) return;
    gridApi.autoSizeAllColumns();
    loadColumns();
  }

  function resetColumns() {
    if (!gridApi) return;
    gridApi.resetColumnState();
    loadColumns();
  }

  function handleDragStart(e: DragEvent, index: number) {
    e.dataTransfer?.setData("text/plain", String(index));
    e.dataTransfer!.effectAllowed = "move";
    draggedIndex = index;
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    e.dataTransfer!.dropEffect = "move";
    if (draggedIndex === null || draggedIndex === index) return;
    const newColumns = [...columns];
    const [draggedItem] = newColumns.splice(draggedIndex, 1);
    newColumns.splice(index, 0, draggedItem);
    draggedIndex = index;
    columns = newColumns;
  }

  function handleDragEnd() {
    if (gridApi && draggedIndex !== null) {
      const columnState: ColumnState[] = columns.map((col) => ({
        colId: col.id,
        hide: !col.visible,
        pinned: col.pinned,
        width: col.width,
      }));
      gridApi.applyColumnState({ state: columnState, applyOrder: true });
    }
    draggedIndex = null;
  }

  const sectionClass = "space-y-4 p-1";
  const actionBtnClass =
    "px-2 py-1.5 text-[10px] font-medium rounded border transition-colors flex items-center justify-center gap-1.5 flex-1";
</script>

<div class={sectionClass}>
  <!-- Stats & Search -->
  <div class="space-y-2">
    <div class="flex items-center justify-between px-1">
      <span class="text-[10px] text-zinc-500">
        <span class="text-white font-medium">{visibleCount}</span
        >/{columns.length} visibles
      </span>
      {#if pinnedLeftCount > 0 || pinnedRightCount > 0}
        <span class="text-[10px] text-zinc-500">
          <span class="text-blue-400 font-medium"
            >{pinnedLeftCount + pinnedRightCount}</span
          > fijadas
        </span>
      {/if}
    </div>

    <div class="relative">
      <Search
        size={14}
        class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"
      />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Buscar columna..."
        class="w-full bg-black/20 border border-white/10 rounded-lg pl-9 pr-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all"
      />
    </div>
  </div>

  <!-- Quick Actions Grid -->
  <div class="grid grid-cols-2 gap-2">
    <div class="flex gap-1">
      <button
        onclick={showAll}
        class="{actionBtnClass} text-green-500 bg-green-500/10 border-green-500/20 hover:bg-green-500/20"
      >
        <Eye size={12} /> Mostrar Todo
      </button>
      <button
        onclick={hideAll}
        class="{actionBtnClass} text-red-500 bg-red-500/10 border-red-500/20 hover:bg-red-500/20"
      >
        <EyeOff size={12} /> Ocultar Todo
      </button>
    </div>
    <div class="flex gap-1">
      <button
        onclick={unpinAll}
        class="{actionBtnClass} text-zinc-400 bg-black/20 border-white/10 hover:bg-white/5 hover:text-white"
      >
        Desfijar
      </button>
      <button
        onclick={autosizeAll}
        class="{actionBtnClass} text-blue-400 bg-blue-500/10 border-blue-500/20 hover:bg-blue-500/20"
      >
        Auto-ajustar
      </button>
    </div>
  </div>

  <!-- Column List -->
  <div class="border border-white/5 rounded-lg bg-black/10 overflow-hidden">
    <div
      class="max-h-[300px] overflow-y-auto custom-scrollbar p-1 space-y-0.5"
      role="list"
    >
      {#each filteredColumns as column, index (column.id)}
        <div
          draggable="true"
          ondragstart={(e) => handleDragStart(e, index)}
          ondragover={(e) => handleDragOver(e, index)}
          ondragend={handleDragEnd}
          class="group flex items-center gap-2 p-2 rounded transition-all cursor-grab active:cursor-grabbing border border-transparent
          {draggedIndex === index
            ? 'opacity-50 bg-blue-500/10 border-blue-500/30'
            : 'hover:bg-white/5 hover:border-white/5'}"
        >
          <!-- Drag Handle -->
          <div class="text-zinc-600 group-hover:text-zinc-400 cursor-move">
            <GripVertical size={12} />
          </div>

          <!-- Toggle -->
          <button
            onclick={() => toggleVisibility(column.id)}
            class="p-1 rounded transition-colors
              {column.visible
              ? 'text-green-500 hover:bg-green-500/10'
              : 'text-zinc-600 hover:bg-zinc-700'}"
          >
            {#if column.visible}
              <Eye size={14} />
            {:else}
              <EyeOff size={14} />
            {/if}
          </button>

          <!-- Name -->
          <span
            class="flex-1 text-xs truncate {column.visible
              ? 'text-zinc-300'
              : 'text-zinc-500'}"
          >
            {column.name}
          </span>

          <!-- Pin Controls -->
          <div
            class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <button
              onclick={() =>
                setPinned(column.id, column.pinned === "left" ? null : "left")}
              class="p-1 rounded transition-colors {column.pinned === 'left'
                ? 'text-blue-400 bg-blue-500/20'
                : 'text-zinc-600 hover:text-zinc-300 hover:bg-white/10'}"
              title="Fijar izquierda"
            >
              <ArrowLeftToLine size={12} />
            </button>
            <button
              onclick={() =>
                setPinned(
                  column.id,
                  column.pinned === "right" ? null : "right",
                )}
              class="p-1 rounded transition-colors {column.pinned === 'right'
                ? 'text-blue-400 bg-blue-500/20'
                : 'text-zinc-600 hover:text-zinc-300 hover:bg-white/10'}"
              title="Fijar derecha"
            >
              <ArrowRightToLine size={12} />
            </button>
          </div>

          {#if column.pinned}
            <span
              class="px-1.5 py-0.5 text-[9px] font-bold rounded uppercase tracking-wider
             {column.pinned === 'left'
                ? 'bg-blue-500/20 text-blue-500'
                : 'bg-purple-500/20 text-purple-500'}"
            >
              {column.pinned === "left" ? "Izq" : "Der"}
            </span>
          {/if}
        </div>
      {/each}

      {#if filteredColumns.length === 0}
        <div class="py-8 text-center text-zinc-500">
          <Columns size={20} class="mx-auto mb-2 opacity-50" />
          <p class="text-xs">Sin columnas</p>
        </div>
      {/if}
    </div>
  </div>

  <button
    onclick={resetColumns}
    class="w-full flex items-center justify-center gap-2 py-2 rounded-md
      bg-black/20 border border-white/10 text-xs text-zinc-400 font-medium
      hover:bg-white/5 hover:text-white transition-colors h-[34px]"
  >
    <RotateCcw size={14} />
    Restaurar original
  </button>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
  }
</style>
