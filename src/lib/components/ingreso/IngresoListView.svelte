<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import { AlertCircle, RotateCw } from "lucide-svelte";
  import type { ColDef } from "@ag-grid-community/core";

  // Components
  import SearchBar from "$lib/components/shared/SearchBar.svelte";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import IngresoFormModal from "./IngresoFormModal.svelte";

  // Logic
  import { invoke } from "@tauri-apps/api/core";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import { currentUser } from "$lib/stores/auth";
  import { keyboardCommand, clearCommand } from "$lib/stores/keyboardCommands";
  import { activeTabId } from "$lib/stores/tabs";

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

  // Keyboard shortcut listener for Ctrl+N
  // Track last processed timestamp to avoid reacting to stale commands
  let lastProcessedTimestamp = 0;

  $effect(() => {
    const cmd = $keyboardCommand;
    // Only process if: command exists, is new, and this tab is active
    if (
      cmd?.command === "create-new" &&
      cmd.timestamp > lastProcessedTimestamp &&
      $activeTabId === tabId
    ) {
      lastProcessedTimestamp = cmd.timestamp;
      showModal = true;
      clearCommand();
    }
  });

  // ==========================================
  // COLUMNS
  // ==========================================
  let columnDefs = $derived.by((): ColDef<any>[] => {
    return [
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
          if (!params.value) return "";
          return new Date(params.value).toLocaleDateString("es-ES", {
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
          if (!params.value) return "";
          return new Date(params.value).toLocaleTimeString("es-ES", {
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
          if (!params.value) return "-";
          return new Date(params.value).toLocaleDateString("es-ES", {
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
          if (!params.value) return "-";
          return new Date(params.value).toLocaleTimeString("es-ES", {
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
          const entrada = new Date(params.data.fechaHoraIngreso);
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
  });

  // ==========================================
  // TOOLBAR BUTTONS
  // ==========================================
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    return {
      default: [
        createCustomButton.nuevo(() => handleNuevoIngreso()),
        {
          id: "reload-data",
          label: "Actualizar",
          icon: RotateCw,
          onClick: loadIngresos,
          variant: "default" as const,
          tooltip: "Recargar lista",
        },
      ],
      singleSelect: [],
      multiSelect: [],
    };
  });

  // ==========================================
  // HANDLERS
  // ==========================================
  async function loadIngresos() {
    loading = true;
    error = "";
    try {
      const data = await invoke("get_ingresos_abiertos");
      ingresos = data as any[];
    } catch (err: any) {
      error = err.message || "Error al cargar ingresos";
      toast.error(error);
    } finally {
      loading = false;
    }
  }

  function handleNuevoIngreso() {
    showModal = true;
  }

  function handleModalComplete() {
    showModal = false;
    loadIngresos();
  }

  async function handleSalida(ingreso: any) {
    if (!confirm(`¿Registrar salida para ${ingreso.nombreCompleto}?`)) return;

    try {
      const command =
        ingreso.tipoIngreso === "contratista"
          ? "register_exit_contratista"
          : ingreso.tipoIngreso === "proveedor"
            ? "registrar_salida_proveedor"
            : "registrar_salida_visita";

      await invoke(command, {
        id: ingreso.id,
        usuarioId: $currentUser?.id || "00000000-0000-0000-0000-000000000000",
        devolvioGafete: true,
        observaciones: null,
      });

      toast.success("Salida registrada exitosamente");
      loadIngresos();
    } catch (err: any) {
      toast.error("Error al registrar salida: " + err.message);
    }
  }

  // ==========================================
  // LIFECYCLE
  // ==========================================
  onMount(() => {
    loadIngresos();
  });
</script>

<div class="flex h-full flex-col relative bg-surface-1">
  <!-- Header -->
  <div class="border-b border-surface px-6 py-4 bg-surface-2">
    <div class="flex items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-semibold text-primary">Ingresos Activos</h2>
        <p class="mt-1 text-sm text-secondary">
          Personas actualmente en planta
        </p>
      </div>
      <div class="flex-1 max-w-md">
        <SearchBar placeholder="Buscar por nombre, gafete..." limit={10} />
      </div>
    </div>
  </div>

  <!-- Content -->
  <div class="flex-1 overflow-hidden relative bg-surface-1">
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
    {:else if ingresos.length === 0}
      <div class="flex h-full items-center justify-center">
        <div class="text-center">
          <AlertCircle size={48} class="mx-auto text-secondary" />
          <p class="mt-4 text-lg font-medium text-primary">
            No hay ingresos activos
          </p>
          <p class="mt-2 text-sm text-secondary">
            Registra un nuevo ingreso para comenzar
          </p>
          <button
            onclick={handleNuevoIngreso}
            class="mt-4 px-4 py-2 bg-accent text-white rounded-md hover:opacity-90 transition-opacity"
          >
            Nuevo Ingreso
          </button>
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
      />
    {/if}
  </div>
</div>

<!-- Modal -->
<IngresoFormModal bind:show={showModal} on:complete={handleModalComplete} />
