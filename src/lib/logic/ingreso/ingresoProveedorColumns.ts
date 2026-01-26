import type { ColumnDefinition } from 'tabulator-tables';
import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export interface IngresoProveedorColumnHandlers {
	onSalida: (ingreso: IngresoProveedor) => void;
}

function parseDate(value: any): Date | null {
	if (!value) return null;
	let dateStr = String(value);
	if (dateStr.startsWith("d'") && dateStr.endsWith("'")) {
		dateStr = dateStr.slice(2, -1);
	}
	const d = new Date(dateStr);
	return isNaN(d.getTime()) ? null : d;
}

export const getIngresoProveedorColumns = (
	handlers: IngresoProveedorColumnHandlers,
	viewMode: 'actives' | 'history'
): ColumnDefinition[] => {
	const cols: ColumnDefinition[] = [
		{
			title: 'Gafete',
			field: 'gafeteNumero',
			width: 90,
			headerFilter: 'input',
			formatter: (cell) => {
				const val = cell.getValue();
				return val
					? `<span style="font-family:monospace; font-size:12px; font-weight:700" class="bg-surface-3 px-1.5 py-0.5 rounded text-accent">${val}</span>`
					: '<span style="font-size:11px" class="text-secondary italic">S/G</span>';
			}
		},
		{
			title: 'Nombre',
			field: 'nombreCompleto',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => {
				return `<span class="font-medium text-primary">${cell.getValue() || ''}</span>`;
			}
		},
		{
			title: 'Cédula',
			field: 'cedula',
			width: 130,
			headerFilter: 'input',
			formatter: (cell) =>
				`<span class="font-mono text-xs text-secondary">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Empresa',
			field: 'empresaNombre',
			width: 180,
			headerFilter: 'input'
		},
		{
			title: 'Modo',
			field: 'modoIngresoDisplay',
			width: 100,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => `<span class="capitalize">${cell.getValue() || ''}</span>`
		},
		{
			title: 'Entrada',
			field: 'fechaHoraIngreso',
			width: 125,
			headerFilter: 'date',
			formatter: (cell) => {
				const d = parseDate(cell.getValue());
				return d
					? d.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' })
					: '-';
			}
		},
		{
			title: 'Hora',
			field: 'fechaHoraIngreso_hora',
			width: 90,
			formatter: (cell) => {
				const d = parseDate(cell.getData().fechaHoraIngreso);
				return d
					? `<span class="font-mono text-primary">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>`
					: '-';
			}
		}
	];

	if (viewMode === 'history') {
		cols.push(
			{
				title: 'Salida',
				field: 'fechaHoraSalida',
				width: 125,
				headerFilter: 'date',
				formatter: (cell) => {
					const d = parseDate(cell.getValue());
					return d
						? d.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' })
						: '-';
				}
			},
			{
				title: 'Hora Salida',
				field: 'fechaHoraSalida_hora',
				width: 100,
				formatter: (cell) => {
					const d = parseDate(cell.getData().fechaHoraSalida);
					return d
						? `<span class="font-mono text-primary">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>`
						: '-';
				}
			},
			{
				title: 'Permanencia',
				field: 'tiempoPermanenciaTexto',
				width: 110,
				formatter: (cell) => {
					const val = cell.getValue();
					return val ? `<span class="text-secondary">${val}</span>` : '-';
				}
			},
			{
				title: 'Área Visitada',
				field: 'areaVisitada',
				width: 150,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Motivo',
				field: 'motivo',
				width: 180,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Registró Ingreso',
				field: 'usuarioIngresoNombre',
				width: 150,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-xs text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Registró Salida',
				field: 'usuarioSalidaNombre',
				width: 150,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-xs text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Observaciones',
				field: 'observaciones',
				width: 200,
				formatter: (cell) => {
					const val = cell.getValue();
					return val ? `<span class="text-xs text-secondary italic">${val}</span>` : '-';
				}
			}
		);
	} else {
		// En modo activos, agregar área y motivo también
		cols.push(
			{
				title: 'Área Visitada',
				field: 'areaVisitada',
				width: 150,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Motivo',
				field: 'motivo',
				width: 180,
				headerFilter: 'input',
				formatter: (cell) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
			},
			{
				title: 'Permanencia',
				field: 'tiempoPermanenciaTexto',
				width: 110,
				formatter: (cell) => {
					const val = cell.getValue();
					return val ? `<span class="text-secondary">${val}</span>` : '-';
				}
			}
		);
	}

	if (viewMode === 'actives') {
		cols.push({
			title: 'Acciones',
			width: 100,
			headerSort: false,
			hozAlign: 'center',
			formatter: () =>
				createGridBadge({
					text: 'Salida',
					color: 'red',
					isButton: true,
					className: 'salida-btn'
				}),
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				const button = target.closest('.salida-btn');
				if (button) {
					handlers.onSalida(cell.getRow().getData() as IngresoProveedor);
				}
			}
		});
	}

	return cols;
};
