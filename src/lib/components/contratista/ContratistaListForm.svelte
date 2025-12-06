<!-- src/lib/components/contratista/ContratistaListForm.svelte -->
<script lang="ts">
  import { fade } from "svelte/transition";
  import { AlertCircle, Filter, Trash2 } from "lucide-svelte";
  import type { ContratistaResponse } from "$lib/types/contratista";
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
    contratistas?: ContratistaResponse[];
    loading?: boolean;
    error?: string;
    filteredData?: ContratistaResponse[];
    blockedContratistas?: Set<string>;
    columnDefs: ColDef<ContratistaResponse>[];
    estadoFilter?: string;
    praindFilter?: string;
    onRefresh: () => void;
    onReindex?: () => void;
    onEstadoFilterChange: (filter: string) => void;
    onPraindFilterChange: (filter: string) => void;
    onClearAllFilters: () => void;
    onSearchSelect: (e: CustomEvent<SearchResult>) => void;
    onSearchClear: () => void;
    onNewContratista?: () => void;
    onEditContratista?: (contratista: ContratistaResponse) => void;
    onViewInfo?: (contratista: ContratistaResponse) => void;
    onViewHistory?: (contratista: ContratistaResponse) => void;
    onViewVehicles?: (contratista: ContratistaResponse) => void;
    onViewBadges?: (contratista: ContratistaResponse) => void;
    onDeleteContratista?: (contratista: ContratistaResponse) => void;
    onDeleteMultiple?: (contratistas: ContratistaResponse[]) => void;
  }

  let {
    contratistas = [],
    loading = false,
    error = "",
    filteredData = [],
    columnDefs,
    estadoFilter = "todos",
    praindFilter = "todos",
    onRefresh,
    onReindex,
    onEstadoFilterChange,
    onPraindFilterChange,
    onClearAllFilters,
    onSearchSelect,
    onSearchClear,
    onNewContratista,
    onEditContratista,
    onViewInfo,
    onViewHistory,
    onViewVehicles,
    onViewBadges,
    onDeleteContratista,
    onDeleteMultiple,
  }: Props = $props();

  // Estado para selección
  let selectedRows = $state<ContratistaResponse[]>([]);

  // Estados para dropdowns
  let showEstadoDropdown = $state(false);
  let showPraindDropdown = $state(false);

  // Obtener label del filtro actual
  const estadoLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      activo: "Activos",
      inactivo: "Inactivos",
      suspendido: "Suspendidos",
    };
    return labels[estadoFilter] || "Estado";
  });

  const praindLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      vigente: "Vigentes",
      "por-vencer": "Por vencer",
      vencido: "Vencidos",
    };
    return labels[praindFilter] || "PRAIND";
  });

  const hasActiveFilters = $derived(
    estadoFilter !== "todos" || praindFilter !== "todos",
  );

  // Custom buttons por contexto
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    // Botones de filtro para el contexto default
    const filterButtons: CustomToolbarButton[] = [
      // Filtro de Estado
      {
        id: "filter-estado",
        label: `Estado: ${estadoLabel}`,
        icon: Filter,
        onClick: () => {
          showEstadoDropdown = !showEstadoDropdown;
          showPraindDropdown = false;
        },
        variant: (estadoFilter !== "todos" ? "primary" : "default") as
          | "primary"
          | "default",
        tooltip: "Filtrar por estado",
      },
      // Filtro de PRAIND
      {
        id: "filter-praind",
        label: `PRAIND: ${praindLabel}`,
        icon: Filter,
        onClick: () => {
          showPraindDropdown = !showPraindDropdown;
          showEstadoDropdown = false;
        },
        variant: (praindFilter !== "todos" ? "primary" : "default") as
          | "primary"
          | "default",
        tooltip: "Filtrar por PRAIND",
      },
    ];

    // Si hay filtros activos, agregar botón de limpiar
    if (hasActiveFilters) {
      filterButtons.push({
        id: "clear-filters",
        label: "Limpiar Filtros",
        icon: Filter,
        onClick: () => {
          onClearAllFilters();
          showEstadoDropdown = false;
          showPraindDropdown = false;
        },
        variant: "danger" as const,
        tooltip: "Limpiar todos los filtros",
      });
    }

    return {
      default: [
        createCustomButton.nuevo(() => {
          onNewContratista?.();
        }),
        // Botones estándar movidos aquí para estar DESPUÉS de Nuevo
        ...COMMON_DEFAULT_BUTTONS.filter((b) =>
          ["autosize-all", "reset-columns", "select-all"].includes(b.id),
        ).map((b) => ({
          id: b.id,
          label: b.label,
          icon: b.icon,
          tooltip: b.tooltip,
          onClick: undefined, // undefined explícito para que AGGridToolbar use el handler común
          useCommonHandler: true, // Flag para indicar que es un botón estándar
        })),
        createCustomButton.importar(() => {
          console.log("Importar contratistas");
        }),
        {
          id: "reindex",
          label: "Reindexar",
          icon: AlertCircle,
          onClick: () => onReindex?.(),
          variant: "default" as const,
          tooltip: "Reparar índice de búsqueda",
        },
      ],

      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) onEditContratista?.(selected);
        }),
        createCustomButton.historial(() => {
          if (selected) onViewHistory?.(selected);
        }),
      ],

      multiSelect: [
        {
          id: "delete-multiple",
          label: "Eliminar Seleccionados",
          icon: Trash2,
          onClick: () => {
            onDeleteMultiple?.(selectedRows);
          },
          variant: "danger" as const,
          tooltip: `Eliminar ${selectedRows.length} contratistas`,
        },
      ],
    };
  });

  function handleEstadoSelect(value: string) {
    onEstadoFilterChange(value);
    showEstadoDropdown = false;
  }

  function handlePraindSelect(value: string) {
    onPraindFilterChange(value);
    showPraindDropdown = false;
  }

  // Cerrar dropdowns al hacer click fuera
  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;

    // No cerrar si se hizo click en:
    // 1. Los dropdowns mismos
    // 2. Los botones de filtro (buscar por el texto del botón o el contenedor del botón)
    const clickedDropdown = target.closest(".filter-dropdown-container");
    const clickedFilterButton = target.closest("[data-filter-button]");

    if (!clickedDropdown && !clickedFilterButton) {
      showEstadoDropdown = false;
      showPraindDropdown = false;
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
          Lista de Contratistas
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de todos los contratistas registrados
        </p>
      </div>

      <div class="flex-1 max-w-md">
        <SearchBar
          placeholder="Buscar por nombre, cédula o empresa..."
          limit={10}
          on:select={onSearchSelect}
          on:clear={onSearchClear}
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
            <div class="font-medium">Error al cargar contratistas</div>
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
          <p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
        </div>
      </div>
    {:else if contratistas.length === 0}
      <div class="flex h-full items-center justify-center bg-[#1e1e1e]">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">
            No hay contratistas registrados
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Los contratistas aparecerán aquí una vez sean registrados
          </p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="contratista-list"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="contratistas-list-columns"
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
        <button
          onclick={() => handleEstadoSelect("suspendido")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {estadoFilter === 'suspendido'
            ? 'bg-blue-500/20 text-blue-400'
            : ''}"
        >
          Suspendidos
        </button>
      </div>
    {/if}

    {#if showPraindDropdown}
      <div
        class="absolute top-16 left-52 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => handlePraindSelect("todos")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {praindFilter === 'todos' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Todos PRAIND
        </button>
        <button
          onclick={() => handlePraindSelect("vigente")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {praindFilter === 'vigente' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Vigentes
        </button>
        <button
          onclick={() => handlePraindSelect("por-vencer")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {praindFilter === 'por-vencer'
            ? 'bg-blue-500/20 text-blue-400'
            : ''}"
        >
          Por vencer (≤30 días)
        </button>
        <button
          onclick={() => handlePraindSelect("vencido")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {praindFilter === 'vencido' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Vencidos
        </button>
      </div>
    {/if}
  </div>
</div>
