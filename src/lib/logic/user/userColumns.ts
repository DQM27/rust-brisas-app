import type { ColumnDefinition } from 'tabulator-tables';
import type { UserResponse } from '$lib/types/user';
import { createGridBadge, type BadgeColor } from '$lib/components/tabulator/gridBadge';

export interface UserColumnHandlers {
	onStatusToggle: (id: string, currentStatus: boolean) => void;
}

export const getUserColumns = (handlers: UserColumnHandlers): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			frozen: true,
			headerFilter: 'input',
			formatter: (cell) => `<span class="cell-code">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombre',
			width: 250,
			headerFilter: 'input',
			formatter: (cell) => {
				const user = cell.getData() as UserResponse;
				if (!user) return '';
				const nombre = [user.nombre, user.segundoNombre, user.apellido, user.segundoApellido]
					.filter(Boolean)
					.join(' ');
				return `<span class="cell-name">${nombre}</span>`;
			}
		},
		{
			title: 'Email',
			field: 'email',
			width: 250,
			headerFilter: 'input'
		},
		{
			title: 'Rol',
			field: 'roleName',
			width: 130,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const role = (cell.getValue() || '').toLowerCase();
				const colors: Record<string, BadgeColor> = {
					admin: 'blue',
					supervisor: 'amber',
					guardia: 'gray'
				};
				return createGridBadge({
					text: role.toUpperCase() || 'N/A',
					color: colors[role] || 'gray'
				});
			}
		},
		{
			title: 'Estado',
			field: 'isActive',
			width: 130,
			hozAlign: 'center',
			headerFilter: 'list',
			headerFilterParams: {
				values: { true: 'Activo', false: 'Inactivo' },
				clearable: true
			},
			formatter: (cell) => {
				const isActive = cell.getValue();
				return createGridBadge({
					text: isActive ? 'Activo' : 'Inactivo',
					color: isActive ? 'green' : 'red',
					isButton: true,
					className: 'status-btn'
				});
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				if (target.classList.contains('status-btn')) {
					const row = cell.getData() as UserResponse;
					handlers.onStatusToggle(row.id, row.isActive);
				}
			}
		},
		{
			title: 'Teléfono',
			field: 'telefono',
			width: 140,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Gafete',
			field: 'numeroGafete',
			width: 110,
			formatter: (cell) => `<span class="font-mono">${cell.getValue() || '-'}</span>`
		},
		{
			title: 'Fecha Inicio',
			field: 'fechaInicioLabores',
			width: 130,
			formatter: (cell) => formatDate(cell.getValue())
		},
		{
			title: 'Operación',
			field: 'operacion',
			width: 120,
			visible: false
		},
		{
			title: 'Venc. Portación',
			field: 'vencimientoPortacion',
			width: 130,
			visible: false,
			formatter: (cell) => formatDate(cell.getValue())
		},
		{
			title: 'Fecha Nacimiento',
			field: 'fechaNacimiento',
			width: 130,
			visible: false,
			formatter: (cell) => formatDate(cell.getValue())
		},
		{
			title: 'Dirección',
			field: 'direccion',
			width: 200,
			visible: false,
			formatter: (cell) => `<div class="whitespace-normal">${cell.getValue() || '-'}</div>`
		},
		{
			title: 'Contacto Emergencia',
			field: 'contactoEmergenciaNombre',
			width: 160,
			visible: false
		},
		{
			title: 'Tel. Emergencia',
			field: 'contactoEmergenciaTelefono',
			width: 140,
			visible: false,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Creado',
			field: 'createdAt',
			width: 150,
			visible: false,
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '-';
				const date = new Date(val);
				return date.toLocaleString('es-PA', {
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

function formatDate(value: any): string {
	if (!value) return '-';
	const val = String(value);
	try {
		const [year, month, day] = val.split('T')[0].split('-').map(Number);
		if (!year || !month || !day) return val;
		return `${day.toString().padStart(2, '0')}/${month.toString().padStart(2, '0')}/${year}`;
	} catch (e) {
		return val;
	}
}
