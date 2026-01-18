<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		TabulatorFull as Tabulator,
		type ColumnDefinition,
		type Options
	} from 'tabulator-tables';
	import {
		createTabulatorController,
		defaultTabulatorOptions
	} from '$lib/logic/tabulator/tabulatorController';
	import 'tabulator-tables/dist/css/tabulator_midnight.min.css';

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
	}

	let {
		data = [],
		columns = [],
		options = {},
		class: className = '',
		height = '100%',
		placeholder = 'No Data Available',
		layout = 'fitDataFill',
		responsiveLayout,
		pagination = false,
		paginationSize = 10,
		searchable = false,
		downloadable = false,
		withCheckboxSelection = false,
		toolbarActions,
		onRowSelectionChanged,
		persistenceID,
		persistenceMode = 'local'
	}: Props = $props();

	let table: Tabulator | undefined;
	let tableElement: HTMLElement;

	// Construct final columns
	let finalColumns = $derived.by(() => {
		let cols = [...columns];
		if (withCheckboxSelection) {
			cols.unshift({
				formatter: 'rowSelection',
				titleFormatter: 'rowSelection',
				title: '', // Required by types
				headerSort: false,
				hozAlign: 'center',
				width: 40,
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
				...options
			});

			controller.setTable(table);

			table.on('rowSelectionChanged', (data: any[], rows: any[]) => {
				if (onRowSelectionChanged) {
					onRowSelectionChanged(data, rows);
				}
			});
		}
	});

	// Reactive Updates
	$effect(() => {
		if (table && data) {
			table.replaceData($state.snapshot(data));
		}
	});

	// Derived columns update check? Tabulator handles mutations differently,
	// but if columns structure changes significantly we might need setColumns.
	// However, usually columns are static structure-wise.
	// If we need dynamic columns, we'd need another effect or smart diffing.
	// For now, assuming columns don't change structure after init (only visibility which is internal).

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

	<div bind:this={tableElement}></div>
</div>

<style>
	.tabulator-wrapper {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		background: transparent;
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

	/* Global Overrides for Tabulator - Minimalist Premium */
	:global(.tabulator) {
		border: none !important;
		background-color: transparent !important;
		font-family: 'Inter', system-ui, sans-serif;
	}

	/* Header Styling */
	:global(.tabulator-header) {
		background-color: transparent !important;
		border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
		border-top: none !important;
		color: #a1a1aa !important; /* zinc-400 */
		font-weight: 500;
		font-size: 0.85rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	:global(.tabulator-headers .tabulator-col) {
		background-color: transparent !important;
		border-right: none !important; /* Remove vertical borders */
		padding: 8px 4px !important;
	}

	:global(.tabulator-headers .tabulator-col:hover) {
		background-color: rgba(255, 255, 255, 0.02) !important;
	}

	:global(.tabulator-headers .tabulator-col-content) {
		padding: 4px !important;
	}

	:global(.tabulator-col-title) {
		padding-bottom: 4px;
	}

	/* Row Styling */
	:global(.tabulator-row) {
		background-color: transparent !important;
		color: #e4e4e7 !important; /* zinc-200 */
		border-bottom: 1px solid rgba(255, 255, 255, 0.03) !important;
		font-size: 0.9rem;
		min-height: 44px !important; /* slightly taller for breathability */
	}

	:global(.tabulator-row .tabulator-cell) {
		border-right: none !important;
		padding: 10px 8px !important;
		display: inline-flex !important;
		align-items: center;
	}

	/* Subtle Zebra Striping */
	:global(.tabulator-row.tabulator-row-even) {
		background-color: rgba(255, 255, 255, 0.015) !important;
	}

	:global(.tabulator-row:hover) {
		background-color: rgba(255, 255, 255, 0.05) !important;
		transition: background-color 0.15s ease;
	}

	/* Selection Styling */
	:global(.tabulator-row.tabulator-selected) {
		background-color: rgba(59, 130, 246, 0.1) !important; /* Blue-500 subtle */
		border-left: 2px solid #3b82f6 !important; /* Blue accent */
	}

	:global(.tabulator-row.tabulator-selected:hover) {
		background-color: rgba(59, 130, 246, 0.15) !important;
	}

	/* Footer Pagination */
	:global(.tabulator-footer) {
		background-color: transparent !important;
		border-top: 1px solid rgba(255, 255, 255, 0.08) !important;
		padding: 12px 0 !important;
	}

	:global(.tabulator-page) {
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: #a1a1aa;
		background: transparent;
		margin: 0 2px;
		font-size: 0.8rem;
		padding: 4px 10px;
	}

	:global(.tabulator-page.active) {
		background: #27272a; /* zinc-800 */
		border-color: #3f3f46;
		color: #fff;
		font-weight: 600;
	}

	:global(.tabulator-page:hover:not(.active)) {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	/* Modern Filter Inputs */
	:global(.tabulator-header-filter input) {
		background-color: transparent !important;
		border: none !important;
		border-bottom: 1px solid rgba(255, 255, 255, 0.15) !important;
		color: #e2e8f0 !important;
		padding: 4px 0 !important;
		font-size: 0.8rem;
		transition: border-color 0.2s;
		width: 100%;
		margin-top: 4px;
	}

	:global(.tabulator-header-filter input:focus) {
		border-bottom-color: #3b82f6 !important;
		outline: none;
	}

	:global(.tabulator-header-filter input::placeholder) {
		color: rgba(255, 255, 255, 0.2);
		font-style: italic;
	}

	/* Frozen Columns blending */
	:global(.tabulator-col.tabulator-frozen),
	:global(.tabulator-col.tabulator-frozen.tabulator-col-group) {
		background-color: #18181b !important; /* zinc-900 match sidebar likely */
		z-index: 10 !important;
	}

	/* Ensure row cells that are frozen match the row background correctly */
	/* This is tricky with transparency. Best to give them a solid background match if possible, 
       OR use backdrop-filter blur if supported effectively, but solid is safer for artifacts. */
	:global(.tabulator-row .tabulator-cell.tabulator-frozen) {
		background-color: #18181b !important;
	}
	:global(.tabulator-row.tabulator-row-even .tabulator-cell.tabulator-frozen) {
		background-color: #1c1c1f !important; /* Slightly lighter for even rows */
	}
	:global(.tabulator-row:hover .tabulator-cell.tabulator-frozen) {
		background-color: #27272a !important; /* Hover state for frozen */
	}
	:global(.tabulator-row.tabulator-selected .tabulator-cell.tabulator-frozen) {
		background-color: #1e293b !important; /* Slate-800ish for selected */
	}
</style>
