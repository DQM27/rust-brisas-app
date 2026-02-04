<!-- src/lib/components/audit/AuditLogView.svelte -->
<!-- Vista unificada de auditoría de sistema siguiendo patrón de grids -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { auditService, type SysLogEntry } from '$lib/services/auditService';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import { RefreshCw, AlertCircle } from 'lucide-svelte';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'audit-log' }: Props = $props();

	// State
	let tableData = $state<SysLogEntry[]>([]);
	let isLoading = $state(false);
	let error = $state('');
	let searchTerm = $state('');
	let gridWrapper: any = $state(null);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters-audit') === 'true'
			: false
	);

	// Toolbar columns metadata
	let toolbarColumns = $state<
		{ field: string; title: string; visible: boolean; frozen: boolean }[]
	>([]);

	// Column definitions
	const columns = [
		{
			title: 'Fecha',
			field: 'access_date',
			width: 160,
			sorter: 'string',
			headerFilter: 'input',
			formatter: (cell: any) => {
				const val = cell.getValue();
				if (!val) return '';
				try {
					const date = new Date(val);
					return date.toLocaleString('es-CR', {
						year: 'numeric',
						month: '2-digit',
						day: '2-digit',
						hour: '2-digit',
						minute: '2-digit',
						second: '2-digit',
						hour12: false
					});
				} catch {
					return val;
				}
			}
		},
		{
			title: 'Usuario',
			field: 'user_name',
			headerFilter: 'input',
			widthGrow: 1,
			minWidth: 150
		},
		{
			title: 'Evento',
			field: 'event_type',
			width: 100,
			headerFilter: 'list',
			headerFilterParams: {
				values: {
					'': 'Todos',
					LOGIN: 'LOGIN',
					LOGOUT: 'LOGOUT',
					TIMEOUT: 'TIMEOUT',
					FAILED_LOGIN: 'FAILED_LOGIN'
				}
			},
			formatter: (cell: any) => {
				const val = cell.getValue();
				let color = 'bg-gray-500';
				if (val === 'LOGIN') color = 'bg-green-600';
				if (val === 'LOGOUT') color = 'bg-orange-500';
				if (val === 'TIMEOUT') color = 'bg-red-600';
				if (val === 'FAILED_LOGIN') color = 'bg-red-800';
				return `<span class="inline-flex items-center justify-center px-2 py-0.5 rounded text-[10px] font-bold text-white ${color}">${val}</span>`;
			},
			hozAlign: 'center'
		},
		{
			title: 'Terminal',
			field: 'terminal_name',
			width: 140,
			headerFilter: 'input'
		},
		{
			title: 'ID Terminal',
			field: 'terminal_id',
			width: 140,
			headerFilter: 'input'
		},
		{
			title: 'Duración',
			field: 'duration',
			width: 90,
			hozAlign: 'right',
			formatter: (cell: any) => {
				const val = cell.getValue();
				return val ? `<span class="text-cyan-400">${val}</span>` : '-';
			}
		},
		{
			title: 'IP',
			field: 'ip_address',
			width: 120,
			headerFilter: 'input'
		},
		{
			title: 'Detalles',
			field: 'details',
			widthGrow: 2,
			minWidth: 200,
			formatter: (cell: any) => {
				const val = cell.getValue();
				if (!val) return '';
				// Highlight important details
				if (val.includes('Unexpected') || val.includes('Crash')) {
					return `<span class="text-orange-400">${val}</span>`;
				}
				return val;
			}
		}
	] as any[];

	// Data loading
	async function loadLogs() {
		isLoading = true;
		error = '';
		try {
			tableData = await auditService.getLogs(500, 0);
		} catch (err) {
			console.error('Error loading logs:', err);
			error = 'Error al cargar los registros de auditoría';
		} finally {
			isLoading = false;
		}
	}

	// Search handler
	function handleGridSearch(term: string) {
		searchTerm = term;
		if (!gridWrapper) return;

		if (!term || term.trim().length < 2) {
			gridWrapper.getTable()?.clearFilter();
			return;
		}

		// Search in multiple fields
		gridWrapper.getTable()?.setFilter([
			[
				{ field: 'user_name', type: 'like', value: term },
				{ field: 'event_type', type: 'like', value: term },
				{ field: 'terminal_name', type: 'like', value: term },
				{ field: 'details', type: 'like', value: term }
			]
		]);
	}

	onMount(() => {
		loadLogs();
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary">Auditoría del Sistema</h2>
				<p class="mt-1 text-sm text-secondary">Historial de accesos y eventos de seguridad</p>
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 flex flex-col overflow-hidden relative bg-surface-1">
		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error al cargar auditoría</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else}
			<!-- GridToolbar -->
			<GridToolbar
				{searchTerm}
				onSearch={handleGridSearch}
				onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
				onFitColumns={() => gridWrapper?.fitColumns()}
				onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
				onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
				onToggleFilters={() => {
					showHeaderFilters = !showHeaderFilters;
					if (typeof window !== 'undefined') {
						localStorage.setItem('tabulator-header-filters-audit', String(showHeaderFilters));
					}
					if (gridWrapper) {
						setTimeout(() => {
							gridWrapper.redraw(true);
						}, 50);
					}
				}}
				columns={toolbarColumns}
				hasSelection={false}
			>
				{#snippet primaryActions()}
					<button
						class="flex items-center gap-2 px-3 py-1.5
							   bg-blue-600/10 hover:bg-blue-600/20
							   text-blue-400 hover:text-blue-300
							   border border-blue-500/20 hover:border-blue-500/30
							   rounded-md text-sm font-medium transition-all"
						onclick={loadLogs}
						disabled={isLoading}
					>
						<RefreshCw size={16} class={isLoading ? 'animate-spin' : ''} />
						<span>Actualizar</span>
					</button>
				{/snippet}
			</GridToolbar>

			<!-- Tabulator Grid -->
			<div
				class="flex-1 overflow-hidden relative bg-[#1e1e1e] {showHeaderFilters
					? ''
					: 'hide-filters'}"
			>
				{#if isLoading && tableData.length === 0}
					<div class="absolute inset-0 flex items-center justify-center bg-surface-1/50 z-10">
						<RefreshCw class="animate-spin text-primary" size={32} />
					</div>
				{/if}

				<TabulatorWrapper
					bind:this={gridWrapper}
					bind:toolbarColumns
					data={tableData}
					{columns}
					persistenceID="audit-log-grid-v2"
					options={{
						...defaultTabulatorOptions,
						layout: 'fitData',
						pagination: true,
						paginationSize: 50,
						initialSort: [{ column: 'access_date', dir: 'desc' }]
					}}
				/>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Hide header filters when toggled off */
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
