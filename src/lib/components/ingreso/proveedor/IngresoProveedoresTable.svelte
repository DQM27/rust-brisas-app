<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { ColDef, GridApi } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { ingresoProveedorService } from "$lib/services/ingresoProveedorService";
  import type { IngresoProveedor } from "$lib/types/ingreso-nuevos";
  import SalidaModal from "../common/SalidaModal.svelte";
  import { Download, FileDown, UserPlus, RotateCw } from "lucide-svelte";

  // Export components
  import ExportDialog from "$lib/components/export/ExportDialog.svelte";
  import { exportData, downloadBytes } from "$lib/logic/export";
  import type { ExportOptions } from "$lib/logic/export";

  // SearchBar
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import { selectedSearchStore } from "$lib/stores/searchStore";

  const {
    onRegisterClick,
    onCloseForm,
    isFormOpen = false,
    refreshTrigger = 0,
  } = $props<{
    onRegisterClick?: () => void;
    onCloseForm?: () => void;
    isFormOpen?: boolean;
    refreshTrigger?: number;
  }>();

  $effect(() => {
    // Watch for changes in refreshTrigger
    if (refreshTrigger > 0) {
      loadData();
    }
  });

  let ingresos = $state<IngresoProveedor[]>([]);
  let loading = $state(false);
  let gridApi = $state<GridApi | null>(null);
  let showSalidaModal = $state(false);
  let selectedIngreso = $state<IngresoProveedor | null>(null);

  // View/Filter state
  let viewMode = $state<"activos" | "historial">("activos");

  // Export state
  let showExportDialog = $state(false);
  let exportOnlySelected = $state(false);
  let showPdfPreview = $state(false);
  let pdfPreviewUrl = $state<string | null>(null);
  let pdfPreviewName = $state("documento.pdf");

  async function loadData() {
    loading = true;
    try {
      if (viewMode === "activos") {
        ingresos = await ingresoProveedorService.getActivos();
      } else {
        ingresos = await ingresoProveedorService.getHistorial();
        console.log("Datos HISTORIAL cargados:", ingresos);
      }
    } catch (e) {
      console.error(e);
      toast.error("Error al cargar datos");
    } finally {
      loading = false;
    }
  }

  // Reload when viewMode changes
  $effect(() => {
    loadData();
  });

  onMount(() => {
    // Initial load handled by effect
    const interval = setInterval(() => {
      if (viewMode === "activos") loadData();
    }, 60000);
    return () => clearInterval(interval);
  });

  const columnDefs = $derived.by(() => {
    // Base columns shared by both views
    const baseCols: ColDef<IngresoProveedor>[] = [
      {
        field: "nombre",
        headerName: "Nombre",
        valueGetter: (params: any) =>
          `${params.data?.nombre || ""} ${params.data?.apellido || ""}`,
        flex: 1,
        minWidth: 200,
        cellStyle: { fontWeight: "500" },
      },
      {
        field: "cedula",
        headerName: "Cédula",
        width: 130,
        cellStyle: { fontFamily: "monospace" },
      },
      {
        field: "empresaNombre",
        headerName: "Empresa",
        width: 150,
        valueFormatter: (p: any) => p.value || "Sin empresa",
      },
      {
        field: "areaVisitada",
        headerName: "Área",
        width: 130,
      },
      {
        field: "motivo",
        headerName: "Motivo",
        width: 150,
      },
      {
        field: "gafete",
        headerName: "Gafete",
        width: 100,
        cellRenderer: (params: any) =>
          params.value
            ? `<span class="font-mono font-bold text-blue-600">${params.value}</span>`
            : "-",
      },
      {
        field: "placaVehiculo",
        headerName: "Vehículo",
        width: 120,
        valueFormatter: (p: any) => p.value || "-",
      },
      {
        field: "tipoAutorizacion",
        headerName: "Autorización",
        width: 120,
        valueFormatter: (p: any) => p.value || "N/A",
      },
      {
        field: "modoIngreso",
        headerName: "Modo",
        width: 120,
        valueFormatter: (p: any) => p.value || "N/A",
      },
      {
        field: "usuarioIngresoNombre",
        headerName: "Registró Entrada",
        width: 150,
        valueFormatter: (p: any) => p.value || "N/A",
      },
      {
        field: "usuarioSalidaNombre",
        headerName: "Registró Salida",
        width: 150,
        valueFormatter: (p: any) => p.value || "-",
      },
      {
        field: "fechaIngreso",
        headerName: "Fecha Entrada",
        width: 110,
        valueFormatter: (params: any) =>
          new Date(params.value).toLocaleDateString("es-CR", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric",
          }),
      },
      {
        field: "fechaIngreso",
        headerName: "Hora Entrada",
        width: 100,
        valueFormatter: (params: any) =>
          new Date(params.value).toLocaleTimeString("es-CR", {
            hour: "2-digit",
            minute: "2-digit",
            hour12: false,
          }),
      },
    ];

    // Helper for duration calculation
    const calculateDuration = (start: string, end?: string) => {
      if (!start) return "--:--";
      const startTime = new Date(start).getTime();
      const endTime = end ? new Date(end).getTime() : Date.now();
      const diffMinutes = Math.floor((endTime - startTime) / 60000);

      const hours = Math.floor(diffMinutes / 60);
      const mins = diffMinutes % 60;
      return `${String(hours).padStart(2, "0")}:${String(mins).padStart(2, "0")}`;
    };

    if (viewMode === "activos") {
      // Activos: Calculate duration from entry to NOW
      const activeCols = [
        {
          headerName: "Tiempo",
          width: 100,
          valueGetter: (params: any) =>
            calculateDuration(params.data?.fechaIngreso),
          cellRenderer: (params: any) => {
            return `<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-bold border font-mono bg-green-50 text-green-700 border-green-200">${params.value}</span>`;
          },
        },
        {
          headerName: "Acciones",
          width: 140,
          cellRenderer: () => {
            return `<button class="px-3 py-1 bg-red-100 text-red-700 rounded-md hover:bg-red-200 text-xs font-medium salida-btn">Registrar Salida</button>`;
          },
          onCellClicked: (params: any) => {
            console.log("Acción click para salir:", params.data);
            selectedIngreso = params.data;
            showSalidaModal = true;
          },
        },
      ];
      return [...baseCols, ...activeCols];
    } else {
      // Historial: Calculate duration from entry to exit
      const historyCols = [
        {
          field: "fechaSalida",
          headerName: "Fecha Salida",
          width: 110,
          valueFormatter: (params: any) =>
            params.value
              ? new Date(params.value).toLocaleDateString("es-CR", {
                  day: "2-digit",
                  month: "2-digit",
                  year: "numeric",
                })
              : "-",
        },
        {
          field: "fechaSalida",
          headerName: "Hora Salida",
          width: 100,
          valueFormatter: (params: any) =>
            params.value
              ? new Date(params.value).toLocaleTimeString("es-CR", {
                  hour: "2-digit",
                  minute: "2-digit",
                  hour12: false,
                })
              : "-",
          sort: "desc" as const,
        },
        {
          headerName: "Tiempo",
          width: 100,
          valueGetter: (params: any) =>
            calculateDuration(
              params.data?.fechaIngreso,
              params.data?.fechaSalida,
            ),
          cellRenderer: (params: any) => {
            return `<span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-bold border font-mono bg-gray-50 text-gray-700 border-gray-200">${params.value}</span>`;
          },
        },
      ];
      return [...baseCols, ...historyCols];
    }
  });

  async function handleConfirmSalida(event: CustomEvent) {
    if (!selectedIngreso) return;
    const { observaciones, devolvioGafete } = event.detail;

    try {
      await ingresoProveedorService.registrarSalida(
        selectedIngreso.id,
        $currentUser?.id || "00000000-0000-0000-0000-000000000000",
        observaciones,
        devolvioGafete,
      );
      toast.success("Salida de proveedor registrada");
      showSalidaModal = false;
      selectedIngreso = null;
      loadData();
    } catch (e: any) {
      toast.error(e.message || "Error al registrar salida");
    }
  }

  // Export handlers
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
        if (response.filePath) {
          toast.success(`Archivo guardado: ${response.filePath}`);
        } else if (response.bytes) {
          if (format === "pdf" && options.showPreview) {
            const blob = new Blob([new Uint8Array(response.bytes)], {
              type: "application/pdf",
            });
            pdfPreviewUrl = URL.createObjectURL(blob);
            pdfPreviewName = options.title
              ? `${options.title}.pdf`
              : `export.pdf`;
            showPdfPreview = true;
          } else {
            downloadBytes(
              response.bytes,
              `export.${format === "excel" ? "xlsx" : format}`,
            );
            toast.success("Archivo descargado");
          }
        }
      } else {
        toast.error(response.message || "Error al exportar");
      }
    } catch (error) {
      toast.error("Error al exportar: " + (error as Error).message);
    }
  }

  // Custom Buttons
  const customButtons = $derived.by(() => {
    const defaultButtons: CustomToolbarButton[] = [
      ...(!isFormOpen && viewMode === "activos"
        ? [
            {
              id: "register-ingreso",
              label: "Registrar Ingreso",
              icon: UserPlus,
              variant: "primary" as const,
              tooltip: "Registrar ingreso de proveedor",
              onClick: () => onRegisterClick?.(),
            },
          ]
        : []),
      {
        id: "refresh-data",
        label: "Refrescar",
        icon: RotateCw,
        variant: "default" as const,
        onClick: loadData,
      },
      {
        id: "export-all",
        label: "Exportar Todo",
        icon: Download,
        variant: "primary" as const,
        onClick: () => handleExportClick(false),
      },
    ];

    return {
      default: defaultButtons,
      singleSelect: [
        {
          id: "export-single",
          label: "Exportar",
          icon: FileDown,
          variant: "primary" as const,
          onClick: () => handleExportClick(true),
        },
      ],
      multiSelect: [
        {
          id: "export-multi",
          label: "Exportar Seleccionados",
          icon: FileDown,
          variant: "primary" as const,
          onClick: () => handleExportClick(true),
        },
      ],
    };
  });

  let searchBarRef: SearchBar;
