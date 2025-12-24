<script lang="ts">
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import VisitaFormModal from "$lib/components/visita/VisitaFormModal.svelte";
  import { visitaService } from "$lib/logic/visita/visitaService";
  import { VISITA_COLUMNS } from "$lib/logic/visita/visitaColumns";
  import type {
    CitaPopulated,
    CreateCitaInput,
    CreateVisitanteInput,
  } from "$lib/types/cita";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";
  import { currentUser } from "$lib/stores/auth";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import { activeTabId } from "$lib/stores/tabs";

  interface Props {
    tabId?: string;
  }
  let { tabId = "citas-view" }: Props = $props();

  // Estado
  let visitas = $state<CitaPopulated[]>([]);
  let loading = $state(false);

  // Filtros
  let activeFilter = $state<"hoy" | "pendientes">("hoy");

  // Modal
  let showModal = $state(false);
  let modalLoading = $state(false);

  // Selección
  let selectedRows = $state<CitaPopulated[]>([]);

  // Keyboard shortcut handler for Ctrl+N
  function handleKeydown(e: KeyboardEvent) {
    if ($activeTabId !== tabId) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "n") {
      const target = e.target as HTMLElement;
      if (target.tagName === "TEXTAREA" || target.isContentEditable) return;
      e.preventDefault();
      showModal = true;
    }
  }

  // Carga de datos
  async function loadData() {
    loading = true;
    try {
      if (activeFilter === "hoy") {
        visitas = await visitaService.getCitasHoy();
      } else {
        visitas = await visitaService.getCitasPendientes();
      }
    } catch (e: any) {
      console.error(e);
      toast.error("Error cargando visitas");
    } finally {
      loading = false;
    }
  }

  // Guardar (Crear)
  async function handleSave(data: {
    cita: CreateCitaInput;
    visitante: CreateVisitanteInput;
  }) {
    modalLoading = true;
    try {
      await visitaService.createCita(data.cita, data.visitante);
      toast.success("Visita agendada correctamente");
      loadData();
      return true;
    } catch (e: any) {
      console.error(e);
      toast.error(e.message || "Error al crear visita");
      return false;
    } finally {
      modalLoading = false;
    }
  }

  // Acción: Registrar Ingreso (Desde Grilla)
  async function handleRegistrarIngreso(visita: CitaPopulated) {
    // TODO: Esto idealmente abriría un modal de confirmación con input para Gafete
    // Por ahora simularemos un prompt simple o un modal pequeño dedicado
    // O podríamos reusar un modal de confirmación genérico si existiera.
    // Dado que el usuario pidió refactorizar "como proveedor", proveedor es CRUD simple.
    // Pero Visita tiene acción de ingreso.

    const gafete = prompt(
      `Ingrese número de gafete para ${visita.visitante_nombre_completo}:`,
    );
    if (!gafete) return;

    if (!$currentUser) return;

    const toastId = toast.loading("Registrando ingreso...");
    try {
      await visitaService.procesarIngresoCita(
        visita.id,
        gafete,
        $currentUser.id,
      );
      toast.success("Ingreso registrado", { id: toastId });
      loadData();
    } catch (e: any) {
      toast.error(e.message || "Error al registrar ingreso", { id: toastId });
    }
  }

  // Columnas: Agregar acciones custom si necesario
  const columnDefs = $derived([
    ...VISITA_COLUMNS,
    {
      headerName: "Acciones",
      width: 120,
      cellRenderer: (params: any) => {
        // Esto es solo visual si usamos Vanilla JS en cellRenderer, pero AG Grid Svelte es limitado.
        // Mejor usamos botones de toolbar para acciones sobre seleccionados.
        return "";
      },
      hide: true, // Oculto por ahora, usaremos toolbar
    },
  ] as ColDef<CitaPopulated>[]);

  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    return {
      default: [
        createCustomButton.nuevo(() => (showModal = true)),
        {
          id: "refresh",
          label: "Actualizar",
          category: "data",
          onClick: loadData,
          tooltip: "Recargar lista",
        },
        // Filtro Hoy / Pendientes
        {
          id: "filter-date",
          label: activeFilter === "hoy" ? "Ver Pendientes" : "Ver Hoy",
          category: "ui",
          onClick: () => {
            activeFilter = activeFilter === "hoy" ? "pendientes" : "hoy";
            loadData();
          },
          tooltip: "Alternar entre visitas de hoy y todas las pendientes",
        },
      ],
      singleSelect: [
        {
          id: "check-in",
          label: "Registrar Ingreso",
          category: "action",
          onClick: () => selected && handleRegistrarIngreso(selected),
          disabled: false, // Podríamos validar si ya ingresó, pero GetCitas trae pendientes usualmente
        },
        {
          id: "cancel",
          label: "Cancelar Cita",
          category: "destructive",
          onClick: async () => {
            if (!selected) return;
            if (
              !confirm(
                `¿Cancelar visita de ${selected.visitante_nombre_completo}?`,
              )
            )
              return;

            try {
              await visitaService.cancelarCita(selected.id);
              toast.success("Cita cancelada");
              loadData();
            } catch (e) {
              toast.error("Error al cancelar");
            }
          },
        },
      ],
      multiSelect: [],
    };
  });

  $effect(() => {
    loadData();
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="h-full flex flex-col space-y-4 p-4 animate-fade-in bg-[#1e1e1e]">
  {#if loading && visitas.length === 0}
    <!-- Solo spinner si vacío inicial -->
    <div class="flex h-full items-center justify-center">
      <div class="text-center">
        <div class="animate-spin text-blue-500 text-4xl mb-4">⌛</div>
        <p class="text-gray-400">Cargando visitas...</p>
      </div>
    </div>
  {:else}
    <AGGridWrapper
      gridId="visitas-list"
      rowData={visitas}
      {columnDefs}
      {customButtons}
      onSelectionChanged={(rows) => (selectedRows = rows)}
      getRowId={(params) => params.data.id}
    />
  {/if}
</div>

<VisitaFormModal
  show={showModal}
  loading={modalLoading}
  onSave={handleSave}
  onClose={() => (showModal = false)}
/>
