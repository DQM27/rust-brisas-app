<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { ColDef } from "@ag-grid-community/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import type { IngresoResponse } from "$lib/types/ingreso";
  import { ingresoStore } from "$lib/stores/ingresoStore";

  import SalidaModal from "./SalidaModal.svelte";

  // Estado
  let ingresos: IngresoResponse[] = [];
  let loading = false;
  let showSalidaModal = false;
  let selectedIngreso: IngresoResponse | null = null;
  let formLoading = false;

  // Suscribirse al store
  const unsubscribe = ingresoStore.subscribe((value) => {
    ingresos = value;
  });

  onDestroy(() => {
    unsubscribe();
  });

  // Configuración de columnas
  const columnDefs: ColDef<IngresoResponse>[] = [
    {
      field: "nombreCompleto",
      headerName: "Nombre",
      flex: 1,
      minWidth: 200,
      cellStyle: { fontWeight: "500" },
    },
    {
      field: "empresaNombre",
      headerName: "Empresa",
      width: 150,
    },
    {
      field: "gafeteNumero",
      headerName: "Gafete",
      width: 100,
      cellRenderer: (params: any) =>
        params.value
          ? `<span class="font-mono font-bold text-blue-600">${params.value}</span>`
          : "-",
    },
    {
      field: "vehiculoPlaca",
      headerName: "Vehículo",
      width: 120,
      valueFormatter: (params) => params.value || "-",
    },
    {
      field: "fechaHoraIngreso",
      headerName: "Entrada",
      width: 140,
      valueFormatter: (params) =>
        new Date(params.value).toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
        }),
    },
    {
      field: "tiempoPermanenciaTexto",
      headerName: "Tiempo",
      width: 120,
      cellStyle: { color: "#666" },
    },
    {
      headerName: "Acciones",
      width: 140,
      cellRenderer: (params: any) => {
        return `<button class="px-3 py-1 bg-red-100 text-red-700 rounded-md hover:bg-red-200 text-xs font-medium salida-btn">Registrar Salida</button>`;
      },
      onCellClicked: (params: any) => {
        if (params.event.target.classList.contains("salida-btn")) {
          handleSalidaClick(params.data);
        }
      },
    },
  ];

  // Cargar datos
  async function loadData() {
    loading = true;
    await ingresoStore.load();
    loading = false;
  }

  // Handlers
  function handleSalidaClick(ingreso: IngresoResponse) {
    selectedIngreso = ingreso;
    showSalidaModal = true;
  }

  async function handleConfirmSalida(event: CustomEvent) {
    formLoading = true;
    const result = await ingresoService.registrarSalida({
      ingresoId: event.detail.ingresoId,
      devolvioGafete: event.detail.devolvioGafete,
      observacionesSalida: event.detail.observaciones,
      usuarioSalidaId:
        $currentUser?.id || "00000000-0000-0000-0000-000000000000",
    });

    if (result.ok) {
      toast.success("Salida registrada");
      ingresoStore.remove(result.data.id);
      showSalidaModal = false;
      selectedIngreso = null;
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  onMount(() => {
    loadData();
  });
</script>

<div
  class="flex flex-col bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden h-full"
>
  <!-- Header -->
  <div
    class="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center"
  >
    <h2 class="text-lg font-bold text-gray-900 dark:text-white">
      Personas Adentro ({ingresos?.length ?? 0})
    </h2>
    <button
      on:click={loadData}
      class="text-blue-600 hover:text-blue-800 text-sm font-medium"
    >
      🔄 Actualizar
    </button>
  </div>

  <!-- Tabla -->
  <div class="flex-1">
    <AGGridWrapper
      gridId="entries-list"
      rowData={ingresos}
      {columnDefs}
      getRowId={(params) => params.data.id}
    />
  </div>
</div>

<!-- Modal Salida -->
{#if showSalidaModal && selectedIngreso}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade
  >
    <SalidaModal
      ingreso={selectedIngreso}
      loading={formLoading}
      on:confirm={handleConfirmSalida}
      on:cancel={() => (showSalidaModal = false)}
    />
  </div>
{/if}
