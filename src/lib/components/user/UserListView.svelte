<!-- src/lib/components/user/UserListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import UserListForm from "./UserListForm.svelte";
  import * as userService from "$lib/logic/user/userService";
  import { openTab } from "$lib/stores/tabs";
  import { UserPlus, Edit } from "lucide-svelte";
  import type { UserResponse } from "$lib/types/user";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import {
    createUserListLogic,
    UserListLogic,
  } from "$lib/logic/user/userListLogic";

  import { selectedSearchStore } from "$lib/stores/searchStore";
  import { toast } from "svelte-5-french-toast";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // Estado local
  let users = $state<UserResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let isUpdatingStatus = false;

  // Lógica de presentación
  const listLogic = createUserListLogic();
  const listState = listLogic.getState();

  // Datos filtrados
  let filteredData = $derived.by(() => {
    const _search = $selectedSearchStore;
    return listLogic.getFilteredData(users);
  });

  // --- Acciones de Estado ---
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

      const toastId = toast.loading("Actualizando estado...");

      const result = await userService.changeStatus(id, newStatus);

      if (result.ok) {
        toast.success(`Estado actualizado`, { id: toastId });
      } else {
        // Revertir cambios si falla
        users = oldUsers;
        toast.error(`Error: ${result.error}`, { id: toastId });
      }
    } finally {
      isUpdatingStatus = false;
    }
  }

  // Convertir columnas a ColDef de AG Grid
  let columnDefs = $derived.by((): ColDef<UserResponse>[] => {
    const cols = UserListLogic.getColumns(handleStatusChange);
    return cols.map(
      (col) =>
        ({
          field: String(col.field) as any,
          headerName: col.headerName,
          width: col.width,
          minWidth: col.minWidth,
          flex: col.flex,
          sortable: col.sortable !== false,
          filter: true,
          resizable: true,
          cellRenderer: col.cellRenderer,
          valueFormatter: col.valueFormatter,
          cellStyle: col.cellStyle,
          onCellClicked: col.onCellClicked,
          pinned: col.pinned,
        }) as ColDef<UserResponse>,
    );
  });

  // --- Cargar datos ---
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
      console.error("Error al cargar usuarios:", err);
      error = "Error al cargar usuarios";
    }

    loading = false;
  }

  // --- Acciones ---
  function handleNewUser() {
    openTab({
      componentKey: "user-register",
      title: "Nuevo Usuario",
      data: {},
    });
  }

  function handleEditUser(user: UserResponse) {
    openTab({
      componentKey: "user-profile",
      title: `Editar: ${user.nombre}`,
      data: { userId: user.id },
    });
  }

  function handleViewInfo(user: UserResponse) {
    console.log("Ver información de:", user);
    // TODO: Abrir panel lateral o modal
  }

  function handleViewHistory(user: UserResponse) {
    console.log("Ver historial de:", user);
    // TODO: Abrir tab de historial
  }

  function handleDeleteUser(user: UserResponse) {
    console.log("Eliminar:", user);
    // TODO: Confirmación y eliminar
  }

  function handleDeleteMultiple(users: UserResponse[]) {
    console.log("Eliminar múltiples:", users.length);
    // TODO: Confirmación y eliminar
  }

  // --- Filtros ---
  function handleRoleFilterChange(filter: string) {
    listLogic.setRoleFilter(filter as any);
    listState.roleFilter = filter as any;
  }

  function handleEstadoFilterChange(filter: string) {
    listLogic.setEstadoFilter(filter as any);
    listState.estadoFilter = filter as any;
  }

  function handleClearAllFilters() {
    listLogic.clearAllFilters();
    listState.roleFilter = "todos";
    listState.estadoFilter = "todos";
  }

  // --- Búsqueda ---
  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    console.log("Usuario seleccionado:", e.detail);
  }

  function handleSearchClear() {
    console.log("Búsqueda limpiada");
  }

  onMount(() => {
    loadUsers();
  });
</script>

<UserListForm
  {users}
  {loading}
  {error}
  {filteredData}
  {columnDefs}
  roleFilter={listState.roleFilter}
  estadoFilter={listState.estadoFilter}
  onRefresh={loadUsers}
  onRoleFilterChange={handleRoleFilterChange}
  onEstadoFilterChange={handleEstadoFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
  onNewUser={handleNewUser}
  onEditUser={handleEditUser}
  onViewInfo={handleViewInfo}
  onViewHistory={handleViewHistory}
  onDeleteUser={handleDeleteUser}
  onDeleteMultiple={handleDeleteMultiple}
/>
