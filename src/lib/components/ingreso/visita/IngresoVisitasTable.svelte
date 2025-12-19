<script lang="ts">
  // @ts-nocheck - Svelte 5 runes ($state, $props, $derived, $effect) are not recognized by TS
  import { toast } from "svelte-5-french-toast";
  import { currentUser as userStore } from "$lib/stores/auth";
  import type { ColDef, GridApi } from "@ag-grid-community/core";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { ingresoVisitaService } from "$lib/services/ingresoVisitaService";
  import { citaService } from "$lib/services/citaService";
  import type { IngresoVisita } from "$lib/types/ingreso-nuevos";
  import type { CitaPopulated } from "$lib/types/cita";
  import SalidaModal from "../common/SalidaModal.svelte";
  import { UserPlus, RotateCw, LogIn, Eye } from "lucide-svelte";
  import type { UserResponse } from "$lib/types/user";

  const currentUser = userStore as unknown as any;

  // Modales
  import ProcesarIngresoModal from "./ProcesarIngresoModal.svelte";
  import VisitaDetallesModal from "./VisitaDetallesModal.svelte";

  let props = $props<{
    onRegisterClick?: () => void;
    onEditClick?: (cita: CitaPopulated) => void;
    onCloseForm?: () => void;
    isFormOpen?: boolean;
  }>();

  // Default value handling logic if needed, or use props.isFormOpen directly (undefined check)
  // Svelte 5 props are required or optional. Defaults in destructuring handled that.
  // With `props` object, we access raw values. `props.isFormOpen` might be undefined.
  // We should default it. `const isFormOpen = $derived(props.isFormOpen ?? false);`
  // Ah, using derived for default is better!

  const isFormOpen = $derived(props.isFormOpen ?? false);

  // Sub-vista activa
  type VisitaView = "pendientes" | "activas" | "historial";
  let activeView = $state<VisitaView>("pendientes");

  // Datos
  let citasPendientes = $state<CitaPopulated[]>([]);
  let ingresosActivos = $state<IngresoVisita[]>([]);
  let ingresosHistorial = $state<IngresoVisita[]>([]);
  let loading = $state(false);
  let gridApi = $state<GridApi | null>(null);

  // Modales state
  let showProcesarModal = $state(false);
  let selectedCitaParaProcesar = $state<CitaPopulated | null>(null);
  let showSalidaModal = $state(false);
  let selectedIngresoParaSalida = $state<IngresoVisita | null>(null);
  let showDetallesModal = $state(false);
  let selectedParaDetalles = $state<any>(null);

  // Load data based on active view
  async function loadData() {
    loading = true;
    try {
      if (activeView === "pendientes") {
        citasPendientes = await citaService.getCitasPendientes();
      } else if (activeView === "activas") {
        ingresosActivos = await ingresoVisitaService.getActivos();
      } else {
        ingresosHistorial = await ingresoVisitaService.getHistorial();
      }
    } catch (e: any) {
      console.error(e);
      toast.error("Error al cargar datos");
    } finally {
      loading = false;
    }
  }

  // Reload when view changes
  $effect(() => {
    activeView;
    loadData();
  });

  // Reload when form closes (after create/edit)
  let prevFormOpen: boolean | undefined = undefined;
  $effect(() => {
    const current = isFormOpen; // Read inside effect for proper tracking
    // Detect form closing (was open, now closed)
    if (prevFormOpen !== undefined && prevFormOpen && !current) {
      loadData();
    }
    prevFormOpen = current;
  });

  // ...

  // Column definitions for each view
  const columnDefsPendientes: ColDef<CitaPopulated>[] = [
    {
      field: "visitante_nombre_completo",
      headerName: "Nombre",
      flex: 1,
      minWidth: 180,
    },
    {
      field: "visitante_cedula",
      headerName: "Cédula",
      width: 120,
    },
    {
      field: "fecha_cita",
      headerName: "Fecha",
      width: 100,
      valueFormatter: (p) => new Date(p.value).toLocaleDateString("es-CR"),
    },
    {
      field: "fecha_cita",
      headerName: "Hora",
      width: 80,
      valueFormatter: (p) =>
        new Date(p.value).toLocaleTimeString("es-CR", {
          hour: "2-digit",
          minute: "2-digit",
        }),
    },
    {
      field: "anfitrion",
      headerName: "Anfitrión",
      width: 140,
    },
    {
      field: "area_visitada",
      headerName: "Área",
      width: 120,
    },
    {
      headerName: "Acciones",
      width: 260,
      cellRenderer: () => {
        return `<div class="flex gap-1">
          <button class="px-2 py-1 bg-green-600 text-white rounded text-xs font-medium hover:bg-green-700 procesar-btn">Ingresar</button>
          <button class="px-2 py-1 bg-blue-600 text-white rounded text-xs font-medium hover:bg-blue-700 editar-btn">Editar</button>
          <button class="px-2 py-1 bg-gray-600 text-white rounded text-xs font-medium hover:bg-gray-700 ver-btn">Ver</button>
        </div>`;
      },
      onCellClicked: (params: any) => {
        const target = params.event?.target as HTMLElement;
        if (target?.classList?.contains("procesar-btn")) {
          selectedCitaParaProcesar = params.data;
          showProcesarModal = true;
        } else if (target?.classList?.contains("editar-btn")) {
          props.onEditClick?.(params.data);
        } else if (target?.classList?.contains("ver-btn")) {
          selectedParaDetalles = params.data;
          showDetallesModal = true;
        }
      },
    },
  ];

  const columnDefsActivas: ColDef<IngresoVisita>[] = [
    {
      field: "visitanteNombre",
      headerName: "Nombre",
      valueGetter: (p) =>
        `${p.data?.visitanteNombre || ""} ${p.data?.visitanteApellido || ""}`,
      flex: 1,
      minWidth: 180,
    },
    {
      field: "visitanteCedula",
      headerName: "Cédula",
      width: 120,
    },
    {
      field: "gafete",
      headerName: "Gafete",
      width: 90,
      cellRenderer: (p: any) =>
        p.value
          ? `<span class="font-mono font-bold text-blue-500">${p.value}</span>`
          : "-",
    },
    {
      field: "fechaIngreso",
      headerName: "Entrada",
      width: 90,
      valueFormatter: (p) =>
        new Date(p.value).toLocaleTimeString("es-CR", {
          hour: "2-digit",
          minute: "2-digit",
        }),
    },
    {
      field: "anfitrion",
      headerName: "Anfitrión",
      width: 140,
    },
    {
      field: "areaVisitada",
      headerName: "Área",
      width: 120,
    },
    {
      headerName: "Acciones",
      width: 130,
      cellRenderer: () =>
        `<button class="px-3 py-1 bg-red-100 text-red-700 rounded text-xs font-medium hover:bg-red-200 salida-btn">Salida</button>`,
      onCellClicked: (params: any) => {
        console.log("Salida cell clicked:", params.event?.target);
        const target = params.event?.target as HTMLElement;
        if (
          target?.classList?.contains("salida-btn") ||
          target?.closest?.(".salida-btn")
        ) {
          console.log("Salida button detected, opening modal");
          selectedIngresoParaSalida = params.data;
          showSalidaModal = true;
        }
      },
    },
  ];

  const columnDefsHistorial: ColDef<IngresoVisita>[] = [
    {
      field: "visitanteNombre",
      headerName: "Nombre",
      valueGetter: (p) =>
        `${p.data?.visitanteNombre || ""} ${p.data?.visitanteApellido || ""}`,
      flex: 1,
      minWidth: 180,
    },
    {
      field: "visitanteCedula",
      headerName: "Cédula",
      width: 120,
    },
    {
      field: "fechaIngreso",
      headerName: "Fecha",
      width: 100,
      valueFormatter: (p) => new Date(p.value).toLocaleDateString("es-CR"),
    },
    {
      field: "fechaIngreso",
      headerName: "Entrada",
      width: 80,
      valueFormatter: (p) =>
        new Date(p.value).toLocaleTimeString("es-CR", {
          hour: "2-digit",
          minute: "2-digit",
        }),
    },
    {
      field: "fechaSalida",
      headerName: "Salida",
      width: 80,
      valueFormatter: (p) =>
        p.value
          ? new Date(p.value).toLocaleTimeString("es-CR", {
              hour: "2-digit",
              minute: "2-digit",
            })
          : "-",
    },
    {
      field: "anfitrion",
      headerName: "Anfitrión",
      width: 140,
    },
    {
      headerName: "",
      width: 80,
      cellRenderer: () =>
        `<button class="px-2 py-1 text-gray-400 hover:text-white text-xs ver-btn">Ver</button>`,
      onCellClicked: (params: any) => {
        if (params.event.target.classList.contains("ver-btn")) {
          selectedParaDetalles = params.data;
          showDetallesModal = true;
        }
      },
    },
  ];

  // Dynamic columns based on view
  const columnDefs = $derived.by(() => {
    if (activeView === "pendientes")
      return columnDefsPendientes as ColDef<any>[];
    if (activeView === "activas") return columnDefsActivas as ColDef<any>[];
    return columnDefsHistorial as ColDef<any>[];
  });

  // Dynamic data based on view
  const rowData = $derived.by((): any[] => {
    if (activeView === "pendientes") return citasPendientes;
    if (activeView === "activas") return ingresosActivos;
    return ingresosHistorial;
  });

  // Handlers
  async function handleProcesarIngreso(event: CustomEvent<{ gafete: string }>) {
    if (!selectedCitaParaProcesar || !$currentUser) return;
    try {
      await citaService.procesarIngresoCita(
        selectedCitaParaProcesar.id,
        event.detail.gafete,
        $currentUser.id,
      );
      toast.success("Ingreso procesado correctamente");
      showProcesarModal = false;
      selectedCitaParaProcesar = null;
      loadData();
    } catch (e: any) {
      toast.error(e.message || "Error al procesar ingreso");
    }
  }

  async function handleConfirmSalida(
    event: CustomEvent<{ devolvioGafete: boolean; observaciones?: string }>,
  ) {
    if (!selectedIngresoParaSalida || !$currentUser) return;
    try {
      await ingresoVisitaService.registrarSalida(
        selectedIngresoParaSalida.id,
        $currentUser.id,
        event.detail.devolvioGafete,
        event.detail.observaciones,
      );
      toast.success("Salida registrada");
      showSalidaModal = false;
      selectedIngresoParaSalida = null;
      loadData();
    } catch (e: any) {
      toast.error(e.message || "Error al registrar salida");
    }
  }

  // Toolbar buttons
  const customButtons = $derived.by(() => {
    const buttons: CustomToolbarButton[] = [
      {
        id: "refresh",
        label: "Refrescar",
        icon: RotateCw,
        variant: "default",
        onClick: loadData,
      },
    ];

    if (activeView === "pendientes" && !isFormOpen) {
      buttons.unshift({
        id: "nueva-visita",
        label: "Nueva Visita",
        icon: UserPlus,
        variant: "primary",
        onClick: () => props.onRegisterClick?.(),
      });
    }

    return { default: buttons };
  });

  // View titles
  const viewTitles = {
    pendientes: {
      title: "Citas Pendientes",
      desc: "Visitas programadas pendientes de ingreso",
    },
    activas: {
      title: "Visitas Activas",
      desc: "Visitantes actualmente en las instalaciones",
    },
    historial: { title: "Historial", desc: "Registro de visitas completadas" },
  };
