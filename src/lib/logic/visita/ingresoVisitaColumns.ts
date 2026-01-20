import type { ColumnDefinition } from 'tabulator-tables';
import type { IngresoVisita } from '$lib/types/ingreso-nuevos';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export const getIngresoVisitaColumns = (): ColumnDefinition[] => {
    return [
        {
            title: 'Gafete',
            field: 'gafete',
            width: 90,
            headerFilter: 'input',
            formatter: (cell) => {
                const val = cell.getValue();
                return val
                    ? `<span style="font-family:monospace; font-size:12px; font-weight:700" class="bg-surface-3 px-1.5 py-0.5 rounded text-accent">${val}</span>`
                    : '<span style="font-size:11px" class="text-secondary italic">S/G</span>';
            }
        },
        {
            title: 'Visitante',
            field: 'nombre',
            width: 200,
            headerFilter: 'input',
            formatter: (cell) => {
                const data = cell.getData() as IngresoVisita;
                return `<span style="font-weight:500; color:#e2e8f0">${data.nombre} ${data.apellido || ''}</span>`;
            }
        },
        {
            title: 'Cédula',
            field: 'cedula',
            width: 120,
            headerFilter: 'input',
            formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#9ca3af">${cell.getValue() || ''}</span>`
        },
        {
            title: 'Empresa',
            field: 'empresaNombre',
            width: 150,
            headerFilter: 'input',
            formatter: (cell) => cell.getValue() || '<span class="text-secondary italic">Particular</span>'
        },
        {
            title: 'Anfitrión',
            field: 'anfitrion',
            width: 150,
            headerFilter: 'input'
        },
        {
            title: 'Área',
            field: 'areaVisitada',
            width: 150,
            headerFilter: 'input'
        },
        {
            title: 'Entrada',
            field: 'fechaIngreso',
            width: 125,
            headerFilter: 'date',
            formatter: (cell) => {
                const val = cell.getValue();
                if (!val) return '-';
                return new Date(val).toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' });
            }
        },
        {
            title: 'Hora',
            field: 'fechaIngreso_hora',
            width: 90,
            formatter: (cell) => {
                const val = cell.getData().fechaIngreso;
                if (!val) return '-';
                const d = new Date(val);
                return `<span style="font-family:monospace; color:#e2e8f0">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>`;
            }
        },
        {
            title: 'Registrado Por',
            field: 'usuarioIngresoNombre',
            width: 150,
            headerFilter: 'input'
        },
        {
            title: 'Salida',
            field: 'fechaSalida',
            width: 125,
            headerFilter: 'date',
            formatter: (cell) => {
                const val = cell.getValue();
                if (!val) return '-';
                return new Date(val).toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' });
            }
        },
        {
            title: 'Hora Salida',
            field: 'fechaSalida_hora',
            width: 110,
            formatter: (cell) => {
                const val = cell.getData().fechaSalida;
                if (!val) return '-';
                const d = new Date(val);
                return `<span style="font-family:monospace; color:#e2e8f0">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>`;
            }
        },
        {
            title: 'Salida Por',
            field: 'usuarioSalidaNombre',
            width: 150,
            headerFilter: 'input'
        }
    ];
};
