import type { ColDef, ValueFormatterParams } from '@ag-grid-community/core';
import type { TrashItem } from '$lib/types/trash';

export class TrashColumns {
	/**
	 * Returns a standard column definition for the 'deletedAt' field.
	 */
	static getDeletedAtColumn(): ColDef<TrashItem> {
		return {
			field: 'deletedAt' as any,
			headerName: 'Eliminado',
			width: 150,
			sortable: true,
			valueFormatter: (params: ValueFormatterParams<TrashItem>) => {
				if (!params.value) return 'Recientemente';
				try {
					return new Date(params.value).toLocaleDateString();
				} catch {
					return params.value;
				}
			}
		};
	}
}
