import type { BackupEntry } from '$lib/types/backup';
import type { ColDef, ICellRendererParams, ValueFormatterParams, CellClickedEvent } from '@ag-grid-community/core';

/**
 * Formatea bytes a tamaño legible (KB, MB, GB)
 */
function formatBytes(bytes: number): string {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

/**
 * Formatea fecha ISO a formato legible
 */
function formatDate(isoDate: string): string {
	try {
		const date = new Date(isoDate);
		if (isNaN(date.getTime())) return isoDate;

		const day = date.getDate().toString().padStart(2, '0');
		const month = (date.getMonth() + 1).toString().padStart(2, '0');
		const year = date.getFullYear();
		const hours = date.getHours().toString().padStart(2, '0');
		const minutes = date.getMinutes().toString().padStart(2, '0');

		return `${day}/${month}/${year} - ${hours}:${minutes}`;
	} catch {
		return isoDate;
	}
}

export class BackupColumns {
	static getColumns(handlers: {
		onRestore: (data: BackupEntry) => void;
		onDelete: (data: BackupEntry) => void;
	}): ColDef<BackupEntry>[] {
		return [
			{
				colId: 'nombre',
				field: 'nombre',
				headerName: 'Archivo',
				sortable: true,
				filter: true,
				flex: 2,
				minWidth: 250,
				cellRenderer: (params: ICellRendererParams<BackupEntry>) => {
					const nombre = params.value || '';
					return `
                        <div class="flex items-center gap-2">
                            <span class="text-purple-500">📦</span>
                            <span class="font-medium text-gray-800 dark:text-gray-200 truncate">${nombre}</span>
                        </div>
                    `;
				}
			},
			{
				colId: 'tamano',
				field: 'tamano',
				headerName: 'Tamaño',
				sortable: true,
				filter: true,
				width: 100,
				valueFormatter: (params: ValueFormatterParams<BackupEntry>) => formatBytes(params.value || 0),
				cellStyle: { textAlign: 'right' }
			},
			{
				colId: 'fechaCreacion',
				field: 'fechaCreacion',
				headerName: 'Fecha',
				sortable: true,
				filter: true,
				width: 180,
				valueFormatter: (params: ValueFormatterParams<BackupEntry>) => formatDate(params.value || '')
			},
			{
				colId: 'diasAntiguedad',
				field: 'diasAntiguedad',
				headerName: 'Antigüedad',
				sortable: true,
				filter: true,
				width: 120,
				cellRenderer: (params: ICellRendererParams<BackupEntry>) => {
					const dias = params.value || 0;
					let colorClass = 'text-emerald-600 bg-emerald-500/10';

					if (dias > 25) {
						colorClass = 'text-red-600 bg-red-500/10';
					} else if (dias > 15) {
						colorClass = 'text-amber-600 bg-amber-500/10';
					} else if (dias > 7) {
						colorClass = 'text-blue-600 bg-blue-500/10';
					}

					return `
                        <span class="px-2 py-0.5 rounded-full text-xs font-medium ${colorClass}">
                            ${dias} día${dias !== 1 ? 's' : ''}
                        </span>
                    `;
				}
			},
			{
				colId: 'encryptionType',
				field: 'encryptionType',
				headerName: 'Seguridad',
				sortable: true,
				filter: true,
				width: 140,
				cellStyle: { display: 'flex', alignItems: 'center' },
				cellRenderer: (params: ICellRendererParams<BackupEntry>) => {
					const type = params.value || 'none';
					let icon = '📄';
					let label = 'Sin encriptar';
					let colorClass = 'text-gray-400 bg-gray-500/10';

					if (type === 'local') {
						icon = '🔐';
						label = 'Encriptado';
						colorClass = 'text-emerald-600 bg-emerald-500/10';
					} else if (type === 'portable') {
						icon = '🔑';
						label = 'Portable';
						colorClass = 'text-purple-600 bg-purple-500/10';
					}

					return `
                        <span class="px-2 py-0.5 rounded-full text-xs font-medium ${colorClass}">
                            ${icon} ${label}
                        </span>
                    `;
				}
			},
			{
				colId: 'acciones',
				headerName: 'Acciones',
				width: 180,
				pinned: 'right',
				cellStyle: { display: 'flex', justifyContent: 'center', alignItems: 'center' },
				cellRenderer: () => {
					return `
                        <div class="flex items-center justify-center gap-2 h-full">
                            <button class="restore-btn px-2.5 py-1 text-xs font-medium rounded-md 
                                bg-purple-100 text-purple-700 hover:bg-purple-200 
                                dark:bg-purple-900/30 dark:text-purple-400 dark:hover:bg-purple-900/50
                                transition-colors flex items-center gap-1">
                                ↻ Restaurar
                            </button>
                            <button class="delete-btn px-2 py-1 text-xs font-medium rounded-md 
                                bg-gray-100 text-gray-600 hover:bg-red-100 hover:text-red-600
                                dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-red-900/30 dark:hover:text-red-400
                                transition-colors">
                                🗑️
                            </button>
                        </div>
                    `;
				},
				onCellClicked: (params: CellClickedEvent<BackupEntry>) => {
					const event = params.event;
					const data = params.data;
					const target = params.event?.target as HTMLElement;
					if (!target || !data) return;

					if (target.classList.contains('restore-btn')) {
						handlers.onRestore(data);
					} else if (target.classList.contains('delete-btn')) {
						handlers.onDelete(data);
					}
				}
			}
		];
	}
}
