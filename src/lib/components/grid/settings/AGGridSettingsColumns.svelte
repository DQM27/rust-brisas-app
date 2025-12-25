<script lang="ts">
  import type { GridId } from "$lib/types/agGrid";
  import type { GridApi, ColumnState } from "@ag-grid-community/core";
  import {
    Eye,
    EyeOff,
    GripVertical,
    PinOff,
    ArrowLeftToLine,
    ArrowRightToLine,
    Columns,
    Search,
    Pin,
    Hash,
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

  // Cargar columnas del grid
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

  // Columnas filtradas por búsqueda
  const filteredColumns = $derived(
    columns.filter((col) =>
      col.name.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  // Contadores
  const visibleCount = $derived(columns.filter((c) => c.visible).length);
  const pinnedLeftCount = $derived(
    columns.filter((c) => c.pinned === "left").length,
  );
  const pinnedRightCount = $derived(
    columns.filter((c) => c.pinned === "right").length,
  );

  // Handlers
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

  // Drag & Drop
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
</script>

<div class="space-y-4">
  <!-- Header con stats -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-4 text-xs text-[#8b949e]">
      <span>
        <span class="text-[#e6edf3] font-medium">{visibleCount}</span
        >/{columns.length} visibles
      </span>
      {#if pinnedLeftCount > 0 || pinnedRightCount > 0}
        <span>
          <span class="text-[#e6edf3] font-medium"
            >{pinnedLeftCount + pinnedRightCount}</span
          > fijadas
        </span>
      {/if}
    </div>
  </div>

  <!-- Search -->
  <div class="relative">
    <Search
      size={14}
      class="absolute left-3 top-1/2 -translate-y-1/2 text-[#8b949e]"
    />
    <input
      type="text"
      bind:value={searchQuery}
      placeholder="Buscar columna..."
      class="w-full pl-9 pr-3 py-2 text-sm bg-[#0d1117] border border-[#30363d] rounded-md
        text-[#e6edf3] placeholder:text-[#8b949e] focus:outline-none focus:border-[#58a6ff]"
    />
  </div>

  <!-- Quick Actions -->
  <div class="flex flex-wrap gap-2">
    <button
      onclick={showAll}
      class="px-2.5 py-1.5 text-xs font-medium text-[#238636] bg-[#238636]/10
        border border-[#238636]/30 rounded-md hover:bg-[#238636]/20 transition-colors"
    >
      Mostrar todas
    </button>
    <button
      onclick={hideAll}
      class="px-2.5 py-1.5 text-xs font-medium text-[#f85149] bg-[#f85149]/10
        border border-[#f85149]/30 rounded-md hover:bg-[#f85149]/20 transition-colors"
    >
      Ocultar todas
    </button>
    <button
      onclick={unpinAll}
      class="px-2.5 py-1.5 text-xs font-medium text-[#8b949e] bg-[#21262d]
        border border-[#30363d] rounded-md hover:border-[#8b949e] transition-colors"
    >
      Desfijar todas
    </button>
    <button
      onclick={autosizeAll}
      class="px-2.5 py-1.5 text-xs font-medium text-[#58a6ff] bg-[#58a6ff]/10
        border border-[#58a6ff]/30 rounded-md hover:bg-[#58a6ff]/20 transition-colors"
    >
      Auto-ajustar
    </button>
    <button
      onclick={resetColumns}
      class="px-2.5 py-1.5 text-xs font-medium text-[#8b949e] bg-[#21262d]
        border border-[#30363d] rounded-md hover:border-[#8b949e] transition-colors"
    >
      Reset
    </button>
  </div>

  <!-- Column List -->
  <div class="space-y-1 max-h-80 overflow-y-auto pr-1" role="list">
    {#each filteredColumns as column, index (column.id)}
      <div
        draggable="true"
        ondragstart={(e) => handleDragStart(e, index)}
        ondragover={(e) => handleDragOver(e, index)}
        ondragend={handleDragEnd}
        class="group flex items-center gap-2 p-2 rounded-md transition-all cursor-grab active:cursor-grabbing
          {draggedIndex === index
          ? 'opacity-50 scale-98 bg-[#58a6ff]/20 border border-[#58a6ff]/40'
          : 'bg-[#161b22] border border-[#30363d] hover:border-[#8b949e]'}"
        role="listitem"
      >
        <!-- Drag Handle -->
        <div
          class="text-[#484f58] group-hover:text-[#8b949e] transition-colors"
        >
          <GripVertical size={14} />
        </div>

        <!-- Visibility Toggle -->
        <button
          onclick={() => toggleVisibility(column.id)}
          class="p-1 rounded transition-colors
            {column.visible
            ? 'text-[#238636] hover:bg-[#238636]/10'
            : 'text-[#484f58] hover:bg-[#21262d]'}"
          title={column.visible ? "Ocultar" : "Mostrar"}
        >
          {#if column.visible}
            <Eye size={14} />
          {:else}
            <EyeOff size={14} />
          {/if}
        </button>

        <!-- Column Name -->
        <span
          class="flex-1 text-sm truncate transition-colors
            {column.visible ? 'text-[#e6edf3]' : 'text-[#484f58]'}"
        >
          {column.name}
        </span>

        <!-- Pin Controls -->
        <div
          class="flex items-center gap-0.5 opacity-0 group-hover:opacity-100 transition-opacity"
        >
          <button
            onclick={() =>
              setPinned(column.id, column.pinned === "left" ? null : "left")}
            class="p-1 rounded transition-colors
              {column.pinned === 'left'
              ? 'text-[#58a6ff] bg-[#58a6ff]/10'
              : 'text-[#484f58] hover:text-[#8b949e] hover:bg-[#21262d]'}"
            title="Fijar izquierda"
          >
            <ArrowLeftToLine size={12} />
          </button>
          <button
            onclick={() =>
              setPinned(column.id, column.pinned === "right" ? null : "right")}
            class="p-1 rounded transition-all
              {column.pinned === 'right'
              ? 'text-[#58a6ff] bg-[#58a6ff]/10'
              : 'text-[#484f58] hover:text-[#8b949e] hover:bg-[#21262d]'}"
            title="Fijar derecha"
          >
            <ArrowRightToLine size={12} />
          </button>
        </div>

        <!-- Pin Badge -->
        {#if column.pinned}
          <span
            class="px-1.5 py-0.5 text-[10px] font-medium rounded
              {column.pinned === 'left'
              ? 'bg-[#58a6ff]/20 text-[#58a6ff]'
              : 'bg-[#a371f7]/20 text-[#a371f7]'}"
          >
            {column.pinned === "left" ? "IZQ" : "DER"}
          </span>
        {/if}
      </div>
    {/each}

    {#if filteredColumns.length === 0}
      <div class="py-8 text-center">
        <Columns size={24} class="mx-auto text-[#484f58] mb-2" />
        <p class="text-sm text-[#8b949e]">No se encontraron columnas</p>
      </div>
    {/if}
  </div>

  <!-- Tip -->
  <p class="text-xs text-[#8b949e] text-center">
    Arrastra para reordenar • Click en el ojo para mostrar/ocultar
  </p>
</div>
