<script lang="ts">
  // @ts-nocheck - Svelte 5 runes are not recognized by TS
  import type { GridId } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { Filter, Search, Hash, Trash2 } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Estado
  let paginationSize = $derived(agGridSettings.getPaginationSize(gridId));
  let showFloatingFilters = $derived(
    agGridSettings.getShowFloatingFilters(gridId),
  );
  let enableQuickFilter = $derived(agGridSettings.getEnableQuickFilter(gridId));
  let quickFilterText = $state("");

  // Opciones de paginación
  const paginationOptions = [10, 20, 50, 100, 200, 500];

  // Handlers
  function handlePaginationChange(size: number) {
    paginationSize = size;
    agGridSettings.setPaginationSize(gridId, size);

    if (gridApi) {
      gridApi.setGridOption("paginationPageSize", size);
    }
  }

  function handleFloatingFiltersChange() {
    showFloatingFilters = !showFloatingFilters;
    agGridSettings.setShowFloatingFilters(gridId, showFloatingFilters);

    if (gridApi) {
      const currentDefaultColDef = gridApi.getGridOption("defaultColDef");
      gridApi.setGridOption("defaultColDef", {
        ...currentDefaultColDef,
        floatingFilter: showFloatingFilters,
      });
      gridApi.refreshHeader();
    }
  }

  function handleQuickFilterToggle() {
    enableQuickFilter = !enableQuickFilter;
    agGridSettings.setEnableQuickFilter(gridId, enableQuickFilter);

    if (!enableQuickFilter && gridApi) {
      gridApi.setGridOption("quickFilterText", "");
      quickFilterText = "";
    }
  }

  function handleQuickFilterChange() {
    if (gridApi && enableQuickFilter) {
      gridApi.setGridOption("quickFilterText", quickFilterText);
    }
  }

  function clearAllFilters() {
    if (gridApi) {
      gridApi.setFilterModel(null);
      gridApi.setGridOption("quickFilterText", "");
      quickFilterText = "";
    }
  }

  function clearSort() {
    if (gridApi) {
      gridApi.applyColumnState({ defaultState: { sort: null } });
    }
  }

  // Info de filtros activos
  const activeFiltersCount = $derived.by(() => {
    if (!gridApi) return 0;
    const filterModel = gridApi.getFilterModel();
    return filterModel ? Object.keys(filterModel).length : 0;
  });
</script>

