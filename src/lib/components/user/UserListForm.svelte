<!-- src/lib/components/user/UserListForm.svelte -->
<script lang="ts">
  import { fade } from "svelte/transition";
  import { AlertCircle, Filter } from "lucide-svelte";
  import type { UserResponse } from "$lib/types/user";
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
    users?: UserResponse[];
    loading?: boolean;
    error?: string;
    filteredData?: UserResponse[];
    columnDefs: ColDef<UserResponse>[];
    roleFilter?: string;
    estadoFilter?: string;
    onRefresh: () => void;
    onRoleFilterChange: (filter: string) => void;
    onEstadoFilterChange: (filter: string) => void;
    onClearAllFilters: () => void;
    onSearchSelect: (e: CustomEvent<SearchResult>) => void;
    onSearchClear: () => void;
    onNewUser?: () => void;
    onEditUser?: (user: UserResponse) => void;
    onViewInfo?: (user: UserResponse) => void;
    onDeleteUser?: (user: UserResponse) => void;
    onDeleteMultiple?: (users: UserResponse[]) => void;
  }

  let {
    users = [],
    loading = false,
    error = "",
    filteredData = [],
    columnDefs,
    roleFilter = "todos",
    estadoFilter = "todos",
    onRefresh,
    onRoleFilterChange,
    onEstadoFilterChange,
    onClearAllFilters,
    onSearchSelect,
    onSearchClear,
    onNewUser,
    onEditUser,
    onViewInfo,
    onDeleteUser,
    onDeleteMultiple,
  }: Props = $props();

  // Estado para selección
  let selectedRows = $state<UserResponse[]>([]);

  // Estados para dropdowns
  let showRoleDropdown = $state(false);
  let showEstadoDropdown = $state(false);

  // Obtener label del filtro actual
  const roleLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      admin: "Administradores",
      supervisor: "Supervisores",
      guardia: "Guardias",
    };
    return labels[roleFilter] || "Rol";
  });

  const estadoLabel = $derived.by(() => {
    const labels: Record<string, string> = {
      todos: "Todos",
      activo: "Activos",
      inactivo: "Inactivos",
    };
    return labels[estadoFilter] || "Estado";
  });

  const hasActiveFilters = $derived(
    roleFilter !== "todos" || estadoFilter !== "todos",
  );

  // Custom buttons por contexto
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    // Botones de filtro
    const filterButtons: CustomToolbarButton[] = [
      {
        id: "filter-role",
        label: `Rol: ${roleLabel}`,
        icon: Filter,
        onClick: () => {
          showRoleDropdown = !showRoleDropdown;
          showEstadoDropdown = false;
        },
        variant: (roleFilter !== "todos" ? "primary" : "default") as
          | "primary"
          | "default",
        tooltip: "Filtrar por rol",
      },
      {
        id: "filter-estado",
        label: `Estado: ${estadoLabel}`,
        icon: Filter,
        onClick: () => {
          showEstadoDropdown = !showEstadoDropdown;
          showRoleDropdown = false;
        },
        variant: (estadoFilter !== "todos" ? "primary" : "default") as
          | "primary"
          | "default",
        tooltip: "Filtrar por estado",
      },
    ];

    if (hasActiveFilters) {
      filterButtons.push({
        id: "clear-filters",
        label: "Limpiar Filtros",
        icon: Filter,
        onClick: () => {
          onClearAllFilters();
          showRoleDropdown = false;
          showEstadoDropdown = false;
        },
        variant: "danger" as const,
        tooltip: "Limpiar todos los filtros",
      });
    }

    return {
      default: [
        createCustomButton.nuevo(() => {
          onNewUser?.();
        }),
        createCustomButton.importar(() => {
          console.log("Importar usuarios");
        }),
      ],

      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) onEditUser?.(selected);
        }),
        createCustomButton.eliminar(() => {
          if (selected) onDeleteUser?.(selected);
        }),
      ],

      multiSelect: [
        createCustomButton.eliminar(() => {
          if (selectedRows.length > 0) onDeleteMultiple?.(selectedRows);
        }),
      ],
    };
  });

  function handleRoleSelect(value: string) {
    onRoleFilterChange(value);
    showRoleDropdown = false;
  }

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
      showRoleDropdown = false;
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
        <h2 class="text-xl font-semibold text-gray-100">Lista de Usuarios</h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión y visualización de todos los usuarios del sistema
        </p>
      </div>

      <div class="flex-1 max-w-md">
        <SearchBar
          placeholder="Buscar por nombre, cédula o email..."
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
            <div class="font-medium">Error al cargar usuarios</div>
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
          <p class="mt-4 text-sm text-gray-400">Cargando usuarios...</p>
        </div>
      </div>
    {:else if users.length === 0}
      <div class="flex h-full items-center justify-center bg-[#1e1e1e]">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">
            No hay usuarios registrados
          </p>
          <p class="mt-2 text-sm text-gray-400">
            Los usuarios aparecerán aquí una vez sean registrados
          </p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="users-list"
        {columnDefs}
        rowData={filteredData}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="users-list-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
      />
    {/if}
  </div>

  <!-- Dropdowns flotantes -->
  <div class="filter-dropdown-container">
    {#if showRoleDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        <button
          onclick={() => handleRoleSelect("todos")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {roleFilter === 'todos' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Todos los roles
        </button>
        <button
          onclick={() => handleRoleSelect("admin")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {roleFilter === 'admin' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Administradores
        </button>
        <button
          onclick={() => handleRoleSelect("supervisor")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {roleFilter === 'supervisor' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Supervisores
        </button>
        <button
          onclick={() => handleRoleSelect("guardia")}
          class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors
            {roleFilter === 'guardia' ? 'bg-blue-500/20 text-blue-400' : ''}"
        >
          Guardias
        </button>
      </div>
    {/if}

    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-44 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
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
