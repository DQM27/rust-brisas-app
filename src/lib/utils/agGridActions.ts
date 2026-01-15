// src/lib/utils/agGridActions.ts

import type { GridApi } from '@ag-grid-community/core';
import type { GridId } from '$lib/types/agGrid';

/**
 * Helpers para ejecutar acciones comunes en AG Grid
 */

export function autosizeAllColumns(api: GridApi): void {
	api.autoSizeAllColumns();
}

export function autosizeSelectedColumns(api: GridApi): void {
	const selectedCols = api
		.getColumnState()
		?.filter((col) => col.hide === false)
		.map((col) => col.colId);

	if (selectedCols && selectedCols.length > 0) {
		api.autoSizeColumns(selectedCols);
	}
}

export function resetColumns(api: GridApi): void {
	api.resetColumnState();
}

export function exportToCsv(api: GridApi, gridId: GridId, onlySelected = false): void {
	const date = new Date().toISOString().split('T')[0];
	const fileName = onlySelected ? `${gridId}-selection-${date}.csv` : `${gridId}-${date}.csv`;

	api.exportDataAsCsv({
		fileName,
		onlySelected,
		processCellCallback: (params) => {
			return params.value ?? '';
		}
	});
}

export function exportToJson<T>(api: GridApi, gridId: GridId, onlySelected = false): void {
	const data: T[] = [];

	if (onlySelected) {
		data.push(...api.getSelectedRows());
	} else {
		api.forEachNodeAfterFilterAndSort((node) => {
			if (node.data) data.push(node.data);
		});
	}

	const blob = new Blob([JSON.stringify(data, null, 2)], {
		type: 'application/json'
	});
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = `${gridId}-${new Date().toISOString().split('T')[0]}.json`;
	a.click();
	URL.revokeObjectURL(url);
}

export function selectAllFiltered(api: GridApi): void {
	api.selectAllFiltered();
}

export function deselectAll(api: GridApi): void {
	api.deselectAll();
}

export function copySelectedToClipboard(api: GridApi): void {
	const selected = api.getSelectedRows();
	if (selected.length > 0) {
		const text = JSON.stringify(selected, null, 2);
		navigator.clipboard.writeText(text).catch(console.error);
	}
}

export function toggleFilters(api: GridApi): void {
	const hasFilters = api.getFilterModel();
	api.setFilterModel(hasFilters && Object.keys(hasFilters).length > 0 ? null : {});
}

export function clearAllFilters(api: GridApi): void {
	api.setFilterModel(null);
}

export function clearSort(api: GridApi): void {
	api.applyColumnState({ defaultState: { sort: null } });
}

export function refreshGrid(api: GridApi): void {
	api.refreshCells();
}

export function expandAllGroups(api: GridApi): void {
	api.forEachNode((node) => {
		if (node.group) {
			node.setExpanded(true);
		}
	});
}

export function collapseAllGroups(api: GridApi): void {
	api.forEachNode((node) => {
		if (node.group) {
			node.setExpanded(false);
		}
	});
}
