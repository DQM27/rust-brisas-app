<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { AlertCircle, RotateCw } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  // Components
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import IngresoFormModal from "./IngresoFormModal.svelte";
  import SalidaModal from "./SalidaModal.svelte";
  import QuickExitModal from "./QuickExitModal.svelte";
  import QuickEntryModal from "./QuickEntryModal.svelte";
  import ExportDialog from "$lib/components/export/ExportDialog.svelte";

  // Logic
  import { invoke } from "@tauri-apps/api/core";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import { currentUser } from "$lib/stores/auth";
  import { activeTabId } from "$lib/stores/tabs";
  import { exportData, getAvailableFormats } from "$lib/logic/export";
  import {
    keyboardCommand,
    setActiveContext,
    clearCommand,
  } from "$lib/stores/keyboardCommands";

  // Shared Components
  import DateRangePicker from "$lib/components/shared/DateRangePicker.svelte";
  // import ModuleTabs from "$lib/components/shared/ModuleTabs.svelte"; // Removed as per revert
  import { History, Users, FileText } from "lucide-svelte";
  import { openTab } from "$lib/stores/tabs";
  import ContratistaFormModal from "$lib/components/contratista/ContratistaFormModal.svelte";
  import * as contratistaService from "$lib/logic/contratista/contratistaService";

  // Types
  import type { CustomToolbarButton } from "$lib/types/agGrid";

  // Props
  interface Props {
    tabId?: string;
  }
  let { tabId = "ingreso-list" }: Props = $props();

  // ==========================================
  // STATE
  // ==========================================
  let ingresos = $state<any[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selectedRows = $state<any[]>([]);
  let showModal = $state(false);
  let showContratistaModal = $state(false);

  // Estado para el modal de salida
  let showSalidaModal = $state(false);
  let showQuickExit = $state(false);
  let showQuickEntry = $state(false);
  let personForIngreso = $state<any>(null);
  let selectedIngreso = $state<any>(null);
  let salidaLoading = $state(false);

  // Estado para Exportación
  let gridApi = $state<any>(null);
  let showExportModal = $state(false);
  let availableFormats = $state<string[]>([]);
  let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>(
    [],
  );

  // ==========================================
  // HISTORIAL / VIEW MODE STATE
  // ==========================================
  type ViewMode = "actives" | "history";
  let viewMode = $state<ViewMode>("actives");

  // Rango de fechas por defecto: Hoy
  const today = new Date().toISOString().split("T")[0];
  let dateRange = $state({
    start: today,
    end: today,
  });

  // Suscripción a comandos de teclado centralizados
  let unsubscribeKeyboard: (() => void) | null = null;

  function setupKeyboardSubscription() {
    unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
      if (!event) return;
      if ($activeTabId !== tabId) return;

      switch (event.command) {
        case "create-new":
          if (!showModal && !showSalidaModal && !showQuickEntry) {
            showQuickEntry = true;
            clearCommand();
          }
          break;
        case "escape":
          if (showModal) {
            showModal = false;
            clearCommand();
          } else if (showSalidaModal) {
            showSalidaModal = false;
            selectedIngreso = null;
            clearCommand();
          } else if (showQuickEntry) {
            showQuickEntry = false;
            clearCommand();
          }
          break;
        case "refresh":
          loadIngresos();
          clearCommand();
          break;
        case "save": // Ctrl+S
          if (viewMode === "actives" && !showModal && !showSalidaModal) {
            showQuickExit = true;
            clearCommand();
          }
          break;
      }
    });
  }

  // ==========================================
  // HELPERS
  // ==========================================
  function parseDate(value: any): Date | null {
    if (!value) return null;
    let dateStr = String(value);
    // Remove SurrealDB format wrappers if present (defensive programming)
    if (dateStr.startsWith("d'") && dateStr.endsWith("'")) {
      dateStr = dateStr.slice(2, -1);
    }
    const d = new Date(dateStr);
    return isNaN(d.getTime()) ? null : d;
  }

  // ==========================================
  // COLUMNS
  // ==========================================
  let columnDefs = $derived.by((): ColDef<any>[] => {
    const baseCols: ColDef<any>[] = [
      {
        field: "gafeteNumero",
        headerName: "Gafete",
        width: 100,
        sortable: true,
        filter: true,
        valueFormatter: (params) => {
          if (!params.value) return "S/G";
          return params.value;
        },
      },
      {
        field: "nombreCompleto",
        headerName: "Nombre",
        width: 200,
        sortable: true,
        filter: true,
      },
      {
        field: "cedula",
        headerName: "Cédula",
        width: 130,
        sortable: true,
        filter: true,
      },
      {
        field: "empresaNombre",
        headerName: "Empresa",
        width: 180,
        sortable: true,
        filter: true,
      },
      {
        field: "tipoAutorizacionDisplay",
        headerName: "Autorización",
        width: 130,
        sortable: true,
        filter: true,
      },
      {
        field: "modoIngresoDisplay",
        headerName: "Modo",
        width: 110,
        sortable: true,
        filter: true,
      },
      {
        field: "fechaHoraIngreso",
        headerName: "Fecha Entrada",
        width: 140,
        sortable: true,
        valueFormatter: (params) => {
          const date = parseDate(params.value);
          if (!date) return "";
          return date.toLocaleDateString("es-ES", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric",
          });
        },
      },
      {
        field: "fechaHoraIngreso",
        headerName: "Hora Entrada",
        width: 120,
        sortable: true,
        valueFormatter: (params) => {
          const date = parseDate(params.value);
          if (!date) return "";
          return date.toLocaleTimeString("es-ES", {
            hour: "2-digit",
            minute: "2-digit",
          });
        },
      },
      {
        field: "usuarioIngresoNombre",
        headerName: "Registrado Por",
        width: 150,
        sortable: true,
        filter: true,
      },
      {
        field: "fechaHoraSalida",
        headerName: "Fecha Salida",
        width: 140,
        sortable: true,
        valueFormatter: (params) => {
          const date = parseDate(params.value);
          if (!date) return "-";
          return date.toLocaleDateString("es-ES", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric",
          });
        },
      },
      {
        field: "fechaHoraSalida",
        headerName: "Hora Salida",
        width: 120,
        sortable: true,
        valueFormatter: (params) => {
          const date = parseDate(params.value);
          if (!date) return "-";
          return date.toLocaleTimeString("es-ES", {
            hour: "2-digit",
            minute: "2-digit",
          });
        },
      },
      {
        field: "usuarioSalidaNombre",
        headerName: "Salida Por",
        width: 150,
        sortable: true,
        filter: true,
        valueFormatter: (params) => {
          if (!params.value) return "-";
          return params.value;
        },
      },
      {
        field: "tiempoPermanenciaTexto",
        headerName: "Tiempo Dentro",
        width: 140,
        sortable: true,
        valueGetter: (params) => {
          if (params.data.fechaHoraSalida) {
            return params.data.tiempoPermanenciaTexto || "-";
          }
          // Calcular tiempo transcurrido si aún está adentro
          const entrada = parseDate(params.data.fechaHoraIngreso);
          if (!entrada) return "-";

          const ahora = new Date();
          const diffMs = ahora.getTime() - entrada.getTime();
          const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
          const diffMins = Math.floor(
            (diffMs % (1000 * 60 * 60)) / (1000 * 60),
          );
          return `${diffHours}h ${diffMins}m`;
        },
      },
      {
        field: "actions",
        headerName: "Acciones",
        width: 120,
        sortable: false,
        filter: false,
        pinned: "right",
        cellRenderer: (params: any) => {
          const button = document.createElement("button");
          button.className =
            "px-3 py-1 bg-error text-white rounded-md text-sm hover:opacity-90 transition-opacity";
          button.textContent = "Salida";
          button.onclick = () => handleSalida(params.data);
          return button;
        },
      },
    ];

    // Filter out columns not needed in history mode
    if (viewMode === "history") {
      return baseCols.filter((c) => c.field !== "actions");
    }
    return baseCols;
  });

  // ==========================================
  // TOOLBAR BUTTONS
  // ==========================================
  // ==========================================
  // TOOLBAR BUTTONS
  // ==========================================
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    const defaultButtons: any[] = [
      createCustomButton.exportar(() => handleExportClick()),
    ];

    // Botón List Contratista siempre visible (Action secondary)
    defaultButtons.unshift({
      id: "list-contratista-view",
      label: "List Contratista",
      icon: FileText, // Icono de lista/archivo
      onClick: () => {
        console.log("Abriendo pestaña de contratistas..."); // Debug log
        openTab({
          componentKey: "contratista-list",
          title: "List. Contratistas",
          id: "contratista-list",
          focusOnOpen: true,
        });
      },
      variant: "default",
      tooltip: "Ir a listado completo de contratistas",
    });

    if (viewMode === "actives") {
      // Botón "Nuevo Contratista" (Lanza modal creación rápida)
      defaultButtons.unshift({
        id: "new-contractor",
        label: "Nuevo Contratista",
        icon: Users, // Icono de usuario +
        onClick: () => (showContratistaModal = true),
        variant: "default",
        tooltip: "Registrar nuevo contratista en base de datos",
      });

      // Botón "Nuevo Ingreso" (Lanza modal de ingreso/entrada)
      defaultButtons.unshift(
        createCustomButton.nuevo(
          () => handleNuevoIngreso(),
          false,
          "Nuevo Ingreso",
        ),
      );
    }

    return {
      default: defaultButtons,
      singleSelect: [createCustomButton.exportar(() => handleExportClick())],
      multiSelect: [createCustomButton.exportar(() => handleExportClick())],
    };
  });

  // ==========================================
  // HANDLERS
  // ==========================================
  async function loadIngresos() {
    loading = true;
    error = "";
    try {
      let data;
      if (viewMode === "actives") {
        data = await invoke("get_ingresos_abiertos");
      } else {
        // Modo Historial: Cargar por rango de fechas
        // PROBLEMA: Al concatenar 'T00:00:00Z', se interpreta como UTC.
        // Si el usuario está en UTC-6 (CDMX), el día "7" (00:00 UTC) es en realidad el día 6 por la tarde.
        // SOLUCIÓN: Usar la zona horaria local o mandar ISO pero sabiendo que el backend compara directo.
        // Mejor enfoque: Mandar el rango completo del día LOCAL convertido a UTC para la DB.

        // Pero espera, SurrealDB guarda en UTC.
        // Si quiero ver los registros del día 7 (Local), necesito desde 7T00:00 Local hasta 7T23:59 Local.
        // 7T00:00 Local -> 7T06:00 UTC (si es UTC-6)
        // 7T23:59 Local -> 8T05:59 UTC

        // Vamos a construir fechas locales y sacarle el ISO string real.
        const startLocal = new Date(dateRange.start + "T00:00:00");
        const endLocal = new Date(dateRange.end + "T23:59:59.999");

        const start = startLocal.toISOString();
        const end = endLocal.toISOString();

        data = await invoke("get_salidas_en_rango", {
          fechaInicio: start,
          fechaFin: end,
        });
      }
      ingresos = data as any[];
    } catch (err: any) {
      error = err.message || "Error al cargar datos";
      toast.error(error);
      ingresos = [];
    } finally {
      loading = false;
    }
  }

  function handleDateRangeChange(
    event: CustomEvent<{ startDate: string; endDate: string }>,
  ) {
    dateRange.start = event.detail.startDate;
    dateRange.end = event.detail.endDate;
    loadIngresos();
  }

  function toggleViewMode(mode: ViewMode) {
    if (viewMode === mode) return;
    viewMode = mode;
    loadIngresos();
  }

  function handleNuevoIngreso() {
    showQuickEntry = true;
  }

  function handleQuickEntrySelect(person: any) {
    showQuickEntry = false;
    personForIngreso = person;
    // Dar un pequeño tiempo para que cierre un modal y abra el otro suavemente
    setTimeout(() => {
      showModal = true;
    }, 100);
  }

  function handleModalComplete() {
    showModal = false;
    personForIngreso = null;
    loadIngresos();
  }

  function handleSalida(ingreso: any) {
    selectedIngreso = ingreso;
    showSalidaModal = true;
  }

  function handleQuickExitSelect(ingreso: any) {
    showQuickExit = false;
    // Dar un pequeño tiempo para que cierre un modal y abra el otro suavemente
    setTimeout(() => {
      handleSalida(ingreso);
    }, 100);
  }

  // ==========================================
  // EXPORT
  // ==========================================
  async function handleExportClick() {
    if (!gridApi) return;

    // Obtener formatos disponibles
    availableFormats = await getAvailableFormats();

    // Obtener columnas para el selector
    const cols = gridApi.getAllGridColumns();
    exportColumns = cols
      .map((col: any) => ({
        id: col.getColId(),
        name: col.getColDef().headerName || col.getColId(),
        selected: col.isVisible(),
      }))
      .filter((col: any) => col.id !== "actions" && col.id !== "selection");

    showExportModal = true;
  }

  async function handleExport(format: any, options: any) {
    if (!gridApi) return;

    try {
      const isSelection = selectedRows.length > 0;
      const toastId = toast.loading(
        `Exportando ${isSelection ? "selección" : "todo"} a ${format.toUpperCase()}...`,
      );
      await exportData(gridApi, format, options, isSelection);
      toast.success("Exportación completada", { id: toastId });
    } catch (err: any) {
      if (err.message !== "Exportación cancelada por el usuario") {
        toast.error("Error al exportar: " + err.message);
      }
    }
  }

  async function handleSalidaConfirm(event: CustomEvent) {
    const { devolvioGafete, observaciones } = event.detail;

    if (!selectedIngreso) return;

    // Validar que hay un usuario autenticado
    if (!$currentUser?.id) {
      toast.error(
        "Error: No hay sesión activa. Por favor, inicie sesión nuevamente.",
      );
      return;
    }

    const usuarioId = $currentUser.id;

    try {
      salidaLoading = true;

      if (selectedIngreso.tipoIngreso === "contratista") {
        await invoke("register_exit_contratista", {
          input: {
            ingresoId: selectedIngreso.id,
            devolvioGafete: devolvioGafete,
            usuarioSalidaId: usuarioId,
            observacionesSalida: observaciones,
          },
          usuarioId: usuarioId,
        });
      } else if (selectedIngreso.tipoIngreso === "proveedor") {
        await invoke("registrar_salida_proveedor", {
          id: selectedIngreso.id,
          usuarioId: usuarioId,
          observaciones: observaciones,
          devolvioGafete: devolvioGafete,
        });
      } else {
        await invoke("registrar_salida_visita", {
          id: selectedIngreso.id,
          usuarioId: usuarioId,
          devolvioGafete: devolvioGafete,
          observaciones: observaciones,
        });
      }

      toast.success("Salida registrada exitosamente");
      showSalidaModal = false;
      selectedIngreso = null;
      loadIngresos();
    } catch (err: any) {
      toast.error("Error al registrar salida: " + err.message);
    } finally {
      salidaLoading = false;
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================
  onMount(() => {
    loadIngresos();
    setupKeyboardSubscription();
  });

  onDestroy(() => {
    if (unsubscribeKeyboard) {
      unsubscribeKeyboard();
    }
  });

  // Registrar contexto activo cuando esta pestaña está activa
  $effect(() => {
    if ($activeTabId === tabId) {
      setActiveContext("ingreso-list");
    }
  });
</script>

<div class="flex h-full flex-col relative bg-surface-1">
  <!-- Header -->
  <div class="border-b border-surface px-6 py-4 bg-surface-2">
    <div class="flex flex-col gap-4">
      <!-- Top Row: Title & Toggle -->
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-xl font-semibold text-primary">
            {viewMode === "actives"
              ? "Ingresos Activos"
              : "Historial de Salidas"}
          </h2>
          <p class="mt-1 text-sm text-secondary">
            {viewMode === "actives"
              ? "Personas actualmente en planta"
              : "Registro histórico de visitas finalizadas"}
          </p>
        </div>

        <div class="flex items-center gap-4">
          <!-- View Toggle (Segmented Control) -->
          <div
            class="relative flex items-center bg-surface-3 p-1 rounded-lg isolate"
          >
            <!-- Fondo deslizante animado (Pill) -->
            <div
              class="absolute top-1 bottom-1 rounded-md bg-white dark:bg-zinc-700 shadow-sm transition-all duration-300 ease-in-out z-[-1]"
              style="
                  left: {viewMode === 'actives' ? '4px' : '50%'};
                  right: {viewMode === 'actives' ? '50%' : '4px'};
                  width: calc(50% - 6px);
                "
            ></div>

            <button
              class="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors z-10
                {viewMode === 'actives'
                ? 'text-primary dark:text-white'
                : 'text-secondary hover:text-primary dark:hover:text-zinc-300'}"
              onclick={() => toggleViewMode("actives")}
            >
              <Users
                size={16}
                class={viewMode === "actives"
                  ? "scale-110 transition-transform"
                  : ""}
              />
              Activos
            </button>

            <button
              class="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors z-10
                {viewMode === 'history'
                ? 'text-primary dark:text-white'
                : 'text-secondary hover:text-primary dark:hover:text-zinc-300'}"
              onclick={() => toggleViewMode("history")}
            >
              <History
                size={16}
                class={viewMode === "history"
                  ? "scale-110 transition-transform"
                  : ""}
              />
              Historial
            </button>
          </div>
        </div>
      </div>

      <!-- Bottom Row: Controls -->
      <div class="flex items-center justify-between gap-4">
        <div class="flex-1 max-w-md">
          <SearchBar placeholder="Buscar por nombre, gafete..." limit={10} />
        </div>
      </div>
    </div>
  </div>

  <!-- Content -->
  <div
    class="flex-1 overflow-hidden relative bg-surface-1 border-t border-surface"
  >
    {#snippet toolbarControls()}
      {#if viewMode === "history"}
        <div class="flex items-center" transition:fade={{ duration: 150 }}>
          <DateRangePicker
            startDate={dateRange.start}
            endDate={dateRange.end}
            on:change={handleDateRangeChange}
          />
        </div>
      {/if}
    {/snippet}

    {#if error}
      <div class="p-6">
        <div
          class="flex items-center gap-3 rounded-lg border border-error bg-error bg-opacity-10 p-4 text-error"
          transition:fade
        >
          <AlertCircle size={20} />
          <div>
            <div class="font-medium">Error al cargar ingresos</div>
            <div class="text-sm opacity-90">{error}</div>
          </div>
        </div>
      </div>
    {:else if loading}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <svg
            class="mx-auto h-8 w-8 animate-spin text-accent"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          <p class="mt-4 text-sm text-secondary">Cargando ingresos...</p>
        </div>
      </div>
    {:else}
      <AGGridWrapper
        gridId="ingreso-list"
        {columnDefs}
        rowData={ingresos}
        {customButtons}
        getRowId={(params) => params.data.id}
        persistenceKey="ingresos-activos-columns"
        onSelectionChanged={(rows) => (selectedRows = rows)}
        onGridReady={(api) => (gridApi = api)}
        customToolbarSlot={toolbarControls}
      />
    {/if}
  </div>
</div>

<!-- Modal -->
<IngresoFormModal
  bind:show={showModal}
  initialPerson={personForIngreso}
  on:complete={handleModalComplete}
/>

<!-- Modal Creación Contratista -->
<ContratistaFormModal
  show={showContratistaModal}
  onClose={() => (showContratistaModal = false)}
  onSave={async (data) => {
    // Aquí implementamos la lógica de guardado rápido o delegamos al servicio
    // Como ContratistaFormModal ya maneja onSave internamente si le pasamos la logica...
    // Espera, ContratistaListView maneja la logica de guardado en `handleSaveContratista`.
    // Debemos replicar esa lógica mínima aquí o importar el servicio.
    // El componente `ContratistaFormModal` del código anterior emite `onSave` con la data.
    // Necesitamos llamar a `contratistaService.createContratista`.
    try {
      const res = await contratistaService.createContratista(data as any);
      if (res.ok) {
        toast.success("Contratista creado exitosamente");
        showContratistaModal = false;
        // Opcional: Podríamos abrir el modal de ingreso pre-seleccionando este contratista si se desea.
      } else {
        toast.error(res.error);
      }
    } catch (e) {
      console.error(e);
      toast.error("Error al crear contratista");
    }
  }}
/>

<!-- Modal de Salida -->
<SalidaModal
  bind:show={showSalidaModal}
  ingreso={selectedIngreso}
  loading={salidaLoading}
  on:confirm={handleSalidaConfirm}
  on:close={() => {
    showSalidaModal = false;
    selectedIngreso = null;
  }}
/>

<!-- Modal de Salida Rápida (Buscador) -->
<QuickExitModal
  bind:show={showQuickExit}
  activeEntries={ingresos}
  onSelect={handleQuickExitSelect}
  onClose={() => (showQuickExit = false)}
/>

<!-- Modal de Entrada Rápida (Buscador Global) -->
<QuickEntryModal
  bind:show={showQuickEntry}
  onSelect={handleQuickEntrySelect}
  onClose={() => (showQuickEntry = false)}
/>

{#if showExportModal}
  <ExportDialog
    onExport={handleExport}
    onClose={() => (showExportModal = false)}
    {availableFormats}
    columns={exportColumns}
  />
{/if}
