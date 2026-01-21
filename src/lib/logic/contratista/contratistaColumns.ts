import type { ColumnDefinition } from 'tabulator-tables';
import type { ContratistaResponse, EstadoContratista } from '$lib/types/contratista';
import { createGridBadge, type BadgeColor } from '$lib/components/tabulator/gridBadge';

export interface ContratistaColumnHandlers {
	onStatusChange: (id: string, currentStatus: string) => void;
	onEdit: (contratista: any) => void;
	onDelete: (contratista: any) => void;
	onVehiculoClick: (contratista: any) => void;
}

// Icons (Lucide SVG strings)
const icons = {
	edit: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pencil"><path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/></svg>`,
	trash: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash-2"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/></svg>`,
	car: `<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-car"><path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"/><circle cx="7" cy="17" r="2"/><path d="M9 17h6"/><circle cx="17" cy="17" r="2"/></svg>`
};

export const getContratistaColumns = (handlers: ContratistaColumnHandlers): ColumnDefinition[] => {
	return [
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			headerFilter: 'input',
			visible: true,
			formatter: (cell) => `<span class="cell-code">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombreCompleto',
			width: 220,
			headerFilter: 'input',
			visible: true,
			formatter: (cell) => `<span class="cell-name">${cell.getValue() || ''}</span>`
		},
		{ title: 'Empresa', field: 'empresaNombre', width: 180, headerFilter: 'input', visible: true },
		{ title: 'Vehículo', field: 'vehiculoTipo', width: 120, visible: true },
		{
			title: 'Placa',
			field: 'vehiculoPlaca',
			width: 110,
			visible: true,
			formatter: (cell) => `<span class="cell-code">${cell.getValue() || '-'}</span>`
		},
		{
			title: 'Estado',
			field: 'estado',
			width: 130,
			hozAlign: 'center',
			visible: true,
			formatter: (cell) => {
				const data = cell.getData() as ContratistaResponse;
				if (!data.cedula) return '';
				const val = cell.getValue() as EstadoContratista;
				const colorMap: Record<string, BadgeColor> = {
					activo: 'green',
					inactivo: 'gray',
					suspendido: 'red'
				};
				return createGridBadge({
					text: val || 'N/A',
					color: colorMap[val] || 'gray',
					isButton: true,
					className: 'status-btn'
				});
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				if (target.closest('.status-btn')) {
					const data = cell.getRow().getData();
					handlers.onStatusChange(data.id, data.estado);
				}
			}
		},
		{
			title: 'PRAIND',
			field: 'praindVencido',
			width: 130,
			hozAlign: 'center',
			visible: true,
			formatter: (cell) => {
				const row = cell.getRow().getData() as ContratistaResponse;
				if (!row.cedula) return '';
				if (row.praindVencido) {
					return createGridBadge({ text: 'VENCIDO', color: 'red' });
				} else if (row.diasHastaVencimiento <= 30) {
					return createGridBadge({ text: `${row.diasHastaVencimiento} Días`, color: 'amber' });
				}
				return createGridBadge({ text: 'VIGENTE', color: 'green' });
			}
		},
		{
			title: 'Vencimiento',
			field: 'fechaVencimientoPraind',
			width: 130,
			visible: true,
			formatter: (cell) => {
				const data = cell.getData() as ContratistaResponse;
				if (!data.cedula) return '';
				const val = cell.getValue();
				if (!val) return '';
				return new Date(val).toLocaleDateString('es-PA', { day: 'numeric', month: 'short', year: 'numeric' });
			}
		},
		{
			title: 'Acceso',
			field: 'puedeIngresar',
			width: 130,
			hozAlign: 'center',
			visible: true,
			formatter: (cell) => {
				// Don't show access badge for vehicles (children)
				const row = cell.getRow().getData() as ContratistaResponse;
				if (!row.cedula) return '';
				if (row.estaBloqueado) return createGridBadge({ text: 'BLOQUEADO', color: 'red' });
				if (row.estado !== 'activo') return createGridBadge({ text: 'DENEGADO', color: 'red' });
				if (row.puedeIngresar) return createGridBadge({ text: 'PERMITIDO', color: 'green' });
				return createGridBadge({ text: 'DENEGADO', color: 'red' });
			}
		},
		{
			title: 'Acciones',
			width: 140,
			headerSort: false,
			hozAlign: 'center',
			visible: true,
			formatter: (cell) => {
				const data = cell.getData() as ContratistaResponse;

				// Vehicle Actions (Child Row)
				if (!data.cedula) {
					return `
						<div class="flex items-center justify-center gap-1">
							<button class="edit-btn p-1.5 hover:bg-blue-500/20 rounded text-blue-400 transition-colors" title="Editar Vehículo">${icons.edit}</button>
							<button class="delete-btn p-1.5 hover:bg-red-500/20 rounded text-red-400 transition-colors" title="Eliminar Vehículo">${icons.trash}</button>
						</div>
					`;
				}

				// Contractor Actions (Parent Row)
				return `
                    <div class="flex items-center justify-center gap-1">
                        <button class="edit-btn p-1.5 hover:bg-blue-500/20 rounded text-blue-400 transition-colors" title="Editar">${icons.edit}</button>
                        <button class="car-btn p-1.5 hover:bg-amber-500/20 rounded text-amber-400 transition-colors" title="Vehículos">${icons.car}</button>
                        <button class="delete-btn p-1.5 hover:bg-red-500/20 rounded text-red-400 transition-colors" title="Eliminar">${icons.trash}</button>
                    </div>
                `;
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				const btn = target.closest('button');
				if (!btn) return;

				const data = cell.getRow().getData();

				if (btn.classList.contains('edit-btn')) {
					handlers.onEdit(data);
				} else if (btn.classList.contains('car-btn')) {
					handlers.onVehiculoClick(data);
				} else if (btn.classList.contains('delete-btn')) {
					handlers.onDelete(data);
				}
			}
		}
	];
};

export const getContratistaTrashColumns = (): ColumnDefinition[] => {
	return [
		{ title: 'ID', field: 'id', visible: false },
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			visible: true,
			formatter: (cell) => `<span class="cell-code">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombreCompleto',
			width: 220,
			visible: true,
			formatter: (cell) => `<span class="cell-name">${cell.getValue() || ''}</span>`
		},
		{ title: 'Empresa', field: 'empresaNombre', width: 180, visible: true },
		{
			title: 'Fecha Eliminación',
			field: 'deletedAt',
			width: 160,
			visible: true,
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '-';
				return new Date(val).toLocaleDateString('es-PA', {
					year: 'numeric',
					month: 'short',
					day: 'numeric',
					hour: '2-digit',
					minute: '2-digit'
				});
			}
		}
	];
};
