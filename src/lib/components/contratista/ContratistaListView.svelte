<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import ContratistaListForm from "./ContratistaListForm.svelte";
  import { submitFetchAllContratistas } from "$lib/logic/contratista/submitFetchContratistas";
  import { submitFetchAllListaNegra } from "$lib/logic/listaNegra/submitFetchListaNegra";
  import type { ContratistaResponse } from "$lib/types/contratista";
  import type { SearchResult } from "$lib/types/search.types";
  import { createContratistaListLogic, ContratistaListLogic } from "$lib/logic/contratista/contratistaListLogic";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // Estado local
  let contratistas = $state<ContratistaResponse[]>([]);
  let loading = $state(false);
  let error = $state("");
  let blockedContratistas = $state<Set<string>>(new Set());

  // Lógica de presentación
  const listLogic = createContratistaListLogic();
  const listState = listLogic.getState();

  let filteredData = $derived.by(() => {
    return listLogic.getFilteredData(contratistas);
  });

  let columns = $derived(ContratistaListLogic.getColumns());

  // --- Cargar datos ---
  async function loadContratistas() {
    loading = true;
    error = "";

    try {
      const result = await submitFetchAllContratistas();
      if (result.ok) {
        contratistas = result.contratistas;
        await loadBlockedContratistas();
      } else {
        error = result.error;
      }
    } catch (err) {
      console.error("Error al cargar contratistas:", err);
      error = "Error al cargar contratistas";
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

  // --- Filtros ---
  function handleEstadoFilterChange(filter: string) {
    listLogic.setEstadoFilter(filter as any);
    listState.estadoFilter = filter as any;
  }

  function handlePraindFilterChange(filter: string) {
    listLogic.setPraindFilter(filter as any);
    listState.praindFilter = filter as any;
  }

  function handleClearAllFilters() {
    listLogic.clearAllFilters();
    listState.estadoFilter = "todos";
    listState.praindFilter = "todos";
  }

  // --- Busqueda ---
  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    console.log("Contratista seleccionado:", e.detail);
  }

  function handleSearchClear() {
    console.log("Búsqueda limpiada");
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
  {columns}
  estadoFilter={listState.estadoFilter}
  praindFilter={listState.praindFilter}
  onRefresh={loadContratistas}
  onEstadoFilterChange={handleEstadoFilterChange}
  onPraindFilterChange={handlePraindFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
/>
