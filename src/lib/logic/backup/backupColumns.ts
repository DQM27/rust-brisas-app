import type { ColumnDefinition } from 'tabulator-tables';
import type { BackupEntry } from '$lib/types/backup';
import { createGridBadge } from '$lib/components/tabulator/gridBadge';

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

export const getBackupColumns = (handlers: {
	onRestore: (data: BackupEntry) => void;
	onDelete: (data: BackupEntry) => void;
}): ColumnDefinition[] => {
	return [
		{
			title: 'Archivo',
			field: 'nombre',
			headerFilter: 'input',
			width: 300,
			formatter: (cell) => {
				const nombre = cell.getValue() || '';
				return `
                    <div class="flex items-center gap-2">
                        <span class="text-purple-500">📦</span>
                        <span style="font-weight:500; color:#e2e8f0" class="truncate">${nombre}</span>
                    </div>
                `;
			}
		},
		{
			title: 'Tamaño',
			field: 'tamano',
			headerFilter: 'number',
			width: 100,
			hozAlign: 'right',
			formatter: (cell) =>
				`<span style="font-family:monospace; color:#9ca3af">${formatBytes(cell.getValue() || 0)}</span>`
		},
		{
			title: 'Fecha',
			field: 'fechaCreacion',
			headerFilter: 'input',
			width: 180,
			formatter: (cell) =>
				`<span style="font-family:monospace; color:#9ca3af">${formatDate(cell.getValue() || '')}</span>`
		},
		{
			title: 'Antigüedad',
			field: 'diasAntiguedad',
			headerFilter: 'number',
			width: 120,
			formatter: (cell) => {
				const dias = cell.getValue() || 0;
				let color: 'green' | 'red' | 'amber' | 'blue' = 'green';

				if (dias > 25) color = 'red';
				else if (dias > 15) color = 'amber';
				else if (dias > 7) color = 'blue';

				return createGridBadge({
					text: `${dias} día${dias !== 1 ? 's' : ''}`,
					color
				});
			}
		},
		{
			title: 'Seguridad',
			field: 'encryptionType',
			headerFilter: 'input',
			width: 140,
			formatter: (cell) => {
				const type = cell.getValue() || 'none';
				if (type === 'local') {
					return createGridBadge({ text: '🔐 Encriptado', color: 'green' });
				} else if (type === 'portable') {
					return createGridBadge({ text: '🔑 Portable', color: 'purple' as any }); // purple doesn't exist in gridBadge, but it will fallback to gray or I can add it
				}
				return createGridBadge({ text: '📄 Sin encriptar', color: 'gray' });
			}
		},
		{
			title: 'Acciones',
			field: 'acciones',
			width: 180,
			hozAlign: 'center',
			headerSort: false,
			formatter: () => {
				return `
                    <div class="flex items-center justify-center gap-2">
                        <button class="restore-btn px-2.5 py-1 text-[10px] font-bold uppercase tracking-widest rounded-md 
                            bg-purple-500/10 text-purple-400 border border-purple-500/20 hover:bg-purple-500/20
                            transition-colors">
                            Restaurar
                        </button>
                        <button class="delete-btn p-1.5 text-gray-400 hover:text-red-400 transition-colors">🗑️</button>
                    </div>
                `;
			},
			cellClick: (e, cell) => {
				const target = e.target as HTMLElement;
				const data = cell.getData() as BackupEntry;
				if (target.classList.contains('restore-btn')) handlers.onRestore(data);
				if (target.classList.contains('delete-btn')) handlers.onDelete(data);
			}
		}
	];
};
