import type { ContratistaResponse, EstadoContratista } from "$lib/types/contratista";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

export class ContratistaColumns {
  // Column configuration
  static getColumns(
    onStatusToggle?: (id: string, currentStatus: any) => void
  ): ColDef<ContratistaResponse>[] {
    return [
      {
        colId: "cedula",
        field: "cedula",
        headerName: "Cédula",
        width: 130,
        pinned: "left",
        cellStyle: { fontFamily: "monospace", fontSize: "13px" },
      },
      {
        colId: "nombreCompleto",
        field: "nombreCompleto",
        headerName: "Nombre Completo",
        flex: 1,
        minWidth: 200,
        cellStyle: { fontWeight: 500 },
      },
      {
        colId: "empresaNombre",
        field: "empresaNombre",
        headerName: "Empresa",
        flex: 1,
        minWidth: 180,
      },
      {
        colId: "vehiculoTipo",
        field: "vehiculoTipo",
        headerName: "Vehículo",
        width: 120,
        valueFormatter: (params) => params.value || "-",
      },
      {
        colId: "vehiculoPlaca",
        field: "vehiculoPlaca",
        headerName: "Placa",
        width: 100,
        valueFormatter: (params) => params.value || "-",
        cellStyle: { fontFamily: "monospace" },
      },
      {
        colId: "estado",
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

            // Prevenir propagación para evitar disparos dobles
            params.event.stopPropagation();

            const row = params.data as ContratistaResponse;
            onStatusToggle(row.id, row.estado);
          }
        }
      },
      {
        colId: "praindVencido",
        field: "praindVencido",
        headerName: "PRAIND",
        width: 130,
        cellRenderer: (params: ICellRendererParams) => {
          const row = params.data as ContratistaResponse;
          return ContratistaColumns.formatPraindBadge(row);
        },
      },
      {
        colId: "fechaVencimientoPraind",
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
        colId: "puedeIngresar",
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
        headerName: "Nombre",
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
        colId: "deletedAt",
        field: "deletedAt",
        headerName: "Fecha Eliminación",
        width: 150,
        valueFormatter: (params) => {
          if (!params.value) return "-";
          return new Date(params.value).toLocaleDateString("es-PA", {
            year: 'numeric', month: '2-digit', day: '2-digit',
            hour: '2-digit', minute: '2-digit'
          });
        },
      },
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