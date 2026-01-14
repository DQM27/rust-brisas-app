
import type { ColDef } from "@ag-grid-community/core";
import type { VisitanteResponse } from "$lib/types/visitante";

// ============================================
// COLUMNAS
// ============================================

export class VisitanteColumns {
    static getColumns(): ColDef<VisitanteResponse>[] {
        return [
            {
                colId: "cedula",
                field: "cedula",
                headerName: "Cédula",
                filter: "agTextColumnFilter",
                width: 150,
                pinned: "left",
            },
            {
                colId: "nombre",
                field: "nombre",
                headerName: "Nombre",
                filter: "agTextColumnFilter",
                width: 150,
                valueGetter: (params) => params.data ? `${params.data.nombre}` : '',
            },
            {
                colId: "apellido",
                field: "apellido",
                headerName: "Apellido",
                filter: "agTextColumnFilter",
                width: 150,
            },
            {
                colId: "empresaNombre",
                field: "empresaNombre",
                headerName: "Empresa",
                filter: "agTextColumnFilter",
                width: 200,
                valueGetter: (params) => params.data?.empresaNombre || 'N/A',
            },
            {
                colId: "hasVehicle",
                field: "hasVehicle",
                headerName: "Vehículo",
                width: 100,
                cellRenderer: (params: any) => {
                    return params.value ? 'Sí' : 'No';
                },
            },
            {
                colId: "createdAt",
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
                colId: "nombreCompleto",
                headerName: "Nombre",
                flex: 1,
                minWidth: 200,
                valueGetter: (params) => {
                    if (!params.data) return "";
                    return [params.data.nombre, params.data.apellido].filter(Boolean).join(" ");
                },
            },
            {
                field: "empresaNombre",
                headerName: "Empresa",
                width: 200,
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
            }
        ];
    }
}

export const VISITANTE_COLUMNS = VisitanteColumns.getColumns();
