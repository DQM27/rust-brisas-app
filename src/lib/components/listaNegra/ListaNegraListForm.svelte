<!-- src/lib/components/listaNegra/ListaNegraListForm.svelte -->
<script lang="ts">
  import { fade } from "svelte/transition";
  import {
    AlertCircle,
    Filter,
    Plus,
    UserX,
    ShieldAlert,
    RefreshCw,
  } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { createCustomButton } from "$lib/config/agGridConfigs";

  interface Props {
    bloqueados?: ListaNegraResponse[];
    loading?: boolean;
    error?: string;
    filteredData?: ListaNegraResponse[];
    columnDefs: ColDef<ListaNegraResponse>[];
    estadoFilter?: string;
    tipoFilter?: string;
    onRefresh: () => void;
    onEstadoFilterChange: (filter: string) => void;
    onTipoFilterChange: (filter: string) => void;
    onClearAllFilters: () => void;
    onAddToBlacklist?: () => void;
    onUnblock?: (bloqueado: ListaNegraResponse) => void;
  }

  let {
    bloqueados = [],
    loading = false,
    error = "",
    filteredData = [],
    columnDefs,
    estadoFilter = "todos",
    tipoFilter = "todos",
    onRefresh,
    onEstadoFilterChange,
    onTipoFilterChange,
    onClearAllFilters,
    onAddToBlacklist,
    onUnblock,
  }: Props = $props();

  // Estado para selección
  let selectedRows = $state<ListaNegraResponse[]>([]);

  // Estados para dropdowns
  let showEstadoDropdown = $state(false);
  let showTipoDropdown = $state(false);

  // Labels de filtros
  const estadoLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      activo: "Bloqueados",
      inactivo: "Desbloqueados",
    };
    return labels[estadoFilter] || "Estado";
  });

  const tipoLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      permanente: "Permanentes",
      temporal: "Temporales",
    };
    return labels[tipoFilter] || "Tipo";
  });

  const hasActiveFilters = $derived(
    estadoFilter !== "todos" || tipoFilter !== "todos",
  );

  // Custom buttons por contexto
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    // Botones por defecto - Solo los esenciales
    // Botones por defecto - Solo los esenciales
    const defaultButtons: CustomToolbarButton[] = [];

    if (onAddToBlacklist) {
      defaultButtons.push({
        id: "add-blacklist",
        label: "Agregar a Lista Negra",
        icon: Plus,
        onClick: () => onAddToBlacklist?.(),
        variant: "danger" as const,
        tooltip: "Agregar nueva persona a lista negra",
      });
    }

    defaultButtons.push({
      id: "refresh",
      label: "Actualizar",
      icon: RefreshCw,
      onClick: onRefresh,
      variant: "default" as const,
      tooltip: "Actualizar lista",
    });

    defaultButtons.push({
      id: "filter-estado",
      label: `Estado: ${estadoLabel}`,
      icon: Filter,
      onClick: () => {
        showEstadoDropdown = !showEstadoDropdown;
        showTipoDropdown = false;
      },
      variant: (estadoFilter !== "todos" ? "primary" : "default") as
        | "primary"
        | "default",
      tooltip: "Filtrar por estado",
    });

    // Agregar Filtro de Tipo solo si está activo o hay filtros activos
    if (tipoFilter !== "todos" || hasActiveFilters) {
      defaultButtons.push({
        id: "filter-tipo",
        label: `Tipo: ${tipoLabel}`,
        icon: Filter,
        onClick: () => {
          showTipoDropdown = !showTipoDropdown;
          showEstadoDropdown = false;
        },
        variant: (tipoFilter !== "todos" ? "primary" : "default") as
          | "primary"
          | "default",
        tooltip: "Filtrar por tipo de bloqueo",
      });
    }

    // Botón para limpiar filtros solo si hay filtros activos
    if (hasActiveFilters) {
      defaultButtons.push({
        id: "clear-filters",
        label: "Limpiar",
        icon: Filter,
        onClick: () => {
          onClearAllFilters();
          showEstadoDropdown = false;
          showTipoDropdown = false;
        },
        variant: "danger" as const,
        tooltip: "Limpiar todos los filtros",
      });
    }

    return {
      default: defaultButtons,

      singleSelect: [
        ...(onUnblock
          ? [
              {
                id: "unblock",
                label: selected?.isActive ? "Desbloquear" : "Re-bloquear",
                icon: ShieldAlert,
                onClick: () => {
                  if (selected) onUnblock?.(selected);
                },
                variant: selected?.isActive
                  ? ("success" as const)
                  : ("danger" as const),
                tooltip: selected?.isActive
                  ? "Desbloquear persona"
                  : "Volver a bloquear persona",
              },
            ]
          : []),
      ],

      multiSelect: [],
    };
  });

  function handleEstadoSelect(value: string) {
    onEstadoFilterChange(value);
    showEstadoDropdown = false;
  }

  function handleTipoSelect(value: string) {
    onTipoFilterChange(value);
    showTipoDropdown = false;
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    const clickedDropdown = target.closest(".filter-dropdown-container");
    const clickedFilterButton = target.closest("[data-filter-button]");

    if (!clickedDropdown && !clickedFilterButton) {
      showEstadoDropdown = false;
      showTipoDropdown = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-surface-1">
  <!-- Header con SearchBar -->
  <div class="border-b border-surface px-6 py-4 bg-surface-2">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-primary">Lista Negra</h2>
        <p class="mt-1 text-sm text-secondary">
          Gestión de personas bloqueadas del acceso a las instalaciones
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
  <div class="flex-1 overflow-hidden relative bg-surface-1">
    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-error bg-error p-4 text-error"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar lista negra</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center bg-surface-1">
        <div class="text-center">
          <svg
            class="mx-auto h-8 w-8 animate-spin text-error"
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
          <p class="mt-4 text-sm text-secondary">Cargando lista negra...</p>
        </div>
      </div>
    {:else if bloqueados.length === 0}
      <div class="flex h-full items-center justify-center bg-surface-1">
        <div class="text-center">
          <UserX size={48} class="mx-auto text-tertiary" />
          <p class="mt-4 text-lg font-medium text-primary">
            No hay personas bloqueadas
          </p>
          <p class="mt-2 text-sm text-secondary">
            Las personas bloqueadas aparecerán aquí
          </p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="lista-negra-list"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>

  <!-- Dropdowns flotantes -->
  <div class="filter-dropdown-container">
    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-surface-2 border border-surface rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => handleEstadoSelect("todos")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {estadoFilter === 'todos' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Todos los estados
        </button>
        <button
          onclick={() => handleEstadoSelect("activo")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {estadoFilter === 'activo' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Bloqueados activos
        </button>
        <button
          onclick={() => handleEstadoSelect("inactivo")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {estadoFilter === 'inactivo' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Desbloqueados
        </button>
      </div>
    {/if}

    {#if showTipoDropdown}
      <div
        class="absolute top-16 left-52 z-50 bg-surface-2 border border-surface rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => handleTipoSelect("todos")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {tipoFilter === 'todos' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Todos los tipos
        </button>
        <button
          onclick={() => handleTipoSelect("permanente")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {tipoFilter === 'permanente' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Permanentes
        </button>
        <button
          onclick={() => handleTipoSelect("temporal")}
          class="w-full px-4 py-2 text-left text-sm text-primary hover:bg-surface-hover transition-colors
            {tipoFilter === 'temporal' ? 'bg-accent-subtle text-accent' : ''}"
        >
          Temporales
        </button>
      </div>
    {/if}
  </div>
</div>
