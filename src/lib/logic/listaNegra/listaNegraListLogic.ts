// lib/logic/listaNegra/listaNegraListLogic.ts
import { get } from 'svelte/store';
import { selectedSearchStore } from "$lib/stores/searchStore";
import type { ListaNegraResponse } from "$lib/types/listaNegra";
import type { DataTableColumn } from "$lib/types/dataTable";

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
      // Si hay un resultado seleccionado, mostrar solo ese
      filtered = filtered.filter((b) => b.id === selectedSearch.result!.id);
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
  static getColumns(): DataTableColumn<ListaNegraResponse>[] {
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
        cellRenderer: (params) => {
          const empresa = params.value || "Sin empresa";
          return `<span class="text-sm text-gray-400">${empresa}</span>`;
        },
      },
      {
        field: "isActive",
        headerName: "Estado",
        width: 130,
        cellRenderer: (params) => {
          return ListaNegraListLogic.formatEstadoBadge(params.value);
        },
      },
      {
        field: "esBloqueoPermanente",
        headerName: "Tipo",
        width: 140,
        cellRenderer: (params) => {
          return ListaNegraListLogic.formatTipoBadge(params.value);
        },
      },
      {
        field: "motivoBloqueo",
        headerName: "Motivo de Bloqueo",
        flex: 1,
        minWidth: 250,
        cellRenderer: (params) => {
          const motivo = params.value || "Sin motivo especificado";
          return `<span class="text-xs text-gray-300 line-clamp-2">${motivo}</span>`;
        },
      },
      {
        field: "bloqueadoPor",
        headerName: "Bloqueado Por",
        width: 160,
        cellRenderer: (params) => {
          const usuario = params.value || "Sistema";
          return `<span class="text-sm text-gray-400">${usuario}</span>`;
        },
      },
      {
        field: "fechaBloqueo",
        headerName: "Fecha Bloqueo",
        width: 140,
        valueFormatter: (params) => {
          if (!params.value) return "N/A";
          return new Date(params.value).toLocaleDateString("es-CR");
        },
        cellStyle: { color: "#9CA3AF" },
      },
      {
        field: "fechaDesbloqueo",
        headerName: "Fecha Desbloqueo",
        width: 160,
        cellRenderer: (params) => {
          const fecha = params.value;
          if (!fecha) {
            return `<span class="text-xs text-gray-500">No desbloqueado</span>`;
          }
          return `<span class="text-xs text-gray-400">${new Date(fecha).toLocaleDateString("es-CR")}</span>`;
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