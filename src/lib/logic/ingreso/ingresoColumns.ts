import type { ColumnDefinition } from 'tabulator-tables';
import type { IngresoResponse } from '$lib/types/ingreso';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

export interface IngresoColumnHandlers {
	onSalida: (ingreso: IngresoResponse) => void;
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

export const getIngresoColumns = (
	handlers: IngresoColumnHandlers,
	viewMode: 'actives' | 'history'
): ColumnDefinition[] => {
	const cols: ColumnDefinition[] = [
		{
			title: 'Gafete',
			field: 'gafeteNumero',
			width: 80,
			headerFilter: 'input',
			formatter: (cell) => {
				const val = cell.getValue();
				return val
					? `<span class="font-mono text-xs bg-surface-3 px-1.5 py-0.5 rounded">${val}</span>`
					: '<span class="text-secondary text-xs italic">S/G</span>';
			}
		},
		{
			title: 'Nombre',
			field: 'nombreCompleto',
			width: 200,
			headerFilter: 'input',
			formatter: (cell) => `<span class="font-medium text-primary">${cell.getValue() || ''}</span>`
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
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true }
		},
		{
			title: 'Autorización',
			field: 'tipoAutorizacionDisplay',
			width: 140,
			headerFilter: 'list',
			headerFilterParams: { valuesLookup: 'active', clearable: true },
			formatter: (cell) => createGridBadge({ text: cell.getValue() || '-', color: 'blue' })
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
			title: 'Fecha Entrada',
			field: 'fechaHoraIngreso', // Usamos el campo base para filtrar mejor
			width: 125,
			headerFilter: 'date',
			headerFilterFunc: (headerValue, rowValue) => {
				if (!headerValue || !rowValue) return true;
				const rowDate = new Date(rowValue).toISOString().split('T')[0];
				return rowDate === headerValue;
			},
			formatter: (cell) => {
				const d = parseDate(cell.getValue());
				return d
					? d.toLocaleDateString('es-PA', { day: '2-digit', month: '2-digit', year: 'numeric' })
					: '-';
			}
		},
		{
			title: 'Hora Entrada',
			field: 'fechaHoraIngreso_hora', // Campo único
			width: 100,
			formatter: (cell) => {
				const d = parseDate(cell.getData().fechaHoraIngreso);
				return d
					? `<span class="font-mono text-primary">${d.toLocaleTimeString('es-PA', { hour: '2-digit', minute: '2-digit', hour12: false })}</span>`
					: '-';
			}
		},
		{
			title: 'Registrado Por',
			field: 'usuarioIngresoNombre',
			width: 150,
			headerFilter: 'input'
		}
	];

	// Optional columns based on viewMode
	if (viewMode === 'history') {
		cols.push(
			{
				title: 'Fecha Salida',
				field: 'fechaHoraSalida',
				width: 125,
				headerFilter: 'date',
				headerFilterFunc: (headerValue, rowValue) => {
					if (!headerValue || !rowValue) return true;
					const rowDate = new Date(rowValue).toISOString().split('T')[0];
					return rowDate === headerValue;
				},
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
				title: 'Salida Por',
				field: 'usuarioSalidaNombre',
				width: 150
			}
		);
	}

	// Calculate and show Time Inside (actives or history)
	cols.push({
		title: 'Tiempo Dentro',
		field: 'tiempoPermanenciaTexto',
		width: 110,
		formatter: (cell) => {
			const data = cell.getData() as IngresoResponse;
			if (data.fechaHoraSalida) return data.tiempoPermanenciaTexto || '-';

			const entrada = parseDate(data.fechaHoraIngreso);
			if (!entrada) return '-';

			const diffMs = new Date().getTime() - entrada.getTime();
			const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
			const diffMins = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
			return `<span class="text-secondary">${diffHours}h ${diffMins}m</span>`;
		}
	});

	// Actions (Active Only)
	if (viewMode === 'actives') {
		cols.push({
			title: 'Acciones',
			width: 90,
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
				if (target.classList.contains('salida-btn')) {
					handlers.onSalida(cell.getRow().getData() as IngresoResponse);
				}
			}
		});
	}

	return cols;
};
