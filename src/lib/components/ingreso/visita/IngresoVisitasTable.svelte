<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { ColDef, GridApi } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { ingresoVisitaService } from "$lib/services/ingresoVisitaService";
  import type { IngresoVisita } from "$lib/types/ingreso-nuevos";
  import SalidaModal from "../common/SalidaModal.svelte";
  import { Download, FileDown, UserPlus, RotateCw } from "lucide-svelte";

  // Export components
  import ExportDialog from "$lib/components/export/ExportDialog.svelte";
  import PdfPreviewModal from "$lib/components/export/PdfPreviewModal.svelte";
  import { exportData, downloadBytes } from "$lib/logic/export";
  import type { ExportOptions } from "$lib/logic/export";

  // SearchBar
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import { selectedSearchStore } from "$lib/stores/searchStore";

  import { shortcutService } from "$lib/services/shortcutService";

  const {
    onRegisterClick,
    onCloseForm,
    isFormOpen = false,
  } = $props<{
    onRegisterClick?: () => void;
    onCloseForm?: () => void;
    isFormOpen?: boolean;
  }>();

  let ingresos = $state<IngresoVisita[]>([]);
  let loading = $state(false);
  let gridApi = $state<GridApi | null>(null);
  let showSalidaModal = $state(false);
  let selectedIngreso = $state<IngresoVisita | null>(null);

  // Export state
  let showExportDialog = $state(false);
  let exportOnlySelected = $state(false);
  let showPdfPreview = $state(false);
  let pdfPreviewUrl = $state<string | null>(null);
  let pdfPreviewName = $state("documento.pdf");

  async function loadData() {
    loading = true;
    try {
      ingresos = await ingresoVisitaService.getActivos();
    } catch (e) {
      console.error(e);
      toast.error("Error al cargar visitas activas");
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadData();
    const interval = setInterval(loadData, 60000);
    return () => clearInterval(interval);
  });

  const columnDefs: ColDef<IngresoVisita>[] = [
    {
      field: "visitanteNombre",
      headerName: "Nombre",
      valueGetter: (params) =>
        `${params.data?.visitanteNombre || ""} ${params.data?.visitanteApellido || ""}`,
      flex: 1,
      minWidth: 200,
    },
    {
      field: "visitanteCedula",
      headerName: "Cédula",
      width: 130,
    },
    {
      field: "visitanteEmpresa",
      headerName: "Empresa / Proc.",
      width: 150,
      valueFormatter: (p) => p.value || "-",
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
      field: "anfitrion",
      headerName: "Visita a",
      width: 150,
    },
    {
      field: "areaVisitada",
      headerName: "Área",
      width: 130,
    },
    {
      field: "fechaIngreso",
      headerName: "Entrada",
      width: 110,
      valueFormatter: (params) =>
        new Date(params.value).toLocaleTimeString("es-CR", {
          hour: "2-digit",
          minute: "2-digit",
          hour12: false,
        }),
    },
    {
      headerName: "Acciones",
      width: 140,
      cellRenderer: () => {
        return `<button class="px-3 py-1 bg-red-100 text-red-700 rounded-md hover:bg-red-200 text-xs font-medium salida-btn">Registrar Salida</button>`;
      },
      onCellClicked: (params: any) => {
        if (params.event.target.classList.contains("salida-btn")) {
          selectedIngreso = params.data;
          showSalidaModal = true;
        }
      },
    },
  ];

  async function handleConfirmSalida(event: CustomEvent) {
    if (!selectedIngreso) return;
    const { observaciones } = event.detail;

    try {
      await ingresoVisitaService.registrarSalida(
        selectedIngreso.id,
        $currentUser?.id || "00000000-0000-0000-0000-000000000000",
        observaciones,
      );
      toast.success("Salida de visita registrada");
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
      ...(!isFormOpen
        ? [
            {
              id: "register-ingreso",
              label: "Registrar Visita",
              icon: UserPlus,
              variant: "primary" as const,
              tooltip: "Nueva visita",
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
        <h2 class="text-xl font-semibold text-gray-100">Visitas Activas</h2>
        <p class="mt-1 text-sm text-gray-400">
          Control de visitantes dentro de las instalaciones
        </p>
      </div>
      <div class="flex-1 max-w-md">
        <SearchBar
          bind:this={searchBarRef}
          placeholder="Buscar visita..."
          limit={5}
          disabled={loading}
          on:clear={() => selectedSearchStore.clear()}
        />
      </div>
    </div>
  </div>

  <div class="flex-1 relative overflow-hidden bg-[#1e1e1e]">
    <AGGridWrapper
      gridId="visitas-activas-grid"
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
    nombreCompleto: `${selectedIngreso.visitanteNombre} ${selectedIngreso.visitanteApellido}`,
    cedula: selectedIngreso.visitanteCedula,
    gafeteNumero: selectedIngreso.gafete,
    // Add other required fields by IngresoResponse if strict, but for modal only these are used
  }}
  <SalidaModal
    ingreso={modalData as any}
    on:cancel={() => (showSalidaModal = false)}
    on:confirm={handleConfirmSalida}
  />
{/if}

{#if showExportDialog}
  <ExportDialog
    onClose={() => (showExportDialog = false)}
    onExport={handleExport}
  />
{/if}

{#if showPdfPreview}
  <PdfPreviewModal
    onClose={() => (showPdfPreview = false)}
    pdfUrl={pdfPreviewUrl || ""}
    fileName={pdfPreviewName}
  />
{/if}
