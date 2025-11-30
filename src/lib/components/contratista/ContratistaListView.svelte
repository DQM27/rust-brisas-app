<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import ContratistaListForm from "./ContratistaListForm.svelte";
  import * as contratistaService from "$lib/logic/contratista/contratistaService";
  import * as listaNegraService from "$lib/logic/listaNegra/listaNegraService";
  import { openTab } from "$lib/stores/tabs"; // ← Importar openTab
  import { UserPlus, Edit } from "lucide-svelte";
  import type { ContratistaResponse } from "$lib/types/contratista";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import {
    createContratistaListLogic,
    ContratistaListLogic,
  } from "$lib/logic/contratista/contratistaListLogic";

  import { selectedSearchStore } from "$lib/stores/searchStore";
  import { reindexAllContratistas } from "$lib/api/searchService";
  import { toast } from "svelte-5-french-toast";

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
    // Suscribirse a cambios en la búsqueda seleccionada
    const _search = $selectedSearchStore;
    return listLogic.getFilteredData(contratistas);
  });

  // Convertir columnas a ColDef de AG Grid
  let columnDefs = $derived.by((): ColDef<ContratistaResponse>[] => {
    const cols = ContratistaListLogic.getColumns();
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
        }) as ColDef<ContratistaResponse>,
    );
  });

  // --- Cargar datos ---
  async function loadContratistas() {
    loading = true;
    error = "";

    try {
      const result = await contratistaService.submitFetchAllContratistas();
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
    const result = await listaNegraService.fetchAll();
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

  // --- Acciones ---
  function handleNewContratista() {
    openTab({
      componentKey: "contratista",
      title: "Nuevo Contratista",
      data: {},
    });
  }

  function handleEditContratista(contratista: ContratistaResponse) {
    openTab({
      componentKey: "contratista",
      title: `Editar: ${contratista.nombre}`,
      data: { contratistaId: contratista.id },
    });
  }

  function handleViewInfo(contratista: ContratistaResponse) {
    console.log("Ver información de:", contratista);
    // TODO: Abrir panel lateral o modal con información detallada
  }

  function handleViewHistory(contratista: ContratistaResponse) {
    console.log("Ver historial de:", contratista);
    // TODO: Abrir tab o modal con historial de ingresos
  }

  function handleViewVehicles(contratista: ContratistaResponse) {
    console.log("Ver vehículos de:", contratista);
    // TODO: Abrir tab de vehículos filtrado por este contratista
  }

  function handleViewBadges(contratista: ContratistaResponse) {
    console.log("Ver carnets de:", contratista);
    // TODO: Abrir tab de carnets filtrado por este contratista
  }

  function handleDeleteContratista(contratista: ContratistaResponse) {
    // TODO: Mostrar confirmación y eliminar
    console.log("Eliminar:", contratista);
  }

  function handleDeleteMultiple(contratistas: ContratistaResponse[]) {
    // TODO: Mostrar confirmación y eliminar múltiples
    console.log("Eliminar múltiples:", contratistas.length);
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

  // --- Búsqueda ---
  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    console.log("Contratista seleccionado:", e.detail);
  }

  function handleSearchClear() {
    console.log("Búsqueda limpiada");
  }

  async function handleReindex() {
    const toastId = toast.loading("Reindexando búsqueda...");
    try {
      await reindexAllContratistas();
      toast.success("Índice de búsqueda actualizado", { id: toastId });
    } catch (err) {
      console.error("Error al reindexar:", err);
      toast.error("Error al reindexar", { id: toastId });
    }
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
  {columnDefs}
  estadoFilter={listState.estadoFilter}
  praindFilter={listState.praindFilter}
  onRefresh={loadContratistas}
  onReindex={handleReindex}
  onEstadoFilterChange={handleEstadoFilterChange}
  onPraindFilterChange={handlePraindFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
  onNewContratista={handleNewContratista}
  onEditContratista={handleEditContratista}
  onViewInfo={handleViewInfo}
  onViewHistory={handleViewHistory}
  onViewVehicles={handleViewVehicles}
  onViewBadges={handleViewBadges}
  onDeleteContratista={handleDeleteContratista}
  onDeleteMultiple={handleDeleteMultiple}
/>
