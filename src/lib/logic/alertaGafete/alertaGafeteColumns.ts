import type { ColumnDefinition } from 'tabulator-tables';
import type { AlertaGafeteResponse } from '$lib/types/ingreso';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

function formatDateSlashed(val: string) {
	if (!val) return '-';
	try {
		let cleanVal = String(val);
		if (cleanVal.startsWith("d'") || cleanVal.startsWith('d"')) {
			cleanVal = cleanVal.substring(2, cleanVal.length - 1);
		}
		const date = new Date(cleanVal);
		if (isNaN(date.getTime())) return val;
		return date.toLocaleDateString('es-PA', {
			day: '2-digit',
			month: '2-digit',
			year: 'numeric'
		});
	} catch {
		return val;
	}
}

function formatMilitaryTime(val: string) {
	if (!val) return '-';
	try {
		let cleanVal = String(val);
		if (cleanVal.startsWith("d'") || cleanVal.startsWith('d"')) {
			cleanVal = cleanVal.substring(2, cleanVal.length - 1);
		}
		const date = new Date(cleanVal);
		if (isNaN(date.getTime())) return val;
		return date.toLocaleTimeString('es-PA', {
			hour: '2-digit',
			minute: '2-digit',
			hour12: false
		});
	} catch {
		return val;
	}
}

export function getAlertaGafeteColumns(callbacks: {
	onResolve?: (alerta: AlertaGafeteResponse) => void;
	hideActions?: boolean;
}): ColumnDefinition[] {
	const isHistory = callbacks.hideActions; // hideActions is used as 'isHistory' flag essentially

	const columns: ColumnDefinition[] = [
		{
			title: 'Gafete #',
			field: 'gafeteNumero',
			width: 90,
			hozAlign: 'center',
			headerFilter: 'input',
			formatter: (cell) =>
				`<span class="cell-code font-bold tracking-wide">${cell.getValue()}</span>`
		},
		{
			title: 'Tipo',
			field: 'id',
			width: 120,
			hozAlign: 'center',
			headerSort: false,
			formatter: (cell: any) => {
				const data = cell.getData() as AlertaGafeteResponse;
				if (data.ingresoContratistaId) {
					return createGridBadge({ text: 'Contratista', color: 'blue' });
				}
				if (data.ingresoProveedorId) {
					return createGridBadge({ text: 'Proveedor', color: 'amber' });
				}
				if (data.ingresoVisitaId) {
					return createGridBadge({ text: 'Visita', color: 'blue' });
				}
				return createGridBadge({ text: 'General', color: 'gray' });
			}
		},
		{
			title: 'Persona',
			field: 'nombreCompleto',
			width: 220,
			headerFilter: 'input',
			formatter: (cell: any) => `<span class="text-primary font-medium">${cell.getValue()}</span>`
		},
		{
			title: 'Cédula',
			field: 'cedula',
			width: 110,
			headerFilter: 'input',
			formatter: (cell: any) => `<span class="text-secondary">${cell.getValue() || '-'}</span>`
		},
		{
			title: 'F. Reporte',
			field: 'fechaReporte',
			width: 100,
			hozAlign: 'center',
			formatter: (cell: any) =>
				`<span class="text-secondary text-[11px]">${formatDateSlashed(cell.getValue())}</span>`
		},
		{
			title: 'H. Reporte',
			field: 'fechaReporte',
			width: 80,
			hozAlign: 'center',
			formatter: (cell: any) =>
				`<span class="text-secondary font-medium text-[11px]">${formatMilitaryTime(cell.getValue())}</span>`
		}
	];

	if (isHistory) {
		columns.push(
			{
				title: 'Estado',
				field: 'resuelto',
				width: 110,
				hozAlign: 'center',
				formatter: (cell: any) => {
					const isResuelto = cell.getValue();
					return isResuelto
						? createGridBadge({ text: 'Resuelto', color: 'green', withDot: true })
						: createGridBadge({ text: 'Pendiente', color: 'red', withDot: true });
				}
			},
			{
				title: 'F. Resuelto',
				field: 'fechaResolucion',
				width: 100,
				hozAlign: 'center',
				formatter: (cell: any) => {
					const val = cell.getValue();
					if (!val) return `<span class="text-gray-500 opacity-30">-</span>`;
					return `<span class="text-green-500/80 text-[11px]">${formatDateSlashed(val)}</span>`;
				}
			},
			{
				title: 'H. Resuelto',
				field: 'fechaResolucion',
				width: 80,
				hozAlign: 'center',
				formatter: (cell: any) => {
					const val = cell.getValue();
					if (!val) return `<span class="text-gray-500 opacity-30">-</span>`;
					return `<span class="text-green-500 font-medium text-[11px]">${formatMilitaryTime(val)}</span>`;
				}
			}
		);
	}

	columns.push({
		title: 'Reportado por',
		field: 'reportadoPorNombre',
		width: 150,
		hozAlign: 'left',
		formatter: (cell) =>
			`<span class="text-secondary text-[11px] font-medium">${cell.getValue() || 'Sistema'}</span>`
	});

	if (isHistory) {
		columns.push({
			title: 'Resuelto por',
			field: 'resueltoPorNombre',
			width: 150,
			hozAlign: 'left',
			formatter: (cell) => {
				const val = cell.getValue();
				if (!val) return `<span class="text-gray-500 opacity-30">-</span>`;
				return `<span class="text-green-400 text-[11px] font-medium">${val}</span>`;
			}
		});
	}

	if (isHistory) {
		columns.push({
			title: 'Notas Resolución',
			field: 'notasResolucion',
			width: 200,
			formatter: 'textarea'
		});
	}

	columns.push({
		title: 'Observación Inicial',
		field: 'notas',
		width: 200,
		formatter: 'textarea'
	});

	if (!callbacks.hideActions) {
		columns.push({
			title: 'Acciones',
			field: 'actions',
			width: 100,
			hozAlign: 'center',
			headerSort: false,
			formatter: (cell: any) => {
				const data = cell.getData();
				if (data.resuelto) return '';
				return createGridBadge({
					text: 'Resolver',
					color: 'blue',
					isButton: true,
					className: 'resolve-btn'
				});
			},
			cellClick: (e: any, cell: any) => {
				const target = e.target as HTMLElement;
				if (target.classList.contains('resolve-btn')) {
					callbacks.onResolve?.(cell.getData());
				}
			}
		});
	}

	return columns;
}