</script>

<div class="flex h-full flex-col bg-[#0d1117]">
  <!-- Header con sub-tabs -->
  <div class="border-b border-[#30363d] px-4 py-3 bg-[#161b22]">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold text-[#f0f6fc]">
          {viewTitles[activeView].title}
        </h2>
        <p class="text-xs text-[#8d96a0]">{viewTitles[activeView].desc}</p>
      </div>

      <!-- Sub-tabs -->
      <div class="flex gap-1 bg-[#0d1117] p-1 rounded-lg">
        <button
          class="px-3 py-1.5 text-xs font-medium rounded transition-colors {activeView ===
          'pendientes'
            ? 'bg-[#238636] text-white'
            : 'text-[#8d96a0] hover:text-white hover:bg-[#21262d]'}"
          onclick={() => (activeView = "pendientes")}
        >
          Pendientes
        </button>
        <button
          class="px-3 py-1.5 text-xs font-medium rounded transition-colors {activeView ===
          'activas'
            ? 'bg-[#1f6feb] text-white'
            : 'text-[#8d96a0] hover:text-white hover:bg-[#21262d]'}"
          onclick={() => (activeView = "activas")}
        >
          Activas
        </button>
        <button
          class="px-3 py-1.5 text-xs font-medium rounded transition-colors {activeView ===
          'historial'
            ? 'bg-[#6e7681] text-white'
            : 'text-[#8d96a0] hover:text-white hover:bg-[#21262d]'}"
          onclick={() => (activeView = "historial")}
        >
          Historial
        </button>
      </div>
    </div>
  </div>

  <!-- Grid -->
  <div class="flex-1 relative overflow-hidden">
    <AGGridWrapper
      gridId="visitas-activas-grid"
      {columnDefs}
      {rowData}
      {customButtons}
      onGridReady={(api) => (gridApi = api)}
    />
  </div>
