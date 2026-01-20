import type { ColumnDefinition } from 'tabulator-tables';
import type { GafeteResponse } from '$lib/types/gafete';

export interface GafeteColumnHandlers {
	onResolve: (data: GafeteResponse) => void;
	onRecover: (data: GafeteResponse) => void;
	onLost: (data: GafeteResponse) => void;
	onDamage: (data: GafeteResponse) => void;
	onDelete: (data: GafeteResponse) => void;
	onEdit: (data: GafeteResponse) => void;
}

export const getGafeteColumns = (handlers: GafeteColumnHandlers): ColumnDefinition[] => {
	return [
		{
			title: 'Número',
			field: 'numero',
			width: 100,
			headerFilter: 'input',
			frozen: true,
			formatter: (cell) => `<span class="font-mono font-bold text-white text-sm tracking-widest">${cell.getValue()}</span>`
		},
		{
			title: 'Tipo',
			field: 'tipoDisplay',
			width: 130,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const tipo = (cell.getData() as GafeteResponse).tipo;
				const baseClass = 'inline-flex items-center px-2 py-0.5 rounded text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';

				const types: Record<string, string> = {
					contratista: 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20',
					proveedor: 'bg-amber-500/10 text-amber-400 border-amber-500/20',
					visita: 'bg-violet-500/10 text-violet-400 border-violet-500/20'
				};

				const colorClass = types[tipo] || 'bg-gray-500/10 text-gray-400 border-gray-500/20';
				return `<span class="${baseClass} ${colorClass}">${cell.getValue()}</span>`;
			}
		},
		{
			title: 'Estado',
			field: 'status',
			width: 150,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const status = cell.getValue();
				const baseClass = 'inline-flex items-center px-2.5 py-1 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none';

				let classes = '';
				let icon = '';
				let label = '';

				switch (status) {
					case 'disponible':
					case 'activo':
						classes = 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20';
						icon = '●';
						label = 'Disponible';
						break;
					case 'en_uso':
						classes = 'bg-blue-500/10 text-blue-400 border-blue-500/20';
						icon = '○';
						label = 'En Uso';
						break;
					case 'perdido':
						classes = 'bg-red-500/10 text-red-400 border-red-500/20';
						icon = '⚠';
						label = 'Perdido';
						break;
					case 'danado':
						classes = 'bg-rose-500/10 text-rose-400 border-rose-500/20';
						icon = '⚡';
						label = 'Dañado';
						break;
					case 'extraviado':
						classes = 'bg-amber-500/10 text-amber-400 border-amber-500/20';
						icon = '?';
						label = 'Extraviado';
						break;
					default:
						classes = 'bg-gray-500/10 text-gray-400 border-gray-500/20';
						icon = '-';
						label = status;
				}

				return `<span class="${baseClass} ${classes}"><span class="mr-1.5 opacity-70">${icon}</span>${label}</span>`;
			}
		},
		{
			title: 'Fecha Reporte',
			field: 'fechaPerdido',
			width: 150,
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return '-';
				return new Date(val).toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' });
			}
		},
		{
			title: 'Persona reporte',
			field: 'quienPerdio',
			width: 180,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Reportado Por',
			field: 'reportadoPorNombre',
			width: 160,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Resolución',
			field: 'resueltoPorNombre',
			width: 150,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Notas',
			field: 'notas',
			width: 200,
			formatter: (cell) => cell.getValue() || '-'
		},
		{
			title: 'Acciones',
			field: 'acciones',
			width: 220,
			frozen: false,
			hozAlign: 'right',
			headerSort: false,
			formatter: (cell) => {
				const data = cell.getData() as GafeteResponse;
				const status = data.status;
				let buttons = '<div class="flex items-center justify-end gap-1.5">';

				if (status === 'perdido') {
					buttons += `<button class="action-btn resolve-btn px-2 py-1 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded hover:bg-emerald-500/20 text-[10px] font-bold uppercase transition-colors">Resolver</button>`;
				}

				if (status !== 'perdido') {
					buttons += `<button class="action-btn edit-btn p-1.5 text-gray-400 hover:text-white transition-colors" title="Editar">✏️</button>`;

					if (status === 'extraviado') {
						buttons += `<button class="action-btn recover-btn px-2 py-1 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded hover:bg-emerald-500/20 text-[10px] font-bold uppercase transition-colors">Recuperar</button>`;
					} else if (status !== 'danado') {
						buttons += `<button class="action-btn lost-btn px-2 py-1 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded hover:bg-amber-500/20 text-[10px] font-bold uppercase transition-colors" title="Marcar como Extraviado">?</button>`;
						buttons += `<button class="action-btn damage-btn px-2 py-1 bg-rose-500/10 text-rose-400 border border-rose-500/20 rounded hover:bg-rose-500/20 text-[10px] font-bold uppercase transition-colors" title="Marcar como Dañado">⚡</button>`;
					}

					if (status === 'danado') {
						buttons += `<button class="action-btn recover-btn px-2 py-1 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded hover:bg-emerald-500/20 text-[10px] font-bold uppercase transition-colors" title="Reparado">✓ Rep</button>`;
					}

					if (status === 'danado' || status === 'disponible') {
						buttons += `<button class="action-btn delete-btn p-1.5 text-gray-400 hover:text-red-400 transition-colors" title="Eliminar">🗑️</button>`;
					}
				}

				buttons += '</div>';
				return buttons;
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				const data = cell.getData() as GafeteResponse;

				if (target.classList.contains('resolve-btn')) handlers.onResolve(data);
				if (target.classList.contains('edit-btn')) handlers.onEdit(data);
				if (target.classList.contains('recover-btn')) handlers.onRecover(data);
				if (target.classList.contains('lost-btn')) handlers.onLost(data);
				if (target.classList.contains('damage-btn')) handlers.onDamage(data);
				if (target.classList.contains('delete-btn')) handlers.onDelete(data);
			}
		}
	];
};
