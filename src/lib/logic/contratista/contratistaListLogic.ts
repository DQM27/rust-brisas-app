// lib/logic/contratista/contratistaListLogic.ts
import { get } from 'svelte/store';
import { searchStore } from "$lib/stores/searchStore";
import type { ContratistaResponse, EstadoContratista } from "$lib/types/contratista";
import type { DataTableColumn } from "$lib/types/dataTable";

export interface ContratistaListState {
  estadoFilter: "todos" | "activo" | "inactivo" | "suspendido";
  praindFilter: "todos" | "vigente" | "vencido" | "por-vencer";
}

export class ContratistaListLogic {
  private state: ContratistaListState;

  constructor() {
    this.state = {
      estadoFilter: "todos",
      praindFilter: "todos"
    };
  }

  getState(): ContratistaListState {
    return this.state;
  }

  getFilteredData(contratistas: ContratistaResponse[]): ContratistaResponse[] {
    let filtered = contratistas;

    // Filtro de estado
    if (this.state.estadoFilter !== "todos") {
      filtered = filtered.filter((c) => c.estado === this.state.estadoFilter);
    }

    // Filtro de PRAIND
    if (this.state.praindFilter === "vigente") {
      filtered = filtered.filter(
        (c) => !c.praindVencido && c.diasHastaVencimiento > 30,
      );
    } else if (this.state.praindFilter === "vencido") {
      filtered = filtered.filter((c) => c.praindVencido);
    } else if (this.state.praindFilter === "por-vencer") {
      filtered = filtered.filter(
        (c) => !c.praindVencido && c.diasHastaVencimiento <= 30,
      );
    }

    // Filtro por búsqueda de Tantivy
    const searchState = get(searchStore);
    if (searchState.results.length > 0) {
      const searchIds = new Set(searchState.results.map((r) => r.id));
      filtered = filtered.filter((c) => searchIds.has(c.id));
    }

    return filtered;
  }

  getStats(contratistas: ContratistaResponse[]) {
    return {
      total: contratistas.length,
      activos: contratistas.filter((c) => c.estado === "activo").length,
      vencidos: contratistas.filter((c) => c.praindVencido).length,
      porVencer: contratistas.filter(
        (c) => !c.praindVencido && c.diasHastaVencimiento <= 30,
      ).length,
    };
  }

  // Actions
  setEstadoFilter(filter: ContratistaListState['estadoFilter']): void {
    this.state.estadoFilter = filter;
  }

  setPraindFilter(filter: ContratistaListState['praindFilter']): void {
    this.state.praindFilter = filter;
  }

  clearAllFilters(): void {
    this.state.estadoFilter = "todos";
    this.state.praindFilter = "todos";
    searchStore.clearResults();
  }

  // Column configuration
  static getColumns(): DataTableColumn<ContratistaResponse>[] {
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
        field: "estado",
        headerName: "Estado",
        width: 130,
        cellRenderer: (params) => {
          const estado = params.value as EstadoContratista;
          return ContratistaListLogic.formatEstadoBadge(estado);
        },
      },
      {
        field: "praindVencido",
        headerName: "PRAIND",
        width: 130,
        cellRenderer: (params) => {
          const row = params.data as ContratistaResponse;
          return ContratistaListLogic.formatPraindBadge(row);
        },
      },
      {
        field: "fechaVencimientoPraind",
        headerName: "Vencimiento",
        width: 130,
        valueFormatter: (params) => {
          if (!params.value) return "";
          const date = new Date(params.value);
          return date.toLocaleDateString("es-PA", {
            year: "numeric",
            month: "short",
            day: "numeric",
          });
        },
      },
      {
        field: "puedeIngresar",
        headerName: "Acceso",
        width: 130,
        cellRenderer: (params) => {
          const canEnter = params.value;
          if (canEnter) {
            return `
              <span class="inline-flex items-center gap-1.5 rounded-full border border-green-500/20 bg-green-500/10 px-2.5 py-1 text-xs font-medium text-green-400">
                Permitido
              </span>
            `;
          } else {
            return `
              <span class="inline-flex items-center gap-1.5 rounded-full border border-red-500/20 bg-red-500/10 px-2.5 py-1 text-xs font-medium text-red-400">
                Denegado
              </span>
            `;
          }
        },
      },
    ];
  }

  // Helper methods
  static formatEstadoBadge(estado: EstadoContratista): string {
    const badges = {
      activo: "bg-green-500/10 text-green-400 border-green-500/20",
      inactivo: "bg-gray-500/10 text-gray-400 border-gray-500/20", 
      suspendido: "bg-red-500/10 text-red-400 border-red-500/20",
    };
    
    const badgeClass = badges[estado] || badges.inactivo;
    const displayText = estado ? estado.charAt(0).toUpperCase() + estado.slice(1) : 'N/A';
    
    return `
      <span class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium ${badgeClass}">
        ${displayText}
      </span>
    `;
  }

  static formatPraindBadge(row: ContratistaResponse): string {
    let badgeClass = "";
    let text = "";

    if (row.praindVencido) {
      badgeClass = "bg-red-500/10 text-red-400 border-red-500/20";
      text = "Vencido";
    } else if (row.diasHastaVencimiento <= 30) {
      badgeClass = "bg-yellow-500/10 text-yellow-400 border-yellow-500/20";
      text = `${row.diasHastaVencimiento} días`;
    } else {
      badgeClass = "bg-green-500/10 text-green-400 border-green-500/20";
      text = "Vigente";
    }

    return `
      <span class="inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs font-medium ${badgeClass}">
        ${text}
      </span>
    `;
  }
}

export function createContratistaListLogic(): ContratistaListLogic {
  return new ContratistaListLogic();
}