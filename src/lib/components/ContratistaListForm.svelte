<!-- ==========================================
// src/lib/components/contratista/ContratistaListForm.svelte
// Componente visual limpio - Solo presentación
// ========================================== -->

<script lang="ts">
  import { fade } from "svelte/transition";
  import {
    AlertCircle,
    XCircle,
    Filter,
  } from "lucide-svelte";
  import type { ContratistaResponse } from "$lib/types/contratista";
  import type { SearchResult } from "$lib/types/search.types";
  import type { DataTableColumn } from "$lib/types/dataTable";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import DataTable from "$lib/components/common/DataTable.svelte";

  // Props usando sintaxis de Svelte 5
  interface Props {
    contratistas?: ContratistaResponse[];
    loading?: boolean;
    error?: string;
    blockedContratistas?: Set<string>;
    filteredData?: ContratistaResponse[];
    stats?: { total: number; activos: number; vencidos: number; porVencer: number };
    columns: DataTableColumn<ContratistaResponse>[];
    estadoFilter?: string;
    praindFilter?: string;
    onRefresh: () => void;
    onBlock: (data: any) => Promise<void>;
    onUnblock: (data: any) => Promise<void>;
    onEstadoFilterChange: (filter: string) => void;
    onPraindFilterChange: (filter: string) => void;
    onClearAllFilters: () => void;
    onSearchSelect: (e: CustomEvent<SearchResult>) => void;
    onSearchClear: () => void;
  }

  let {
    contratistas = [],
    loading = false,
    error = "",
    blockedContratistas = new Set(),
    filteredData = [],
    stats = { total: 0, activos: 0, vencidos: 0, porVencer: 0 },
    columns,
    estadoFilter = $bindable("todos"),
    praindFilter = $bindable("todos"),
    onRefresh,
    onBlock,
    onUnblock,
    onEstadoFilterChange,
    onPraindFilterChange,
    onClearAllFilters,
    onSearchSelect,
    onSearchClear,
  }: Props = $props();

  // Handlers con type safety
  function handleEstadoFilterChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    if (target && onEstadoFilterChange) {
      onEstadoFilterChange(target.value);
    }
  }

  function handlePraindFilterChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    if (target && onPraindFilterChange) {
      onPraindFilterChange(target.value);
    }
  }
</script>

<div class="flex h-full flex-col bg-[#1e1e1e]">
  <!-- Header -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          Lista de Contratistas
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de todos los contratistas registrados
        </p>
      </div>
    </div>

    <!-- Stats -->
    <div class="mt-4 grid grid-cols-4 gap-4">
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Total</div>
        <div class="mt-1 text-2xl font-semibold text-white">{stats.total}</div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Activos</div>
        <div class="mt-1 text-2xl font-semibold text-green-400">
          {stats.activos}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">PRAIND Vencido</div>
        <div class="mt-1 text-2xl font-semibold text-red-400">
          {stats.vencidos}
        </div>
      </div>
      <div class="rounded-lg bg-[#1e1e1e] p-3 ring-1 ring-white/5">
        <div class="text-xs text-gray-400">Por Vencer</div>
        <div class="mt-1 text-2xl font-semibold text-yellow-400">
          {stats.porVencer}
        </div>
      </div>
    </div>
  </div>

  <!-- Search & Filters Bar -->
  <div class="border-b border-white/10 bg-[#252526] px-6 py-4">
    <div class="flex flex-wrap items-center gap-4">
      <!-- Tantivy Search Bar -->
      <div class="flex-1 min-w-[300px]">
        <SearchBar
          placeholder="Buscar por nombre, cédula o empresa..."
          limit={10}
          on:select={onSearchSelect}
          on:clear={onSearchClear}
        />
      </div>

      <!-- Estado Filter -->
      <div class="flex items-center gap-2">
        <Filter size={16} class="text-gray-400" />
        <select
          bind:value={estadoFilter}
          on:change={handleEstadoFilterChange}
          class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="todos">Todos los estados</option>
          <option value="activo">Activos</option>
          <option value="inactivo">Inactivos</option>
          <option value="suspendido">Suspendidos</option>
        </select>
      </div>

      <!-- PRAIND Filter -->
      <select
        bind:value={praindFilter}
        on:change={handlePraindFilterChange}
        class="rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
      >
        <option value="todos">Todos PRAIND</option>
        <option value="vigente">Vigentes</option>
        <option value="por-vencer">Por vencer (≤30 días)</option>
        <option value="vencido">Vencidos</option>
      </select>

      <!-- Clear Filters -->
      {#if estadoFilter !== "todos" || praindFilter !== "todos"}
        <button
          on:click={onClearAllFilters}
          class="flex items-center gap-2 rounded-lg border border-white/10 bg-[#1e1e1e] px-3 py-2 text-sm text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-300"
        >
          <XCircle size={14} />
          Limpiar filtros
        </button>
      {/if}
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar contratistas</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg class="mx-auto h-8 w-8 animate-spin text-blue-500" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
          </svg>
          <p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
        </div>
      </div>
    {:else if contratistas.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-600" />
          <p class="mt-4 text-lg font-medium text-gray-400">
            No hay contratistas registrados
          </p>
          <p class="mt-2 text-sm text-gray-500">
            Los contratistas aparecerán aquí una vez sean registrados
          </p>
        </div>
      </div>
    {:else}
      <!-- AG Grid DataTable con filtros avanzados, animaciones y CSV export -->
      <DataTable
        data={filteredData}
        {columns}
        storageKey="contratistas-list"
        rowSelection={false}
        pagination={true}
        paginationPageSize={20}
        paginationPageSizeSelector={[10, 20, 30, 50, 100]}
        getRowId={(row) => row.id}
        height="100%"
        toolbarConfig={{
          showColumnSelector: true,
          showExport: true,
          showAutoSize: true,
        }}
        exportConfig={{
          fileName: `contratistas-${new Date().toISOString().split("T")[0]}.csv`,
        }}
        enableAnimations={true}
        animateRows={true}
        enableAdvancedFilters={true}
      />
    {/if}
  </div>   
</div>