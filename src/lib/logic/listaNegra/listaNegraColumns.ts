// src/lib/logic/listaNegra/listaNegraColumns.ts
import type { ListaNegraResponse } from '$lib/types/listaNegra';
import type { ColDef, ICellRendererParams } from '@ag-grid-community/core';

export class ListaNegraColumns {
	// Column configuration
	// Helper para parsear fechas de SurrealDB
	private static parseDate(value: any): Date | null {
		if (!value) return null;
		if (value instanceof Date) return value;

		let dateStr = String(value);
		// Remove SurrealDB format wrappers if present
		if (dateStr.startsWith("d'") && dateStr.endsWith("'")) {
			dateStr = dateStr.slice(2, -1);
		}

		const d = new Date(dateStr);
		return isNaN(d.getTime()) ? null : d;
	}

	// Column configuration
	static getColumns(): ColDef<ListaNegraResponse>[] {
		return [
			{
				colId: 'cedula',
				field: 'cedula',
				headerName: 'Cédula',
				width: 130,
				pinned: 'left',
				cellStyle: { fontFamily: 'monospace', fontSize: '13px' }
			},
			{
				colId: 'nombreCompleto',
				field: 'nombreCompleto',
				headerName: 'Nombre Completo',
				flex: 1,
				minWidth: 200,
				cellStyle: { fontWeight: 500 },
				valueGetter: (params) => {
					const data = params.data as any;
					if (!data) return '';
					return data.nombreCompleto || data.nombre_completo || `${data.nombre} ${data.apellido}`;
				}
			},
			{
				colId: 'empresa',
				headerName: 'Empresa',
				field: 'empresaNombre', // Added for export/filter compatibility
				flex: 1,
				minWidth: 150,
				valueGetter: (params) => {
					const data = params.data as any;
					// Debug Empresa
					console.log('Debug Empresa:', {
						nombre: data?.empresaNombre,
						snake: data?.empresa_nombre,
						full: data?.empresa
					});
					if (!data) return '';
					return data.empresaNombre || data.empresa_nombre || 'Sin empresa';
				}
			},
			{
				colId: 'nivelSeveridad',
				field: 'nivelSeveridad',
				headerName: 'Nivel',
				width: 100,
				valueGetter: (params) => {
					const data = params.data as any;
					return data?.nivelSeveridad || data?.nivel_severidad;
				},
				cellRenderer: (params: ICellRendererParams) => {
					return ListaNegraColumns.formatNivelBadge(params.value);
				}
			},
			{
				colId: 'isActive',
				field: 'isActive',
				headerName: 'Estado',
				width: 130,
				valueGetter: (params) => {
					const data = params.data as any;
					if (!data) return false;
					// is_active puede ser boolean o string "true"/"false" desde DB a veces
					const val = data.isActive !== undefined ? data.isActive : data.is_active;
					return val;
				},
				cellRenderer: (params: ICellRendererParams) => {
					return ListaNegraColumns.formatEstadoBadge(params.value);
				}
			},
			{
				colId: 'motivoBloqueo',
				field: 'motivoBloqueo',
				headerName: 'Motivo',
				flex: 1,
				minWidth: 200,
				valueGetter: (params) => {
					const data = params.data as any;
					return data?.motivoBloqueo || data?.motivo_bloqueo || 'Sin motivo especificado';
				},
				cellRenderer: (params: ICellRendererParams) => {
					return `<span class="text-sm text-gray-300 truncate">${params.value}</span>`;
				}
			},
			{
				colId: 'bloqueadoPor',
				field: 'bloqueadoPor',
				headerName: 'Bloqueado Por',
				width: 160,
				valueGetter: (params) => {
					const data = params.data as any;
					return (
						data?.bloqueadoPorNombre ||
						data?.bloqueado_por_nombre ||
						data?.bloqueadoPor ||
						'Sistema'
					);
				},
				cellRenderer: (params: ICellRendererParams) => {
					return `<span class="text-sm text-gray-400">${params.value}</span>`;
				}
			},
			{
				colId: 'fecha',
				headerName: 'Fecha',
				width: 120,
				valueGetter: (params) => {
					const data = params.data as any;
					if (!data) return null;
					return data.createdAt || data.created_at;
				},
				valueFormatter: (params) => {
					const date = ListaNegraColumns.parseDate(params.value);
					if (!date) return 'N/A';
					return date.toLocaleDateString('es-CR', {
						day: '2-digit',
						month: '2-digit',
						year: 'numeric'
					});
				}
			},
			{
				colId: 'hora',
				headerName: 'Hora',
				width: 100,
				valueGetter: (params) => {
					const data = params.data as any;
					if (!data) return null;
					return data.createdAt || data.created_at;
				},
				valueFormatter: (params) => {
					const date = ListaNegraColumns.parseDate(params.value);
					if (!date) return '';
					return date.toLocaleTimeString('es-CR', {
						hour: '2-digit',
						minute: '2-digit'
					});
				}
			}
		];
	}

	// Helper methods
	static formatNivelBadge(nivel: string): string {
		const baseClass =
			'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border';

		switch (nivel) {
			case 'ALTO':
				return `<span class="${baseClass} bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800">ALTO</span>`;
			case 'MEDIO':
				return `<span class="${baseClass} bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/30 dark:text-yellow-300 dark:border-yellow-800">MEDIO</span>`;
			case 'BAJO':
				return `<span class="${baseClass} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800">BAJO</span>`;
			default:
				return `<span class="${baseClass} bg-gray-50 text-gray-700 border-gray-200 dark:bg-gray-900/30 dark:text-gray-300 dark:border-gray-800">${nivel || 'N/A'}</span>`;
		}
	}

	static formatEstadoBadge(isActive: boolean): string {
		const baseClass =
			'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border';

		if (isActive) {
			// Red (GitHub Closed/Blocked)
			const classes =
				'bg-red-50 text-red-700 border-red-200 dark:bg-red-900/30 dark:text-red-300 dark:border-red-800';
			return `<span class="${baseClass} ${classes}">● Bloqueado</span>`;
		} else {
			// Green (GitHub Open)
			const classes =
				'bg-green-50 text-green-700 border-green-200 dark:bg-green-900/30 dark:text-green-300 dark:border-green-800';
			return `<span class="${baseClass} ${classes}">✓ Desbloqueado</span>`;
		}
	}
}