</div>

<!-- Modal: Procesar Ingreso -->
{#if showProcesarModal && selectedCitaParaProcesar}
  <ProcesarIngresoModal
    cita={selectedCitaParaProcesar}
    onCancel={() => {
      showProcesarModal = false;
      selectedCitaParaProcesar = null;
    }}
    onConfirm={handleProcesarIngreso}
  />
{/if}

<!-- Modal: Registrar Salida -->
{#if showSalidaModal && selectedIngresoParaSalida}
  {@const modalData = {
    id: selectedIngresoParaSalida.id,
    nombreCompleto: `${selectedIngresoParaSalida.visitanteNombre} ${selectedIngresoParaSalida.visitanteApellido}`,
    cedula: selectedIngresoParaSalida.visitanteCedula,
    gafeteNumero: selectedIngresoParaSalida.gafete,
  }}
  <!-- Overlay backdrop -->
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
  >
    <SalidaModal
      ingreso={modalData as any}
      on:cancel={() => {
        showSalidaModal = false;
        selectedIngresoParaSalida = null;
      }}
      on:confirm={handleConfirmSalida}
    />
  </div>
{/if}

<!-- Modal: Ver Detalles -->
{#if showDetallesModal && selectedParaDetalles}
  <VisitaDetallesModal
    data={selectedParaDetalles}
    onClose={() => {
      showDetallesModal = false;
      selectedParaDetalles = null;
    }}
  />
{/if}
