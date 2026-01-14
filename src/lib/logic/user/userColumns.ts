// src/lib/logic/user/userColumns.ts
import type { UserResponse } from "$lib/types/user";
import type { ColDef } from "@ag-grid-community/core";

export class UserColumns {
    // Column configuration
    static getColumns(
        onStatusToggle?: (id: string, currentStatus: boolean) => void
    ): (ColDef<UserResponse> | any)[] {
        return [
            // ... (resto de columnas igual)
            {
                colId: "cedula",
                field: "cedula",
                headerName: "Cédula",
                width: 130,
                pinned: "left",
                cellStyle: { fontFamily: "monospace", fontSize: "13px" },
            },
            {
                colId: "nombre",
                field: "nombre",
                headerName: "Nombre Completo",
                width: 250,
                minWidth: 100,
                cellStyle: { fontWeight: 500 },
                valueFormatter: (params: any) => {
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
                colId: "email",
                field: "email",
                headerName: "Email",
                width: 250,
                minWidth: 100,
            },
            {
                colId: "roleName",
                field: "roleName",
                headerName: "Rol",
                width: 130,
                cellRenderer: (params: any) => {
                    return UserColumns.formatRoleBadge(params.value);
                },
            },
            {
                colId: "isActive",
                field: "isActive",
                headerName: "Estado",
                width: 130,
                cellRenderer: (params: any) => {
                    return UserColumns.formatEstadoBadge(params.value);
                },
                onCellClicked: (params: any) => {
                    if (onStatusToggle && params.data && params.event) {
                        const target = params.event.target as HTMLElement;
                        if (target && target.tagName !== "BUTTON") return;
                        params.event.stopPropagation();
                        const row = params.data as UserResponse;
                        onStatusToggle(row.id, row.isActive);
                    }
                }
            },
            {
                colId: "telefono",
                field: "telefono",
                headerName: "Teléfono",
                width: 140,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                colId: "numeroGafete",
                field: "numeroGafete",
                headerName: "Gafete",
                width: 110,
                valueFormatter: (params: any) => params.value || "-",
                cellStyle: { fontFamily: "monospace" },
            },
            {
                colId: "fechaInicioLabores",
                field: "fechaInicioLabores",
                headerName: "Fecha Inicio",
                width: 130,
                valueFormatter: (params: any) => {
                    return UserColumns.formatDate(params.value);
                },
            },
            {
                colId: "operacion",
                field: "operacion",
                headerName: "Operación",
                width: 120,
                hide: true,
            },
            {
                colId: "vencimientoPortacion",
                field: "vencimientoPortacion",
                headerName: "Venc. Portación",
                width: 130,
                hide: true,
                valueFormatter: (params: any) => {
                    return UserColumns.formatDate(params.value);
                }
            },
            {
                colId: "fechaNacimiento",
                field: "fechaNacimiento",
                headerName: "Fecha Nacimiento",
                width: 130,
                hide: true,
                valueFormatter: (params: any) => {
                    return UserColumns.formatDate(params.value);
                }
            },
            {
                colId: "direccion",
                field: "direccion",
                headerName: "Dirección",
                width: 200,
                hide: true,
                wrapText: true,
                autoHeight: true,
            },
            {
                colId: "contactoEmergenciaNombre",
                field: "contactoEmergenciaNombre",
                headerName: "Contacto Emergencia",
                width: 160,
                hide: true,
            },
            {
                colId: "contactoEmergenciaTelefono",
                field: "contactoEmergenciaTelefono",
                headerName: "Tel. Emergencia",
                width: 140,
                hide: true,
                valueFormatter: (params: any) => params.value || "-",
            },
            {
                colId: "createdAt",
                field: "createdAt",
                headerName: "Creado",
                width: 150,
                hide: true,
                valueFormatter: (params: any) => {
                    if (!params.value) return "-";
                    const date = new Date(params.value);
                    const day = date.getDate().toString().padStart(2, '0');
                    const month = (date.getMonth() + 1).toString().padStart(2, '0');
                    const year = date.getFullYear();
                    const hours = date.getHours().toString().padStart(2, '0');
                    const minutes = date.getMinutes().toString().padStart(2, '0');
                    return `${day}/${month}/${year} - ${hours}:${minutes}`;
                }
            },
        ];
    }

    // Helper methods
    static formatDate(value: string | null | undefined): string {
        if (!value) return "-";
        const val = String(value);
        // Manejo básico de fecha ISO o YYYY-MM-DD
        const [year, month, day] = val.split('T')[0].split('-').map(Number);
        if (!year || !month || !day) return val;

        // Retornar formato estricto DD/MM/YYYY con ceros a la izquierda
        const dd = day.toString().padStart(2, '0');
        const mm = month.toString().padStart(2, '0');
        return `${dd}/${mm}/${year}`;
    }

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
