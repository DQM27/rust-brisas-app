// ============================================
// src/lib/logic/proveedor/proveedorListLogic.ts
// ============================================
// Logic for Proveedor list/grid (columns, filters, badges)

import { get } from 'svelte/store';
import { selectedSearchStore } from "$lib/stores/searchStore";
import type { ProveedorResponse, EstadoProveedor } from "$lib/types/proveedor";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

// ============================================
// STATE INTERFACE
// ============================================

export interface ProveedorListState {
    estadoFilter: "todos" | "activo" | "inactivo" | "suspendido";
}

// ============================================
// LOGIC CLASS
// ============================================

export class ProveedorListLogic {
    private state: ProveedorListState;

    constructor() {
        this.state = {
            estadoFilter: "todos"
        };
    }

    getState(): ProveedorListState {
        return this.state;
    }

    getFilteredData(proveedores: ProveedorResponse[]): ProveedorResponse[] {
        let filtered = proveedores;

        // Filtro por búsqueda seleccionada (tiene prioridad)
        const selectedSearch = get(selectedSearchStore);
        if (selectedSearch.result) {
            filtered = filtered.filter((p) => p.id === selectedSearch.result!.id);
            return filtered;
        }

        // Filtro de estado
        if (this.state.estadoFilter !== "todos") {
            filtered = filtered.filter((p) =>
                p.estado?.toLowerCase() === this.state.estadoFilter
            );
        }

        return filtered;
    }

    getStats(proveedores: ProveedorResponse[]) {
        return {
            total: proveedores.length,
            activos: proveedores.filter((p) => p.estado?.toLowerCase() === "activo").length,
            inactivos: proveedores.filter((p) => p.estado?.toLowerCase() === "inactivo").length,
        };
    }

    // Actions
    setEstadoFilter(filter: ProveedorListState['estadoFilter']): void {
        this.state.estadoFilter = filter;
    }

    clearAllFilters(): void {
        this.state.estadoFilter = "todos";
        selectedSearchStore.clear();
    }

    // Column configuration
    static getColumns(
        onStatusToggle?: (id: string, currentStatus: string) => void
    ): ColDef<ProveedorResponse>[] {
        return [
            {
                field: "cedula",
                headerName: "Cédula",
                width: 130,
                pinned: "left",
                cellStyle: { fontFamily: "monospace", fontSize: "13px" },
            },
            {
                field: "nombre",
                headerName: "Nombre Completo",
                flex: 1,
                minWidth: 200,
                cellStyle: { fontWeight: 500 },
                valueGetter: (params) => {
                    const d = params.data;
                    if (!d) return "";
                    return [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
                        .filter(Boolean)
                        .join(" ");
                },
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
                    const estado = params.value as string;
                    return ProveedorListLogic.formatEstadoBadge(estado);
                },
                onCellClicked: (params) => {
                    if (onStatusToggle && params.data) {
                        const row = params.data as ProveedorResponse;
                        onStatusToggle(row.id, row.estado as string);
                    }
                }
            },
            {
                field: "puedeIngresar",
                headerName: "Acceso",
                width: 130,
                cellRenderer: (params: ICellRendererParams) => {
                    const row = params.data as ProveedorResponse;
                    return ProveedorListLogic.formatAccesoBadge(row);
                },
            },
        ];
    }

    // ============================================
    // BADGE HELPERS
    // ============================================

    static formatEstadoBadge(estado: string): string {
        const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

        const badges: Record<string, string> = {
            activo: "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800",
            inactivo: "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
            suspendido: "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800",
        };

        const estadoLower = estado?.toLowerCase() || 'inactivo';
        const badgeClass = badges[estadoLower] || badges.inactivo;
        const displayText = estado ? estado.charAt(0).toUpperCase() + estado.slice(1).toLowerCase() : 'N/A';

        return `
      <button 
        class="${baseClass} ${badgeClass}"
        title="Clic para cambiar estado"
      >
        ${displayText}
      </button>
    `;
    }

    static formatAccesoBadge(row: ProveedorResponse): string {
        const redBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
        const greenBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";

        // Si está inactivo o suspendido, el acceso es denegado
        if (row.estado?.toLowerCase() !== 'activo') {
            return `<span class="${redBadge}">Denegado</span>`;
        }

        if (row.puedeIngresar) {
            return `<span class="${greenBadge}">Permitido</span>`;
        } else {
            return `<span class="${redBadge}">Denegado</span>`;
        }
    }
}

// ============================================
// FACTORY FUNCTION
// ============================================

export function createProveedorListLogic(): ProveedorListLogic {
    return new ProveedorListLogic();
}
