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

    // ========= TIPO (con badges estilo GitHub) =========
    {
      field: "tipoDisplay",
      headerName: "Tipo",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const tipo = params.data.tipo;
        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";
        let colorClass = "";

        switch (tipo) {
          case "contratista":
            // Indigo/Blue
            colorClass =
              "bg-indigo-50 text-indigo-700 border-indigo-200 dark:bg-indigo-900/30 dark:text-indigo-300 dark:border-indigo-800";
            break;
          case "proveedor":
            // Amber/Yellow
            colorClass =
              "bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800";
            break;
          case "visita":
            // Emerald/Green
            colorClass =
              "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
            break;
          default:
            // Gray
            colorClass =
              "bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700";
        }

        return `<span class="${baseClass} ${colorClass}">${params.value}</span>`;
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
        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

        if (status === "disponible") {
          const classes =
            "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
          return `<span class="${baseClass} ${classes}">✔ Disponible</span>`;
        } else if (status === "en_uso") {
          const classes =
            "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800";
          return `<span class="${baseClass} ${classes}">◉ En Uso</span>`;
        } else if (status === "perdido") {
          const classes =
            "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
          return `<span class="${baseClass} ${classes}">✖ Perdido</span>`;
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

        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

        return params.value
          ? `<span class="${baseClass} bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800">Sí</span>`
          : `<span class="${baseClass} bg-orange-50 text-orange-700 border-orange-200 dark:bg-orange-900/30 dark:text-orange-300 dark:border-orange-800">Pendiente</span>`;
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
