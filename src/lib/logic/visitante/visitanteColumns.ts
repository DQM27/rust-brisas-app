import type { ColumnDefinition } from 'tabulator-tables';
import type { VisitanteResponse } from '$lib/types/visitante';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export const getVisitanteColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 140,
			headerFilter: 'input',
			frozen: true,
			formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombre',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => {
				const data = cell.getData() as VisitanteResponse;
				return `<span style="font-weight:500; color:#e2e8f0">${data.nombre} ${data.apellido || ''}</span>`;
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => cell.getValue() || '<span class="text-secondary italic">N/A</span>'
		},
		{
			title: 'Vehículo',
			field: 'hasVehicle',
			width: 100,
			hozAlign: 'center',
			formatter: (cell) => {
				return cell.getValue()
					? createGridBadge({ text: 'Sí', color: 'blue' })
					: createGridBadge({ text: 'No', color: 'gray' });
			}
		},
		{
			title: 'Fecha Registro',
			field: 'createdAt',
			width: 180,
			headerFilter: 'date',
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '';
				return new Date(val).toLocaleString('es-ES', {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric',
					hour: '2-digit',
					minute: '2-digit'
				});
			}
		}
	];
};

export const getVisitanteTrashColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 140,
			formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombreCompleto',
			width: 250,
			formatter: (cell) => {
				const data = cell.getData() as VisitanteResponse;
				return `<span style="font-weight:500; color:#e2e8f0">${data.nombre} ${data.apellido || ''}</span>`;
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 200
		},
		{
			title: 'Fecha Eliminación',
			field: 'deletedAt',
			width: 180,
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '-';
				return new Date(val).toLocaleString('es-ES', {
					day: '2-digit',
					month: '2-digit',
					year: 'numeric',
					hour: '2-digit',
					minute: '2-digit'
				});
			}
		}
	];
};
