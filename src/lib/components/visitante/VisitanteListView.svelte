<!-- src/lib/components/visitante/VisitanteListView.svelte -->
<script lang="ts">
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  // Assuming we might have a form modal for Visitante, or we reuse one.
  // For now I will assume we might need to create one or use a placeholder.
  // I will comment out the modal import for now if it doesn't exist, but I should probably create it too or use a generic one.
  // User asked for soft delete, not necessarily full CRUD UI creation if it didn't exist, but "Visitante" usually implies CRUD.
  // I'll assume standard CRUD pattern.
  // import VisitanteFormModal from "$lib/components/visitante/VisitanteFormModal.svelte";

  import {
    searchVisitantes,
    deleteVisitante,
    restoreVisitante,
    getArchivedVisitantes,
  } from "$lib/logic/visitante/visitanteService";
  import { VISITANTE_COLUMNS } from "$lib/logic/visitante/visitanteColumns";
  import { createCustomButton } from "$lib/config/agGridConfigs";
  import type { VisitanteResponse } from "$lib/types/visitante";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";
  import { activeTabId } from "$lib/stores/tabs";

  interface Props {
    tabId?: string;
  }
  let { tabId = "visitante-list" }: Props = $props();

  // Estado del Grid
  let visitantes = $state<VisitanteResponse[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  // Filtros
  let activeFilter = $state<"todos" | "papelera">("todos");

  // Selección
  let selectedRows = $state<VisitanteResponse[]>([]);

  // TODO: Modal state if we implement Create/Edit
  // let showModal = $state(false);
  // let selectedVisitante = $state<VisitanteResponse | null>(null);

  // Carga inicial
  const loadData = async () => {
    loading = true;
    error = null;

    let res;
    if (activeFilter === "papelera") {
      res = await getArchivedVisitantes();
    } else {
      // Default search all (empty query)
      res = await searchVisitantes("");
    }

    if (res.ok) {
      visitantes = res.data;
    } else {
      error = res.error;
      toast.error(res.error);
    }
    loading = false;
  };

  // Eliminar
  async function confirmDelete(visitante: VisitanteResponse) {
    if (
      !confirm(
        `¿Estás seguro de eliminar al visitante "${visitante.nombre} ${visitante.apellido}"?`,
      )
    )
      return;

    const res = await deleteVisitante(visitante.id);
    if (res.ok) {
      toast.success("Visitante eliminado");
      loadData();
    } else {
      toast.error(res.error);
    }
  }

  // Restaurar
  async function confirmRestore(visitante: VisitanteResponse) {
    if (
      !confirm(
        `¿Estás seguro de restaurar al visitante "${visitante.nombre} ${visitante.apellido}"?`,
      )
    )
      return;

    const res = await restoreVisitante(visitante.id);
    if (res.ok) {
      toast.success("Visitante restaurado");
      loadData();
    } else {
      toast.error(res.error);
    }
  }

  // Botones Custom
  const customButtons = $derived.by(() => {
    const selected = selectedRows[0];

    if (activeFilter === "papelera") {
      return {
        default: [
          {
            id: "filter-active",
            label: "Ver Activos",
            category: "ui",
            onClick: () => {
              activeFilter = "todos";
              loadData();
            },
            tooltip: "Volver a la lista de activos",
          },
          {
            id: "refresh",
            label: "Actualizar",
            category: "data",
            onClick: loadData,
            tooltip: "Recargar lista",
          },
        ],
        singleSelect: [
          {
            id: "restore",
            label: "Restaurar",
            category: "action",
            onClick: () => {
              if (selected) confirmRestore(selected);
            },
            tooltip: "Restaurar visitante seleccionado",
          },
        ],
        multiSelect: [],
      };
    }

    return {
      default: [
        // {
        //   id: "nuevo",
        //   label: "Nuevo",
        //   ... createCustomButton.nuevo(() => openFormModal(null)) ...
        // },
        {
          id: "refresh",
          label: "Actualizar",
          category: "data",
          onClick: loadData,
          tooltip: "Recargar lista",
        },
        {
          id: "filter-trash",
          label: "Papelera",
          category: "ui",
          onClick: () => {
            activeFilter = "papelera";
            loadData();
          },
          tooltip: "Ver visitantes eliminados",
        },
      ],
      singleSelect: [
        // Edit button would go here
        createCustomButton.eliminar(() => {
          if (selected) confirmDelete(selected);
        }),
      ],
      multiSelect: [],
    };
  });

  // Definición de columnas
  const columnDefs = $derived([
    ...VISITANTE_COLUMNS,
  ] as ColDef<VisitanteResponse>[]);

  $effect(() => {
    loadData();
  });
</script>

<div class="h-full flex flex-col space-y-4 p-4 animate-fade-in bg-[#1e1e1e]">
  {#if loading}
    <div class="flex h-full items-center justify-center">
      <div class="text-center">
        <div class="animate-spin text-blue-500 text-4xl mb-4">⌛</div>
        <p class="text-gray-400">Cargando visitantes...</p>
      </div>
    </div>
  {:else}
    <AGGridWrapper
      gridId="visitante-list"
      rowData={visitantes}
      {columnDefs}
      {customButtons}
      onSelectionChanged={(rows) => {
        selectedRows = rows;
      }}
      getRowId={(params) => params.data.id}
    />
  {/if}
</div>

<!-- Modal Placeholder -->
<!-- <VisitanteFormModal ... /> -->
