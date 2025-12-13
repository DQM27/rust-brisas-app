<!-- src/lib/components/proveedor/ProveedorListView.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import ProveedorListForm from "./ProveedorListForm.svelte";
  import {
    proveedorService,
    type ProveedorResponse,
  } from "$lib/services/proveedorService";
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
    return proveedores.filter((p) => p.estado === estadoFilter);
  });

  // Definición de columnas
  let columnDefs = $derived.by((): ColDef<ProveedorResponse>[] => [
    {
      field: "cedula",
      headerName: "Cédula",
      width: 130,
      sortable: true,
      filter: true,
    },
    {
      field: "nombre",
      headerName: "Nombre",
      flex: 1,
      minWidth: 150,
      sortable: true,
      filter: true,
      valueFormatter: (params) => {
        const d = params.data;
        if (!d) return "";
        const full = [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
          .filter(Boolean)
          .join(" ");
        return full || d.nombre;
      },
    },
    {
      field: "empresaNombre",
      headerName: "Empresa",
      flex: 1,
      minWidth: 150,
      sortable: true,
      filter: true,
    },
    {
      field: "vehiculoPlaca",
      headerName: "Placa",
      width: 100,
      sortable: true,
      filter: true,
      valueFormatter: (params) => params.value || "—",
    },
    {
      field: "vehiculoTipo",
      headerName: "Tipo Vehículo",
      width: 120,
      sortable: true,
      filter: true,
      valueFormatter: (params) => params.value || "—",
    },
    {
      field: "estado",
      headerName: "Estado",
      width: 100,
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const estado = params.value;
        const color = estado === "activo" ? "#22c55e" : "#ef4444";
        return `<span style="color: ${color}; font-weight: 500;">${estado}</span>`;
      },
    },
  ]);

  // --- Cargar datos ---
  async function loadProveedores() {
    loading = true;
    error = "";

    try {
      proveedores = await proveedorService.getAll();
    } catch (err: any) {
      console.error("Error al cargar proveedores:", err);
      error = err.message || "Error al cargar proveedores";
    }

    loading = false;
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
