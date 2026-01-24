import type { ColumnDefinition } from 'tabulator-tables';
import type { AlertaGafeteResponse } from '$lib/types/ingreso';

function formatDateTime(val: any) {
    if (!val) return '-';
    try {
        const date = new Date(val);
        return date.toLocaleString('es-PA', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
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
            width: 100,
            hozAlign: 'center',
            headerFilter: 'input'
        },
        {
            title: 'Tipo',
            field: 'tipoGafete', // Virtual field
            width: 120,
            hozAlign: 'center',
            formatter: (cell: any) => {
                const data = cell.getData();
                if (data.ingresoContratistaId) return '<span class="px-2 py-0.5 rounded-full bg-blue-500/10 text-blue-400 text-[10px] font-bold uppercase">Contratista</span>';
                if (data.ingresoProveedorId) return '<span class="px-2 py-0.5 rounded-full bg-purple-500/10 text-purple-400 text-[10px] font-bold uppercase">Proveedor</span>';
                if (data.ingresoVisitaId) return '<span class="px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 text-[10px] font-bold uppercase">Visita</span>';
                return '<span class="px-2 py-0.5 rounded-full bg-gray-500/10 text-gray-400 text-[10px] font-bold uppercase">General</span>';
            }
        },
        {
            title: 'Persona',
            field: 'nombreCompleto',
            width: 250,
            headerFilter: 'input',
            formatter: (cell: any) => {
                const data = cell.getData();
                return `
					<div class="flex flex-col">
						<span class="font-bold text-primary">${data.nombreCompleto}</span>
						<span class="text-[10px] text-tertiary">${data.cedula}</span>
					</div>
				`;
            }
        },
        {
            title: 'Fecha Reporte',
            field: 'fechaReporte',
            width: 180,
            formatter: (cell: any) => formatDateTime(cell.getValue())
        },
        {
            title: 'Estado',
            field: 'resuelto',
            width: 120,
            hozAlign: 'center',
            formatter: (cell: any) => {
                const isResuelto = cell.getValue();
                if (isResuelto) {
                    return '<span class="px-2 py-1 rounded-full bg-emerald-500/10 text-emerald-400 text-xs font-bold uppercase border border-emerald-500/20">Resuelto</span>';
                } else {
                    return '<span class="px-2 py-1 rounded-full bg-red-500/10 text-red-400 text-xs font-bold uppercase border border-red-500/20 animate-pulse">Pendiente</span>';
                }
            }
        },
        {
            title: 'Reportado por',
            field: 'reportadoPorNombre',
            width: 180,
            hozAlign: 'left'
        },
        {
            title: 'Observación',
            field: 'notas',
            width: 300,
            formatter: 'textarea'
        },
        {
            title: 'Acciones',
            field: 'actions',
            width: 120,
            hozAlign: 'center',
            headerSort: false,
            formatter: (cell: any) => {
                const data = cell.getData();
                if (data.resuelto) return '';

                const btn = document.createElement('button');
                btn.className = 'px-3 py-1 bg-accent/20 text-accent border border-accent/30 rounded-md hover:bg-accent/30 transition-colors text-xs font-bold uppercase';
                btn.innerHTML = 'Resolver';
                btn.onclick = () => callbacks.onResolve?.(data);
                return btn;
            }
        }
    ];
}
