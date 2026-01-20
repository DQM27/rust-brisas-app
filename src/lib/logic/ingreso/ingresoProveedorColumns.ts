import type { ColumnDefinition } from 'tabulator-tables';
import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export interface IngresoProveedorColumnHandlers {
    onSalida: (ingreso: IngresoProveedor) => void;
}

function parseDate(value: any): Date | null {
    if (!value) return null;
    let dateStr = String(value);
    if (dateStr.startsWith("d'") && dateStr.endsWith("'")) {
        dateStr = dateStr.slice(2, -1);
    }
    const d = new Date(dateStr);
    return isNaN(d.getTime()) ? null : d;
}

export const getIngresoProveedorColumns = (
    handlers: IngresoProveedorColumnHandlers,
    viewMode: 'actives' | 'history'
): ColumnDefinition[] => {
    const cols: ColumnDefinition[] = [
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
            title: 'Nombre',
            field: 'nombre',
            width: 200,
            headerFilter: 'input',
            formatter: (cell) => {
                const data = cell.getData() as IngresoProveedor;
                return `<span style="font-weight:500; color:#e2e8f0">${data.nombre} ${data.apellido}</span>`;
            }
        },
        {
            title: 'Cédula',
            field: 'cedula',
            width: 130,
            headerFilter: 'input',
            formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#9ca3af">${cell.getValue() || ''}</span>`
        },
        {
            title: 'Empresa',
            field: 'empresaNombre',
            width: 180,
            headerFilter: 'input'
        },
        {
            title: 'Modo',
            field: 'modoIngreso',
            width: 100,
            headerFilter: 'list',
            headerFilterParams: { valuesLookup: 'active', clearable: true },
            formatter: (cell) => `<span class="capitalize">${cell.getValue() || ''}</span>`
        },
        {
            title: 'Entrada',
            field: 'fechaIngreso',
            width: 125,
            headerFilter: 'date',
            formatter: (cell) => {
                const d = parseDate(cell.getValue());
                return d ? d.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' }) : '-';
            }
        },
        {
            title: 'Hora',
            field: 'fechaIngreso_hora',
            width: 90,
            formatter: (cell) => {
                const d = parseDate(cell.getData().fechaIngreso);
                return d ? `<span class="font-mono text-white">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>` : '-';
            }
        }
    ];

    if (viewMode === 'history') {
        cols.push(
            {
                title: 'Salida',
                field: 'fechaSalida',
                width: 125,
                headerFilter: 'date',
                formatter: (cell) => {
                    const d = parseDate(cell.getValue());
                    return d ? d.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' }) : '-';
                }
            },
            {
                title: 'Hora Salida',
                field: 'fechaSalida_hora',
                width: 100,
                formatter: (cell) => {
                    const d = parseDate(cell.getData().fechaSalida);
                    return d ? `<span class="font-mono text-white">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>` : '-';
                }
            }
        );
    }

    cols.push({
        title: 'Permanencia',
        field: 'tiempoPermanencia',
        width: 120,
        formatter: (cell) => {
            const data = cell.getData() as IngresoProveedor;
            const start = parseDate(data.fechaIngreso);
            const end = data.fechaSalida ? parseDate(data.fechaSalida) : new Date();

            if (!start || !end) return '-';
            const diffMs = end.getTime() - start.getTime();
            const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
            const diffMins = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
            return `<span class="text-secondary">${diffHours}h ${diffMins}m</span>`;
        }
    });

    if (viewMode === 'actives') {
        cols.push({
            title: 'Acciones',
            width: 100,
            headerSort: false,
            hozAlign: 'center',
            formatter: () =>
                createGridBadge({
                    text: 'Salida',
                    color: 'red',
                    isButton: true,
                    className: 'salida-btn'
                }),
            cellClick: (e, cell) => {
                const target = e.target as HTMLElement;
                const button = target.closest('.salida-btn');
                if (button) {
                    handlers.onSalida(cell.getRow().getData() as IngresoProveedor);
                }
            }
        });
    }

    return cols;
};
