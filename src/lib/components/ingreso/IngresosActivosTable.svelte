<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { currentUser } from "$lib/stores/auth";
  import type { ColDef, GridApi } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import {
    Download,
    FileDown,
    UserCheck,
    History,
    UserPlus,
  } from "lucide-svelte";

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
  import { exportData, downloadBytes } from "$lib/logic/export";
  import type { ExportOptions } from "$lib/logic/export";

  // ✅ NUEVO: Importar SearchBar y store de búsqueda
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import { selectedSearchStore } from "$lib/stores/searchStore";
  import type { SearchResult } from "$lib/types/search.types";

  // ✅ NUEVO: Importar store de tiempo y utilidad de evaluación
  import { currentTime } from "$lib/stores/timeStore";
  import { evaluateTimeStatus } from "$lib/logic/ingreso/ingresoService";

  // Props
  const {
    onRegisterClick,
    onCloseForm,
    isFormOpen = false,
  } = $props<{
    onRegisterClick?: () => void;
    onCloseForm?: () => void;
    isFormOpen?: boolean;
  }>();

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

  // Flag para prevenir sobrescritura de estado durante cambios de modo
  let isRestoringState = false;

  // ✅ Estado para rango de fechas en historial
  let startDate = $state(new Date().toISOString().split("T")[0]);
  let endDate = $state(new Date().toISOString().split("T")[0]);

  // ✅ Eliminar polling de 30 segundos
  // El tiempo ahora es reactivo gracias a currentTime store
  // Solo necesitamos recargar si queremos datos nuevos del backend (nuevos ingresos)
  // Dejamos un polling más lento (e.g. 5 min) o dependemos de SSE/WebSockets en el futuro
  // Por ahora, eliminamos el intervalo agresivo.

  // Si se desea mantener actualización de lista (no tiempo), podemos dejarlo cada 2-5 min
  $effect(() => {
    let interval = setInterval(() => {
      if (showingActive) loadData();
    }, 60000 * 5); // 5 minutos para nuevos ingresos

    return () => clearInterval(interval);
  });

  // ✅ Suscribirse al store con nueva estructura
  // ✅ Suscribirse al store
  const unsubscribe = ingresoStore.subscribe((storeState) => {
    ingresos = storeState.data;
    if (showingActive) {
      loading = storeState.loading;
    }
  });

  // ✅ Computed: Ingresos procesados con tiempo real y FILTRADO POR BÚSQUEDA
  // Suscribirse a cambios en la búsqueda
  const selectedSearch = $derived($selectedSearchStore.result);

  const processedIngresos = $derived.by(() => {
    let data = ingresos;

    // 1. Filtrar por búsqueda si hay un resultado seleccionado
    if (selectedSearch) {
      data = data.filter((item) => item.contratistaId === selectedSearch.id);
    }

    // 2. Si no es activo, devolver datos filtrados
    if (!showingActive) return data;

    // 3. Procesar tiempo real para activos
    return data.map((ingreso) => {
      // Calcular estado de tiempo en tiempo real
      if (ingreso.estaAdentro && ingreso.fechaHoraIngreso) {
        const entryDate = new Date(ingreso.fechaHoraIngreso);
        const status = evaluateTimeStatus(entryDate, $currentTime);

        // Retornar copia con alerta actualizada
        return {
          ...ingreso,
          alertaTiempo: status,
        } as IngresoConEstadoResponse;
      }
      return ingreso;
    });
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
      cellRenderer: (params: any) => {
        let tiempo = "--:--";

        // Obtener texto del tiempo
        if (params.data) {
          if (params.data.tiempoPermanenciaTexto && !showingActive) {
            tiempo = params.data.tiempoPermanenciaTexto;
          } else if (
            tieneAlertaTiempo(params.data) &&
            params.data.alertaTiempo.minutosTranscurridos != null
          ) {
            tiempo = formatearTiempoReloj(
              params.data.alertaTiempo.minutosTranscurridos,
            );
          }
        }

        // Estilo base
        const baseClass =
          "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-bold border font-mono";

        // Determinar color
        let colorClass =
          "bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700"; // Default/Historial

        if (showingActive && params.data && tieneAlertaTiempo(params.data)) {
          const estado = params.data.alertaTiempo.estado;
          switch (estado) {
            case "tiempo_excedido":
              // Red
              colorClass =
                "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
              break;
            case "alerta_temprana":
              // Orange/Yellow
              colorClass =
                "bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800";
              break;
            case "normal":
              // Green
              colorClass =
                "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";
              break;
          }
        }

        return `<span class="${baseClass} ${colorClass}">${tiempo}</span>`;
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
    if (isRestoringState) return;

    try {
      const mode = getCurrentMode(); // Obtener modo actual, no capturado
      const columnState = api.getColumnState();
      localStorage.setItem(getStorageKey(mode), JSON.stringify(columnState));
      // console.log(`💾 Guardado estado de columnas para modo: ${mode}`);
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
          // console.log(`📥 Cargado estado de columnas para modo: ${mode}`);
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
  let previousMode = "activos";

  $effect(() => {
    if (gridApi) {
      const currentMode = showingActive ? "activos" : "salidas";

      // Si cambió el modo
      if (currentMode !== previousMode) {
        // Nota: El guardado del estado anterior se hace en el onClick del botón toggle
        // para asegurar que se guarde antes de que cambie showingActive

        // Pequeño delay para cargar el estado del nuevo modo
        setTimeout(() => {
          if (gridApi) {
            loadColumnState(gridApi, currentMode);
            // Desbloquear guardado después de restaurar
            // Damos un poco más de tiempo para que se asienten los eventos internos
            setTimeout(() => {
              isRestoringState = false;
            }, 500);
          }
        }, 200);

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

      // No cargar estado aquí directo, dejar que onGridReady o el cambio de modo lo manejen
      // para evitar conflictos con el ciclo de vida inicial
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
        ...(showingActive && !isFormOpen
          ? [
              {
                id: "register-ingreso",
                label: "Registrar Ingreso",
                icon: UserPlus,
                variant: "primary" as const,
                tooltip: "Abrir formulario de registro",
                onClick: () => {
                  onRegisterClick?.();
                },
              },
            ]
          : []),

        {
          id: "toggle-view",
          label: showingActive ? "Ver Salidas" : "Ver Activos",
          icon: showingActive ? History : UserCheck,
          variant: "default" as const,
          tooltip: showingActive
            ? "Cambiar a vista de personas que ya salieron"
            : "Cambiar a vista de personas adentro",
          onClick: () => {
            // 0. Cerrar formulario si está abierto
            onCloseForm?.();

            // 1. Guardar estado actual ANTES de cambiar
            if (gridApi) saveColumnState(gridApi);

            // 2. Bloquear guardado automático (para evitar que se guarde el reset de columnas)
            isRestoringState = true;

            // 3. Cambiar modo
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

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
  <div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-gray-100">Control de Ingresos</h2>
        <p class="mt-1 text-sm text-gray-400">
          Gestión de entradas y salidas en tiempo real
        </p>
      </div>

      <!-- ✅ SearchBar -->
      <div class="flex-1 max-w-md">
        <SearchBar
          placeholder="Buscar persona para ver su ingreso..."
          limit={5}
          disabled={loading}
          on:clear={() => selectedSearchStore.clear()}
        />
      </div>
    </div>
  </div>

  <!-- Tabla o Empty State -->
  <div class="flex-1 relative overflow-hidden bg-[#1e1e1e]">
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
      rowData={processedIngresos}
      {customButtons}
      onGridReady={(api) => {
        gridApi = api;

        // Restaurar estado de columnas guardado con un pequeño delay
        // para asegurar que el grid esté listo para animaciones
        setTimeout(() => {
          const mode = getCurrentMode();
          loadColumnState(api, mode);

          // ✅ Attach listeners AFTER loading state to prevent overwriting with defaults
          // during initialization or columnDefs updates
          api.addEventListener("columnMoved", () => saveColumnState(api));
          api.addEventListener("columnResized", () => saveColumnState(api));
          api.addEventListener("columnVisible", () => saveColumnState(api));
        }, 300); // 300ms to be safe after autoSize (150ms)
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
    columns={gridApi?.getColumns()?.map((col) => ({
      id: col.getColId(),
      name: col.getColDef().headerName || col.getColId(),
      selected: col.isVisible(),
    })) || []}
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
