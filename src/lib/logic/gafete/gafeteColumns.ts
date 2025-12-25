import type { GafeteResponse } from "$lib/types/gafete";
import type { ColDef } from "@ag-grid-community/core";

export class GafeteColumns {
    static getColumns(handlers: {
        onResolve: (data: GafeteResponse) => void;
        onRecover: (data: GafeteResponse) => void;
        onLost: (data: GafeteResponse) => void;
        onDamage: (data: GafeteResponse) => void;
        onDelete: (data: GafeteResponse) => void;
        onEdit: (data: GafeteResponse) => void;
    }): ColDef<GafeteResponse>[] {
        return [
            {
                field: "numero",
                headerName: "Número",
                sortable: true,
                filter: true,
                cellStyle: { fontWeight: "bold" },
                width: 120,
            },
            {
                field: "tipoDisplay",
                headerName: "Tipo",
                sortable: true,
                filter: true,
                width: 130,
                cellRenderer: (params: any) => {
                    const tipo = params.data.tipo;
                    const baseClass =
                        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";
                    let colorClass = "";

                    switch (tipo) {
                        case "contratista":
                            colorClass =
                                "bg-indigo-50 text-indigo-700 border-indigo-200 dark:bg-indigo-900/30 dark:text-indigo-300 dark:border-indigo-800";
                            break;
                        case "proveedor":
                            colorClass =
                                "bg-amber-50 text-amber-700 border-amber-200 dark:bg-amber-900/30 dark:text-amber-300 dark:border-amber-800";
                            break;
                        case "visita":
                            colorClass =
                                "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
                            break;
                        default:
                            colorClass =
                                "bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700";
                    }

                    return `<span class="${baseClass} ${colorClass}">${params.value}</span>`;
                },
            },
            {
                field: "status",
                headerName: "Estado",
                sortable: true,
                filter: true,
                width: 140,
                cellRenderer: (params: any) => {
                    const status = params.value;
                    const baseClass =
                        "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border uppercase tracking-wide";

                    let classes = "";
                    let icon = "";
                    let label = "";

                    switch (status) {
                        case "disponible":
                            classes =
                                "bg-emerald-50 text-emerald-700 border-emerald-200 dark:bg-emerald-900/30 dark:text-emerald-300 dark:border-emerald-800";
                            icon = "✔";
                            label = "Disponible";
                            break;
                        case "en_uso":
                            classes =
                                "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800";
                            icon = "◉";
                            label = "En Uso";
                            break;
                        case "perdido":
                            classes =
                                "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800";
                            icon = "✖";
                            label = "Perdido";
                            break;
                        case "danado":
                            classes =
                                "bg-rose-100 text-rose-800 border-rose-300 dark:bg-rose-900/50 dark:text-rose-200 dark:border-rose-700";
                            icon = "⚡";
                            label = "Dañado";
                            break;
                        case "extraviado":
                            classes =
                                "bg-amber-100 text-amber-800 border-amber-300 dark:bg-amber-900/50 dark:text-amber-200 dark:border-amber-700";
                            icon = "❓";
                            label = "Extraviado";
                            break;
                        default:
                            classes =
                                "bg-gray-100 text-gray-800 border-gray-300 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600";
                            icon = "-";
                            label = status;
                    }

                    return `<span class="${baseClass} ${classes}"><span class="mr-1 opacity-75">${icon}</span> ${label}</span>`;
                },
            },
            {
                field: "fechaPerdido",
                headerName: "Fecha Reporte",
                sortable: true,
                filter: true,
                width: 130,
                valueFormatter: (params: any) => {
                    if (!params.value) return "-";
                    return new Date(params.value).toLocaleDateString();
                },
            },
            {
                field: "quienPerdio",
                headerName: "Persona que Perdió",
                sortable: true,
                filter: true,
                width: 180,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                field: "reportadoPorNombre",
                headerName: "Reportado Por",
                sortable: true,
                filter: true,
                width: 160,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                field: "resueltoPorNombre",
                headerName: "Resuelto Por",
                sortable: true,
                filter: true,
                width: 150,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                field: "fechaResolucion",
                headerName: "Fecha Resolución",
                sortable: true,
                filter: true,
                width: 160,
                valueFormatter: (params: any) => {
                    if (!params.value) return "-";
                    return new Date(params.value).toLocaleDateString();
                },
            },
            {
                field: "notas",
                headerName: "Notas",
                sortable: true,
                filter: true,
                width: 200,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                headerName: "Acciones",
                width: 220,
                pinned: "right",
                cellRenderer: (params: any) => {
                    const status = params.data.status;
                    let buttons = "";

                    if (status === "perdido") {
                        buttons += `
              <button class="mr-2 px-2 py-1 bg-green-100 text-green-700 rounded hover:bg-green-200 text-xs font-medium resolve-btn">
                ✓ Resolver
              </button>
            `;
                    }

                    if (status !== "perdido") {
                        // Botón Editar
                        buttons += `
                <button class="mr-2 px-2 py-1 bg-gray-100 text-gray-700 rounded hover:bg-gray-200 text-xs font-medium edit-btn" title="Editar">
                  ✏️
                </button>
              `;

                        if (status === "extraviado") {
                            buttons += `
                <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn">
                  ↻ Recuperar
                </button>
              `;
                        } else if (status !== "danado") {
                            buttons += `
                <button class="mr-2 px-2 py-1 bg-amber-100 text-amber-700 rounded hover:bg-amber-200 text-xs font-medium lost-btn" title="Marcar como Extraviado">
                  ❓ Ext
                </button>
                <button class="mr-2 px-2 py-1 bg-rose-100 text-rose-700 rounded hover:bg-rose-200 text-xs font-medium damage-btn" title="Marcar como Dañado">
                  ⚡ Dañ
                </button>
              `;
                        }

                        if (status === "danado") {
                            buttons += `
                <button class="mr-2 px-2 py-1 bg-emerald-100 text-emerald-700 rounded hover:bg-emerald-200 text-xs font-medium recover-btn" title="Marcar como Reparado/Activo">
                  ↻ Rep
                </button>
              `;
                        }

                        if (status === "danado" || status === "disponible") {
                            buttons += `
                <button class="px-2 py-1 bg-gray-100 text-gray-700 rounded hover:bg-red-100 hover:text-red-700 text-xs font-medium delete-btn" title="Eliminar">
                  🗑️
                </button>
              `;
                        }
                    }

                    return buttons || `<span class="text-xs text-gray-400">-</span>`;
                },
                onCellClicked: (params: any) => {
                    const event = params.event;
                    const data = params.data;
                    const target = event.target as HTMLElement;

                    if (target.classList.contains("resolve-btn")) {
                        handlers.onResolve(data);
                    } else if (target.classList.contains("edit-btn")) {
                        handlers.onEdit(data);
                    } else if (target.classList.contains("recover-btn")) {
                        handlers.onRecover(data);
                    } else if (target.classList.contains("lost-btn")) {
                        handlers.onLost(data);
                    } else if (target.classList.contains("damage-btn")) {
                        handlers.onDamage(data);
                    } else if (target.classList.contains("delete-btn")) {
                        handlers.onDelete(data);
                    }
                },
            },
        ];
    }
}
