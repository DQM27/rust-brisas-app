import type { ColumnDefinition } from 'tabulator-tables';
import type { ListaNegraResponse } from '$lib/types/listaNegra';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export const getListaNegraColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			frozen: true,
			headerFilter: 'input',
			formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombreCompleto',
			minWidth: 200,
			headerFilter: 'input',
			formatter: (cell) => {
				const data = cell.getData() as any;
				const nombre = data.nombreCompleto || data.nombre_completo || `${data.nombre} ${data.apellido}`;
				return `<span style="font-weight:500; color:#e2e8f0">${nombre || ''}</span>`;
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			headerFilter: 'input',
			minWidth: 150,
			formatter: (cell) => {
				const data = cell.getData() as any;
				return `<span style="color:#9ca3af">${data.empresaNombre || data.empresa_nombre || 'Sin empresa'}</span>`;
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
				if (nivel === 'ALTO') return createGridBadge({ text: 'ALTO', color: 'red' });
				if (nivel === 'MEDIO') return createGridBadge({ text: 'MEDIO', color: 'amber' });
				if (nivel === 'BAJO') return createGridBadge({ text: 'BAJO', color: 'blue' });
				return createGridBadge({ text: nivel || 'N/A', color: 'gray' });
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
				if (isActive) {
					return createGridBadge({ text: 'Bloqueado', color: 'red', withDot: true });
				} else {
					return createGridBadge({ text: 'Desbloqueado', color: 'green', withCheck: true });
				}
			}
		},
		{
			title: 'Motivo',
			field: 'motivoBloqueo',
			minWidth: 200,
			formatter: (cell) => {
				const val = cell.getValue() || (cell.getData() as any).motivo_bloqueo || 'Sin motivo';
				return `<span style="color:#9ca3af" class="truncate block max-w-xs" title="${val}">${val}</span>`;
			}
		},
		{
			title: 'Bloqueado Por',
			field: 'bloqueadoPorNombre',
			width: 160,
			formatter: (cell) => {
				const data = cell.getData() as any;
				const nombre = data.bloqueadoPorNombre || data.bloqueado_por_nombre || data.bloqueadoPor || 'Sistema';
				return `<span style="color:#9ca3af">${nombre}</span>`;
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
