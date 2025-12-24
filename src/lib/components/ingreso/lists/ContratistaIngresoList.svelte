<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";

  // State
  let rowData: any[] = [];
  let loading = false;

  async function loadData() {
    loading = true;
    try {
      const allActive: any[] = await invoke("get_ingresos_abiertos");
      rowData = activeFilter(allActive);
    } catch (e) {
      console.error("Error loading contratistas activos", e);
    } finally {
      loading = false;
    }
  }

  function activeFilter(data: any[]): any[] {
    return data.filter(
      (d: any) =>
        d.tipo_ingreso === "Contratista" || d.tipo_ingreso === "contratista",
    );
  }

  onMount(() => {
    loadData();
  });

  // Column Defs
  const columnDefs = [
    {
      field: "gafete",
      headerName: "Gafete",
      width: 100,
      sortable: true,
      filter: true,
    },
    {
      field: "nombre_completo",
      headerName: "Contratista",
      flex: 1,
      valueGetter: (params: any) => {
        return (
          params.data.nombre_completo ||
          params.data.contratista?.nombre +
            " " +
            params.data.contratista?.apellido
        );
      },
    },
    { field: "empresa", headerName: "Empresa", flex: 1 },
    {
      field: "fecha_ingreso",
      headerName: "Hora Entrada",
      width: 180,
      cellRenderer: (params: any) => {
        if (!params.value) return "";
        return new Date(params.value).toLocaleTimeString([], {
          hour: "2-digit",
          minute: "2-digit",
        });
      },
    },
    {
      headerName: "Acciones",
      pinned: "right" as "right",
      width: 120,
      cellRenderer: "actionButtonRenderer",
      cellRendererParams: {
        onClick: (data: any) => handleSalida(data),
        label: "Salida",
        class: "btn-error btn-xs",
      },
    },
  ];

  async function handleSalida(data: any) {
    if (!confirm(`¿Registrar salida para ${data.nombre_completo}?`)) return;

    try {
      // Standard Exit
      await invoke("register_exit_contratista", {
        id: data.id,
        usuarioId: "SYSTEM",
        devolvioGafete: true,
        observaciones: null,
      });
      loadData();
    } catch (e) {
      alert("Error al registrar salida: " + e);
    }
  }
</script>

<div
  class="h-full flex flex-col bg-base-100 rounded-xl shadow-sm border border-base-200 overflow-hidden"
>
  <div class="p-4 border-b border-base-200 flex justify-between items-center">
    <h3 class="font-bold text-lg">Contratistas en Planta</h3>
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
    <AGGridWrapper {rowData} {columnDefs} gridId="contratista-ingreso-list" />
  </div>
</div>