<div class="space-y-6">
  <!-- Paginación -->
  <section>
    <div class="flex items-center gap-2 mb-3">
      <Hash size={16} class="text-[#58a6ff]" />
      <h3 class="text-sm font-medium text-[#e6edf3]">Paginación</h3>
    </div>

    <div class="grid grid-cols-3 sm:grid-cols-6 gap-2">
      {#each paginationOptions as size}
        <button
          onclick={() => handlePaginationChange(size)}
          class="py-2 px-3 text-sm rounded-md border transition-all
            {paginationSize === size
            ? 'border-[#58a6ff] bg-[#58a6ff]/10 text-[#58a6ff] font-medium'
            : 'border-[#30363d] bg-[#161b22] text-[#8b949e] hover:border-[#8b949e]'}"
        >
          {size}
        </button>
      {/each}
    </div>

    <p class="text-xs text-[#8b949e] mt-2">Registros por página en la tabla</p>
  </section>

  <!-- Quick Filter -->
  <section>
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-2">
        <Search size={16} class="text-[#238636]" />
        <h3 class="text-sm font-medium text-[#e6edf3]">Búsqueda rápida</h3>
      </div>
      <label class="flex items-center gap-2 cursor-pointer">
        <span class="text-xs text-[#8b949e]">
          {enableQuickFilter ? "Activo" : "Inactivo"}
        </span>
        <input
          type="checkbox"
          checked={enableQuickFilter}
          onchange={handleQuickFilterToggle}
          class="w-4 h-4 rounded bg-[#0d1117] border-[#30363d] text-[#238636] focus:ring-[#238636]"
        />
      </label>
    </div>

    {#if enableQuickFilter}
      <div class="relative">
        <Search
          size={14}
          class="absolute left-3 top-1/2 -translate-y-1/2 text-[#8b949e]"
        />
        <input
          type="text"
          bind:value={quickFilterText}
          oninput={handleQuickFilterChange}
          placeholder="Escribir para filtrar en todas las columnas..."
          class="w-full pl-9 pr-3 py-2.5 text-sm bg-[#0d1117] border border-[#30363d] rounded-md
            text-[#e6edf3] placeholder:text-[#8b949e] focus:outline-none focus:border-[#238636]/50"
        />
        {#if quickFilterText}
          <button
            onclick={() => {
              quickFilterText = "";
              handleQuickFilterChange();
            }}
            class="absolute right-3 top-1/2 -translate-y-1/2 text-[#8b949e] hover:text-[#e6edf3]"
          >
            ×
          </button>
        {/if}
      </div>
      <p class="text-xs text-[#8b949e] mt-2">
        Filtra instantáneamente en todas las columnas visibles
      </p>
    {:else}
      <div
        class="p-4 rounded-md bg-[#161b22] border border-[#30363d] text-center"
      >
        <p class="text-sm text-[#8b949e]">
          Activa la búsqueda rápida para filtrar en todas las columnas
        </p>
      </div>
    {/if}
  </section>

  <!-- Filtros flotantes -->
  <section>
    <div class="flex items-center gap-2 mb-3">
      <Filter size={16} class="text-[#a371f7]" />
      <h3 class="text-sm font-medium text-[#e6edf3]">Filtros de columna</h3>
    </div>

    <label
      class="flex items-center justify-between p-3 rounded-md bg-[#161b22] border border-[#30363d]
        hover:border-[#8b949e] cursor-pointer transition-colors"
    >
      <div>
        <p class="text-sm text-[#e6edf3]">Filtros flotantes</p>
        <p class="text-xs text-[#8b949e]">
          Muestra campo de filtro debajo de cada encabezado
        </p>
      </div>
      <input
        type="checkbox"
        checked={showFloatingFilters}
        onchange={handleFloatingFiltersChange}
        class="w-4 h-4 rounded bg-[#0d1117] border-[#30363d] text-[#a371f7] focus:ring-[#a371f7]"
      />
    </label>
  </section>

  <!-- Acciones de filtros -->
  <section>
    <div class="flex items-center gap-2 mb-3">
      <Trash2 size={16} class="text-[#f85149]" />
      <h3 class="text-sm font-medium text-[#e6edf3]">Limpiar</h3>
    </div>

    <div class="flex gap-2">
      <button
        onclick={clearAllFilters}
        class="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-md
          bg-[#f85149]/10 border border-[#f85149]/20 text-sm text-[#f85149]
          hover:bg-[#f85149]/20 transition-colors"
      >
        <Filter size={14} />
        Limpiar filtros
        {#if activeFiltersCount > 0}
          <span class="px-1.5 py-0.5 text-xs bg-[#f85149]/30 rounded">
            {activeFiltersCount}
          </span>
        {/if}
      </button>
      <button
        onclick={clearSort}
        class="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-md
          bg-[#21262d] border border-[#30363d] text-sm text-[#8b949e]
          hover:border-[#8b949e] transition-colors"
      >
        Limpiar orden
      </button>
    </div>
  </section>

  <!-- Info -->
  <div class="p-3 rounded-md bg-[#161b22] border border-[#30363d]">
    <p class="text-xs text-[#8b949e]">
      <strong class="text-[#e6edf3]">Tip:</strong> Usa Shift+Click en los encabezados
      para ordenar por múltiples columnas. Los filtros se combinan con AND.
    </p>
  </div>
</div>
