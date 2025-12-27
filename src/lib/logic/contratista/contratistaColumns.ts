import type { ContratistaResponse, EstadoContratista } from "$lib/types/contratista";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

export class ContratistaColumns {
  // Column configuration
  static getColumns(
    onStatusToggle?: (id: string, currentStatus: any) => void
  ): ColDef<ContratistaResponse>[] {
    return [
      {
        field: "cedula",
        headerName: "Cédula",
        width: 130,
        pinned: "left",
        cellStyle: { fontFamily: "monospace", fontSize: "13px" },
      },
      {
        field: "nombreCompleto",
        headerName: "Nombre Completo",
        flex: 1,
        minWidth: 200,
        cellStyle: { fontWeight: 500 },
      },
      {
        field: "empresaNombre",
        headerName: "Empresa",
        flex: 1,
        minWidth: 180,
      },
      {
        field: "vehiculoTipo",
        headerName: "Vehículo",
        width: 120,
        valueFormatter: (params) => params.value || "-",
      },
      {
        field: "vehiculoPlaca",
        headerName: "Placa",
        width: 100,
        valueFormatter: (params) => params.value || "-",
        cellStyle: { fontFamily: "monospace" },
      },
      {
        field: "estado",
        headerName: "Estado",
        width: 130,
        cellRenderer: (params: ICellRendererParams) => {
          const estado = params.value as EstadoContratista;
          return ContratistaColumns.formatEstadoBadge(estado);
        },
        onCellClicked: (params) => {
          if (onStatusToggle && params.data && params.event) {
            const target = params.event.target as HTMLElement;
            // Prevent toggle if clicking elsewhere (though cellClicked is usually specific)
            if (target && target.tagName !== "BUTTON") return;

            const row = params.data as ContratistaResponse;
            onStatusToggle(row.id, row.estado);
          }
        }
      },
      {
        field: "praindVencido",
        headerName: "PRAIND",
        width: 130,
        cellRenderer: (params: ICellRendererParams) => {
          const row = params.data as ContratistaResponse;
          return ContratistaColumns.formatPraindBadge(row);
        },
      },
      {
        field: "fechaVencimientoPraind",
        headerName: "Vencimiento",
        width: 130,
        valueFormatter: (params) => {
          if (!params.value) return "";
          const date = new Date(params.value);
          const day = date.getUTCDate();
          const month = date.toLocaleDateString("es-PA", { month: "short", timeZone: "UTC" });
          const year = date.getUTCFullYear();
          return `${day} ${month} ${year}`;
        },
      },
      {
        field: "puedeIngresar",
        headerName: "Acceso",
        width: 130,
        cellRenderer: (params: ICellRendererParams) => {
          const row = params.data as ContratistaResponse;
          return ContratistaColumns.formatAccesoBadge(row);
        }
      },
    ];
  }

  static getTrashColumns(): ColDef<ContratistaResponse>[] {
    return [
      {
        field: "cedula",
        headerName: "Cédula",
        width: 130,
        pinned: "left",
        cellStyle: { fontFamily: "monospace", fontSize: "13px" },
      },
      {
        field: "nombreCompleto",
        headerName: "Nombre Completo",
        flex: 1,
        minWidth: 200,
        cellStyle: { fontWeight: 500 },
      },
      {
        field: "empresaNombre",
        headerName: "Empresa",
        flex: 1,
        minWidth: 180,
      },
      {
        field: "vehiculoPlaca",
        headerName: "Placa",
        width: 100,
        valueFormatter: (params) => params.value || "-",
        cellStyle: { fontFamily: "monospace" },
      },
      // Columns specific to trash can be added here if needed (e.g. deletedAt)
    ];
  }

  // Helper methods
  static formatEstadoBadge(estado: EstadoContratista): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

    const badges = {
      activo: "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800",
      inactivo: "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
      suspendido: "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800",
    };

    const badgeClass = badges[estado] || badges.inactivo;
    const displayText = estado ? estado.charAt(0).toUpperCase() + estado.slice(1) : 'N/A';

    return `
      <button 
        class="${baseClass} ${badgeClass}"
        title="Clic para cambiar estado"
      >
        ${displayText}
      </button>
    `;
  }

  static formatPraindBadge(row: ContratistaResponse): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";
    let badgeClass = "";
    let text = "";

    if (row.praindVencido) {
      badgeClass = "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
      text = "Vencido";
    } else if (row.diasHastaVencimiento <= 30) {
      badgeClass = "bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800";
      text = `${row.diasHastaVencimiento} días`;
    } else {
      badgeClass = "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";
      text = "Vigente";
    }

    return `
      <span class="${baseClass} ${badgeClass}">
        ${text}
      </span>
    `;
  }

  static formatAccesoBadge(row: ContratistaResponse): string {
    const redBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
    const greenBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";

    if (row.estaBloqueado) {
      return `<span class="${redBadge}">Bloqueado</span>`;
    }
    if (row.estado !== 'activo') {
      return `<span class="${redBadge}">Denegado</span>`;
    }
    if (row.puedeIngresar) {
      return `<span class="${greenBadge}">Permitido</span>`;
    } else {
      return `<span class="${redBadge}">Denegado</span>`;
    }
  }
}