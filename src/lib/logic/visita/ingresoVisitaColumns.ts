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
        width: 100,
        valueFormatter: (params) => params.value || 'S/G'
    },
    {
        colId: 'visitante',
        headerName: 'Visitante',
        flex: 2,
        minWidth: 200,
        valueGetter: (params) => {
            if (!params.data) return '';
            return `${params.data.visitanteNombre} ${params.data.visitanteApellido}`;
        }
    },
    {
        colId: 'visitanteCedula',
        field: 'visitanteCedula',
        headerName: 'Cédula',
        width: 120
    },
    {
        colId: 'visitanteEmpresa',
        field: 'visitanteEmpresa',
        headerName: 'Empresa / Procedencia',
        flex: 1,
        minWidth: 150,
        valueFormatter: (params) => params.value || 'Particular'
    },
    {
        colId: 'anfitrion',
        field: 'anfitrion',
        headerName: 'Anfitrión',
        width: 150
    },
    {
        colId: 'areaVisitada',
        field: 'areaVisitada',
        headerName: 'Área',
        width: 150
    },
    {
        colId: 'fechaIngreso',
        field: 'fechaIngreso',
        headerName: 'Fecha Entrada',
        width: 120,
        valueFormatter: dateFormatter
    },
    {
        colId: 'horaIngreso',
        field: 'fechaIngreso',
        headerName: 'Hora Entrada',
        width: 100,
        valueFormatter: timeFormatter
    },
    {
        colId: 'fechaSalida',
        field: 'fechaSalida',
        headerName: 'Fecha Salida',
        width: 120,
        valueFormatter: dateFormatter
    },
    {
        colId: 'horaSalida',
        field: 'fechaSalida',
        headerName: 'Hora Salida',
        width: 100,
        valueFormatter: timeFormatter
    }
];
