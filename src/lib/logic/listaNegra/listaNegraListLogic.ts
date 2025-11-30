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
    if (esPermanente) {
      return `
        <span class="inline-flex items-center gap-1.5 rounded-full border border-purple-500/20 bg-purple-500/10 px-2.5 py-1 text-xs font-medium text-purple-400">
          Permanente
        </span>
      `;
    } else {
      return `
        <span class="inline-flex items-center gap-1.5 rounded-full border border-yellow-500/20 bg-yellow-500/10 px-2.5 py-1 text-xs font-medium text-yellow-400">
          Temporal
        </span>
      `;
    }
  }

  static formatEstadoBadge(isActive: boolean): string {
    if (isActive) {
      return `
        <span class="inline-flex items-center gap-1.5 rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium text-red-400">
          ● Bloqueado
        </span>
      `;
    } else {
      return `
        <span class="inline-flex items-center gap-1.5 rounded-full border border-green-500/20 bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-400">
          ✓ Desbloqueado
        </span>
      `;
    }
  }
}

export function createListaNegraListLogic(): ListaNegraListLogic {
  return new ListaNegraListLogic();
}