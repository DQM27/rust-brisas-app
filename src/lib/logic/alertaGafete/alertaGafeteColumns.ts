import type { ColumnDefinition } from 'tabulator-tables';
import type { AlertaGafeteResponse } from '$lib/types/ingreso';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

function formatDateSlashed(val: string) {
    if (!val) return '-';
    try {
        let cleanVal = String(val);
        if (cleanVal.startsWith("d'") || cleanVal.startsWith('d"')) {
            cleanVal = cleanVal.substring(2, cleanVal.length - 1);
        }
        const date = new Date(cleanVal);
        if (isNaN(date.getTime())) return val;
        return date.toLocaleDateString('es-PA', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric'
        });
    } catch {
        return val;
    }
}

function formatMilitaryTime(val: string) {
    if (!val) return '-';
    try {
        let cleanVal = String(val);
        if (cleanVal.startsWith("d'") || cleanVal.startsWith('d"')) {
            cleanVal = cleanVal.substring(2, cleanVal.length - 1);
        }
        const date = new Date(cleanVal);
        if (isNaN(date.getTime())) return val;
        return date.toLocaleTimeString('es-PA', {
            hour: '2-digit',
            minute: '2-digit',
            hour12: false
        });
    } catch {
        return val;
    }
}

export function getAlertaGafeteColumns(callbacks: {
    onResolve?: (alerta: AlertaGafeteResponse) => void;
}): ColumnDefinition[] {
    return [
        {
            title: 'Gafete #',
            field: 'gafeteNumero',
            width: 90,
            hozAlign: 'center',
            headerFilter: 'input',
            formatter: (cell) => `<span class="cell-code font-bold tracking-wide">${cell.getValue()}</span>`
        },
        {
            title: 'Tipo',
            field: 'id',
            width: 120,
            hozAlign: 'center',
            headerSort: false,
            formatter: (cell: any) => {
                const data = cell.getData() as AlertaGafeteResponse;
                if (data.ingresoContratistaId) {
                    return createGridBadge({ text: 'Contratista', color: 'blue' });
                }
                if (data.ingresoProveedorId) {
                    return createGridBadge({ text: 'Proveedor', color: 'amber' });
                }
                if (data.ingresoVisitaId) {
                    return createGridBadge({ text: 'Visita', color: 'blue' });
                }
                return createGridBadge({ text: 'General', color: 'gray' });
            }
        },
        {
            title: 'Persona',
            field: 'nombreCompleto',
            width: 220,
            headerFilter: 'input',
            formatter: (cell: any) => `<span class="text-primary font-medium">${cell.getValue()}</span>`
        },
        {
            title: 'Cédula',
            field: 'cedula',
            width: 110,
            headerFilter: 'input',
            formatter: (cell: any) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
        },
        {
            title: 'Empresa',
            field: 'empresaNombre',
            width: 180,
            headerFilter: 'input',
            formatter: (cell: any) => `<span class="text-secondary uppercase text-[11px] font-medium">${cell.getValue() || 'S/E'}</span>`
        },
        {
            title: 'Fecha',
            field: 'fechaReporte',
            width: 110,
            hozAlign: 'center',
            formatter: (cell: any) => `<span class="text-secondary">${formatDateSlashed(cell.getValue())}</span>`
        },
        {
            title: 'Hora',
            field: 'fechaReporte',
            width: 80,
            hozAlign: 'center',
            formatter: (cell: any) => `<span class="text-secondary font-medium">${formatMilitaryTime(cell.getValue())}</span>`
        },
        {
            title: 'Estado',
            field: 'resuelto',
            width: 110,
            hozAlign: 'center',
            formatter: (cell: any) => {
                const isResuelto = cell.getValue();
                return isResuelto
                    ? createGridBadge({ text: 'Resuelto', color: 'green', withDot: true })
                    : createGridBadge({ text: 'Pendiente', color: 'red', withDot: true });
            }
        },
        {
            title: 'Reportado por',
            field: 'reportadoPorNombre',
            width: 180,
            hozAlign: 'left',
            formatter: (cell) => `<span class="text-secondary text-[11px] font-medium">${cell.getValue() || 'Sistema'}</span>`
        },
        {
            title: 'Observación',
            field: 'notas',
            width: 250,
            formatter: 'textarea'
        },
        {
            title: 'Acciones',
            field: 'actions',
            width: 110,
            hozAlign: 'center',
            headerSort: false,
            formatter: (cell: any) => {
                const data = cell.getData();
                if (data.resuelto) return '';
                return createGridBadge({ text: 'Resolver', color: 'blue', isButton: true, className: 'resolve-btn' });
            },
            cellClick: (e: any, cell: any) => {
                const target = e.target as HTMLElement;
                if (target.classList.contains('resolve-btn')) {
                    callbacks.onResolve?.(cell.getData());
                }
            }
        }
    ];
}
