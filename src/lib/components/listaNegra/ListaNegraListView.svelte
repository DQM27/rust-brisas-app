<!-- src/lib/components/listaNegra/ListaNegraListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
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
      toast.success("Persona agregada a lista negra");
    } else {
      toast.error(result.error || "Error al agregar a lista negra");
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
      const usuario = $currentUser;
      const bloqueadoPor = usuario
        ? `${usuario.nombre} ${usuario.apellido}`
        : "Sistema";
      result = await listaNegraService.reblock(
        data.id,
        data.motivoDesbloqueo || "Re-bloqueo manual",
        data.observaciones,
        bloqueadoPor,
      );
    }

    if (result.ok) {
      await loadListaNegra();
      selectedBloqueado = null;
      toast.success(
        bloqueado.isActive ? "Persona desbloqueada" : "Persona re-bloqueada",
      );
    } else {
      toast.error(result.error || "Error en la operación");
    }
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

  // --- Form helpers ---
  let formRef = $state<any>(null);

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
  onAddToBlacklist={canManage ? handleAddToBlacklist : undefined}
  onUnblock={canManage ? handleUnblock : undefined}
/>

<!-- Modal para agregar a lista negra -->
{#if showAddModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <button
      class="absolute inset-0 w-full h-full bg-black/60 backdrop-blur-sm border-0 cursor-default"
      onclick={closeModal}
      aria-label="Cerrar modal"
    ></button>
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
