// src/lib/logic/user/userColumns.ts
import type { ColumnDefinition } from 'tabulator-tables';
import type { UserResponse } from '$lib/types/user';

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
			formatter: (cell) => `<span class="font-mono text-xs">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombre',
			width: 250,
			headerFilter: 'input',
			formatter: (cell) => {
				const user = cell.getData() as UserResponse;
				if (!user) return '';
				return [user.nombre, user.segundoNombre, user.apellido, user.segundoApellido]
					.filter(Boolean)
					.join(' ');
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
				const role = cell.getValue();
				const baseClass = 'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none';
				const badges: Record<string, string> = {
					admin: 'bg-purple-500/10 text-purple-400 border-purple-500/20',
					supervisor: 'bg-blue-500/10 text-blue-400 border-blue-500/20',
					guardia: 'bg-gray-500/10 text-gray-400 border-gray-500/20'
				};
				const badgeClass = badges[role] || badges.guardia;
				const displayText = role ? role.toUpperCase() : 'N/A';
				return `<span class="${baseClass} ${badgeClass}">${displayText}</span>`;
			}
		},
		{
			title: 'Estado',
			field: 'isActive',
			width: 130,
			hozAlign: 'center',
			headerFilter: 'list',
			headerFilterParams: {
				values: { "true": "Activo", "false": "Inactivo" },
				clearable: true
			},
			formatter: (cell) => {
				const isActive = cell.getValue();
				const baseClass = 'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none cursor-pointer hover:opacity-80 transition-opacity';
				const activeBadge = 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
				const inactiveBadge = 'bg-red-500/10 text-red-400 border-red-500/20';
				const badgeClass = isActive ? activeBadge : inactiveBadge;
				const displayText = isActive ? 'Activo' : 'Inactivo';
				return `<button class="status-btn ${baseClass} ${badgeClass}">${displayText}</button>`;
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
