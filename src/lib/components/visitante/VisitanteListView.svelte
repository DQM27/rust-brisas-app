<!-- src/lib/components/visitante/VisitanteListView.svelte -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { Plus, Pencil, Trash2, X } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import VisitanteFormModal from '$lib/components/visitante/VisitanteFormModal.svelte';

	// Logic & Services
	import {
		listVisitantes,
		createVisitante,
		updateVisitante,
		deleteVisitante
	} from '$lib/logic/visitante/visitanteService';
	import { getVisitanteColumns } from '$lib/logic/visitante/visitanteColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	// Types
	import type {
		VisitanteResponse,
		CreateVisitanteInput,
		UpdateVisitanteInput
	} from '$lib/types/visitante';

	// Stores
	import { activeTabId } from '$lib/stores/tabs';
	import { get } from 'svelte/store';
	import {
		shortcutCommand,
		setActiveContext,
		clearCommand,
		shortcutRegistry
	} from '$lib/shortcuts';
	import { searchByType } from '$lib/api/searchService';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'visitante-list', data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			setTimeout(() => {
				if (!showModal) openFormModal(null);
			}, 100);
		}

		if (data?.search) {
			setTimeout(() => {
				handleSearch(data.search);
			}, 300);
		}
	});

	// State
	let visitantes = $state<VisitanteResponse[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let selectedRows = $state<VisitanteResponse[]>([]);
	let searchTerm = $state('');

	// Modals State
	let showModal = $state(false);
	let selectedVisitante = $state<VisitanteResponse | null>(null);
	let modalLoading = $state(false);

	// Grid State
	let gridWrapper = $state<any>(null);
	let toolbarColumns = $state<
		{ field: string; title: string; visible: boolean; frozen: boolean }[]
	>([]);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Keyboard Subscription
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = shortcutCommand.subscribe((event) => {
			if (!event) return;
			const current = get(activeTabId);
			if (current !== tabId) return;

			switch (event.command) {
				case 'create':
					if (!showModal) {
						openFormModal(null);
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) showModal = false;
					clearCommand();
					break;
				case 'refresh':
					loadData();
					clearCommand();
					break;
			}
		});
	}

	// Data Loading
	const loadData = async () => {
		loading = true;
		error = null;
		try {
			const res = await listVisitantes();
			if (res.ok) {
				visitantes = res.data;
				if (gridWrapper) {
					gridWrapper.replaceData(visitantes);
				}
			} else {
				error = res.error;
				toast.error(res.error);
			}
		} finally {
			loading = false;
		}
	};

	// Columns Definition
	let columns = $derived(getVisitanteColumns());

	// Modal Handlers
	function openFormModal(visitante: VisitanteResponse | null) {
		selectedVisitante = visitante;
		showModal = true;
	}

	async function handleSave(data: CreateVisitanteInput | UpdateVisitanteInput) {
		modalLoading = true;
		try {
			let result;
			if (selectedVisitante) {
				result = await updateVisitante(selectedVisitante.id, data as UpdateVisitanteInput);
			} else {
				result = await createVisitante(data as CreateVisitanteInput);
			}

			if (result.ok) {
				toast.success(selectedVisitante ? 'Visitante actualizado' : 'Visitante creado');
				loadData();
				showModal = false;
				return true;
			} else {
				toast.error(result.error);
				return false;
			}
		} finally {
			modalLoading = false;
		}
	}

	// Delete Handlers
	async function confirmDelete(visitante: VisitanteResponse) {
		if (
			!confirm(
				`¿Estás seguro de eliminar al visitante "${visitante.nombre} ${visitante.apellido}"?`
			)
		)
			return;
		const res = await deleteVisitante(visitante.id);
		if (res.ok) {
			toast.success('Visitante eliminado');
			loadData();
		} else {
			toast.error(res.error);
		}
	}

	async function handleDeleteMultiple(selection: VisitanteResponse[]) {
		if (!confirm(`¿Eliminar ${selection.length} visitantes?`)) return;
		const toastId = toast.loading('Eliminando...');
		let errors = 0;
		for (const p of selection) {
			const res = await deleteVisitante(p.id);
			if (!res.ok) errors++;
		}
		if (errors === 0) {
			toast.success('Visitantes eliminados', { id: toastId });
		} else {
			toast.error(`Errores: ${errors}`, { id: toastId });
		}
		loadData();
		gridWrapper?.deselectAll();
	}

	async function handleSearch(term: string) {
		searchTerm = term;
		if (!term || term.trim().length < 2) {
			if (gridWrapper) gridWrapper.replaceData(visitantes);
			return;
		}
		try {
			const results = await searchByType(term, 'visitante', 100);
			const matchedIds = new Set(results.map((r) => r.id));
			const filtered = visitantes.filter((v) => matchedIds.has(v.id));
			if (gridWrapper) gridWrapper.replaceData(filtered);
		} catch (e) {
			console.error('Error en búsqueda inteligente:', e);
			if (gridWrapper) gridWrapper.getTable()?.setFilter('nombre', 'like', term);
		}
	}

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

	// Lifecycle
	onMount(() => {
		loadData();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === tabId) {
			shortcutRegistry.setScope('list');
			setActiveContext('visitante-list');
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary">Lista de Visitantes</h2>
				<p class="mt-1 text-sm text-secondary">Gestión y visualización de visitantes registrados</p>
			</div>
		</div>
	</div>

	<!-- Toolbar -->
	<GridToolbar
		bind:searchTerm
		onSearch={handleSearch}
		hasSelection={selectedRows.length > 0}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
						title="Cancelar selección"
					>
						<X size={14} /> Cancelar
					</button>

					{#if selectedRows.length === 1}
						<button
							onclick={() => openFormModal(selectedRows[0])}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded-md hover:bg-amber-500/20 text-sm font-medium transition-colors"
						>
							<Pencil size={14} /> Editar
						</button>
					{/if}

					<button
						onclick={() => handleDeleteMultiple(selectedRows)}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
					>
						<Trash2 size={14} /> Eliminar ({selectedRows.length})
					</button>
				</div>
			{:else}
				<button
					onclick={() => openFormModal(null)}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<Plus size={14} /> Nuevo Visitante
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<!-- Content -->
	<div
		class="flex-1 overflow-hidden relative bg-surface-1 border-t border-surface {showHeaderFilters
			? ''
			: 'hide-filters'}"
	>
		{#if loading && visitantes.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else if error}
			<div class="p-8 text-center text-red-400">{error}</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={visitantes}
				{columns}
				withCheckboxSelection={true}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				onRowDblClick={(e: any, row: any) => openFormModal(row.getData())}
				persistenceID="visitante-list-v1"
				options={{
					...defaultTabulatorOptions,
					placeholder: 'No hay visitantes registrados'
				}}
			/>
		{/if}
	</div>
</div>

<VisitanteFormModal
	show={showModal}
	visitante={selectedVisitante}
	loading={modalLoading}
	onSave={handleSave}
	onClose={() => (showModal = false)}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
