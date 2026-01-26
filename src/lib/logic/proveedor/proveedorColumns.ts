import type { ColumnDefinition } from 'tabulator-tables';
import type { ProveedorResponse } from '$lib/types/proveedor';
import { createGridBadge, type BadgeColor } from '$lib/components/tabulator/gridBadge';

export interface ProveedorColumnHandlers {
	onStatusToggle: (id: string, currentStatus: string) => void;
}

export const getProveedorColumns = (handlers: ProveedorColumnHandlers): ColumnDefinition[] => {
	return [
		{
			title: 'Nombre Completo',
			field: 'nombre',
			width: 250,
			headerFilter: 'input',
			formatter: (cell) => {
				const d = cell.getData() as ProveedorResponse;
				if (!d) return '';
				const nombre = [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
					.filter(Boolean)
					.join(' ');
				return `<span style="font-weight:500; color:#e2e8f0">${nombre}</span>`;
			}
		},
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			frozen: true,
			headerFilter: 'input',
			formatter: (cell) =>
				`<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 200,
			headerFilter: 'input'
		},
		{
			title: 'Vehículo',
			field: 'vehiculoTipo',
			width: 120,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Placa',
			field: 'vehiculoPlaca',
			width: 100,
			headerFilter: 'input',
			formatter: (cell) => `<span class="font-mono">${cell.getValue() || '-'}</span>`
		},
		{
			title: 'Estado',
			field: 'estado',
			width: 130,
			hozAlign: 'center',
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const estado = (cell.getValue() || 'INACTIVO').toLowerCase();
				const colorMap: Record<string, BadgeColor> = {
					activo: 'green',
					inactivo: 'gray',
					suspendido: 'red'
				};
				return createGridBadge({
					text: estado,
					color: colorMap[estado] || 'gray',
					isButton: true,
					className: 'status-btn'
				});
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				if (target.classList.contains('status-btn')) {
					const data = cell.getData() as ProveedorResponse;
					handlers.onStatusToggle(data.id, data.estado);
				}
			}
		},
		{
			title: 'Acceso',
			field: 'puedeIngresar',
			width: 130,
			hozAlign: 'center',
			formatter: (cell) => {
				const row = cell.getData() as ProveedorResponse;
				if (row.estado?.toLowerCase() !== 'activo') {
					return createGridBadge({ text: 'Denegado', color: 'red' });
				}

				return row.puedeIngresar
					? createGridBadge({ text: 'Permitido', color: 'green' })
					: createGridBadge({ text: 'Denegado', color: 'red' });
			}
		}
	];
};

export const getProveedorTrashColumns = (): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			formatter: (cell) =>
				`<span style="font-family:monospace; font-size:13px; color:#f3f4f6">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombre',
			width: 250,
			formatter: (cell) => {
				const d = cell.getData() as ProveedorResponse;
				if (!d) return '';
				const nombre = [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
					.filter(Boolean)
					.join(' ');
				return `<span style="font-weight:500; color:#e2e8f0">${nombre}</span>`;
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
			width: 150,
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '-';
				return new Date(val).toLocaleDateString('es-PA', {
					year: 'numeric',
					month: '2-digit',
					day: '2-digit',
					hour: '2-digit',
					minute: '2-digit'
				});
			}
		}
	];
};
