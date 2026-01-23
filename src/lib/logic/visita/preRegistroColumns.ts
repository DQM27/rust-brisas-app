import type { ColumnDefinition } from 'tabulator-tables';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export const getPreRegistroColumns = (
    onIngreso: (row: any) => void,
    onDelete: (id: string) => void,
    showFilters: boolean = false
): ColumnDefinition[] => [
        {
            title: 'Nombre Completo',
            field: 'nombre',
            formatter: (cell) => {
                const data = cell.getData();
                return `${data.nombre} ${data.apellido}`;
            },
            widthGrow: 2,
            minWidth: 150,
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Cédula',
            field: 'cedula',
            width: 120,
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Empresa',
            field: 'empresa_nombre',
            widthGrow: 1.5,
            minWidth: 150,
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Anfitrión',
            field: 'anfitrion',
            widthGrow: 1,
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Área',
            field: 'area_visitada',
            widthGrow: 1,
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Fecha E.',
            field: 'fecha_esperada',
            width: 110,
            hozAlign: 'center',
            headerFilter: showFilters ? 'input' : undefined
        },
        {
            title: 'Hora E.',
            field: 'hora_esperada',
            width: 90,
            hozAlign: 'center'
        },
        {
            title: 'Acciones',
            formatter: () => {
                return `
				<div class="flex items-center justify-center gap-2">
					<button class="ingreso-btn bg-blue-500/10 text-blue-500 hover:bg-blue-500/20 p-1 rounded transition-colors" title="Dar Ingreso">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-log-in"><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><polyline points="10 17 15 12 10 7"/><line x1="15" x2="3" y1="12" y2="12"/></svg>
					</button>
					<button class="delete-btn bg-red-500/10 text-red-500 hover:bg-red-500/20 p-1 rounded transition-colors" title="Borrar">
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash-2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/></svg>
					</button>
				</div>
			`;
            },
            width: 100,
            headerSort: false,
            hozAlign: 'center',
            cellClick: (e, cell) => {
                const target = e.target as HTMLElement;
                const btn = target.closest('button');
                if (!btn) return;
                const rowData = cell.getData();

                if (btn.classList.contains('ingreso-btn')) {
                    e.stopImmediatePropagation();
                    onIngreso(rowData);
                } else if (btn.classList.contains('delete-btn')) {
                    e.stopImmediatePropagation();
                    // Normalize ID just in case
                    let id = rowData.id;
                    if (typeof id === 'object' && id?.id?.String) {
                        id = `${id.tb}:${id.id.String}`;
                    }
                    onDelete(id.toString());
                }
            }
        }
    ];
