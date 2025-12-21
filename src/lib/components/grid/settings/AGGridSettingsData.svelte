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
      <Hash size={16} class="text-blue-400" />
      <h3 class="text-sm font-medium text-white">Paginación</h3>
    </div>

    <div class="grid grid-cols-3 sm:grid-cols-6 gap-2">
      {#each paginationOptions as size}
        <button
          onclick={() => handlePaginationChange(size)}
          class="py-2 px-3 text-sm rounded-lg border transition-all
            {paginationSize === size
            ? 'border-blue-500 bg-blue-500/10 text-blue-400 font-medium'
            : 'border-white/10 bg-[#252526] text-gray-300 hover:border-white/20'}"
        >
          {size}
        </button>
      {/each}
    </div>

    <p class="text-xs text-gray-500 mt-2">Registros por página en la tabla</p>
  </section>

  <!-- Quick Filter -->
  <section>
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-2">
        <Search size={16} class="text-green-400" />
        <h3 class="text-sm font-medium text-white">Búsqueda rápida</h3>
      </div>
      <label class="flex items-center gap-2 cursor-pointer">
        <span class="text-xs text-gray-400">
          {enableQuickFilter ? "Activo" : "Inactivo"}
        </span>
        <input
          type="checkbox"
          checked={enableQuickFilter}
          onchange={handleQuickFilterToggle}
          class="w-4 h-4 rounded bg-[#1e1e1e] border-white/20 text-green-500 focus:ring-green-500"
        />
      </label>
    </div>

    {#if enableQuickFilter}
      <div class="relative">
        <Search
          size={14}
          class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
        />
        <input
          type="text"
          bind:value={quickFilterText}
          oninput={handleQuickFilterChange}
          placeholder="Escribir para filtrar en todas las columnas..."
          class="w-full pl-9 pr-3 py-2.5 text-sm bg-[#252526] border border-white/10 rounded-lg
            text-white placeholder:text-gray-500 focus:outline-none focus:border-green-500/50"
        />
        {#if quickFilterText}
          <button
            onclick={() => {
              quickFilterText = "";
              handleQuickFilterChange();
            }}
            class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-white"
          >
            ×
          </button>
        {/if}
      </div>
      <p class="text-xs text-gray-500 mt-2">
        Filtra instantáneamente en todas las columnas visibles
      </p>
    {:else}
      <div
        class="p-4 rounded-lg bg-[#252526] border border-white/10 text-center"
      >
        <p class="text-sm text-gray-400">
          Activa la búsqueda rápida para filtrar en todas las columnas
        </p>
      </div>
    {/if}
  </section>

  <!-- Filtros flotantes -->
  <section>
    <div class="flex items-center gap-2 mb-3">
      <Filter size={16} class="text-purple-400" />
      <h3 class="text-sm font-medium text-white">Filtros de columna</h3>
    </div>

    <label
      class="flex items-center justify-between p-3 rounded-lg bg-[#252526] border border-white/10
        hover:border-white/20 cursor-pointer transition-colors"
    >
      <div>
        <p class="text-sm text-white">Filtros flotantes</p>
        <p class="text-xs text-gray-500">
          Muestra campo de filtro debajo de cada encabezado
        </p>
      </div>
      <input
        type="checkbox"
        checked={showFloatingFilters}
        onchange={handleFloatingFiltersChange}
        class="w-4 h-4 rounded bg-[#1e1e1e] border-white/20 text-purple-500 focus:ring-purple-500"
      />
    </label>
  </section>

  <!-- Acciones de filtros -->
  <section>
    <div class="flex items-center gap-2 mb-3">
      <Trash2 size={16} class="text-red-400" />
      <h3 class="text-sm font-medium text-white">Limpiar</h3>
    </div>

    <div class="flex gap-2">
      <button
        onclick={clearAllFilters}
        class="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-lg
          bg-red-500/10 border border-red-500/20 text-sm text-red-400
          hover:bg-red-500/20 transition-colors"
      >
        <Filter size={14} />
        Limpiar filtros
        {#if activeFiltersCount > 0}
          <span class="px-1.5 py-0.5 text-xs bg-red-500/30 rounded">
            {activeFiltersCount}
          </span>
        {/if}
      </button>
      <button
        onclick={clearSort}
        class="flex-1 flex items-center justify-center gap-2 py-2.5 rounded-lg
          bg-[#252526] border border-white/10 text-sm text-gray-300
          hover:bg-white/5 hover:border-white/20 transition-colors"
      >
        Limpiar orden
      </button>
    </div>
  </section>

  <!-- Info -->
  <div class="p-3 rounded-lg bg-[#252526] border border-white/10">
    <p class="text-xs text-gray-400">
      <strong class="text-gray-300">Tip:</strong> Usa Shift+Click en los encabezados
      para ordenar por múltiples columnas. Los filtros se combinan con AND.
    </p>
  </div>
</div>
