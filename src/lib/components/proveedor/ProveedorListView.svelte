<!-- src/lib/components/proveedor/ProveedorListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import ProveedorListForm from "./ProveedorListForm.svelte";
  import {
    fetchAllProveedores,
    changeStatus,
    type ServiceResult,
  } from "$lib/logic/proveedor/proveedorService";
  import { ProveedorListLogic } from "$lib/logic/proveedor/proveedorListLogic";
  import type { ProveedorResponse } from "$lib/types/proveedor";
  import { openTab } from "$lib/stores/tabs";
  import type { SearchResult } from "$lib/types/search.types";
  import type { ColDef } from "@ag-grid-community/core";
  import { toast } from "svelte-5-french-toast";

  interface Props {
    tabId: string;
    data?: any;
  }

  let { tabId, data }: Props = $props();

  // Estado local
  let proveedores = $state<ProveedorResponse[]>([]);
  let loading = $state(false);
  let error = $state("");

  // Filtros
  let estadoFilter = $state("todos");

  // Datos filtrados
  let filteredData = $derived.by(() => {
    if (estadoFilter === "todos") return proveedores;
    return proveedores.filter((p) => p.estado?.toLowerCase() === estadoFilter);
  });

  // Definición de columnas usando ProveedorListLogic
  let columnDefs = $derived.by((): ColDef<ProveedorResponse>[] =>
    ProveedorListLogic.getColumns(handleStatusToggle),
  );

  // --- Cargar datos ---
  async function loadProveedores() {
    loading = true;
    error = "";

    const result = await fetchAllProveedores();

    if (result.ok) {
      proveedores = result.data;
    } else {
      error = result.error;
    }

    loading = false;
  }

  // --- Toggle de estado ---
  let isTogglingStatus = false;

  async function handleStatusToggle(id: string, currentStatus: string) {
    if (isTogglingStatus) return;
    isTogglingStatus = true;

    const statusLower = currentStatus?.toLowerCase() || "inactivo";
    const newStatus = statusLower === "activo" ? "INACTIVO" : "ACTIVO";

    const result = await changeStatus(id, newStatus);

    if (result.ok) {
      // Update local state instead of reloading
      proveedores = proveedores.map((p) =>
        p.id === id
          ? {
              ...p,
              estado: result.data.estado,
              puedeIngresar: result.data.puedeIngresar,
            }
          : p,
      );
      toast.success(`Estado cambiado a ${newStatus.toLowerCase()}`);
    } else {
      toast.error(result.error);
    }

    isTogglingStatus = false;
  }

  // --- Acciones ---
  function handleNewProveedor() {
    openTab({
      componentKey: "proveedor",
      title: "Nuevo Proveedor",
      data: {},
    });
  }

  function handleEditProveedor(proveedor: ProveedorResponse) {
    openTab({
      componentKey: "proveedor",
      title: `Editar: ${proveedor.nombre}`,
      data: { proveedorId: proveedor.id, initialData: proveedor },
    });
  }

  function handleViewInfo(proveedor: ProveedorResponse) {
    console.log("Ver información de:", proveedor);
  }

  function handleViewVehicles(proveedor: ProveedorResponse) {
    console.log("Ver vehículos de:", proveedor);
  }

  // --- Filtros ---
  function handleEstadoFilterChange(filter: string) {
    estadoFilter = filter;
  }

  function handleClearAllFilters() {
    estadoFilter = "todos";
  }

  // --- Búsqueda ---
  function handleSearchSelect(e: CustomEvent<SearchResult>) {
    console.log("Proveedor seleccionado:", e.detail);
  }

  function handleSearchClear() {
    console.log("Búsqueda limpiada");
  }

  onMount(() => {
    loadProveedores();
  });
</script>

<ProveedorListForm
  {proveedores}
  {loading}
  {error}
  {filteredData}
  {columnDefs}
  {estadoFilter}
  onRefresh={loadProveedores}
  onEstadoFilterChange={handleEstadoFilterChange}
  onClearAllFilters={handleClearAllFilters}
  onSearchSelect={handleSearchSelect}
  onSearchClear={handleSearchClear}
  onNewProveedor={handleNewProveedor}
  onEditProveedor={handleEditProveedor}
  onViewInfo={handleViewInfo}
  onViewVehicles={handleViewVehicles}
/>
