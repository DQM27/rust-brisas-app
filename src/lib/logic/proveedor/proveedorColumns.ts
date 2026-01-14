// ============================================
// src/lib/logic/proveedor/proveedorColumns.ts
// ============================================

import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";
import type { ProveedorResponse } from "$lib/types/proveedor";

// ============================================
// BADGE RENDERERS
// ============================================

export function formatEstadoBadge(estado: string, onClick?: () => void): string {
    const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

    const badges: Record<string, string> = {
        activo: "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800",
        inactivo: "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
        suspendido: "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800",
    };

    const estadoLower = estado?.toLowerCase() || 'inactivo';
    const badgeClass = badges[estadoLower] || badges.inactivo;
    const displayText = estado ? estado.charAt(0).toUpperCase() + estado.slice(1).toLowerCase() : 'N/A';

    // Nota: El evento onClick se maneja via onCellClicked en la definición de columna
    return `<span class="${baseClass} ${badgeClass}">${displayText}</span>`;
}

export function formatAccesoBadge(row: ProveedorResponse): string {
    const redBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
    const greenBadge = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";

    if (row.estado?.toLowerCase() !== 'activo') {
        return `<span class="${redBadge}">Denegado</span>`;
    }

    if (row.puedeIngresar) {
        return `<span class="${greenBadge}">Permitido</span>`;
    } else {
        return `<span class="${redBadge}">Denegado</span>`;
    }
}

// ============================================
// COLUMNAS
// ============================================

/**
 * Obtiene las definiciones de columnas para el Grid de Proveedores
 * @param onStatusToggle Callback opcional para cuando se hace click en el estado
 */
// ============================================
// COLUMNAS - CLASSIC STYLE
// ============================================

export class ProveedorColumns {
    static getColumns(onStatusToggle?: (id: string, currentStatus: any) => void): ColDef<ProveedorResponse>[] {
        return [
            {
                colId: "nombre",
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
                filter: "agTextColumnFilter",
            },
            {
                colId: "cedula",
                field: "cedula",
                headerName: "Cédula",
                width: 130,
                pinned: "left",
                cellStyle: { fontFamily: "monospace", fontSize: "13px" },
                filter: "agTextColumnFilter",
            },
            {
                colId: "empresaNombre",
                field: "empresaNombre",
                headerName: "Empresa",
                flex: 1,
                minWidth: 180,
                filter: "agTextColumnFilter",
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
                    const estado = params.value as string;
                    return formatEstadoBadge(estado);
                },
                cellClass: "cursor-pointer",
                onCellClicked: (params) => {
                    if (onStatusToggle && params.data && params.event) {
                        const target = params.event.target as HTMLElement;
                        if (target.closest('span')) {
                            params.event.stopPropagation();
                            onStatusToggle(params.data.id, params.data.estado);
                        }
                    }
                }
            },
            {
                colId: "puedeIngresar",
                field: "puedeIngresar",
                headerName: "Acceso",
                width: 130,
                cellRenderer: (params: ICellRendererParams) => {
                    const row = params.data as ProveedorResponse;
                    return formatAccesoBadge(row);
                },
            },
        ];
    }

    static getTrashColumns(): ColDef<ProveedorResponse>[] {
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
                field: "deletedAt" as any,
                headerName: "Eliminado",
                width: 150,
                valueFormatter: (params) => params.value ? new Date(params.value).toLocaleDateString() : 'Recientemente'
            }
        ];
    }
}

// Keep export for backward compatibility if other files import PROVEEDOR_COLUMNS directly
export const PROVEEDOR_COLUMNS = ProveedorColumns.getColumns();
