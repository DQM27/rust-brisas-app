<!-- src/lib/components/listaNegra/ListaNegraListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import ListaNegraListForm from "./ListaNegraListForm.svelte";
  import ListaNegraForm from "./ListaNegraForm.svelte";
  import BlacklistConfirmModal from "./blacklistForm/BlacklistConfirmModal.svelte";
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";
  import { selectedSearchStore } from "$lib/stores/searchStore";
  import type {
    ListaNegraResponse,
    AddToListaNegraInput,
  } from "$lib/types/listaNegra";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import {
    createListaNegraListLogic,
    ListaNegraListLogic,
  } from "$lib/logic/listaNegra/listaNegraListLogic";
  import { currentUser } from "$lib/stores/auth";
  import { can } from "$lib/logic/permissions";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // Estado local
  let bloqueados = $state<ListaNegraResponse[]>([]);
  let loading = $state(false);
  let error = $state("");

  // Permisos derivados
  let canManage = $derived(can($currentUser, "MANAGE_BLACKLIST"));
  let canViewReason = $derived(can($currentUser, "VIEW_BLACKLIST_REASON"));

  // Modales
  let showAddModal = $state(false);
  let selectedBloqueado = $state<ListaNegraResponse | null>(null);
  let addFormLoading = $state(false);

  // Estados para modal de desbloqueo
  let showUnblockModal = $state(false);
  let motivoDesbloqueo = $state("");
  let observacionesDesbloqueo = $state("");

  // Lógica de presentación
  const listLogic = createListaNegraListLogic();
  const listState = listLogic.getState();

  // Datos filtrados - Reactivo al store de búsqueda
  let filteredData = $derived.by(() => {
    // Forzar reactividad al store
    $selectedSearchStore;
    return listLogic.getFilteredData(bloqueados);
  });

  // Convertir columnas a ColDef de AG Grid
  let columnDefs = $derived.by((): ColDef<ListaNegraResponse>[] => {
    const cols = ListaNegraListLogic.getColumns();
    return cols
      .filter((col) => {
        if (col.field === "motivoBloqueo" && !canViewReason) return false;
        return true;
      })
      .map(
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
          }) as ColDef<ListaNegraResponse>,
      );
  });

  // --- Cargar datos ---
  async function loadListaNegra() {
    loading = true;
    error = "";

    const result = await listaNegraService.fetchAll();

    if (result.ok && result.data) {
      bloqueados = result.data.bloqueados;
    } else if (!result.ok) {
      error = result.error;
    } else {
      error = "Error al cargar lista negra";
    }

    loading = false;
  }

  // --- Acciones ---
  function handleAddToBlacklist() {
    showAddModal = true;
  }

  async function handleAddSubmit(input: AddToListaNegraInput) {
    addFormLoading = true;

    const result = await listaNegraService.add(input);

    if (result.ok) {
      await loadListaNegra();
      showAddModal = false;
      resetForm();
      // TODO: Toast de éxito
    } else {
      // TODO: Toast de error
      console.error("Error al agregar a lista negra:", result.error);
    }

    addFormLoading = false;
  }

  function handleUnblock(bloqueado: ListaNegraResponse) {
    selectedBloqueado = bloqueado;
    showUnblockModal = true;
    motivoDesbloqueo = "";
    observacionesDesbloqueo = "";
  }

  async function handleUnblockSubmit(data: {
    id: string;
    motivoDesbloqueo?: string;
    observaciones?: string;
  }) {
    const bloqueado = bloqueados.find((b) => b.id === data.id);
    if (!bloqueado) return;

    let result;

    if (bloqueado.isActive) {
      // Desbloquear (remove)
      result = await listaNegraService.unblock(
        data.id,
        data.motivoDesbloqueo || "Desbloqueo manual",
        data.observaciones,
      );
    } else {
      // Re-bloquear (reactivate)
      result = await listaNegraService.reblock(
        data.id,
        data.motivoDesbloqueo || "Re-bloqueo manual",
        data.observaciones,
        "usuario_actual", // TODO: Obtener usuario actual del store
      );
    }

    if (result.ok) {
      await loadListaNegra();
      selectedBloqueado = null;
      // TODO: Toast de éxito
    } else {
      // TODO: Toast de error
      console.error("Error:", result.error);
    }
  }

  function handleViewInfo(bloqueado: ListaNegraResponse) {
    console.log("Ver información de:", bloqueado);
    // TODO: Abrir panel lateral o modal con información detallada
  }

  function handleViewHistory(bloqueado: ListaNegraResponse) {
    console.log("Ver historial de:", bloqueado);
    // TODO: Abrir tab o modal con historial
  }

  // --- Filtros ---
  function handleEstadoFilterChange(filter: string) {
    listLogic.setEstadoFilter(filter as any);
    listState.estadoFilter = filter as any;
  }

  function handleTipoFilterChange(filter: string) {
    listLogic.setTipoFilter(filter as any);
    listState.tipoFilter = filter as any;
  }

  function handleClearAllFilters() {
    listLogic.clearAllFilters();
    listState.estadoFilter = "todos";
    listState.tipoFilter = "todos";
  }

  // --- Búsqueda ---
  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    console.log("Bloqueado seleccionado:", e.detail);
  }

  function handleSearchClear() {
    console.log("Búsqueda limpiada");
  }

  // --- Form helpers ---
  let formRef: any;

  function resetForm() {
    if (formRef && formRef.reset) {
      formRef.reset();
    }
  }

  function closeModal() {
    showAddModal = false;
    resetForm();
  }

  function closeUnblockModal() {
    showUnblockModal = false;
    selectedBloqueado = null;
    motivoDesbloqueo = "";
    observacionesDesbloqueo = "";
  }

  async function confirmUnblock() {
    if (!selectedBloqueado) return;

    await handleUnblockSubmit({
      id: selectedBloqueado.id,
      motivoDesbloqueo,
      observaciones: observacionesDesbloqueo.trim() || undefined,
    });

    closeUnblockModal();
  }

  $effect(() => {
    if (!showAddModal) {
      addFormLoading = false;
    }
  });

  onMount(() => {
    loadListaNegra();
  });
</script>

<ListaNegraListForm
  {bloqueados}
  {loading}
  {error}
  {filteredData}
  {columnDefs}
  estadoFilter={listState.estadoFilter}
  tipoFilter={listState.tipoFilter}
  onRefresh={loadListaNegra}
  onEstadoFilterChange={handleEstadoFilterChange}
  onTipoFilterChange={handleTipoFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
  onAddToBlacklist={canManage ? handleAddToBlacklist : undefined}
  onUnblock={canManage ? handleUnblock : undefined}
  onViewInfo={handleViewInfo}
  onViewHistory={handleViewHistory}
/>

<!-- Modal para agregar a lista negra -->
{#if showAddModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={closeModal}
    ></div>
    <div
      class="relative z-10 w-full max-w-4xl"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <ListaNegraForm
        bind:this={formRef}
        loading={addFormLoading}
        onSubmit={handleAddSubmit}
        onUnblock={handleUnblockSubmit}
      />
    </div>
  </div>
{/if}

<!-- Modal para desbloquear/rebloquear -->
<BlacklistConfirmModal
  show={showUnblockModal}
  contratistaName={selectedBloqueado?.nombreCompleto || ""}
  motivo={motivoDesbloqueo}
  observaciones={observacionesDesbloqueo}
  onConfirm={confirmUnblock}
  onCancel={closeUnblockModal}
  onMotivoChange={(v) => (motivoDesbloqueo = v)}
  onObservacionesChange={(v) => (observacionesDesbloqueo = v)}
/>
