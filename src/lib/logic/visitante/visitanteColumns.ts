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
			formatter: (cell) =>
				`<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombre',
			width: 250,
			headerFilter: 'input',
			formatter: (cell) => {
				const data = cell.getData() as any;
				// Si es un hijo (vehículo), no mostramos el nombre del visitante
				if (data._isChild) {
					return '';
				}
				return `<span style="font-weight:600; color:#e2e8f0">${data.nombre} ${data.apellido || ''}</span>`;
			}
		},
		{
			title: 'Tipo',
			field: 'vehiculoTipo',
			width: 120,
			formatter: (cell) => {
				const data = cell.getData() as any;
				if (!data.vehiculoTipo) return '';
				const brand = data.vehiculoMarca ? ` - ${data.vehiculoMarca}` : '';
				return `<span class="text-blue-400 capitalize"><i class="lucide-car mr-2"></i>${data.vehiculoTipo}${brand}</span>`;
			}
		},
		{
			title: 'Placa',
			field: 'vehiculoPlaca',
			width: 120,
			formatter: (cell) => {
				const data = cell.getData() as any;
				if (!data.vehiculoPlaca) return '';
				return `<span class="font-mono font-medium text-blue-300 bg-blue-500/10 px-2 py-0.5 rounded border border-blue-500/20">${data.vehiculoPlaca}</span>`;
			}
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => cell.getValue() || ''
		},
		{
			title: 'Vehículo',
			field: 'hasVehicle',
			width: 100,
			hozAlign: 'center',
			formatter: (cell) => {
				const data = cell.getData() as any;
				if (data._isChild) return '';
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
					year: 'numeric'
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
			formatter: (cell) =>
				`<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
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
