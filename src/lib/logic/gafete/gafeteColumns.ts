import type { ColumnDefinition } from 'tabulator-tables';
import type { GafeteResponse } from '$lib/types/gafete';
import { createGridBadge, type BadgeColor } from '$lib/components/tabulator/gridBadge';

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
			formatter: (cell) => `<span style="font-family:monospace; font-size:13px; color:#f3f4f6; font-weight:700; letter-spacing:0.05em;">${cell.getValue()}</span>`
		},
		{
			title: 'Tipo',
			field: 'tipoDisplay',
			width: 130,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => {
				const tipo = (cell.getData() as GafeteResponse).tipo;
				const types: Record<string, BadgeColor> = {
					contratista: 'blue',
					proveedor: 'amber',
					visita: 'blue'
				};
				return createGridBadge({
					text: cell.getValue(),
					color: types[tipo] || 'gray'
				});
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
				switch (status) {
					case 'disponible':
					case 'activo':
						return createGridBadge({ text: 'Disponible', color: 'green', withDot: true });
					case 'en_uso':
						return createGridBadge({ text: 'En Uso', color: 'blue', withDot: true });
					case 'perdido':
						return createGridBadge({ text: 'Perdido', color: 'red', withDot: true });
					case 'danado':
						return createGridBadge({ text: 'Dañado', color: 'red', withDot: true });
					case 'extraviado':
						return createGridBadge({ text: 'Extraviado', color: 'amber', withDot: true });
					default:
						return createGridBadge({ text: status || 'N/A', color: 'gray' });
				}
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
					buttons += createGridBadge({ text: 'Resolver', color: 'green', isButton: true, className: 'resolve-btn' });
				}

				if (status !== 'perdido') {
					buttons += `<button class="action-btn edit-btn p-1.5 text-gray-400 hover:text-white transition-colors" title="Editar">✏️</button>`;

					if (status === 'extraviado') {
						buttons += createGridBadge({ text: 'Recuperar', color: 'green', isButton: true, className: 'recover-btn' });
					} else if (status !== 'danado') {
						buttons += createGridBadge({ text: '?', color: 'amber', isButton: true, className: 'lost-btn' });
						buttons += createGridBadge({ text: '⚡', color: 'red', isButton: true, className: 'damage-btn' });
					}

					if (status === 'danado') {
						buttons += createGridBadge({ text: '✓ Rep', color: 'green', isButton: true, className: 'recover-btn' });
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
