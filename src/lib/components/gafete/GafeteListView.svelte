<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { toast } from "svelte-5-french-toast";
  import type { ColDef } from "@ag-grid-community/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  import * as gafeteService from "$lib/logic/gafete/gafeteService";
  import type { GafeteResponse } from "$lib/types/gafete";
  import GafeteForm from "./GafeteForm.svelte";

  // Estado
  let gafetes: GafeteResponse[] = [];
  let loading = false;
  let showModal = false;
  let selectedGafete: GafeteResponse | null = null;
  let formLoading = false;

  // Configuración de columnas AG Grid
  const columnDefs: ColDef<GafeteResponse>[] = [
    {
      field: "numero",
      headerName: "Número",
      sortable: true,
      filter: true,
      cellStyle: { fontWeight: "bold" },
    },
    {
      field: "tipoDisplay",
      headerName: "Tipo",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        const tipo = params.data.tipo;
        let colorClass = "bg-gray-100 text-gray-800";

        switch (tipo) {
          case "contratista":
            colorClass =
              "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200";
            break;
          case "proveedor":
            colorClass =
              "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200";
            break;
          case "visita":
            colorClass =
              "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200";
            break;
          default:
            colorClass =
              "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300";
        }

        return `<span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${colorClass}">${params.value}</span>`;
      },
    },
    {
      field: "estaDisponible",
      headerName: "Estado",
      sortable: true,
      filter: true,
      cellRenderer: (params: any) => {
        return params.value
          ? `<span class="text-green-600 font-medium">Disponible</span>`
          : `<span class="text-red-500 font-medium">En Uso</span>`;
      },
    },
    {
      headerName: "Acciones",
      width: 120,
      cellRenderer: (params: any) => {
        return `
          <button class="text-blue-600 hover:text-blue-900 mr-3 edit-btn">Editar</button>
          <button class="text-red-600 hover:text-red-900 delete-btn">Eliminar</button>
        `;
      },
      onCellClicked: (params: any) => {
        const event = params.event;
        if (event.target.classList.contains("edit-btn")) {
          handleEdit(params.data);
        } else if (event.target.classList.contains("delete-btn")) {
          handleDelete(params.data);
        }
      },
    },
  ];

  // Cargar datos
  async function loadGafetes() {
    loading = true;
    const result = await gafeteService.fetchAll();
    if (result.ok) {
      gafetes = result.data.gafetes;
    } else {
      toast.error(result.error);
    }
    loading = false;
  }

  // Manejadores
  function handleNew() {
    selectedGafete = null;
    showModal = true;
  }

  function handleEdit(gafete: GafeteResponse) {
    selectedGafete = gafete;
    showModal = true;
  }

  async function handleDelete(gafete: GafeteResponse) {
    if (!confirm(`¿Estás seguro de eliminar el gafete ${gafete.numero}?`))
      return;

    const result = await gafeteService.remove(gafete.numero);
    if (result.ok) {
      toast.success("Gafete eliminado");
      loadGafetes();
    } else {
      toast.error(result.error);
    }
  }

  async function handleFormSubmit(event: CustomEvent) {
    formLoading = true;
    const data = event.detail;
    let result;

    if (selectedGafete) {
      result = await gafeteService.update(selectedGafete.numero, data);
    } else {
      result = await gafeteService.create(data);
    }

    if (result.ok) {
      toast.success(selectedGafete ? "Gafete actualizado" : "Gafete creado");
      showModal = false;
      loadGafetes();
    } else {
      toast.error(result.error);
    }
    formLoading = false;
  }

  onMount(() => {
    loadGafetes();
  });
</script>

<div class="h-full flex flex-col space-y-4 p-4">
  <!-- Header -->
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
      Gestión de Gafetes
    </h1>
    <button
      on:click={handleNew}
      class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors"
    >
      + Nuevo Gafete
    </button>
  </div>

  <!-- Tabla -->
  <div
    class="flex-1 bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden"
  >
    <AGGridWrapper
      gridId="badges-list"
      rowData={gafetes}
      {columnDefs}
      getRowId={(params) => params.data.numero}
    />
  </div>
</div>

<!-- Modal -->
{#if showModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade
  >
    <GafeteForm
      initialData={selectedGafete}
      loading={formLoading}
      on:submit={handleFormSubmit}
      on:cancel={() => (showModal = false)}
    />
  </div>
{/if}
