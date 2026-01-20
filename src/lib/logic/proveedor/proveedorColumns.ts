// src/lib/logic/proveedor/proveedorColumns.ts
import type { ColumnDefinition } from 'tabulator-tables';
import type { ProveedorResponse } from '$lib/types/proveedor';

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
				return [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
					.filter(Boolean)
					.join(' ');
			}
		},
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			frozen: true,
			headerFilter: 'input',
			formatter: (cell) => `<span class="font-mono text-xs">${cell.getValue() || ''}</span>`
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
				const baseClass = 'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none cursor-pointer hover:opacity-80 transition-opacity';

				const badges: Record<string, string> = {
					activo: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20',
					inactivo: 'bg-gray-500/10 text-gray-400 border-gray-500/20',
					suspendido: 'bg-red-500/10 text-red-400 border-red-500/20'
				};

				const badgeClass = badges[estado] || badges.inactivo;
				const displayText = estado.toUpperCase();
				return `<button class="status-btn ${baseClass} ${badgeClass}">${displayText}</button>`;
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
				const baseClass = 'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none';
				const redBadge = 'bg-red-500/10 text-red-400 border-red-500/20';
				const greenBadge = 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';

				if (row.estado?.toLowerCase() !== 'activo') {
					return `<span class="${baseClass} ${redBadge}">Denegado</span>`;
				}

				return row.puedeIngresar
					? `<span class="${baseClass} ${greenBadge}">Permitido</span>`
					: `<span class="${baseClass} ${redBadge}">Denegado</span>`;
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
			formatter: (cell) => `<span class="font-mono text-xs">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre',
			field: 'nombre',
			width: 250,
			formatter: (cell) => {
				const d = cell.getData() as ProveedorResponse;
				if (!d) return '';
				return [d.nombre, d.segundoNombre, d.apellido, d.segundoApellido]
					.filter(Boolean)
					.join(' ');
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
