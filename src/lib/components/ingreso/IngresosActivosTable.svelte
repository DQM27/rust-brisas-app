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
  import type {
    IngresoResponse,
    IngresoConEstadoResponse,
    EstadoPermanencia,
  } from "$lib/types/ingreso";
  import { ingresoStore } from "$lib/stores/ingresoStore";

  import SalidaModal from "./SalidaModal.svelte";
  import DateRangePicker from "$lib/components/shared/DateRangePicker.svelte";

  // ✅ NUEVO: Importar componentes de exportación
  import ExportDialog from "$lib/components/export/ExportDialog.svelte";
  import PdfPreviewModal from "$lib/components/export/PdfPreviewModal.svelte";
  import { exportData, downloadBytes } from "$lib/services/exportService";
  import type { ExportOptions } from "$lib/services/exportService";

  // Estado
  let ingresos = $state<(IngresoResponse | IngresoConEstadoResponse)[]>([]);
  let loading = $state(false);
  let showSalidaModal = $state(false);
  let selectedIngreso = $state<
    IngresoResponse | IngresoConEstadoResponse | null
  >(null);
  let formLoading = $state(false);

  // ✅ NUEVO: Estado para exportación
  let gridApi = $state<GridApi | null>(null);
  let showExportDialog = $state(false);
  let exportOnlySelected = $state(false);

  // Estado para preview de PDF
  let showPdfPreview = $state(false);
  let pdfPreviewUrl = $state<string | null>(null);
  let pdfPreviewName = $state("documento.pdf");

  // Estado para alternar entre activos y salidas
  let showingActive = $state(true);

  // ✅ Estado para rango de fechas en historial
  let startDate = $state(new Date().toISOString().split("T")[0]);
  let endDate = $state(new Date().toISOString().split("T")[0]);

  // ✅ Auto-refresh cada 30 segundos en modo activos
  let refreshInterval: NodeJS.Timeout | null = null;

  $effect(() => {
    // Limpiar interval anterior si existe
    if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }

    // Solo auto-refresh en modo activos
    if (showingActive) {
      console.log("⏰ Auto-refresh activado (cada 30s)");
      refreshInterval = setInterval(() => {
        console.log("🔄 Auto-refresh: recargando datos...");
        loadData();
      }, 30000); // 30 segundos
    }

    // Cleanup
    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
    };
  });

  // ✅ Suscribirse al store con nueva estructura
  const unsubscribe = ingresoStore.subscribe((storeState) => {
    ingresos = storeState.data;
    if (showingActive) {
      loading = storeState.loading;
    }
  });

  onDestroy(() => {
    unsubscribe();
  });

  // Configuración de columnas - cambia según el modo
  const columnDefs = $derived.by((): ColDef<IngresoResponse>[] => {
    const baseColumns: ColDef<IngresoResponse>[] = [
      {
        field: "nombreCompleto",
        headerName: "Nombre",
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
        headerName: "Fecha Entrada",
        width: 110,
        valueFormatter: (params) =>
          new Date(params.value).toLocaleDateString("es-CR", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric",
          }),
        filterValueGetter: (params) => {
          if (!params.data) return "";
          return new Date(params.data.fechaHoraIngreso).toLocaleDateString(
            "es-CR",
            {
              day: "2-digit",
              month: "2-digit",
              year: "numeric",
            },
          );
        },
      },
      {
        field: "fechaHoraIngreso",
        headerName: "Hora Entrada",
        width: 90,
        valueFormatter: (params) =>
          new Date(params.value).toLocaleTimeString("es-CR", {
            hour: "2-digit",
            minute: "2-digit",
            hour12: false,
          }),
        filterValueGetter: (params) => {
          if (!params.data) return "";
          return new Date(params.data.fechaHoraIngreso).toLocaleTimeString(
            "es-CR",
            {
              hour: "2-digit",
              minute: "2-digit",
              hour12: false,
            },
          );
        },
      },
    ];

    // Agregar columnas condicionales según el modo
    if (!showingActive) {
      // Modo historial: agregar columnas de fecha y hora de salida
      baseColumns.push({
        field: "fechaHoraSalida" as any,
        headerName: "Fecha Salida",
        width: 110,
        valueFormatter: (params) =>
          params.value
            ? new Date(params.value).toLocaleDateString("es-CR", {
                day: "2-digit",
                month: "2-digit",
                year: "numeric",
              })
            : "-",
        filterValueGetter: (params) => {
          if (!params.data || !params.data.fechaHoraSalida) return "-";
          return new Date(params.data.fechaHoraSalida).toLocaleDateString(
            "es-CR",
            {
              day: "2-digit",
              month: "2-digit",
              year: "numeric",
            },
          );
        },
      } as ColDef<IngresoResponse>);

      baseColumns.push({
        field: "fechaHoraSalida" as any,
        headerName: "Hora Salida",
        width: 90,
        valueFormatter: (params) =>
          params.value
            ? new Date(params.value).toLocaleTimeString("es-CR", {
                hour: "2-digit",
                minute: "2-digit",
                hour12: false,
              })
            : "-",
        filterValueGetter: (params) => {
          if (!params.data || !params.data.fechaHoraSalida) return "-";
          return new Date(params.data.fechaHoraSalida).toLocaleTimeString(
            "es-CR",
            {
              hour: "2-digit",
              minute: "2-digit",
              hour12: false,
            },
          );
        },
      } as ColDef<IngresoResponse>);
    }

    // Siempre agregar columna de tiempo de permanencia CON COLORES (formato reloj)
    baseColumns.push({
      field: "tiempoPermanenciaTexto",
      headerName: "Tiempo",
      width: 120,
      valueGetter: (params) => {
        if (!params.data) return "--:--";

        // Si tiene tiempoPermanenciaTexto y no está en modo activos, usarlo
        if (params.data.tiempoPermanenciaTexto && !showingActive) {
          return params.data.tiempoPermanenciaTexto;
        }

        // Si tiene alertaTiempo, calcular desde minutosTranscurridos en formato reloj
        if (
          tieneAlertaTiempo(params.data) &&
          params.data.alertaTiempo.minutosTranscurridos != null
        ) {
          return formatearTiempoReloj(
            params.data.alertaTiempo.minutosTranscurridos,
          );
        }

        return "--:--";
      },
      cellStyle: (params) => {
        // Estilo base para historial (texto blanco en fondo negro)
        if (!params.data || !showingActive || !tieneAlertaTiempo(params.data)) {
          return {
            color: "#ffffff",
            fontWeight: "700",
            fontSize: "15px",
            fontFamily: "monospace",
            textAlign: "center",
          };
        }

        // Obtener el estado de la alerta para modo activos
        const estado = params.data.alertaTiempo.estado;

        switch (estado) {
          case "tiempo_excedido":
            return {
              backgroundColor: "#fee2e2",
              color: "#7f1d1d",
              fontWeight: "700",
              fontSize: "15px",
              fontFamily: "monospace",
              textAlign: "center",
              borderLeft: "4px solid #ef4444",
              paddingLeft: "8px",
            };
          case "alerta_temprana":
            return {
              backgroundColor: "#fef3c7",
              color: "#713f12",
              fontWeight: "700",
              fontSize: "15px",
              fontFamily: "monospace",
              textAlign: "center",
              borderLeft: "4px solid #f59e0b",
              paddingLeft: "8px",
            };
          case "normal":
          default:
            return {
              backgroundColor: "#d1fae5",
              color: "#064e3b",
              fontWeight: "700",
              fontSize: "15px",
              fontFamily: "monospace",
              textAlign: "center",
              borderLeft: "4px solid #10b981",
              paddingLeft: "8px",
            };
        }
      },
    } as ColDef<IngresoResponse>);

    if (showingActive) {
      // Modo activos: agregar columna de acciones
      baseColumns.push({
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
      } as ColDef<IngresoResponse>);
    }

    return baseColumns;
  });

  // Cargar datos
  async function loadData() {
    if (showingActive) {
      // Para activos, usa el store (que ya tiene su propio loading state)
      await ingresoStore.load();
    } else {
      // Para salidas, usar el método de rango de fechas optimizado
      loading = true;

      const result = await ingresoService.fetchSalidasEnRango(
        startDate,
        endDate,
      );

      if (result.ok) {
        ingresos = result.data;

        if (result.data.length === 0) {
          toast("No se encontraron salidas en el rango seleccionado", {
            icon: "ℹ️",
          });
        }
      } else {
        console.error("Error al cargar salidas:", result.error);
        toast.error(result.error);
        ingresos = [];
      }

      loading = false;
    }
  }

  // Handler para cambio de rango de fechas
  // Funciones para guardar/cargar estado de columnas por modo
  function getStorageKey(mode: string): string {
    return `ag-grid-entries-list-${mode}-columns`;
  }

  function getCurrentMode(): string {
    return showingActive ? "activos" : "salidas";
  }

  function saveColumnState(api: GridApi) {
    try {
      const mode = getCurrentMode(); // Obtener modo actual, no capturado
      const columnState = api.getColumnState();
      localStorage.setItem(getStorageKey(mode), JSON.stringify(columnState));
      console.log(`💾 Guardado estado de columnas para modo: ${mode}`);
    } catch (e) {
      console.error("Error guardando estado de columnas:", e);
    }
  }

  function loadColumnState(api: GridApi, mode: string) {
    try {
      const stored = localStorage.getItem(getStorageKey(mode));
      if (stored) {
        const columnState = JSON.parse(stored);

        // Intentar aplicar el estado
        try {
          api.applyColumnState({ state: columnState, applyOrder: true });
          console.log(`📥 Cargado estado de columnas para modo: ${mode}`);
        } catch (applyError) {
          // Si falla al aplicar el estado, limpiar localStorage corrupto
          console.warn(
            `⚠️ Estado de columnas corrupto para modo ${mode}, limpiando...`,
            applyError,
          );
          localStorage.removeItem(getStorageKey(mode));
        }
      }
    } catch (e) {
      console.error("Error cargando estado de columnas:", e);
      // Limpiar localStorage si hay error de parsing
      localStorage.removeItem(getStorageKey(mode));
    }
  }

  // Effect para guardar estado actual y cargar nuevo al cambiar de modo
  let previousMode = showingActive ? "activos" : "salidas";

  $effect(() => {
    if (gridApi) {
      const currentMode = showingActive ? "activos" : "salidas";

      // Si cambió el modo
      if (currentMode !== previousMode) {
        // Guardar estado del modo anterior
        saveColumnState(gridApi);

        // Pequeño delay para cargar el estado del nuevo modo
        setTimeout(() => {
          if (gridApi) {
            loadColumnState(gridApi, currentMode);
          }
        }, 100);

        previousMode = currentMode;
      }
    }
  });

  function handleDateRangeChange(event: CustomEvent) {
    startDate = event.detail.startDate;
    endDate = event.detail.endDate;
    loadData();
  }

  // Effect para actualizar columnas cuando cambia el modo
  $effect(() => {
    if (gridApi && columnDefs) {
      // Forzar actualización de columnas
      gridApi.setGridOption("columnDefs", columnDefs);
    }
  });

  // Helper para verificar si tiene alertaTiempo
  function tieneAlertaTiempo(
    ingreso: any,
  ): ingreso is IngresoConEstadoResponse {
    return ingreso && "alertaTiempo" in ingreso;
  }

  // Helper para formatear minutos a formato reloj digital (HH:MM)
  function formatearTiempoReloj(minutos: number): string {
    if (minutos < 0) return "--:--";

    const horas = Math.floor(minutos / 60);
    const mins = minutos % 60;

    // Formato digital con padding de ceros
    const horasStr = String(horas).padStart(2, "0");
    const minsStr = String(mins).padStart(2, "0");

    return `${horasStr}:${minsStr}`;
  }

  // Handlers de salida
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
        // ✅ Prioridad 1: Si hay filePath, el archivo se guardó exitosamente
        if (response.filePath) {
          toast.success(`Archivo guardado: ${response.filePath}`);
        }
        // ✅ Prioridad 2: Si hay bytes (modo preview o descarga directa)
        else if (response.bytes) {
          if (format === "pdf" && options.showPreview) {
            // Generar URL Blob para el modal
            try {
              const blob = new Blob([new Uint8Array(response.bytes)], {
                type: "application/pdf",
              });
              const url = URL.createObjectURL(blob);

              pdfPreviewUrl = url;
              pdfPreviewName = options.title
                ? `${options.title}.pdf`
                : `export-${Date.now()}.pdf`;
              showPdfPreview = true;

              toast.success("Abriendo vista previa...");
            } catch (e) {
              console.error("Error creando blob URL:", e);
              toast.error("Error al generar vista previa");
            }
          } else {
            // Descarga directa (fallback o si no es PDF preview)
            const fileName = options.title
              ? `${options.title}.${format === "excel" ? "xlsx" : format}`
              : `export-${Date.now()}.${format === "excel" ? "xlsx" : format}`;

            downloadBytes(response.bytes, fileName);
            toast.success("Archivo descargado");
          }
        } else {
          toast.error("Exportación completada pero no se recibió el archivo");
        }
      } else {
        toast.error(response.message || "Error al exportar");
      }
    } catch (error) {
      toast.error("Error al exportar: " + (error as Error).message);
    }
  }

  // ✅ NUEVO: Custom buttons para exportación y toggle
  const customButtons = $derived.by(
    (): {
      default: CustomToolbarButton[];
      singleSelect: CustomToolbarButton[];
      multiSelect: CustomToolbarButton[];
    } => {
      const defaultButtons: CustomToolbarButton[] = [
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
      ];

      return {
        default: defaultButtons,
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
      };
    },
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
    {#if loading}
      <div
        transition:fade
        class="absolute inset-0 flex items-center justify-center bg-white/50 dark:bg-gray-800/50 z-10"
      >
        <div class="text-center">
          <div
            class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"
          ></div>
          <p class="mt-3 text-sm text-gray-600 dark:text-gray-400">
            Cargando...
          </p>
        </div>
      </div>
    {/if}

    <AGGridWrapper
      gridId="entries-list"
      {columnDefs}
      rowData={ingresos}
      {customButtons}
      onGridReady={(api) => {
        gridApi = api;

        // Restaurar estado de columnas guardado
        const mode = getCurrentMode();
        loadColumnState(api, mode);

        // Guardar estado cuando las columnas cambien (sin pasar mode, usa getCurrentMode())
        api.addEventListener("columnMoved", () => saveColumnState(api));
        api.addEventListener("columnResized", () => saveColumnState(api));
        api.addEventListener("columnVisible", () => saveColumnState(api));
      }}
      getRowId={(params) => params.data.id}
    >
      {#snippet customToolbarSlot()}
        <!-- ✅ DateRangePicker solo visible en modo salidas -->
        {#if !showingActive}
          <DateRangePicker
            {startDate}
            {endDate}
            label="Periodo"
            on:change={handleDateRangeChange}
          />
        {/if}
      {/snippet}
    </AGGridWrapper>
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

<!-- ✅ NUEVO: Modal de exportación -->
{#if showExportDialog}
  <ExportDialog
    onExport={handleExport}
    onClose={() => (showExportDialog = false)}
  />
{/if}

<!-- ✅ NUEVO: Modal de Preview PDF -->
{#if showPdfPreview && pdfPreviewUrl}
  <PdfPreviewModal
    pdfUrl={pdfPreviewUrl}
    fileName={pdfPreviewName}
    onClose={() => {
      showPdfPreview = false;
      // Revocar URL para liberar memoria
      if (pdfPreviewUrl) {
        URL.revokeObjectURL(pdfPreviewUrl);
        pdfPreviewUrl = null;
      }
    }}
  />
{/if}
