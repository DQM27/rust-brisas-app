
import type { ColDef } from "@ag-grid-community/core";
import type { VisitanteResponse } from "$lib/types/visitante";

// ============================================
// COLUMNAS
// ============================================

export class VisitanteColumns {
    static getColumns(): ColDef<VisitanteResponse>[] {
        return [
            {
                field: "cedula",
                headerName: "Cédula",
                filter: "agTextColumnFilter",
                width: 150,
                pinned: "left",
            },
            {
                field: "nombre",
                headerName: "Nombre",
                filter: "agTextColumnFilter",
                width: 150,
                valueGetter: (params) => params.data ? `${params.data.nombre}` : '',
            },
            {
                field: "apellido",
                headerName: "Apellido",
                filter: "agTextColumnFilter",
                width: 150,
            },
            {
                field: "empresaNombre",
                headerName: "Empresa",
                filter: "agTextColumnFilter",
                width: 200,
                valueGetter: (params) => params.data?.empresaNombre || 'N/A',
            },
            {
                field: "hasVehicle",
                headerName: "Vehículo",
                width: 100,
                cellRenderer: (params: any) => {
                    return params.value ? 'Sí' : 'No';
                },
            },
            {
                field: "createdAt",
                headerName: "Fecha Registro",
                filter: "agDateColumnFilter",
                width: 180,
                valueFormatter: (params) => {
                    if (!params.value) return "";
                    return new Date(params.value).toLocaleString("es-ES");
                },
            },
        ];
    }

    static getTrashColumns(): ColDef<VisitanteResponse>[] {
        return [
            {
                field: "cedula",
                headerName: "Cédula",
                width: 150,
                pinned: "left",
            },
            {
                field: "nombre",
                headerName: "Nombre",
                width: 150,
            },
            {
                field: "apellido",
                headerName: "Apellido",
                width: 150,
            },
            {
                field: "empresaNombre",
                headerName: "Empresa",
                width: 200,
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

export const VISITANTE_COLUMNS = VisitanteColumns.getColumns();
