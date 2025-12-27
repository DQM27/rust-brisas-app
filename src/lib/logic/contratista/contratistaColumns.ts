// lib/logic/contratista/contratistaListLogic.ts
import { get } from 'svelte/store';
import { selectedSearchStore } from "$lib/stores/searchStore";
import type { ContratistaResponse, EstadoContratista } from "$lib/types/contratista";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

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

    // CAMBIADO: Filtro por búsqueda seleccionada (tiene prioridad)
    const selectedSearch = get(selectedSearchStore);
    if (selectedSearch.result) {
      // Si hay un resultado seleccionado, mostrar solo ese
      filtered = filtered.filter((c) => c.id === selectedSearch.result!.id);
      return filtered; // Retornar temprano, ignorando otros filtros
    }

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
    selectedSearchStore.clear(); // NUEVO: Limpiar búsqueda seleccionada
  }

  // Column configuration
  static getColumns(
    onStatusToggle?: (id: string, currentStatus: EstadoContratista) => void
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
          return ContratistaListLogic.formatEstadoBadge(estado); // Ahora retorna un botón
        },
        onCellClicked: (params) => {
          if (onStatusToggle && params.data) {
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
          // Use UTC components to avoid timezone shift
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

          // Estilo GitHub "Closed" / "Failure" (Rojo)
          const redBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";

          // Estilo GitHub "Open" / "Success" (Verde)
          const greenBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";

          if (row.estaBloqueado) {
            return `<span class="${redBadge}">Bloqueado</span>`;
          }

          // Si está inactivo o suspendido, el acceso es denegado
          if (row.estado !== 'activo') {
            return `<span class="${redBadge}">Denegado</span>`;
          }

          if (row.puedeIngresar) {
            return `<span class="${greenBadge}">Permitido</span>`;
          } else {
            return `<span class="${redBadge}">Denegado</span>`;
          }
        },
      },
    ];
  }

  // Helper methods
  static formatEstadoBadge(estado: EstadoContratista): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

    const badges = {
      // GitHub Open (Green)
      activo: "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800",
      // GitHub Draft/Gray (Gray)
      inactivo: "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
      // GitHub Closed (Red/Purple) -> Usamos Rojo para suspendido
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
      // Red
      badgeClass = "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
      text = "Vencido";
    } else if (row.diasHastaVencimiento <= 30) {
      // Yellow/Orange (Attention)
      badgeClass = "bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800";
      text = `${row.diasHastaVencimiento} días`;
    } else {
      // Green
      badgeClass = "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";
      text = "Vigente";
    }

    return `
      <span class="${baseClass} ${badgeClass}">
        ${text}
      </span>
    `;
  }
}

export function createContratistaListLogic(): ContratistaListLogic {
  return new ContratistaListLogic();
}