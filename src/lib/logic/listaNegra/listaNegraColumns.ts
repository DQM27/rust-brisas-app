import type { ColumnDefinition } from 'tabulator-tables';
import type { ListaNegraResponse } from '$lib/types/listaNegra';

export const getListaNegraColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			frozen: true,
			headerFilter: 'input',
			formatter: (cell) => `<span class="font-mono text-sm text-gray-100">${cell.getValue()}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombreCompleto',
			minWidth: 200,
			headerFilter: 'input',
			formatter: (cell) => {
				const data = cell.getData() as any;
				const nombre = data.nombreCompleto || data.nombre_completo || `${data.nombre} ${data.apellido}`;
				return `<span class="font-medium text-white">${nombre}</span>`;
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			headerFilter: 'input',
			minWidth: 150,
			formatter: (cell) => {
				const data = cell.getData() as any;
				return `<span>${data.empresaNombre || data.empresa_nombre || 'Sin empresa'}</span>`;
			}
		},
		{
			title: 'Nivel',
			field: 'nivelSeveridad',
			width: 100,
			hozAlign: 'center',
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const nivel = cell.getValue();
				const baseClass = 'inline-flex items-center px-2 py-0.5 rounded text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';

				switch (nivel) {
					case 'ALTO':
						return `<span class="${baseClass} bg-red-500/10 text-red-400 border-red-500/20">ALTO</span>`;
					case 'MEDIO':
						return `<span class="${baseClass} bg-amber-500/10 text-amber-400 border-amber-500/20">MEDIO</span>`;
					case 'BAJO':
						return `<span class="${baseClass} bg-blue-500/10 text-blue-400 border-blue-500/20">BAJO</span>`;
					default:
						return `<span class="${baseClass} bg-gray-500/10 text-gray-400 border-gray-500/20">${nivel || 'N/A'}</span>`;
				}
			}
		},
		{
			title: 'Estado',
			field: 'isActive',
			width: 130,
			headerFilter: 'list',
			headerFilterParams: {
				values: { true: 'Seleccionado', false: 'No Seleccionado' },
				clearable: true
			},
			formatter: (cell) => {
				const isActive = cell.getValue();
				const baseClass = 'inline-flex items-center px-2.5 py-1 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none';

				if (isActive) {
					return `<span class="${baseClass} bg-red-500/10 text-red-400 border-red-500/20"><span class="mr-1.5 opacity-70">●</span>Bloqueado</span>`;
				} else {
					return `<span class="${baseClass} bg-emerald-500/10 text-emerald-400 border-emerald-500/20"><span class="mr-1.5 opacity-70">✓</span>Desbloqueado</span>`;
				}
			}
		},
		{
			title: 'Motivo',
			field: 'motivoBloqueo',
			minWidth: 200,
			formatter: (cell) => {
				const val = cell.getValue() || (cell.getData() as any).motivo_bloqueo || 'Sin motivo';
				return `<span class="text-xs text-gray-400 truncate block max-w-xs" title="${val}">${val}</span>`;
			}
		},
		{
			title: 'Bloqueado Por',
			field: 'bloqueadoPorNombre',
			width: 160,
			formatter: (cell) => {
				const data = cell.getData() as any;
				const nombre = data.bloqueadoPorNombre || data.bloqueado_por_nombre || data.bloqueadoPor || 'Sistema';
				return `<span class="text-xs text-secondary italic">${nombre}</span>`;
			}
		},
		{
			title: 'Fecha',
			field: 'createdAt',
			width: 120,
			formatter: (cell) => {
				const val = cell.getValue() || (cell.getData() as any).created_at;
				if (!val) return '-';
				const date = new Date(val);
				return `<span>${date.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' })}</span>`;
			}
		}
	];
};
