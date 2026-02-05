<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		TabulatorFull as Tabulator,
		type ColumnDefinition,
		type Options
	} from 'tabulator-tables';
	import { toastService } from '$lib/services/toastService';
	import {
		createTabulatorController,
		defaultTabulatorOptions
	} from '$lib/logic/tabulator/tabulatorController';
	// CSS base simple de Tabulator - nuestras variables CSS lo personalizan
	import 'tabulator-tables/dist/css/tabulator_simple.min.css';

	interface Props {
		data: any[];
		columns: ColumnDefinition[];
		options?: Options;
		class?: string;
		height?: string | number;
		placeholder?: string;
		layout?: 'fitData' | 'fitColumns' | 'fitDataFill' | 'fitDataStretch';
		responsiveLayout?: 'hide' | 'collapse';
		pagination?: boolean;
		paginationSize?: number;
		// Add-ons
		searchable?: boolean;
		downloadable?: boolean;
		withCheckboxSelection?: boolean; // New prop for auto-checkbox column
		toolbarActions?: import('svelte').Snippet;
		onRowSelectionChanged?: (data: any[], rows: any[]) => void;
		persistenceID?: string; // Unique ID for storing table state
		persistenceMode?: 'local' | 'cookie';
		groupBy?: string | string[] | ((data: any) => string); // New: Grouping field
		columnCalculations?: boolean; // New: Enable footer calculations
		rowContextMenu?: any[]; // New: Context menu items
		toolbarColumns?: { field: string; title: string; visible: boolean; frozen: boolean }[]; // Bindable
		onRowDblClick?: (e: any, row: any) => void;
	}

	let {
		data = [],
		columns = [],
		options = {},
		class: className = '',
		height = '100%',
		placeholder = 'No Data Available',
		layout = 'fitData',
		responsiveLayout,
		pagination = false,
		paginationSize = 10,
		searchable = false,
		downloadable = false,
		withCheckboxSelection = false,
		toolbarActions,
		onRowSelectionChanged,
		persistenceID,
		persistenceMode = 'local',
		groupBy,
		columnCalculations = false,
		rowContextMenu = [],
		toolbarColumns = $bindable([]),
		onRowDblClick
	}: Props = $props();

	let table: Tabulator | undefined;
	let tableElement: HTMLElement;
	let isTableBuilt = $state(false);

	// Construct final columns
	let finalColumns = $derived.by(() => {
		let cols = [...columns];
		if (withCheckboxSelection) {
			cols.unshift({
				formatter: 'rowSelection',
				titleFormatter: 'rowSelection',
				title: '', // Required by types
				field: 'select',
				headerSort: false,
				hozAlign: 'center',
				width: 30,
				frozen: true,
				headerHozAlign: 'center'
			});
		}
		return cols;
	});

	// Initialize controller logic
	const controller = createTabulatorController();

	// Export table API for parent access
	export const getTable = controller.getTable;
	export const replaceData = controller.replaceData;
	export const updateRow = controller.updateRow;
	export const deleteRow = controller.deleteRow;
	export const addData = controller.addData;
	export const redraw = controller.redraw;
	export const deselectAll = controller.deselectAll;

	// Separate initialization from updates to prevent full rebuilds
	onMount(() => {
		if (tableElement) {
			table = new Tabulator(tableElement, {
				...defaultTabulatorOptions,
				height,
				layout,
				responsiveLayout,
				pagination,
				paginationSize,
				placeholder,
				data: $state.snapshot(data) || [], // Initial data
				columns: finalColumns as any[],
				resizableColumnFit: false, // Fix for resize bug
				// Persistence
				persistence: persistenceID
					? {
							sort: true,
							filter: true,
							columns: true
						}
					: false,
				persistenceID: persistenceID,
				persistenceMode: persistenceMode,
				// Pro Features Setup
				groupBy,
				columnCalcs: columnCalculations ? 'both' : false,
				rowContextMenu,
				...options
			});

			controller.setTable(table);

			table.on('rowSelectionChanged', (data: any[], rows: any[]) => {
				if (onRowSelectionChanged) {
					onRowSelectionChanged(data, rows);
				}
			});

			table.on('tableBuilt', () => {
				isTableBuilt = true;
			});

			if (onRowDblClick) {
				table.on('rowDblClick', onRowDblClick);
			} else if (options && (options as any).rowDblClick) {
				table.on('rowDblClick', (options as any).rowDblClick);
			}

			// Sync toolbar columns when table is ready
			table.on('tableBuilt', updateToolbarColumns);
			table.on('columnVisibilityChanged', updateToolbarColumns);
			table.on('columnMoved', updateToolbarColumns);
		}
	});

	// Reactive Updates for Data
	$effect(() => {
		if (table && isTableBuilt && data) {
			table.replaceData($state.snapshot(data));
		}
	});

	// Reactive Updates for Columns
	$effect(() => {
		if (table && isTableBuilt && finalColumns) {
			// NOTA: No usamos $state.snapshot aquí porque las columnas contienen funciones (formatters)
			// que Svelte 5 no puede clonar. Tabulator maneja bien los proxies de columnas.
			table.setColumns(finalColumns as any);
		}
	});

	// Reactive Updates for Grouping
	$effect(() => {
		if (table && isTableBuilt) {
			table.setGroupBy(groupBy as any);
		}
	});

	// Derived columns update check? Tabulator handles mutations differently,
	// but if columns structure changes significantly we might need setColumns.
	// However, usually columns are static structure-wise.
	// If we need dynamic columns, we'd need another effect or smart diffing.
	// For now, assuming columns don't change structure after init (only visibility which is internal).

	// --- PRO Column Management (Centralized) ---
	function updateToolbarColumns() {
		if (!table || !isTableBuilt) return;
		const allCols = table.getColumns();
		const seenFields = new Set<string>();
		const cleanCols: any[] = [];

		allCols.forEach((c: any) => {
			const def = c.getDefinition();
			const field = c.getField();
			const title = def.title;

			if (!title || title === '' || title === 'Acciones') return;

			const fieldKey = field || title;
			if (seenFields.has(fieldKey)) return;
			seenFields.add(fieldKey);

			cleanCols.push({
				field: fieldKey,
				title: title,
				visible: c.isVisible(),
				frozen: def.frozen === true || def.frozen === 'left'
			});
		});

		toolbarColumns = cleanCols;
	}

	export function toggleColumn(field: string) {
		const column = table?.getColumn(field);
		if (column) {
			column.isVisible() ? column.hide() : column.show();
			updateToolbarColumns();
		}
	}

	export function toggleFreeze(field: string) {
		const column = table?.getColumn(field);
		if (column) {
			const def = column.getDefinition();
			const currentlyFrozen = (def.frozen as any) === true || (def.frozen as any) === 'left';
			const newState = !currentlyFrozen;
			column.updateDefinition({ frozen: newState } as any);

			setTimeout(() => {
				updateToolbarColumns();
				redraw(true);
			}, 10);
		}
	}

	export function autoSizeColumns() {
		if (table) {
			const cols = table.getColumnDefinitions().map((col: any) => ({ ...col, width: undefined }));
			table.setColumns(cols);
			toastService.success('Columnas ajustadas al contenido');
		}
	}

	export function fitColumns() {
		if (table) {
			const containerWidth = table.element.clientWidth;
			const columns = table.getColumns();
			if (columns.length === 0) return;
			const columnWidth = Math.floor(containerWidth / columns.length);
			const remainder = containerWidth - columnWidth * columns.length;

			const cols = table.getColumnDefinitions().map((col: any, index: number) => ({
				...col,
				width: index === columns.length - 1 ? columnWidth + remainder : columnWidth
			}));
			table.setColumns(cols);
			toastService.success('Columnas ajustadas al ancho');
		}
	}

	// Export toolbar state for snippets
	export const getToolbarColumns = () => toolbarColumns;

	// Cleanup
	onDestroy(() => {
		table?.destroy();
	});

	// Helper functions for "God" features
	function downloadCSV() {
		table?.download('csv', 'data.csv');
	}

	function downloadJSON() {
		table?.download('json', 'data.json');
	}
