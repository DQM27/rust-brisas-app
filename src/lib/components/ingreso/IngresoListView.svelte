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

  // Types
  import type { CustomToolbarButton } from "$lib/types/agGrid";

  // ==========================================
  // STATE
  // ==========================================
  let ingresos = $state<any[]>([]);
  let loading = $state(false);
  let error = $state("");
  let selectedRows = $state<any[]>([]);
  let showModal = $state(false);

  // ==========================================
  // COLUMNS
  // ==========================================
  let columnDefs = $derived.by((): ColDef<any>[] => {
    return [
      {
        field: "gafete",
        headerName: "Gafete",
        width: 100,
        sortable: true,
        filter: true,
      },
      {
        field: "nombre_completo",
        headerName: "Persona",
        flex: 1,
        sortable: true,
        filter: true,
      },
      {
        field: "tipo_ingreso",
        headerName: "Tipo",
        width: 120,
        sortable: true,
        filter: true,
      },
      {
        field: "empresa",
        headerName: "Empresa",
        flex: 1,
        sortable: true,
        filter: true,
      },
      {
        field: "fecha_ingreso",
        headerName: "Hora Ingreso",
        width: 150,
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
        field: "modo_ingreso",
        headerName: "Modo",
        width: 120,
        sortable: true,
        filter: true,
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
          id: "refresh",
          label: "Actualizar",
          icon: RotateCw,
          onClick: loadIngresos,
          variant: "default" as const,
          tooltip: "Recargar lista",
        },
      ],
      singleSelect: [
        {
          id: "salida",
          label: "Registrar Salida",
          onClick: () => {
            if (selected) handleSalida(selected);
          },
          variant: "destructive" as const,
          tooltip: "Registrar salida",
        },
      ],
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
    if (!confirm(`¿Registrar salida para ${ingreso.nombre_completo}?`)) return;

    try {
      const command =
        ingreso.tipo_ingreso === "Contratista"
          ? "register_exit_contratista"
          : ingreso.tipo_ingreso === "Proveedor"
            ? "registrar_salida_proveedor"
            : "registrar_salida_visita";

      await invoke(command, {
        id: ingreso.id,
        usuarioId: "SYSTEM",
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
