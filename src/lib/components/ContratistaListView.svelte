<script lang="ts">
  import { onMount } from "svelte";
  import ContratistaListForm from "./ContratistaListForm.svelte";
  import { submitFetchAllContratistas } from "$lib/logic/contratista/submitFetchContratistas";
  import { submitFetchAllListaNegra } from "$lib/logic/listaNegra/submitFetchListaNegra";
  import { submitAddToListaNegra } from "$lib/logic/listaNegra/submitAddToListaNegra";
  import { submitUnblockListaNegra } from "$lib/logic/listaNegra/submitUnblockListaNegra";
  import { listaNegra } from "$lib/api/listaNegra";
  import { currentUser } from "$lib/stores/auth";
  import type { ContratistaResponse } from "$lib/types/contratista";
  import type { SearchResult } from "$lib/types/search.types";
  import { createContratistaListLogic, ContratistaListLogic } from "$lib/logic/contratista/contratistaListLogic";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let blockedContratistas = $state<Set<string>>(new Set());

  // Inicializar lógica de presentación
  const listLogic = createContratistaListLogic();
  const listState = listLogic.getState();

  // Datos derivados para la UI - USANDO $derived
  let filteredData = $derived(listLogic.getFilteredData(contratistas));
  let stats = $derived(listLogic.getStats(contratistas));
  let columns = $derived(ContratistaListLogic.getColumns());

  // Sincronizar loading con la lógica - USANDO $effect
  $effect(() => {
    // Si necesitas hacer algo cuando cambie loading, lo pones aquí
    // Por ejemplo: listLogic.setLoading(loading);
  });

  async function loadContratistas() {
    loading = true;
    error = "";

    const result = await submitFetchAllContratistas();

    if (result.ok) {
      contratistas = result.contratistas;
      await loadBlockedContratistas();
    } else {
      error = result.error;
    }

    loading = false;
  }

  async function loadBlockedContratistas() {
    const result = await submitFetchAllListaNegra();

    if (result.ok) {
      const blocked = new Set<string>();
      result.data.bloqueados.forEach((b) => {
        if (b.isActive && b.contratistaId) {
          blocked.add(b.contratistaId);
        }
      });
      blockedContratistas = blocked;
    }
  }

  async function handleBlock(data: {
    contratistaId: string;
    motivoBloqueo: string;
    observaciones?: string;
  }) {
    const usuario = $currentUser;
    if (!usuario) {
      console.error("No hay usuario autenticado");
      return;
    }

    const bloqueadoPor = `${usuario.nombre} ${usuario.apellido}`;

    const result = await submitAddToListaNegra({
      ...data,
      bloqueadoPor,
    });

    if (result.ok) {
      await loadBlockedContratistas();
      await loadContratistas();
    } else {
      console.error("Error al bloquear:", result.error);
    }
  }

  async function handleUnblock(data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) {
    try {
      const contratista = contratistas.find((c) => c.id === data.id);
      if (!contratista) {
        console.error("Contratista no encontrado");
        return;
      }

      const bloqueado = await listaNegra.getByCedula(contratista.cedula);
      if (!bloqueado) {
        console.error("Registro de lista negra no encontrado");
        return;
      }

      const result = await submitUnblockListaNegra(
        bloqueado.id,
        data.motivoDesbloqueo,
        data.observaciones,
      );

      if (result.ok) {
        await loadBlockedContratistas();
        await loadContratistas();
      } else {
        console.error("Error al desbloquear:", result.error);
      }
    } catch (error) {
      console.error("Error en handleUnblock:", error);
    }
  }

  // Handlers para los eventos del componente de presentación
  function handleEstadoFilterChange(filter: string) {
    listLogic.setEstadoFilter(filter as any);
  }

  function handlePraindFilterChange(filter: string) {
    listLogic.setPraindFilter(filter as any);
  }

  function handleClearAllFilters() {
    listLogic.clearAllFilters();
  }

  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    // La lógica ya está en la store, no necesita hacer nada
  }

  function handleSearchClear() {
    // La lógica ya está en la store, no necesita hacer nada
  }

  onMount(() => {
    loadContratistas();
  });
</script>

<ContratistaListForm
  {contratistas}
  {loading}
  {error}
  {blockedContratistas}
  {filteredData}
  {stats}
  {columns}
  estadoFilter={listState.estadoFilter}
  praindFilter={listState.praindFilter}
  onRefresh={loadContratistas}
  onBlock={handleBlock}
  onUnblock={handleUnblock}
  onEstadoFilterChange={handleEstadoFilterChange}
  onPraindFilterChange={handlePraindFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
/>