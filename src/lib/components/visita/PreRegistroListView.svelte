<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { toast } from 'svelte-5-french-toast';
	import { UserPlus, RefreshCw, X, Calendar, Search } from 'lucide-svelte';
	import type { ColumnDefinition } from 'tabulator-tables';

	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import PreRegistroFormModal from '$lib/components/visita/PreRegistroFormModal.svelte';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';

	interface Props {
		showHeader?: boolean;
		showTitle?: boolean;
	}

	let { showHeader = true, showTitle = true }: Props = $props();

	let preRegistros = $state<PreRegistroVisita[]>([]);
	let loading = $state(false);
	let searchTerm = $state('');
	let gridWrapper = $state<any>(null);
	let selectedRows = $state<any[]>([]);

	// Modal state
	let showCreateModal = $state(false);
	let editingPreRegistro = $state<PreRegistroVisita | null>(null);

	const columns: ColumnDefinition[] = [
		{ title: 'Cédula', field: 'cedula', width: 120 },
		{
			title: 'Visitante',
			field: 'nombre',
			formatter: (cell: any) => {
				const data = cell.getData();
				return `${data.nombre} ${data.apellido} ${data.segundoNombre || ''} ${data.segundoApellido || ''}`.trim();
			}
		},
		{ title: 'Empresa', field: 'empresaNombre', width: 150 },
		{ title: 'Anfitrión', field: 'anfitrion', width: 150 },
		{
			title: 'Fecha Esperada',
			field: 'fechaEsperada',
			width: 140,
			formatter: (cell: any) => {
				const val = cell.getValue();
				return val ? new Date(val).toLocaleDateString() : '';
			}
		},
		{ title: 'Hora', field: 'horaEsperada', width: 80 },
		{
			title: 'Estado',
			field: 'estado',
			width: 100,
			hozAlign: 'center' as 'center',
			formatter: (cell: any) => {
				const val = cell.getValue();
				let color = 'bg-gray-500';
				if (val === 'PENDIENTE') color = 'bg-yellow-500/20 text-yellow-500';
				if (val === 'COMPLETADO') color = 'bg-green-500/20 text-green-500';
				if (val === 'CANCELADO') color = 'bg-red-500/20 text-red-500';
				return `<span class="px-2 py-0.5 rounded text-xs font-semibold ${color}">${val}</span>`;
			}
		},
		{
			title: 'Acciones',
			width: 120,
			hozAlign: 'center' as 'center',
			headerSort: false,
			formatter: (cell: any) => {
				return `<button class="p-1 text-red-400 hover:text-red-300 transition-colors" title="Cancelar"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 18 18"/></svg></button>`;
			},
			cellClick: (e: any, cell: any) => {
				e.stopPropagation();
				handleCancelPreRegistro(cell.getRow().getData().id.toString());
			}
		}
	];

	// Grid State
	let toolbarColumns = $state<any[]>([]);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	function handleToggleFilters() {
		showHeaderFilters = !showHeaderFilters;
		if (typeof window !== 'undefined') {
			localStorage.setItem('tabulator-header-filters', String(showHeaderFilters));
		}
		if (gridWrapper) {
			setTimeout(() => {
				gridWrapper.redraw(true);
			}, 50);
		}
	}

	export async function loadData() {
		loading = true;
		try {
			preRegistros = await preRegistroVisitaService.getPendientes();
		} catch (error) {
			console.error(error);
			toast.error('Error al cargar pre-registros');
		} finally {
			loading = false;
		}
	}

	// Expose methods if needed by parent
	export function setSearchTerm(term: string) {
		searchTerm = term;
		gridWrapper?.setFilter(term); // Assuming wrapper or tabulator logic handles this or we filter locally
	}

	async function handleCancelPreRegistro(id: string) {
		if (!confirm('¿Estás seguro de cancelar este pre-registro?')) return;
		try {
			await preRegistroVisitaService.cancel(id);
			toast.success('Pre-registro cancelado');
			loadData();
		} catch (e) {
			console.error(e);
			toast.error('Error al cancelar');
		}
	}

	function handleRowDblClick(e: any, row: any) {
		editingPreRegistro = row.getData();
		showCreateModal = true;
	}

	onMount(() => {
		loadData();
	});

	// Auto-update toolbar columns when grid is ready or changes
	// This is handled by binding in TabulatorWrapper
</script>

<div class="flex flex-col h-full bg-surface-1">
	<GridToolbar
		bind:searchTerm
		hasSelection={selectedRows.length > 0}
		selectionCount={selectedRows.length}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={() => handleToggleFilters()}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<button
					onclick={() => gridWrapper?.deselectAll()}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
				>
					<X size={14} /> Cancelar
				</button>
			{:else}
				<button
					onclick={loadData}
					class="p-1.5 text-secondary hover:text-primary transition-colors hover:bg-surface-3 rounded-lg border border-surface"
					title="Actualizar"
				>
					<RefreshCw size={16} class={loading ? 'animate-spin' : ''} />
				</button>
				<button
					onclick={() => {
						editingPreRegistro = null;
						showCreateModal = true;
					}}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<UserPlus size={14} />
					{showTitle ? 'Nuevo Pre-Registro' : 'Nuevo'}
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<div class="flex-1 overflow-hidden p-0 relative {showHeaderFilters ? '' : 'hide-filters'}">
		{#if loading && preRegistros.length === 0}
			<div
				class="absolute inset-0 flex items-center justify-center bg-surface-1/50 z-10 backdrop-blur-[1px]"
			>
				<span class="loading loading-spinner loading-md text-primary"></span>
			</div>
		{/if}

		<TabulatorWrapper
			bind:this={gridWrapper}
			bind:toolbarColumns
			data={preRegistros}
			{columns}
			class="h-full"
			pagination={true}
			layout="fitColumns"
			onRowDblClick={handleRowDblClick}
			withCheckboxSelection={true}
			onRowSelectionChanged={(data) => (selectedRows = data)}
			options={{
				...defaultTabulatorOptions,
				placeholder: 'No hay pre-registros pendientes'
			}}
		/>
	</div>
</div>

<PreRegistroFormModal
	bind:show={showCreateModal}
	initialData={editingPreRegistro}
	onClose={() => (showCreateModal = false)}
	on:success={() => {
		loadData();
		showCreateModal = false;
	}}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
