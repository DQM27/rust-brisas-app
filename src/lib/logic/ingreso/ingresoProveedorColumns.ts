import type { ColumnDefinition } from 'tabulator-tables';
import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';

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
                    ? `<span class="font-mono text-xs bg-surface-3 px-1.5 py-0.5 rounded text-accent">${val}</span>`
                    : '<span class="text-secondary text-xs italic">S/G</span>';
            }
        },
        {
            title: 'Nombre',
            field: 'nombre',
            width: 200,
            headerFilter: 'input',
            formatter: (cell) => {
                const data = cell.getData() as IngresoProveedor;
                return `<span class="font-medium text-white">${data.nombre} ${data.apellido}</span>`;
            }
        },
        {
            title: 'Cédula',
            field: 'cedula',
            width: 130,
            headerFilter: 'input',
            formatter: (cell) => `<span class="font-mono text-xs text-secondary">${cell.getValue() || ''}</span>`
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
                `<button class="salida-btn text-error hover:bg-error/10 p-1.5 rounded-md transition-colors" title="Registrar Salida">
                    <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>
                </button>`,
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
