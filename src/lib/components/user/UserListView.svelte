<!-- src/lib/components/user/UserListView.svelte -->
<!-- Vista unificada: Lista de usuarios + Modal para CRUD -->
<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { AlertCircle, Filter, RotateCw } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  // Components
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import UserFormModal from "./UserFormModal.svelte";

  // Logic & Config
  import * as userService from "$lib/logic/user/userService";
  import { UserColumns } from "$lib/logic/user/userColumns";
  import { createCustomButton } from "$lib/config/agGridConfigs";

  // Types
  import type {
    UserResponse,
    CreateUserInput,
    UpdateUserInput,
  } from "$lib/types/user";
  import type { CustomToolbarButton } from "$lib/types/agGrid";

  // Stores
  import { selectedSearchStore } from "$lib/stores/searchStore";
  import { currentUser } from "$lib/stores/auth";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // ==========================================
  // ESTADO LOCAL
  // ==========================================
  let users = $state<UserResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let isUpdatingStatus = false;

  // Estado de filtros (inline, sin clase externa)
  let roleFilter = $state<"todos" | "admin" | "supervisor" | "guardia">(
    "todos",
  );
  let estadoFilter = $state<"todos" | "activo" | "inactivo">("todos");

  // Estado para selección en grid
  let selectedRows = $state<UserResponse[]>([]);

  // Estado para dropdowns de filtros
  let showRoleDropdown = $state(false);
  let showEstadoDropdown = $state(false);

  // Estado para modal
  let showModal = $state(false);
  let editingUser = $state<UserResponse | null>(null);
  let modalLoading = $state(false);

  // ==========================================
  // DERIVADOS
  // ==========================================

  // Datos filtrados (reemplaza UserListLogic.getFilteredData)
  let filteredData = $derived.by(() => {
    let filtered = users;

    // Filtro por búsqueda global (prioridad)
    const selectedSearch = $selectedSearchStore;
    if (selectedSearch.result) {
      return filtered.filter((u) => u.id === selectedSearch.result!.id);
    }

    // Filtro de rol
    if (roleFilter !== "todos") {
      filtered = filtered.filter((u) => u.role === roleFilter);
    }

    // Filtro de estado
    if (estadoFilter !== "todos") {
      const isActive = estadoFilter === "activo";
      filtered = filtered.filter((u) => u.isActive === isActive);
    }

    return filtered;
  });

  // Labels de filtros
  const roleLabel = $derived(
    {
      todos: "Todos",
      admin: "Admins",
      supervisor: "Supervisores",
      guardia: "Guardias",
    }[roleFilter],
  );
  const estadoLabel = $derived(
    { todos: "Todos", activo: "Activos", inactivo: "Inactivos" }[estadoFilter],
  );
  const hasActiveFilters = $derived(
    roleFilter !== "todos" || estadoFilter !== "todos",
  );

  // Columnas de AG Grid (usando helper estático existente)
  let columnDefs = $derived.by((): ColDef<UserResponse>[] => {
    return UserColumns.getColumns(handleStatusChange);
  });

  // Botones personalizados por contexto
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    return {
      default: [
        createCustomButton.nuevo(() => openModal(null)),
        {
          id: "refresh",
          label: "Actualizar",
          icon: RotateCw,
          onClick: loadUsers,
          variant: "default" as const,
          tooltip: "Recargar lista de usuarios",
        },
      ],
      singleSelect: [
        createCustomButton.editar(() => {
          if (selected) openModal(selected);
        }),
        createCustomButton.eliminar(() => {
          if (selected) handleDeleteUser(selected);
        }),
      ],
      multiSelect: [
        createCustomButton.eliminar(() => {
          if (selectedRows.length > 0) handleDeleteMultiple(selectedRows);
        }),
      ],
    };
  });

  // ==========================================
  // HANDLERS - DATA
  // ==========================================

  async function loadUsers() {
    loading = true;
    error = "";
    try {
      const result = await userService.fetchAllUsers();
      if (result.ok) {
        users = result.data;
      } else {
        error = result.error;
      }
    } catch (err) {
      error = "Error al cargar usuarios";
    }
    loading = false;
  }

  // ==========================================
  // HANDLERS - MODAL
  // ==========================================

  function openModal(user: UserResponse | null) {
    editingUser = user;
    showModal = true;
  }

  function closeModal() {
    showModal = false;
    editingUser = null;
  }

  async function handleSaveUser(
    data: CreateUserInput | UpdateUserInput,
  ): Promise<boolean> {
    modalLoading = true;
    try {
      if (editingUser) {
        // Modo edición
        const result = await userService.updateUser(
          editingUser.id,
          data as UpdateUserInput,
        );
        if (result.ok) {
          toast.success("Usuario actualizado");
          users = users.map((u) =>
            u.id === editingUser!.id ? result.data : u,
          );
          // closeModal(); // Dejamos que el modal controle el cierre
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      } else {
        // Modo creación
        const result = await userService.createUser(data as CreateUserInput);
        if (result.ok) {
          toast.success("Usuario creado");
          await loadUsers(); // Recargar para obtener el nuevo usuario
          // closeModal(); // Dejamos que el modal controle el cierre (para mostrar password)
          return true;
        } else {
          toast.error(result.error);
          return false;
        }
      }
    } catch (e) {
      console.error(e);
      toast.error("Error inesperado");
      return false;
    } finally {
      modalLoading = false;
    }
  }

  // ==========================================
  // HANDLERS - STATUS
  // ==========================================

  async function handleStatusChange(id: string, currentStatus: boolean) {
    if (loading || isUpdatingStatus) return;
    try {
      isUpdatingStatus = true;
      const newStatus = !currentStatus;

      // Actualización optimista
      const oldUsers = [...users];
      users = users.map((u) =>
        u.id === id ? { ...u, isActive: newStatus } : u,
      );

      const toastId = toast.loading("Actualizando...");
      const result = await userService.changeStatus(id, newStatus);

      if (result.ok) {
        toast.success("Estado actualizado", { id: toastId });
      } else {
        users = oldUsers;
        toast.error(result.error, { id: toastId });
      }
    } finally {
      isUpdatingStatus = false;
    }
  }

  // ==========================================
  // HANDLERS - DELETE
  // ==========================================

  async function handleDeleteUser(user: UserResponse) {
    if (!confirm(`¿Eliminar a ${user.nombre}?`)) return;
    const toastId = toast.loading("Eliminando...");
    const result = await userService.deleteUser(user.id);
    if (result.ok) {
      toast.success("Usuario eliminado", { id: toastId });
      users = users.filter((u) => u.id !== user.id);
    } else {
      toast.error(result.error, { id: toastId });
    }
  }

  async function handleDeleteMultiple(usersToDelete: UserResponse[]) {
    if (!confirm(`¿Eliminar ${usersToDelete.length} usuarios?`)) return;
    const toastId = toast.loading("Eliminando...");
    let errors = 0;
    for (const u of usersToDelete) {
      const res = await userService.deleteUser(u.id);
      if (!res.ok) errors++;
    }
    if (errors === 0) {
      toast.success("Usuarios eliminados", { id: toastId });
    } else {
      toast.error(`Errores: ${errors}`, { id: toastId });
    }
    loadUsers();
  }

  // ==========================================
  // HANDLERS - FILTROS
  // ==========================================

  function handleRoleSelect(
    value: "todos" | "admin" | "supervisor" | "guardia",
  ) {
    roleFilter = value;
    showRoleDropdown = false;
  }

  function handleEstadoSelect(value: "todos" | "activo" | "inactivo") {
    estadoFilter = value;
    showEstadoDropdown = false;
  }

  function clearFilters() {
    roleFilter = "todos";
    estadoFilter = "todos";
    selectedSearchStore.clear();
    showRoleDropdown = false;
    showEstadoDropdown = false;
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (
      !target.closest(".filter-dropdown-container") &&
      !target.closest("[data-filter-button]")
    ) {
      showRoleDropdown = false;
      showEstadoDropdown = false;
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================

  // Sincronizar cambios del usuario actual (ej: edición desde Sidebar)
  $effect(() => {
    if ($currentUser && users.length > 0) {
      const index = users.findIndex((u) => u.id === $currentUser.id);
      if (index !== -1) {
        // Verificar si hay cambios reales para evitar reactividad innecesaria
        if (JSON.stringify(users[index]) !== JSON.stringify($currentUser)) {
          users[index] = $currentUser;
        }
      }
    }
  });

  onMount(() => {
    loadUsers();
  });
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header -->
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
      <div class="flex h-full items-center justify-center">
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
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-gray-400" />
          <p class="mt-4 text-lg font-medium text-gray-300">No hay usuarios</p>
          <p class="mt-2 text-sm text-gray-400">
            Crea el primer usuario para comenzar
          </p>
          <button
            onclick={() => openModal(null)}
            class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
          >
            Nuevo Usuario
          </button>
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

  <!-- Dropdowns de Filtros -->
  <div class="filter-dropdown-container">
    {#if showRoleDropdown}
      <div
        class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos los roles"], ["admin", "Administradores"], ["supervisor", "Supervisores"], ["guardia", "Guardias"]] as [value, label]}
          <button
            onclick={() => handleRoleSelect(value as any)}
            class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {roleFilter ===
            value
              ? 'bg-blue-500/20 text-blue-400'
              : ''}"
          >
            {label}
          </button>
        {/each}
      </div>
    {/if}

    {#if showEstadoDropdown}
      <div
        class="absolute top-16 left-44 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
        transition:fade={{ duration: 150 }}
      >
        {#each [["todos", "Todos"], ["activo", "Activos"], ["inactivo", "Inactivos"]] as [value, label]}
          <button
            onclick={() => handleEstadoSelect(value as any)}
            class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {estadoFilter ===
            value
              ? 'bg-blue-500/20 text-blue-400'
              : ''}"
          >
            {label}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Modal para Crear/Editar -->
<UserFormModal
  show={showModal}
  user={editingUser}
  loading={modalLoading}
  onSave={handleSaveUser}
  onClose={closeModal}
/>
