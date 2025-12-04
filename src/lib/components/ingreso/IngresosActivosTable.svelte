<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { ColDef, GridApi } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { Download, FileDown, UserCheck, History } from "lucide-svelte";

  import * as ingresoService from "$lib/logic/ingreso/ingresoService";
  import type { IngresoResponse } from "$lib/types/ingreso";
  import { ingresoStore } from "$lib/stores/ingresoStore";

  import SalidaModal from "./SalidaModal.svelte";

  // ✅ NUEVO: Importar componentes de exportación
  import ExportDialog from "$lib/components/export/ExportDialog.svelte";
  import {
    exportData,
    previewPDF,
    downloadBytes,
  } from "$lib/services/exportService";
  import type { ExportOptions } from "$lib/services/exportService";

  // Estado
  let ingresos = $state<IngresoResponse[]>([]);
  let loading = $state(false);
  let showSalidaModal = $state(false);
  let selectedIngreso = $state<IngresoResponse | null>(null);
  let formLoading = $state(false);

  // ✅ NUEVO: Estado para exportación
  let gridApi = $state<GridApi | null>(null);
  let showExportDialog = $state(false);
  let exportOnlySelected = $state(false);

  // Estado para alternar entre activos y salidas
  let showingActive = $state(true);

  // Suscribirse al store
  const unsubscribe = ingresoStore.subscribe((value) => {
    ingresos = value;
  });

  onDestroy(() => {
    unsubscribe();
  });

  // Configuración de columnas - cambia según el modo
  const columnDefs = $derived.by((): ColDef<IngresoResponse>[] => [
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
      field: "tipoAutorizacionDisplay",
      headerName: "Autorización",
      width: 120,
    },
    {
      field: "modoIngresoDisplay",
      headerName: "Modo",
      width: 120,
    },
    {
      field: "usuarioIngresoNombre",
      headerName: "Registró Entrada",
      width: 150,
    },
    {
      field: "usuarioSalidaNombre",
      headerName: "Registró Salida",
      width: 150,
      valueFormatter: (params) => params.value || "-",
    },
    {
      field: "fechaHoraIngreso",
      headerName: "Hora Entrada",
      width: 140,
      valueFormatter: (params) =>
        new Date(params.value).toLocaleString("es-MX", {
          month: "2-digit",
          day: "2-digit",
          hour: "2-digit",
          minute: "2-digit",
        }),
    },
    {
      field: "fechaHoraSalida",
      headerName: "Hora Salida",
      width: 140,
      hide: showingActive, // Solo mostrar en vista de salidas
      valueFormatter: (params) =>
        params.value
          ? new Date(params.value).toLocaleString("es-MX", {
              month: "2-digit",
              day: "2-digit",
              hour: "2-digit",
              minute: "2-digit",
            })
          : "-",
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
      hide: !showingActive, // Solo mostrar en vista activa
      cellRenderer: (params: any) => {
        return `<button class="px-3 py-1 bg-red-100 text-red-700 rounded-md hover:bg-red-200 text-xs font-medium salida-btn">Registrar Salida</button>`;
      },
      onCellClicked: (params: any) => {
        if (params.event.target.classList.contains("salida-btn")) {
          handleSalidaClick(params.data);
        }
      },
    },
  ]);

  // Cargar datos
  async function loadData() {
    loading = true;
    if (showingActive) {
      await ingresoStore.load();
    } else {
      // Cargar salidas del día
      const result = await ingresoService.fetchSalidasDelDia();
      if (result.ok) {
        ingresos = result.data;
      } else {
        console.error("Error al cargar salidas:", result.error);
        toast.error(result.error);
        ingresos = [];
      }
    }
    loading = false;
  }

  // Handlers de salida (sin cambios)
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

  // ✅ NUEVO: Handlers de exportación
  async function handleExportClick(onlySelected: boolean = false) {
    if (!gridApi) {
      toast.error("Grid no está listo");
      return;
    }

    exportOnlySelected = onlySelected;
    showExportDialog = true;
  }

  async function handleExport(
    format: "pdf" | "excel" | "csv",
    options: ExportOptions,
  ) {
    if (!gridApi) return;

    try {
      toast.loading("Exportando...");

      const response = await exportData(
        gridApi,
        format,
        options,
        exportOnlySelected,
      );

      if (response.success) {
        if (format === "pdf" && response.bytes) {
          if (options.showPreview) {
            previewPDF(response.bytes);
            toast.success("PDF abierto en nueva pestaña");
          } else {
            downloadBytes(response.bytes, `personas-adentro-${Date.now()}.pdf`);
            toast.success("PDF descargado exitosamente");
          }
        } else if (response.filePath) {
          toast.success(`Archivo guardado: ${response.filePath}`);
        }
      } else {
        toast.error(response.message || "Error al exportar");
      }
    } catch (error) {
      console.error("Error exportando:", error);
      toast.error("Error al exportar: " + (error as Error).message);
    }
  }

  // ✅ NUEVO: Custom buttons para exportación y toggle
  const customButtons = $derived.by(
    (): {
      default: CustomToolbarButton[];
      singleSelect: CustomToolbarButton[];
      multiSelect: CustomToolbarButton[];
    } => ({
      default: [
        {
          id: "toggle-view",
          label: showingActive ? "Ver Salidas" : "Ver Activos",
          icon: showingActive ? History : UserCheck,
          variant: "default" as const,
          tooltip: showingActive
            ? "Cambiar a vista de personas que ya salieron"
            : "Cambiar a vista de personas adentro",
          onClick: () => {
            showingActive = !showingActive;
            loadData();
          },
        },
        {
          id: "export-all-permanencia",
          label: "Exportar Todo",
          icon: Download,
          variant: "primary" as const,
          tooltip: "Exportar todos los registros de permanencia",
          onClick: () => handleExportClick(false),
        },
      ],
      singleSelect: [
        {
          id: "export-selected-single-permanencia",
          label: "Exportar",
          icon: FileDown,
          variant: "primary" as const,
          tooltip: "Exportar registro seleccionado",
          onClick: () => handleExportClick(true),
        },
      ],
      multiSelect: [
        {
          id: "export-selected-multi-permanencia",
          label: "Exportar Seleccionados",
          icon: FileDown,
          variant: "primary" as const,
          tooltip: "Exportar registros seleccionados",
          onClick: () => handleExportClick(true),
        },
      ],
    }),
  );

  onMount(() => {
    loadData();
  });
</script>

<div
  class="flex flex-col bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden h-full"
>
  <!-- Tabla o Empty State -->
  <div class="flex-1 relative">
    {#if !ingresos || ingresos.length === 0}
      <div
        class="flex h-full items-center justify-center bg-gray-50 dark:bg-gray-900"
      >
        <div class="text-center p-6">
          <div
            class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-blue-100 dark:bg-blue-900 mb-4"
          >
            <svg
              class="h-6 w-6 text-blue-600 dark:text-blue-300"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <h3 class="text-lg font-medium text-gray-900 dark:text-white">
            No hay personas adentro
          </h3>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
            En este momento no hay registros de ingresos activos.
          </p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="entries-list"
        rowData={ingresos}
        {columnDefs}
        {customButtons}
        onGridReady={(api) => (gridApi = api)}
        getRowId={(params) => params.data.id}
      />
    {/if}
  </div>
</div>

<!-- Modal Salida (sin cambios) -->
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

<!-- ✅ NUEVO: Modal de exportación -->
{#if showExportDialog}
  <ExportDialog
    onExport={handleExport}
    onClose={() => (showExportDialog = false)}
  />
{/if}
