// lib/logic/listaNegra/listaNegraListLogic.ts
import { get } from 'svelte/store';
import { selectedSearchStore } from "$lib/stores/searchStore";
import type { ListaNegraResponse } from "$lib/types/listaNegra";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

export interface ListaNegraListState {
  estadoFilter: "todos" | "activo" | "inactivo";
  tipoFilter: "todos" | "permanente" | "temporal";
}

export class ListaNegraListLogic {
  private state: ListaNegraListState;

  constructor() {
    this.state = {
      estadoFilter: "todos",
      tipoFilter: "todos"
    };
  }

  getState(): ListaNegraListState {
    return this.state;
  }

  getFilteredData(bloqueados: ListaNegraResponse[]): ListaNegraResponse[] {
    let filtered = bloqueados;

    // Filtro por búsqueda seleccionada (tiene prioridad)
    const selectedSearch = get(selectedSearchStore);
    if (selectedSearch.result) {
      // Filtrar por cédula del resultado seleccionado
      const cedula = selectedSearch.result.cedula || selectedSearch.result.id;
      filtered = filtered.filter((b) => b.cedula === cedula);
      return filtered; // Retornar temprano, ignorando otros filtros
    }

    // Filtro de estado
    if (this.state.estadoFilter === "activo") {
      filtered = filtered.filter((b) => b.isActive);
    } else if (this.state.estadoFilter === "inactivo") {
      filtered = filtered.filter((b) => !b.isActive);
    }

    // Filtro de tipo
    if (this.state.tipoFilter === "permanente") {
      filtered = filtered.filter((b) => b.esBloqueoPermanente);
    } else if (this.state.tipoFilter === "temporal") {
      filtered = filtered.filter((b) => !b.esBloqueoPermanente);
    }

    return filtered;
  }

  getStats(bloqueados: ListaNegraResponse[]) {
    return {
      total: bloqueados.length,
      activos: bloqueados.filter((b) => b.isActive).length,
      permanentes: bloqueados.filter((b) => b.esBloqueoPermanente).length,
      temporales: bloqueados.filter((b) => !b.esBloqueoPermanente).length,
    };
  }

  // Actions
  setEstadoFilter(filter: ListaNegraListState['estadoFilter']): void {
    this.state.estadoFilter = filter;
  }

  setTipoFilter(filter: ListaNegraListState['tipoFilter']): void {
    this.state.tipoFilter = filter;
  }

  clearAllFilters(): void {
    this.state.estadoFilter = "todos";
    this.state.tipoFilter = "todos";
    selectedSearchStore.clear();
  }

  // Column configuration
  static getColumns(): ColDef<ListaNegraResponse>[] {
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
        cellRenderer: (params: ICellRendererParams) => {
          const empresa = params.value || "Sin empresa";
          return `<span class="text-sm text-gray-400">${empresa}</span>`;
        },
      },
      {
        field: "isActive",
        headerName: "Estado",
        width: 130,
        cellRenderer: (params: ICellRendererParams) => {
          return ListaNegraListLogic.formatEstadoBadge(params.value);
        },
      },
      {
        field: "esBloqueoPermanente",
        headerName: "Tipo",
        width: 140,
        cellRenderer: (params: ICellRendererParams) => {
          return ListaNegraListLogic.formatTipoBadge(params.value);
        },
      },
      {
        field: "motivoBloqueo",
        headerName: "Motivo de Bloqueo",
        flex: 1,
        minWidth: 250,
        cellRenderer: (params: ICellRendererParams) => {
          const motivo = params.value || "Sin motivo especificado";
          return `<span class="text-xs text-gray-300 line-clamp-2">${motivo}</span>`;
        },
      },
      {
        field: "observaciones",
        headerName: "Observaciones",
        flex: 1,
        minWidth: 200,
        cellRenderer: (params: ICellRendererParams) => {
          const obs = params.value;
          if (!obs || obs.trim() === "") {
            return `<span class="text-xs text-gray-500 italic">Sin observaciones</span>`;
          }
          return `<span class="text-xs text-gray-300 line-clamp-2">${obs}</span>`;
        },
      },
      {
        field: "bloqueadoPor",
        headerName: "Bloqueado Por",
        width: 160,
        cellRenderer: (params: ICellRendererParams) => {
          const usuario = params.value || "Sistema";
          return `<span class="text-sm text-gray-400">${usuario}</span>`;
        },
      },
      {
        field: "fechaInicioBloqueo",
        headerName: "Fecha Bloqueo",
        width: 150,
        cellRenderer: (params: ICellRendererParams) => {
          if (!params.value) return `<span class="text-xs text-gray-500">N/A</span>`;
          const fecha = new Date(params.value);
          const fechaStr = fecha.toLocaleDateString("es-CR", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric"
          });
          const horaStr = fecha.toLocaleTimeString("es-CR", {
            hour: "2-digit",
            minute: "2-digit"
          });
          return `
            <div class="text-xs">
              <div class="text-gray-300">${fechaStr}</div>
              <div class="text-gray-500">${horaStr}</div>
            </div>
          `;
        },
      },
      {
        field: "fechaFinBloqueo",
        headerName: "Fecha Desbloqueo",
        width: 170,
        cellRenderer: (params: ICellRendererParams) => {
          const fecha = params.value;
          if (!fecha) {
            return `<span class="text-xs text-gray-500 italic">No desbloqueado</span>`;
          }
          const fechaObj = new Date(fecha);
          const fechaStr = fechaObj.toLocaleDateString("es-CR", {
            day: "2-digit",
            month: "2-digit",
            year: "numeric"
          });
          const horaStr = fechaObj.toLocaleTimeString("es-CR", {
            hour: "2-digit",
            minute: "2-digit"
          });
          return `
            <div class="text-xs">
              <div class="text-green-400">${fechaStr}</div>
              <div class="text-gray-500">${horaStr}</div>
            </div>
          `;
        },
      },
    ];
  }

  // Helper methods
  static formatTipoBadge(esPermanente: boolean): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

    if (esPermanente) {
      // Purple (GitHub Merged/Purple)
      const classes = "bg-purple-50 text-purple-700 border-purple-200 dark:bg-purple-900/30 dark:text-purple-300 dark:border-purple-800";
      return `<span class="${baseClass} ${classes}">Permanente</span>`;
    } else {
      // Yellow (GitHub Attention)
      const classes = "bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800";
      return `<span class="${baseClass} ${classes}">Temporal</span>`;
    }
  }

  static formatEstadoBadge(isActive: boolean): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

    if (isActive) {
      // Red (GitHub Closed/Blocked)
      const classes = "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
      return `<span class="${baseClass} ${classes}">● Bloqueado</span>`;
    } else {
      // Green (GitHub Open)
      const classes = "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";
      return `<span class="${baseClass} ${classes}">✓ Desbloqueado</span>`;
    }
  }
}

export function createListaNegraListLogic(): ListaNegraListLogic {
  return new ListaNegraListLogic();
}