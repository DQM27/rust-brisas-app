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
  import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";
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

  // Helper para formatear badge de estado (igual que contratista)
  function formatEstadoBadge(estado: string): string {
    const baseClass =
      "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

    const badges: Record<string, string> = {
      // GitHub Open (Green)
      activo:
        "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800",
      // GitHub Draft/Gray (Gray)
      inactivo:
        "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
      // GitHub Closed (Red/Purple) -> Usamos Rojo para suspendido
      suspendido:
        "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800",
    };

    const estadoLower = estado?.toLowerCase() || "inactivo";
    const badgeClass = badges[estadoLower] || badges.inactivo;
    const displayText = estado
      ? estado.charAt(0).toUpperCase() + estado.slice(1).toLowerCase()
      : "N/A";

    return `
      <button 
        class="${baseClass} ${badgeClass}"
        title="Clic para cambiar estado"
      >
        ${displayText}
      </button>
    `;
  }

  // Definición de columnas
  let columnDefs = $derived.by((): ColDef<ProveedorResponse>[] => [
    {
      field: "cedula",
      headerName: "Cédula",
      width: 130,
      pinned: "left",
      cellStyle: { fontFamily: "monospace", fontSize: "13px" },
    },
    {
      field: "nombre",
      headerName: "Nombre Completo",
      flex: 1,
      minWidth: 200,
      cellStyle: { fontWeight: 500 },
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
      minWidth: 180,
    },
    {
      field: "vehiculoTipo",
      headerName: "Vehículo",
      width: 120,
      valueFormatter: (params) => params.value || "-",
    },
    {
      field: "vehiculoPlaca",
      headerName: "Placa",
      width: 100,
      valueFormatter: (params) => params.value || "-",
      cellStyle: { fontFamily: "monospace" },
    },
    {
      field: "estado",
      headerName: "Estado",
      width: 130,
      cellRenderer: (params: ICellRendererParams) => {
        const estado = params.value as string;
        return formatEstadoBadge(estado);
      },
      onCellClicked: async (params) => {
        if (params.data) {
          const row = params.data as ProveedorResponse;
          await handleStatusToggle(row.id, row.estado);
        }
      },
    },
    {
      field: "puedeIngresar",
      headerName: "Acceso",
      width: 130,
      cellRenderer: (params: ICellRendererParams) => {
        const row = params.data as ProveedorResponse;

        // Estilo GitHub "Closed" / "Failure" (Rojo)
        const redBadge =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";

        // Estilo GitHub "Open" / "Success" (Verde)
        const greenBadge =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";

        // Si está inactivo o suspendido, el acceso es denegado
        if (row.estado?.toLowerCase() !== "activo") {
          return `<span class="${redBadge}">Denegado</span>`;
        }

        if (row.puedeIngresar) {
          return `<span class="${greenBadge}">Permitido</span>`;
        } else {
          return `<span class="${redBadge}">Denegado</span>`;
        }
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

  // --- Toggle de estado ---
  let isTogglingStatus = false; // Guard to prevent double triggers

  async function handleStatusToggle(id: string, currentStatus: string) {
    // Guard against double-clicks/triggers
    if (isTogglingStatus) return;
    isTogglingStatus = true;

    // Ciclar: activo -> inactivo -> activo
    const statusLower = currentStatus?.toLowerCase() || "inactivo";
    const newStatus = statusLower === "activo" ? "INACTIVO" : "ACTIVO";

    try {
      const updated = await proveedorService.changeStatus(id, newStatus);

      // Update local state instead of reloading (avoids flicker)
      proveedores = proveedores.map((p) =>
        p.id === id
          ? {
              ...p,
              estado: updated.estado,
              puedeIngresar: updated.puedeIngresar,
            }
          : p,
      );

      toast.success(`Estado cambiado a ${newStatus.toLowerCase()}`);
    } catch (err: any) {
      toast.error(err.message || "Error al cambiar estado");
    } finally {
      isTogglingStatus = false;
    }
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
