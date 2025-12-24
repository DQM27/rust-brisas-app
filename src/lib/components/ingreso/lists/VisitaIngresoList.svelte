<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  let rowData: any[] = [];
  let loading = false;

  async function loadData() {
    loading = true;
    try {
      const activeVisitas: any[] = await invoke("get_ingresos_visitas_activos");
      rowData = activeVisitas;
    } catch (e) {
      console.error("Error loading visitas activas", e);
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
      field: "visitante_nombre_completo",
      headerName: "Visitante",
      flex: 1,
      valueGetter: (p: any) =>
        p.data.visitante_nombre + " " + p.data.visitante_apellido,
    },
    { field: "visitante_empresa", headerName: "Empresa", flex: 1 },
    { field: "anfitrion", headerName: "Anfitrión", flex: 1 },
    { field: "area_visitada", headerName: "Área", width: 120 },
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

  /* Action logic can be added here similar to Contratista if needed */
</script>

<div
  class="h-full flex flex-col bg-base-100 rounded-xl shadow-sm border border-base-200 overflow-hidden"
>
  <div
    class="p-4 border-b border-base-200 flex justify-between items-center bg-secondary/5"
  >
    <div class="flex items-center gap-2">
      <h3 class="font-bold text-lg text-secondary">Visitas en Planta</h3>
    </div>
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
    <AGGridWrapper {rowData} {columnDefs} gridId="visita-ingreso-list" />
  </div>
</div>
