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
		onRowSelectionChanged
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

	$effect(() => {
		if (tableElement) {
			table = new Tabulator(tableElement, {
				...defaultTabulatorOptions,
				height,
				layout,
				responsiveLayout,
				pagination,
				paginationSize,
				placeholder,
				data: $state.snapshot(data) || [], // Initial data snapshot
				columns: finalColumns as any[], // Use derived columns (cast to any[] to avoid strict type issues)
				...options // Override with any specific options passed
			});

			controller.setTable(table);

			table.on('rowClick', (e, row) => {
				// Handle row click
			});

			// Listen for selection changes
			table.on('rowSelectionChanged', (data: any[], rows: any[]) => {
				if (onRowSelectionChanged) {
					onRowSelectionChanged(data, rows);
				}
			});
		}

		return () => {
			table?.destroy();
		};
	});

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

	/* Global Overrides for Tabulator to make it look 'Premium' and Glassmorphism-ready */
	:global(.tabulator) {
		border: none !important;
		background-color: rgba(30, 30, 40, 0.6) !important; /* Semi-transparent background */
		backdrop-filter: blur(10px);
		border-radius: 8px;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		overflow: hidden; /* For rounded corners */
		font-family: 'Inter', sans-serif; /* Setup in global or use here */
	}

	:global(.tabulator-header) {
		background-color: rgba(40, 40, 50, 0.8) !important;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1) !important;
		color: #e2e8f0 !important;
		font-weight: 600;
	}

	:global(.tabulator-headers .tabulator-col) {
		background-color: transparent !important;
		border-right: 1px solid rgba(255, 255, 255, 0.05) !important;
	}

	:global(.tabulator-headers .tabulator-col:hover) {
		background-color: rgba(255, 255, 255, 0.05) !important;
	}

	:global(.tabulator-row) {
		background-color: transparent !important;
		color: #cbd5e1 !important;
		border-bottom: 1px solid rgba(255, 255, 255, 0.05) !important;
		transition: background-color 0.2s;
	}

	:global(.tabulator-row.tabulator-row-even) {
		background-color: rgba(255, 255, 255, 0.02) !important;
	}

	:global(.tabulator-row:hover) {
		background-color: rgba(255, 255, 255, 0.1) !important;
		cursor: pointer;
	}

	:global(.tabulator-row.tabulator-selected) {
		background-color: rgba(118, 75, 162, 0.3) !important;
		border-left: 3px solid #764ba2;
	}

	:global(.tabulator-footer) {
		background-color: rgba(40, 40, 50, 0.8) !important;
		border-top: 1px solid rgba(255, 255, 255, 0.1) !important;
	}

	:global(.tabulator-page) {
		border-radius: 4px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		color: #fff;
		background: transparent;
	}

	:global(.tabulator-page.active) {
		background: #764ba2;
		border-color: #764ba2;
		color: #fff;
	}

	/* Input Styling for Filters */
	:global(.tabulator-header-filter input) {
		background-color: rgba(20, 20, 30, 0.8) !important;
		border: 1px solid rgba(255, 255, 255, 0.2) !important;
		color: #e2e8f0 !important;
		border-radius: 4px;
		padding: 4px 8px;
		font-size: 0.9em;
	}

	:global(.tabulator-header-filter input:focus) {
		border-color: #764ba2 !important;
		outline: none;
	}

	/* Fix frozen columns in dark mode */
	:global(.tabulator-col.tabulator-frozen) {
		background-color: rgba(40, 40, 50, 1) !important; /* Solid bg needed for frozen cols */
	}
	:global(.tabulator-row .tabulator-cell.tabulator-frozen) {
		background-color: #1e1e1e !important; /* Match row bg */
	}
	:global(.tabulator-row.tabulator-row-even .tabulator-cell.tabulator-frozen) {
		background-color: #252526 !important; /* Match even row bg */
	}
</style>
