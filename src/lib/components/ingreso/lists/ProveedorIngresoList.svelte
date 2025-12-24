<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  let rowData: any[] = [];
  let loading = false;

  async function loadData() {
    loading = true;
    try {
      const active: any[] = await invoke("get_ingresos_proveedores_activos");
      rowData = active;
    } catch (e) {
      console.error("Error loading proveedores activos", e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadData();
  });

  const columnDefs = [
    { field: "gafete", headerName: "Gafete", width: 90 },
    {
      field: "proveedor_nombre",
      headerName: "Proveedor",
      flex: 1,
      valueGetter: (p: any) =>
        p.data.proveedor_nombre + " " + (p.data.proveedor_apellido || ""),
    },
    { field: "proveedor_empresa", headerName: "Empresa", flex: 1 },
    { field: "guia_remision", headerName: "Guía", width: 120 },
    {
      field: "fecha_ingreso",
      headerName: "Entrada",
      width: 100,
      valueFormatter: (p: any) =>
        p.value
          ? new Date(p.value).toLocaleTimeString([], {
              hour: "2-digit",
              minute: "2-digit",
            })
          : "",
    },
  ];
</script>

<div
  class="h-full flex flex-col bg-base-100 rounded-xl shadow-sm border border-base-200 overflow-hidden"
>
  <div
    class="p-4 border-b border-base-200 flex justify-between items-center bg-accent/5"
  >
    <h3 class="font-bold text-lg text-accent">Proveedores en Planta</h3>
    <button
      class="btn btn-ghost btn-sm btn-circle"
      on:click={loadData}
      aria-label="Refrescar lista"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-5 w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        ><path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
        /></svg
      >
    </button>
  </div>

  <div class="flex-1">
    <AGGridWrapper {rowData} {columnDefs} gridId="proveedor-ingreso-list" />
  </div>
</div>