</script>

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <!-- Header / Search (Simplified compared to Activos, but compatible) -->
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">
          {viewMode === "activos"
            ? "Proveedores Activos"
            : "Historial de Proveedores"}
        </h2>
        <p class="mt-1 text-sm text-gray-400">
          {viewMode === "activos"
            ? "Control de proveedores dentro de las instalaciones"
            : "Registro histórico de visitas de proveedores"}
        </p>
      </div>

      <!-- View Mode Toggle -->
      <div class="flex p-1 bg-gray-800 rounded-lg">
        <button
          class="px-4 py-1.5 text-sm font-medium rounded-md transition-colors {viewMode ===
          'activos'
            ? 'bg-blue-600 text-white shadow-sm'
            : 'text-gray-400 hover:text-white hover:bg-white/5'}"
          onclick={() => (viewMode = "activos")}
        >
          Activos
        </button>
        <button
          class="px-4 py-1.5 text-sm font-medium rounded-md transition-colors {viewMode ===
          'historial'
            ? 'bg-blue-600 text-white shadow-sm'
            : 'text-gray-400 hover:text-white hover:bg-white/5'}"
          onclick={() => (viewMode = "historial")}
        >
          Historial
        </button>
      </div>

      <div class="flex-1 max-w-md">
        <SearchBar
          bind:this={searchBarRef}
          placeholder="Buscar proveedor..."
          limit={5}
          disabled={loading}
          on:clear={() => selectedSearchStore.clear()}
        />
      </div>
    </div>
  </div>

  <div class="flex-1 relative overflow-hidden bg-[#1e1e1e]">
    <AGGridWrapper
      gridId="proveedores-grid"
      {columnDefs}
      rowData={ingresos}
      {customButtons}
      onGridReady={(api) => (gridApi = api)}
    />
  </div>
</div>

{#if showSalidaModal && selectedIngreso}
  <!-- Adapter for SalidaModal which expects specific fields -->
  {@const modalData = {
    id: selectedIngreso.id,
    nombreCompleto: `${selectedIngreso.nombre} ${selectedIngreso.apellido}`,
    cedula: selectedIngreso.cedula,
    gafeteNumero: selectedIngreso.gafete,
  }}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
  >
    <div class="w-full max-w-md">
      <SalidaModal
        ingreso={modalData as any}
        on:cancel={() => (showSalidaModal = false)}
        on:confirm={handleConfirmSalida}
      />
    </div>
  </div>
{/if}

{#if showExportDialog}
  <ExportDialog
    onClose={() => (showExportDialog = false)}
    onExport={handleExport}
  />
{/if}
