<script lang="ts">
	import { onMount } from 'svelte';
	import { CalendarClock } from 'lucide-svelte';
	import { toast } from 'svelte-5-french-toast';
	import { preRegistroVisitaService } from '$lib/services/preRegistroVisitaService';
	import type { PreRegistroVisita } from '$lib/types/ingreso-nuevos';
	import { openTab } from '$lib/stores/tabs';
	import PreRegistroFormModal from '$lib/components/visita/PreRegistroFormModal.svelte';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import { getPreRegistroColumns } from '$lib/logic/visita/preRegistroColumns';

	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import { Plus } from 'lucide-svelte';

	interface Props {
		showHeader?: boolean;
		showTitle?: boolean;
		viewMode?: 'grid' | 'focus';
		toolbarColumns?: any[];
	}
	let {
		showHeader = true,
		showTitle = true,
		viewMode = $bindable('grid'),
		toolbarColumns = $bindable([])
	}: Props = $props();

	let pendientes = $state<PreRegistroVisita[]>([]);
	let loading = $state(false);
	let showCreateModal = $state(false);
	let selectedPreRegistro = $state<PreRegistroVisita | null>(null);

	// Toolbar State
	let searchTerm = $state('');
	let showHeaderFilters = $state(false);

	// Tabulator State
	let gridWrapper = $state<any>(null);
	let tabulatorColumns = $derived(
		getPreRegistroColumns(handleIngreso, handleCancel, showHeaderFilters)
	);

	// Derived filtered data
	let filteredPendientes = $derived.by(() => {
		if (!searchTerm) return pendientes;
		const lower = searchTerm.toLowerCase();
		return pendientes.filter(
			(p) =>
				p.nombre?.toLowerCase().includes(lower) ||
				p.cedula?.toLowerCase().includes(lower) ||
				p.empresaNombre?.toLowerCase().includes(lower) ||
				p.anfitrion?.toLowerCase().includes(lower)
		);
	});

	onMount(() => {
		loadPendientes();
	});

	async function loadPendientes() {
		loading = true;
		try {
			const rawData = await preRegistroVisitaService.getPendientes();

			// Normalize data (Handling SurrealDB IDs)
			pendientes = rawData.map((p) => {
				let id = p.id;
				// Maintain backward compatibility with SurrealDB object IDs
				if (typeof p.id === 'object' && p.id !== null) {
					// @ts-ignore
					if (p.id.tb && p.id.id && p.id.id.String) {
						// @ts-ignore
						id = `${p.id.tb}:${p.id.id.String}`;
					} else {
						id = (p.id as any).toString();
					}
				}

				return {
					...p,
					id: id
				};
			});
			// Wrapper updates via reactive 'data' prop now
		} catch (error) {
			console.error('Error loading pendientes:', error);
			toast.error('Error cargando visitas esperadas');
		} finally {
			loading = false;
		}
	}

	function handleGridSearch(term: string) {
		searchTerm = term;
	}

	function handleIngreso(row: PreRegistroVisita) {
		openTab({
			componentKey: 'visitas-list',
			title: 'Gestión de Visitas',
			id: 'visitas-list',
			data: {
				openCreateModal: true,
				initialPersonId: row.cedula
			},
			focusOnOpen: true
		});
	}

	async function handleCancel(id: string) {
		if (!confirm('¿Está seguro de borrar este registro?')) return;
		try {
			await preRegistroVisitaService.cancel(id);
			toast.success('Registro borrado');
			loadPendientes();
		} catch (e) {
			console.error(e);
			toast.error('Error al borrar');
		}
	}

	function handleRowDblClick(e: any, row: any) {
		const data = row.getData();
		selectedPreRegistro = data;
		showCreateModal = true;
	}

	export function openCreateModal() {
		selectedPreRegistro = null;
		showCreateModal = true;
	}

	export function autoSizeColumns() {
		gridWrapper?.autoSizeColumns();
	}

	export function fitColumns() {
		gridWrapper?.fitColumns();
	}

	export function toggleColumn(field: string) {
		gridWrapper?.toggleColumn(field);
	}

	export function toggleFreeze(field: string) {
		gridWrapper?.toggleFreeze(field);
	}

	export function toggleFilters() {
		showHeaderFilters = !showHeaderFilters;
		if (gridWrapper) {
			setTimeout(() => {
				gridWrapper.redraw(true);
			}, 50);
		}
	}

	export function setSearchTerm(term: string) {
		searchTerm = term;
	}
</script>

<div class="flex flex-col h-full bg-surface-1">
	<style>
		:global(.hide-filters .tabulator-header-filter) {
			display: none !important;
		}
	</style>
	{#if showHeader}
		<div class="px-6 pt-4 pb-2 bg-surface-2 border-b border-surface">
			{#if showTitle}
				<div class="flex items-center justify-between gap-4 mb-4">
					<div>
						<h2 class="text-xl font-semibold text-primary">Visitas Esperadas</h2>
						<p class="text-sm text-secondary">Gestión de pre-registros y agenda</p>
					</div>
				</div>
			{/if}

			<GridToolbar
				{searchTerm}
				onSearch={handleGridSearch}
				onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
				onFitColumns={() => gridWrapper?.fitColumns()}
				onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
				onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
				onToggleFilters={() => {
					showHeaderFilters = !showHeaderFilters;
				}}
				columns={toolbarColumns}
			>
				{#snippet primaryActions()}
					<button class="form-btn-primary gap-2" onclick={openCreateModal}>
						<Plus size={16} />
						<span>Nuevo Pre-Registro</span>
					</button>
				{/snippet}
			</GridToolbar>
		</div>
	{/if}

	<div class="flex-1 overflow-hidden relative {showHeaderFilters ? '' : 'hide-filters'}">
		{#if loading && pendientes.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				data={filteredPendientes}
				columns={tabulatorColumns}
				bind:toolbarColumns
				class="h-full"
				layout="fitColumns"
				placeholder="No hay visitas esperadas"
				onRowDblClick={handleRowDblClick}
			/>
		{/if}
	</div>
</div>

{#if showCreateModal}
	<PreRegistroFormModal
		show={showCreateModal}
		initialData={selectedPreRegistro}
		on:success={loadPendientes}
		onClose={() => {
			showCreateModal = false;
			loadPendientes();
		}}
	/>
{/if}