</script>

<div class="tabulator-wrapper {className}">
	{#if downloadable || searchable || toolbarActions}
		<div class="toolbar">
			{#if searchable}
				<!-- Basic search implementation could go here, or left to parent -->
			{/if}

			<div class="custom-actions">
				{@render toolbarActions?.()}
			</div>

			{#if downloadable}
				<div class="actions">
					<button class="btn-premium" onclick={downloadCSV}>Export CSV</button>
					<button class="btn-premium" onclick={downloadJSON}>Export JSON</button>
				</div>
			{/if}
		</div>
	{/if}

	<div bind:this={tableElement} class="table-element"></div>
</div>

<style>
	.tabulator-wrapper {
		width: 100%;
		height: 100%;
		min-height: 0;
		display: flex;
		flex-direction: column;
		gap: 0;
		background: transparent;
	}

	.table-element {
		flex: 1;
		width: 100%;
		min-height: 0;
	}

	.toolbar {
		display: flex;
		justify-content: flex-end; /* Or space-between */
		padding: 0.5rem;
		background: rgba(30, 30, 30, 0.4); /* Glass-ish */
		border-radius: 8px;
		backdrop-filter: blur(5px);
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-premium {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border: none;
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 4px;
		cursor: pointer;
		font-weight: 600;
		transition:
			transform 0.1s,
			box-shadow 0.1s;
	}

	.btn-premium:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.2);
	}

	/* ==========================================
	   Tabulator Theme - Usando CSS Variables
	   ========================================== */
	:global(.tabulator) {
		font-family:
			'Inter',
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			system-ui,
			sans-serif;
		font-size: 13px;
		background-color: var(--grid-bg) !important;
		border-color: var(--grid-border-emphasis) !important;
	}

	/* Header */
	:global(.tabulator-header) {
		background-color: var(--grid-header-bg) !important;
		border-bottom: 1px solid var(--grid-accent) !important;
		color: var(--grid-text) !important;
		font-size: 11px;
		font-weight: 500;
		letter-spacing: 0.01em;
	}

	:global(.tabulator-col) {
		padding: 4px 4px !important;
		border-right: 1px solid var(--grid-border) !important;
		background-color: var(--grid-header-bg) !important;
	}

	:global(.tabulator-col .tabulator-col-content) {
		padding: 2px;
		white-space: normal !important;
		text-align: center;
	}

	:global(.tabulator-col .tabulator-col-title) {
		white-space: normal !important;
		line-height: 1.1;
	}

	/* Rows */
	:global(.tabulator-row) {
		min-height: 32px;
		background-color: var(--grid-row-bg) !important;
		border-bottom: 1px solid var(--grid-border) !important;
		color: var(--grid-text) !important;
	}

	:global(.tabulator-cell) {
		padding: 6px 4px !important;
		line-height: 1.2;
		border-right: 1px solid var(--grid-border) !important;
	}

	/* Hover */
	:global(.tabulator-row:hover) {
		background-color: var(--grid-row-hover) !important;
	}

	/* Selection */
	:global(.tabulator-row.tabulator-selected) {
		background-color: var(--grid-row-selected) !important;
		border-left: 3px solid var(--grid-accent) !important;
	}

	/* Header filters */
	:global(.tabulator-header-filter input) {
		padding: 6px 8px;
		font-size: 12px;
		border-radius: 4px;
		background-color: var(--grid-row-hover) !important;
		border: 1px solid var(--grid-border-emphasis) !important;
		color: var(--grid-text) !important;
		transition: border-color 0.2s ease;
	}

	:global(.tabulator-header-filter input:focus) {
		outline: none;
		border-color: var(--grid-accent) !important;
		background-color: var(--grid-bg) !important;
	}

	:global(.tabulator-header-filter input::placeholder) {
		color: var(--grid-text-secondary);
		opacity: 0.5;
	}

	/* Frozen columns */
	:global(.tabulator-col.tabulator-frozen) {
		border-left: 1px solid var(--grid-border) !important;
		background-color: var(--grid-header-bg) !important;
	}

	:global(.tabulator-cell.tabulator-frozen) {
		border-left: 1px solid var(--grid-border) !important;
	}

	:global(.tabulator-row .tabulator-cell.tabulator-frozen) {
		background-color: inherit !important;
	}

	:global(.tabulator-row.tabulator-row-even .tabulator-cell.tabulator-frozen) {
		background-color: inherit !important;
	}

	:global(.tabulator-row:hover .tabulator-cell.tabulator-frozen) {
		background-color: inherit !important;
	}

	:global(.tabulator-row.tabulator-selected .tabulator-cell.tabulator-frozen) {
		background-color: inherit !important;
	}

	/* Footer & Pagination */
	:global(.tabulator-footer) {
		background-color: var(--grid-header-bg) !important;
		border-top: 1px solid var(--grid-border-emphasis) !important;
		color: var(--grid-text) !important;
	}

	:global(.tabulator-page) {
		font-size: 13px;
		padding: 6px 12px;
		background-color: var(--grid-row-hover) !important;
		border: 1px solid var(--grid-border-emphasis) !important;
		color: var(--grid-text) !important;
		transition: all 0.15s ease;
	}

	:global(.tabulator-page.active) {
		background-color: var(--grid-accent) !important;
		border-color: var(--grid-accent) !important;
		color: var(--grid-bg) !important;
		font-weight: 600;
	}

	:global(.tabulator-page:hover) {
		background-color: var(--grid-row-selected) !important;
		border-color: var(--grid-accent) !important;
	}

	/* Scrollbars */
	:global(.tabulator-tableholder::-webkit-scrollbar) {
		width: 14px;
		height: 14px;
	}

	:global(.tabulator-tableholder::-webkit-scrollbar-track) {
		background: var(--grid-bg);
	}

	:global(.tabulator-tableholder::-webkit-scrollbar-thumb) {
		background: var(--grid-border-emphasis);
		border-radius: 4px;
	}

	:global(.tabulator-tableholder::-webkit-scrollbar-thumb:hover) {
		background: var(--grid-accent);
		opacity: 0.7;
	}

	/* Grouping Styles */
	:global(.tabulator-group) {
		background: var(--grid-row-selected) !important;
		border-left: 4px solid var(--grid-accent) !important;
		min-height: 28px !important;
	}

	:global(.tabulator-group-toggle) {
		color: var(--grid-accent) !important;
	}

	/* Calculation Rows */
	:global(.tabulator-calcs) {
		background-color: var(--grid-header-bg) !important;
		font-weight: 700 !important;
		color: var(--grid-accent) !important;
	}

	:global(.tabulator-calcs-top) {
		border-bottom: 2px solid var(--grid-border-emphasis) !important;
	}

	:global(.tabulator-calcs-bottom) {
		border-top: 2px solid var(--grid-border-emphasis) !important;
	}

	/* Context Menu */
	:global(.tabulator-menu) {
		background: var(--grid-bg) !important;
		border: 1px solid var(--grid-border-emphasis) !important;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.3) !important;
		border-radius: 6px !important;
		padding: 4px !important;
	}

	:global(.tabulator-menu-item) {
		color: var(--grid-text) !important;
		padding: 6px 12px !important;
		font-size: 12px !important;
		border-radius: 4px !important;
	}

	:global(.tabulator-menu-item:hover) {
		background: var(--grid-accent) !important;
		color: var(--grid-bg) !important;
	}
</style>
