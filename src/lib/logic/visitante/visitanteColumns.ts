import type { ColumnDefinition } from 'tabulator-tables';
import type { VisitanteResponse } from '$lib/types/visitante';

export const getVisitanteColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 140,
			headerFilter: 'input',
			frozen: true,
			formatter: (cell) => `<span class="font-mono text-xs">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombre',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => {
				const data = cell.getData() as VisitanteResponse;
				return `<span class="font-medium text-white">${data.nombre} ${data.apellido || ''}</span>`;
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
					? '<span class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-blue-500/10 text-blue-400 border border-blue-500/20 uppercase">Sí</span>'
					: '<span class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-gray-500/10 text-gray-400 border border-gray-500/20 uppercase">No</span>';
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
			formatter: (cell) => `<span class="font-mono text-xs">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombreCompleto',
			width: 250,
			formatter: (cell) => {
				const data = cell.getData() as VisitanteResponse;
				return `<span class="font-medium text-white">${data.nombre} ${data.apellido || ''}</span>`;
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
