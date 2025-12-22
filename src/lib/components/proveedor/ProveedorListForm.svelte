<!-- src/lib/components/proveedor/ProveedorListForm.svelte -->
<script lang="ts">
  import { fade } from "svelte/transition";
  import { AlertCircle, Filter } from "lucide-svelte";
  import type { ProveedorResponse } from "$lib/types/proveedor";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import {
    createCustomButton,
    COMMON_DEFAULT_BUTTONS,
  } from "$lib/config/agGridConfigs";

  interface Props {
    proveedores?: ProveedorResponse[];
    loading?: boolean;
    error?: string;
    filteredData?: ProveedorResponse[];
    columnDefs: ColDef<ProveedorResponse>[];
    estadoFilter?: string;
    onRefresh: () => void;
    onEstadoFilterChange: (filter: string) => void;
    onClearAllFilters: () => void;
    onNewProveedor?: () => void;
    onEditProveedor?: (proveedor: ProveedorResponse) => void;
  }

  let {
    proveedores = [],
    loading = false,
    error = "",
    filteredData = [],
    columnDefs,
    estadoFilter = "todos",
    onRefresh,
    onEstadoFilterChange,
    onClearAllFilters,
    onNewProveedor,
    onEditProveedor,
  }: Props = $props();

  // Estado para selección
  let selectedRows = $state<ProveedorResponse[]>([]);

  // Estados para dropdowns
  let showEstadoDropdown = $state(false);

  // Obtener label del filtro actual
  const estadoLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      activo: "Activos",
      inactivo: "Inactivos",
    };
    return labels[estadoFilter] || "Estado";
  });

  const hasActiveFilters = $derived(estadoFilter !== "todos");

  // Custom buttons por contexto
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    return {
      default: [
        createCustomButton.nuevo(() => {
          onNewProveedor?.();
        }),
        // Botones estándar
        ...COMMON_DEFAULT_BUTTONS.filter((b) =>
          ["autosize-all", "reset-columns", "select-all"].includes(b.id),
        ).map((b) => ({
          id: b.id,
          label: b.label,
          icon: b.icon,
          tooltip: b.tooltip,
          onClick: undefined,
          useCommonHandler: true,
        })),
      ],

      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) onEditProveedor?.(selected);
        }),
      ],

      multiSelect: [],
    };
  });

  function handleEstadoSelect(value: string) {
    onEstadoFilterChange(value);
    showEstadoDropdown = false;
  }

  // Cerrar dropdowns al hacer click fuera
  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    const clickedDropdown = target.closest(".filter-dropdown-container");
    const clickedFilterButton = target.closest("[data-filter-button]");

    if (!clickedDropdown && !clickedFilterButton) {
      showEstadoDropdown = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header con SearchBar -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          Catálogo de Proveedores
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de proveedores registrados
        </p>
      </div>

      <div class="flex-1 max-w-md">
        <SearchBar
          placeholder="Buscar por nombre, cédula o empresa..."
          limit={10}
        />
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar proveedores</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center bg-[#1e1e1e]">
        <div class="text-center">
          <svg
            class="mx-auto h-8 w-8 animate-spin text-blue-500"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <p class="mt-4 text-sm text-gray-400">Cargando proveedores...</p>
        </div>
      </div>
    {:else if proveedores.length === 0}
      <div class="flex h-full items-center justify-center bg-[#1e1e1e]">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">
            No hay proveedores registrados
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Los proveedores aparecerán aquí una vez sean registrados
          </p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="proveedor-list"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="proveedores-list-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>

  <!-- Dropdowns flotantes -->
  <div class="filter-dropdown-container">
    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => handleEstadoSelect("todos")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {estadoFilter === 'todos' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Todos los estados
        </button>
        <button
          onclick={() => handleEstadoSelect("activo")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {estadoFilter === 'activo' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Activos
        </button>
        <button
          onclick={() => handleEstadoSelect("inactivo")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {estadoFilter === 'inactivo' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Inactivos
        </button>
      </div>
    {/if}
  </div>
</div>
