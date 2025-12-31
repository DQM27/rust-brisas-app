// src/lib/logic/user/userColumns.ts
import type { UserResponse } from "$lib/types/user";
import type { ColDef } from "@ag-grid-community/core";

export class UserColumns {
    // Column configuration
    static getColumns(
        onStatusToggle?: (id: string, currentStatus: boolean) => void
    ): ColDef<UserResponse>[] {
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
                valueFormatter: (params) => {
                    const user = params.data as UserResponse;
                    if (!user) return "";
                    const fullName = [
                        user.nombre,
                        user.segundoNombre,
                        user.apellido,
                        user.segundoApellido
                    ].filter(Boolean).join(" ");
                    return fullName || user.nombre;
                }
            },
            {
                field: "email",
                headerName: "Email",
                flex: 1,
                minWidth: 200,
            },
            {
                field: "roleName",
                headerName: "Rol",
                width: 130,
                cellRenderer: (params: any) => {
                    return UserColumns.formatRoleBadge(params.value);
                },
            },
            {
                field: "isActive",
                headerName: "Estado",
                width: 130,
                cellRenderer: (params: any) => {
                    return UserColumns.formatEstadoBadge(params.value);
                },
                onCellClicked: (params) => {
                    if (onStatusToggle && params.data && params.event) {
                        const target = params.event.target as HTMLElement;
                        // Prevenir toggle si se hace click en el header o algo raro
                        // Verificamos si es el botón para evitar disparos accidentales
                        if (target && target.tagName !== "BUTTON") return;

                        // Prevenir propagación para evitar disparos dobles
                        params.event.stopPropagation();

                        const row = params.data as UserResponse;
                        onStatusToggle(row.id, row.isActive);
                    }
                }
            },
            {
                field: "telefono",
                headerName: "Teléfono",
                width: 140,
                valueFormatter: (params) => params.value || "-",
            },
            {
                field: "numeroGafete",
                headerName: "Gafete",
                width: 110,
                valueFormatter: (params) => params.value || "-",
                cellStyle: { fontFamily: "monospace" },
            },
            {
                field: "fechaInicioLabores",
                headerName: "Fecha Inicio",
                width: 130,
                valueFormatter: (params) => {
                    if (!params.value) return "-";
                    const val = String(params.value);
                    const [year, month, day] = val.split('T')[0].split('-').map(Number);
                    if (!year || !month || !day) return val;
                    const date = new Date(year, month - 1, day);
                    return date.toLocaleDateString("es-PA", {
                        year: "numeric",
                        month: "short",
                        day: "numeric",
                    });
                },
            },
        ];
    }

    // Helper methods (ahora puramente estáticos)
    static formatRoleBadge(role: string): string {
        const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border";

        const badges: Record<string, string> = {
            admin: "bg-purple-50 text-purple-700 border-purple-200 dark:bg-purple-900/30 dark:text-purple-300 dark:border-purple-800",
            supervisor: "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-300 dark:border-blue-800",
            guardia: "bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700",
        };

        const badgeClass = badges[role] || badges.guardia;
        const displayText = role ? role.charAt(0).toUpperCase() + role.slice(1) : 'N/A';

        return `
      <span class="${baseClass} ${badgeClass}">
        ${displayText}
      </span>
    `;
    }

    static formatEstadoBadge(isActive: boolean): string {
        const baseClass = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border cursor-pointer hover:opacity-80 transition-opacity";

        const activeBadge = "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800";
        const inactiveBadge = "bg-gray-50 text-gray-600 border-gray-200 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-700";

        const badgeClass = isActive ? activeBadge : inactiveBadge;
        const displayText = isActive ? "Activo" : "Inactivo";

        return `
      <button 
        class="${baseClass} ${badgeClass}"
        title="Clic para cambiar estado"
      >
        ${displayText}
      </button>
    `;
    }
}
