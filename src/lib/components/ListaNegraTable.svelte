<script lang="ts">
  import DataTable from "$lib/components/common/DataTable.svelte";
  import { UserX, Clock, User, Copy, Eye, Trash2 } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";
  import type {
    DataTableColumn,
    DataTableAction,
    DataTableContextMenuItem,
  } from "$lib/types/dataTable";
  import type { ICellRendererParams } from "@ag-grid-community/core";

  interface Props {
    data: ListaNegraResponse[];
    onUnblock: (bloqueado: ListaNegraResponse) => void;
  }

  let { data, onUnblock }: Props = $props();

  // Definición de columnas
  const columns: DataTableColumn<ListaNegraResponse>[] = [
    {
      field: "cedula",
      headerName: "Cédula",
      width: 130,
      cellStyle: { fontFamily: "monospace", color: "#d4d4d4" },
    },
    {
      field: "nombreCompleto",
      headerName: "Nombre Completo",
      width: 220,
      cellStyle: { fontWeight: 500, color: "#ffffff" },
    },
    {
      field: "empresaNombre",
      headerName: "Empresa",
      width: 180,
      cellStyle: { color: "#a0a0a0" },
    },
    {
      field: "motivoBloqueo",
      headerName: "Motivo",
      width: 300,
      minWidth: 200,
      maxWidth: 500,
      cellStyle: { color: "#d4d4d4" },
      tooltipField: "motivoBloqueo",
      wrapText: true,
      autoHeight: true,
    },
    {
      field: "isActive",
      headerName: "Estado",
      width: 130,
      autoHeight: true,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        const isActive = params.value;
        const bgColor = isActive
          ? "rgba(239, 68, 68, 0.1)"
          : "rgba(107, 114, 128, 0.1)";
        const textColor = isActive ? "#fca5a5" : "#9ca3af";
        const borderColor = isActive
          ? "rgba(239, 68, 68, 0.2)"
          : "rgba(107, 114, 128, 0.2)";
        const icon = isActive ? "🚫" : "⏰";
        const text = isActive ? "Bloqueado" : "Desbloqueado";

        return `
          <div style="
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 2px 8px;
            border-radius: 9999px;
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            font-size: 10px;
            font-weight: 500;
            white-space: nowrap;
          ">
            <span style="font-size: 9px;">${icon}</span>
            ${text}
          </div>
        `;
      },
    },
    {
      field: "esBloqueoPermanente",
      headerName: "Tipo",
      width: 120,
      autoHeight: true,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        const isPermanente = params.value;
        const bgColor = isPermanente
          ? "rgba(168, 85, 247, 0.1)"
          : "rgba(234, 179, 8, 0.1)";
        const textColor = isPermanente ? "#c084fc" : "#fde047";
        const borderColor = isPermanente
          ? "rgba(168, 85, 247, 0.2)"
          : "rgba(234, 179, 8, 0.2)";
        const text = isPermanente ? "Permanente" : "Temporal";

        return `
          <div style="
            display: inline-flex;
            align-items: center;
            gap: 4px;
            padding: 2px 8px;
            border-radius: 9999px;
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            font-size: 10px;
            font-weight: 500;
            white-space: nowrap;
          ">
            ${text}
          </div>
        `;
      },
    },
    {
      field: "bloqueadoPor",
      headerName: "Bloqueado por",
      width: 160,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        return `
          <div style="display: flex; align-items: center; gap: 6px; color: #a0a0a0;">
            <span style="font-size: 12px;">👤</span>
            ${params.value}
          </div>
        `;
      },
    },
    {
      field: "fechaInicioBloqueo",
      headerName: "Desde",
      width: 120,
      valueFormatter: (params) => {
        if (!params.value) return "";
        const date = new Date(params.value);
        return date.toLocaleDateString("es-PA", {
          year: "numeric",
          month: "short",
          day: "numeric",
        });
      },
      cellStyle: { color: "#a0a0a0", fontSize: "11px" },
    },
    {
      field: "diasTranscurridos",
      headerName: "Días",
      width: 90,
      cellRenderer: (params: ICellRendererParams<ListaNegraResponse>) => {
        return `
          <div style="color: #6b7280; font-size: 11px;">
            ${params.value} días
          </div>
        `;
      },
    },
  ];

  // Acciones
  const actions: DataTableAction<ListaNegraResponse>[] = [
    {
      id: "unblock",
      label: "Desbloquear",
      variant: "success",
      show: (row) => row.isActive,
      onClick: (row) => onUnblock(row),
    },
  ];

  // Context menu
  const contextMenuItems: DataTableContextMenuItem<ListaNegraResponse>[] = [
    {
      id: "copy",
      label: "Copiar cédula",
      icon: Copy,
      onClick: (row) => {
        navigator.clipboard.writeText(row.cedula);
      },
    },
    {
      id: "view",
      label: "Ver detalles",
      icon: Eye,
      onClick: (row) => {
        alert(
          `Detalles de ${row.nombreCompleto}:\n\n` +
            `Cédula: ${row.cedula}\n` +
            `Empresa: ${row.empresaNombre || "N/A"}\n` +
            `Motivo: ${row.motivoBloqueo}\n` +
            `Estado: ${row.isActive ? "Bloqueado" : "Desbloqueado"}\n` +
            `Tipo: ${row.esBloqueoPermanente ? "Permanente" : "Temporal"}\n` +
            `Bloqueado por: ${row.bloqueadoPor}\n` +
            `Fecha: ${new Date(row.fechaInicioBloqueo).toLocaleDateString("es-PA")}\n` +
            `Días: ${row.diasTranscurridos}`,
        );
      },
      dividerAfter: true,
    },
    {
      id: "unblock",
      label: "Desbloquear",
      icon: Trash2,
      variant: "danger",
      show: (row) => row.isActive,
      onClick: (row) => onUnblock(row),
    },
  ];
</script>

<DataTable
  {data}
  {columns}
  {actions}
  {contextMenuItems}
  storageKey="lista-negra-table"
  rowSelection="multiple"
  getRowId={(row) => row.id}
  exportConfig={{
    fileName: `lista-negra-${new Date().toISOString().split("T")[0]}.csv`,
  }}
/>
