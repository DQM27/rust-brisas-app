// src/lib/logic/listaNegra/listaNegraColumns.ts
import type { ListaNegraResponse } from "$lib/types/listaNegra";
import type { ColDef, ICellRendererParams } from "@ag-grid-community/core";

export class ListaNegraColumns {
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
                field: "nivelSeveridad",
                headerName: "Nivel",
                width: 100,
                cellRenderer: (params: ICellRendererParams) => {
                    return ListaNegraColumns.formatNivelBadge(params.value);
                },
            },
            {
                field: "isActive",
                headerName: "Estado",
                width: 130,
                cellRenderer: (params: ICellRendererParams) => {
                    return ListaNegraColumns.formatEstadoBadge(params.value);
                },
            },
            {
                field: "motivoBloqueo",
                headerName: "Motivo",
                flex: 1,
                minWidth: 200,
                cellRenderer: (params: ICellRendererParams) => {
                    const motivo = params.value || "Sin motivo especificado";
                    return `<span class="text-xs text-gray-300 line-clamp-2">${motivo}</span>`;
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
                field: "bloqueadoDesde",
                headerName: "Fecha Bloqueo",
                width: 150,
                valueFormatter: (params) => {
                    if (!params.value) return "N/A";
                    const fecha = new Date(params.value);
                    return fecha.toLocaleString("es-CR", {
                        day: "2-digit",
                        month: "2-digit",
                        year: "numeric",
                        hour: "2-digit",
                        minute: "2-digit"
                    });
                },
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
        ];
    }

    // Helper methods
    static formatNivelBadge(nivel: string): string {
        const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

        switch (nivel) {
            case 'ALTO':
                return `<span class="${baseClass} bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800">ALTO</span>`;
            case 'MEDIO':
                return `<span class="${baseClass} bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800">MEDIO</span>`;
            case 'BAJO':
                return `<span class="${baseClass} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800">BAJO</span>`;
            default:
                return `<span class="${baseClass} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800">${nivel || 'N/A'}</span>`;
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
