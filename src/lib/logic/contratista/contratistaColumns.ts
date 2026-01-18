import type { ColumnDefinition } from 'tabulator-tables';
import type { ContratistaResponse, EstadoContratista } from '$lib/types/contratista';

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
		{ title: 'ID', field: 'id', visible: false },
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			headerFilter: 'input',
			visible: true,
			formatter: (cell) => `<span style="font-family:monospace; font-size:13px">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Nombre Completo',
			field: 'nombreCompleto',
			width: 220,
			headerFilter: 'input',
			visible: true,
			formatter: (cell) => `<span style="font-weight:500; color:#e2e8f0">${cell.getValue() || ''}</span>`
		},
		{ title: 'Empresa', field: 'empresaNombre', width: 180, headerFilter: 'input', visible: true },
		{ title: 'Vehículo', field: 'vehiculoTipo', width: 120, visible: true },
		{
			title: 'Placa',
			field: 'vehiculoPlaca',
			width: 110,
			visible: true,
			formatter: (cell) => `<span style="font-family:monospace">${cell.getValue() || '-'}</span>`
		},
		{
			title: 'Estado',
			field: 'estado',
			width: 130,
			hozAlign: 'center',
			visible: true,
			formatter: (cell) => {
				const val = cell.getValue() as EstadoContratista;
				const badges: Record<string, string> = {
					activo: 'bg-green-500/10 text-green-400 border-green-500/20 hover:bg-green-500/20',
					inactivo: 'bg-gray-500/10 text-gray-400 border-gray-500/20 hover:bg-gray-500/20',
					suspendido: 'bg-red-500/10 text-red-400 border-red-500/20 hover:bg-red-500/20'
				};
				const color = badges[val] || badges.inactivo;
				return `<button class="status-btn px-2.5 py-0.5 rounded-full text-xs font-medium border ${color} transition-colors cursor-pointer">${val?.toUpperCase() || 'N/A'}</button>`;
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
				if (row.praindVencido) {
					return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">VENCIDO</span>`;
				} else if (row.diasHastaVencimiento <= 30) {
					return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-500/10 text-amber-400 border border-amber-500/20">${row.diasHastaVencimiento} Días</span>`;
				}
				return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-500/10 text-green-400 border border-green-500/20">VIGENTE</span>`;
			}
		},
		{
			title: 'Vencimiento',
			field: 'fechaVencimientoPraind',
			width: 130,
			visible: true,
			formatter: (cell) => {
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
				const row = cell.getRow().getData() as ContratistaResponse;
				if (row.estaBloqueado) return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">BLOQUEADO</span>`;
				if (row.estado !== 'activo') return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">DENEGADO</span>`;
				if (row.puedeIngresar) return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-500/10 text-green-400 border border-green-500/20">PERMITIDO</span>`;
				return `<span class="px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-500/10 text-red-400 border border-red-500/20">DENEGADO</span>`;
			}
		},
		{
			title: 'Acciones',
			width: 140,
			headerSort: false,
			hozAlign: 'center',
			frozen: true,
			visible: true,
			formatter: () => {
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
