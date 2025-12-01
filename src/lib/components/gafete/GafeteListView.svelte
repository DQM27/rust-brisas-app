<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import * as alertaGafeteService from "$lib/logic/alertaGafete/alertaGafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";
  import GafeteForm from "./GafeteForm.svelte";
  import ResolveAlertModal from "./ResolveAlertModal.svelte";

  // Estado
  let gafetes: GafeteResponse[] = [];
  let loading = false;
  let showModal = false;
  let selectedGafete: GafeteResponse | null = null;
  let formLoading = false;

  // Estado para modal de resolución de alertas
  let showResolveModal = false;
  let selectedAlertGafete: GafeteResponse | null = null;

  // ==========================================
  // COLUMNAS AG GRID
  // ==========================================
  const columnDefs: ColDef<GafeteResponse>[] = [
    {
      field: "numero",
      headerName: "Número",
      sortable: true,
      filter: true,
      cellStyle: { fontWeight: "bold" },
    },

    // ========= TIPO (con badges pastel modernos) =========
    {
      field: "tipoDisplay",
      headerName: "Tipo",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const tipo = params.data.tipo;
        let colorClass = "";

        switch (tipo) {
          case "contratista":
            colorClass =
              "bg-indigo-100 text-indigo-700 border border-indigo-300 dark:bg-indigo-900 dark:text-indigo-200";
            break;
          case "proveedor":
            colorClass =
              "bg-amber-100 text-amber-700 border border-amber-300 dark:bg-amber-900 dark:text-amber-200";
            break;
          case "visita":
            colorClass =
              "bg-emerald-100 text-emerald-700 border border-emerald-300 dark:bg-emerald-900 dark:text-emerald-200";
            break;
          default:
            colorClass =
              "bg-gray-200 text-gray-700 border border-gray-400 dark:bg-gray-700 dark:text-gray-200";
        }

        return `
          <span class="px-3 py-1 text-xs font-semibold rounded-full shadow-sm ${colorClass}">
            ${params.value}
          </span>
        `;
      },
    },

    // ========= ESTADO (3 estados: Disponible, En Uso, Perdido) =========
    {
      field: "status",
      headerName: "Estado",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const status = params.value;

        if (status === "disponible") {
          return `
            <span class="px-2 py-1 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-700 border border-emerald-300 dark:bg-emerald-900 dark:text-emerald-200">
              ✔ Disponible
            </span>
          `;
        } else if (status === "en_uso") {
          return `
            <span class="px-2 py-1 rounded-full text-xs font-semibold bg-blue-100 text-blue-700 border border-blue-300 dark:bg-blue-900 dark:text-blue-200">
              ◉ En Uso
            </span>
          `;
        } else if (status === "perdido") {
          return `
            <span class="px-2 py-1 rounded-full text-xs font-semibold bg-red-100 text-red-700 border border-red-300 dark:bg-red-900 dark:text-red-200">
              ✖ Perdido
            </span>
          `;
        }

        return "-";
      },
    },

    // ========= FECHA PERDIDO =========
    {
      field: "fechaPerdido",
      headerName: "Fecha Perdido",
      sortable: true,
      filter: true,
      width: 150,
      valueFormatter: (params) => {
        if (!params.value) return "-";
        const date = new Date(params.value);
        return date.toLocaleDateString("es-ES");
      },
    },

    // ========= QUIEN LO PERDIÓ =========
    {
      field: "quienPerdio",
      headerName: "Quién lo Perdió",
      sortable: true,
      filter: true,
      width: 180,
      valueFormatter: (params) => params.value || "-",
    },

    // ========= ESTADO ALERTA (Devuelto/Pendiente) =========
    {
      field: "alertaResuelta",
      headerName: "Devuelto",
      sortable: true,
      filter: true,
      width: 120,
      cellRenderer: (params: any) => {
        if (params.value === null || params.value === undefined) return "-";

        return params.value
          ? `<span class="px-2 py-1 rounded-full text-xs font-semibold bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200">Sí</span>`
          : `<span class="px-2 py-1 rounded-full text-xs font-semibold bg-orange-100 text-orange-700 dark:bg-orange-900 dark:text-orange-200">Pendiente</span>`;
      },
    },

    // ========= ACCIONES =========
    {
      headerName: "Acciones",
      width: 150,
      cellRenderer: (params: any) => {
        const status = params.data.status;

        if (status === "perdido") {
          return `
            <button class="px-3 py-1 bg-green-100 text-green-700 rounded-md hover:bg-green-200 text-xs font-medium resolve-btn">
              ✓ Resolver Alerta
            </button>
          `;
        }

        return `<span class="text-xs text-gray-400">-</span>`;
      },
      onCellClicked: (params: any) => {
        const event = params.event;
        if (event.target.classList.contains("resolve-btn")) {
          handleResolve(params.data);
        }
      },
    },
  ];

  // ==========================================
  // Cargar datos
  // ==========================================
  async function loadGafetes() {
    loading = true;
    const result = await gafeteService.fetchAll();
    if (result.ok) {
      gafetes = result.data.gafetes;
    } else {
      toast.error(result.error);
    }
    loading = false;
  }

  // ==========================================
  // Manejadores
  // ==========================================
  function handleNew() {
    selectedGafete = null;
    showModal = true;
  }

  function handleEdit(gafete: GafeteResponse) {
    selectedGafete = gafete;
    showModal = true;
  }

  function handleResolve(gafete: GafeteResponse) {
    selectedAlertGafete = gafete;
    showResolveModal = true;
  }

  async function handleResolveSubmit(notas: string, fechaDevolucion: string) {
    if (!selectedAlertGafete?.alertaId) {
      toast.error("No se encontró el ID de la alerta");
      return;
    }

    formLoading = true;

    const result = await alertaGafeteService.resolverAlerta(
      selectedAlertGafete.alertaId,
      `${notas} - Fecha de devolución/pago: ${fechaDevolucion}`,
    );

    if (result.ok) {
      toast.success("Alerta resuelta correctamente");
      showResolveModal = false;
      selectedAlertGafete = null;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  async function handleDelete(gafete: GafeteResponse) {
    if (!confirm(`¿Estás seguro de eliminar el gafete ${gafete.numero}?`))
      return;

    const result = await gafeteService.remove(gafete.numero);
    if (result.ok) {
      toast.success("Gafete eliminado");
      loadGafetes();
    } else {
      toast.error(result.error);
    }
  }

  async function handleFormSubmit(event: CustomEvent) {
    formLoading = true;
    const data = event.detail;
    let result;

    if (selectedGafete) {
      result = await gafeteService.update(selectedGafete.numero, data);
    } else {
      result = await gafeteService.create(data);
    }

    if (result.ok) {
      toast.success(selectedGafete ? "Gafete actualizado" : "Gafete creado");
      showModal = false;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  onMount(() => {
    loadGafetes();
  });
</script>

<!-- ========================================== -->
<!-- LAYOUT -->
<!-- ========================================== -->
<div class="h-full flex flex-col space-y-4 p-4">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
      Gestión de Gafetes
    </h1>

    <button
      on:click={handleNew}
      class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 transition-colors"
    >
      + Nuevo Gafete
    </button>
  </div>

  <div
    class="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden"
  >
    <AGGridWrapper
      gridId="badges-list"
      rowData={gafetes}
      {columnDefs}
      getRowId={(params) => params.data.numero}
    />
  </div>
</div>

<!-- ========================================== -->
<!-- MODAL -->
<!-- ========================================== -->
{#if showModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade
  >
    <GafeteForm
      initialData={selectedGafete}
      loading={formLoading}
      on:submit={handleFormSubmit}
      on:cancel={() => (showModal = false)}
    />
  </div>
{/if}

<!-- ========================================== -->
<!-- MODAL RESOLVER ALERTA -->
<!-- ========================================== -->
{#if showResolveModal && selectedAlertGafete}
  <ResolveAlertModal
    show={showResolveModal}
    gafeteNumero={selectedAlertGafete.numero}
    nombrePersona={selectedAlertGafete.quienPerdio || "Desconocido"}
    fechaReporte={selectedAlertGafete.fechaPerdido || new Date().toISOString()}
    onResolve={handleResolveSubmit}
    onCancel={() => (showResolveModal = false)}
  />
{/if}
