<script lang="ts">
  import AGGridWrapper from "$lib/components/grid/AGGridWrapper.svelte";
  import { ListaNegraListLogic } from "$lib/logic/listaNegra/listaNegraListLogic";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";
  import type { CustomToolbarButton } from "$lib/types/agGrid";
  import {
    UserX,
    Clock,
    User,
    Copy,
    Eye,
    Trash2,
    Ban,
    CheckCircle,
  } from "lucide-svelte";
  import type { GridApi } from "@ag-grid-community/core";

  interface Props {
    data: ListaNegraResponse[];
    onUnblock: (bloqueado: ListaNegraResponse) => void;
  }

  let { data, onUnblock }: Props = $props();

  // Definición de columnas desde la lógica
  const columns = ListaNegraListLogic.getColumns();

  // Estado
  let gridApi: GridApi | null = null;

  // Botones personalizados para el toolbar
  const customButtons: {
    default?: CustomToolbarButton[];
    singleSelect?: CustomToolbarButton[];
    multiSelect?: CustomToolbarButton[];
  } = {
    singleSelect: [
      {
        id: "view-details",
        label: "Ver detalles",
        icon: Eye,
        onClick: () => {
          const selected = gridApi?.getSelectedRows()[0] as ListaNegraResponse;
          if (selected) showDetails(selected);
        },
      },
      {
        id: "copy-cedula",
        label: "Copiar cédula",
        icon: Copy,
        onClick: () => {
          const selected = gridApi?.getSelectedRows()[0] as ListaNegraResponse;
          if (selected) {
            navigator.clipboard.writeText(selected.cedula);
          }
        },
      },
      {
        id: "toggle-block",
        label: "Gestionar bloqueo",
        icon: Ban, // Icono inicial, cambiará dinámicamente si es posible o se usa uno genérico
        variant: "danger",
        onClick: () => {
          const selected = gridApi?.getSelectedRows()[0] as ListaNegraResponse;
          if (selected) onUnblock(selected);
        },
      },
    ],
  };

  function showDetails(row: ListaNegraResponse) {
    const observaciones = row.observaciones || "Sin observaciones";
    alert(
      `Detalles de ${row.nombreCompleto}:\n\n` +
        `Cédula: ${row.cedula}\n` +
        `Empresa: ${row.empresaNombre || "N/A"}\n` +
        `Motivo: ${row.motivoBloqueo}\n` +
        `Observaciones: ${observaciones}\n` +
        `Estado: ${row.isActive ? "Bloqueado" : "Desbloqueado"}\n` +
        `Tipo: ${row.esBloqueoPermanente ? "Permanente" : "Temporal"}\n` +
        `Bloqueado por: ${row.bloqueadoPor}\n` +
        `Fecha: ${new Date(row.fechaInicioBloqueo).toLocaleDateString("es-PA")}\n` +
        `Días: ${row.diasTranscurridos}`,
    );
  }
</script>

<div class="h-full w-full">
  <AGGridWrapper
    gridId="lista-negra-list"
    rowData={data}
    columnDefs={columns}
    {customButtons}
    onGridReady={(api) => (gridApi = api)}
    onRowDoubleClicked={(row) => showDetails(row)}
    getRowId={(data) => data.id}
  />
</div>
