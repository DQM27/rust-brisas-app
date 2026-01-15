import type { ColDef, ValueFormatterParams } from '@ag-grid-community/core';
import type { IngresoVisita } from '$lib/types/ingreso-nuevos';

const dateFormatter = (params: ValueFormatterParams) => {
    if (!params.value) return '-';
    const date = new Date(params.value);
    return date.toLocaleDateString('es-ES', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric'
    });
};

const timeFormatter = (params: ValueFormatterParams) => {
    if (!params.value) return '-';
    const date = new Date(params.value);
    return date.toLocaleTimeString('es-ES', {
        hour: '2-digit',
        minute: '2-digit'
    });
};

export const INGRESO_VISITA_COLUMNS: ColDef<IngresoVisita>[] = [
    {
        colId: 'gafete',
        field: 'gafete',
        headerName: 'Gafete',
        width: 90,
        minWidth: 80,
        valueFormatter: (params) => params.value || 'S/G',
        cellClass: 'font-mono text-accent'
    },
    {
        colId: 'visitante',
        headerName: 'Visitante',
        flex: 2,
        minWidth: 200,
        sortable: true,
        filter: true,
        valueGetter: (params) => {
            if (!params.data) return '';
            return `${params.data.nombre} ${params.data.apellido}`;
        }
    },
    {
        colId: 'cedula',
        field: 'cedula',
        headerName: 'Cédula',
        width: 120,
        minWidth: 100,
        sortable: true,
        filter: true,
        cellClass: 'font-mono'
    },
    {
        colId: 'empresaNombre',
        field: 'empresaNombre',
        headerName: 'Empresa',
        flex: 1,
        minWidth: 150,
        sortable: true,
        filter: true,
        valueFormatter: (params) => params.value || 'Particular'
    },
    {
        colId: 'anfitrion',
        field: 'anfitrion',
        headerName: 'Anfitrión',
        width: 150,
        sortable: true,
        filter: true
    },
    {
        colId: 'areaVisitada',
        field: 'areaVisitada',
        headerName: 'Área',
        width: 150,
        sortable: true,
        filter: true
    },
    {
        colId: 'fechaIngreso',
        field: 'fechaIngreso',
        headerName: 'Entrada',
        width: 130,
        minWidth: 120,
        sortable: true,
        valueFormatter: dateFormatter
    },
    {
        colId: 'horaIngreso',
        field: 'fechaIngreso',
        headerName: 'Hora',
        width: 100,
        minWidth: 90,
        sortable: true,
        valueFormatter: timeFormatter
    },
    {
        colId: 'usuarioIngresoNombre',
        field: 'usuarioIngresoNombre',
        headerName: 'Registrado Por',
        width: 150,
        minWidth: 120,
        sortable: true,
        filter: true,
        hide: false
    },
    {
        colId: 'fechaSalida',
        field: 'fechaSalida',
        headerName: 'Salida',
        width: 130,
        minWidth: 120,
        sortable: true,
        valueFormatter: dateFormatter
    },
    {
        colId: 'horaSalida',
        field: 'fechaSalida',
        headerName: 'Hora Salida',
        width: 110,
        minWidth: 90,
        sortable: true,
        valueFormatter: timeFormatter
    },
    {
        colId: 'usuarioSalidaNombre',
        field: 'usuarioSalidaNombre',
        headerName: 'Salida Por',
        width: 150,
        minWidth: 120,
        sortable: true,
        filter: true,
        hide: false
    }
];
