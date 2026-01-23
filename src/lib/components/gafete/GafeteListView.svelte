<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { toast } from 'svelte-5-french-toast';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { Plus, X, ListPlus, LayoutGrid } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import GafeteFormModal from './GafeteFormModal.svelte';
	import ResolveAlertModal from './modals/ResolveAlertModal.svelte';
	import BulkCreateGafeteModal from './modals/BulkCreateGafeteModal.svelte';

	// Logic & Services
	import * as gafeteService from '$lib/logic/gafete/gafeteService';
	import * as alertaGafeteService from '$lib/logic/alertaGafete/alertaGafeteService';
	import { getGafeteColumns } from '$lib/logic/gafete/gafeteColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import { currentUser } from '$lib/stores/auth';
	import type { GafeteResponse } from '$lib/types/gafete';

	// Props
	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'gafete-list', data }: Props = $props();

	$effect(() => {
		if (data?.openCreateModal) {
			setTimeout(() => {
				handleNew();
			}, 100);
		}
		if (data?.openCreateBatchModal) {
			setTimeout(() => {
				showBulkModal = true;
			}, 100);
		}
	});

	// State
	let gafetes = $state<GafeteResponse[]>([]);
	let loading = $state(false);
	let searchTerm = $state('');
	let showModal = $state(false);
	let showBulkModal = $state(false);
	let selectedGafete = $state<GafeteResponse | null>(null);
	let formLoading = $state(false);

	// Grid State
	let gridWrapper = $state<any>(null);
	let toolbarColumns = $state<any[]>([]);
	let selectedRows = $state<GafeteResponse[]>([]);

	// Estado para modal de resolución de alertas
	let showResolveModal = $state(false);
	let selectedAlertGafete = $state<GafeteResponse | null>(null);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Event listener cleanup
	let unlistenRefresh: UnlistenFn | null = null;

	// Column definitions
	const columns = $derived(
		getGafeteColumns({
			onResolve: (data) => handleResolve(data),
			onRecover: (data) => changeStatus(data, 'activo'),
			onLost: (data) => changeStatus(data, 'extraviado'),
			onDamage: (data) => changeStatus(data, 'danado'),
			onDelete: (data) => handleDelete(data),
			onEdit: (data) => handleEdit(data)
		})
	);

	async function loadGafetes() {
		loading = true;
		try {
			const result = await gafeteService.fetchAll();
			if (result.ok) {
				gafetes = result.data.gafetes;
			} else {
				toast.error(result.error);
			}
		} catch {
			toast.error('Error inesperado al cargar gafetes');
		} finally {
			loading = false;
		}
	}

	function handleNew() {
		selectedGafete = null;
		showModal = true;
	}

	function handleEdit(gafete: GafeteResponse) {
		selectedGafete = gafete;
		showModal = true;
	}

	function handleResolve(gafete: GafeteResponse) {
		selectedAlertGafete = gafete;
		showResolveModal = true;
	}

	async function changeStatus(data: GafeteResponse, newStatus: string) {
		if (loading) return;
		try {
			loading = true;
			const userId = $currentUser?.id;
			const result = await gafeteService.updateStatus(data.id, newStatus, userId);
			if (result.ok) {
				toast.success(`Estado actualizado a ${newStatus}`);
				await loadGafetes();
			} else {
				toast.error(result.error);
			}
		} finally {
			loading = false;
		}
	}

	async function handleResolveSubmit(notas: string) {
		if (!selectedAlertGafete?.alertaId) return;
		formLoading = true;
		const result = await alertaGafeteService.resolverAlerta(
			selectedAlertGafete.alertaId,
			notas,
			$currentUser?.id
		);
		if (result.ok) {
			toast.success('Alerta resuelta');
			showResolveModal = false;
			selectedAlertGafete = null;
			loadGafetes();
		} else {
			toast.error(result.error);
		}
		formLoading = false;
	}

	async function handleDelete(gafete: GafeteResponse) {
		if (!confirm(`¿Eliminar permanentemente el gafete ${gafete.numero}?`)) return;
		const result = await gafeteService.remove(
			gafete.numero.toString(),
			gafete.tipo,
			$currentUser?.id
		);
		if (result.ok) {
			toast.success('Gafete eliminado');
			loadGafetes();
		} else {
			toast.error(result.error);
		}
	}

	async function handleFormSubmit(data: any) {
		formLoading = true;
		const result = selectedGafete
			? await gafeteService.update(selectedGafete.numero.toString(), selectedGafete.tipo, data)
			: await gafeteService.create(data);

		if (result.ok) {
			toast.success(selectedGafete ? 'Gafete actualizado' : 'Gafete creado');
			showModal = false;
			loadGafetes();
		} else {
			toast.error(result.error);
		}
		formLoading = false;
	}

	async function handleBulkSubmit(data: any) {
		formLoading = true;
		const result = await gafeteService.createRange(data);
		if (result.ok) {
			toast.success(`${result.data.length} gafetes generados`);
			showBulkModal = false;
			loadGafetes();
		} else {
			toast.error(result.error);
		}
		formLoading = false;
	}

	onMount(async () => {
		loadGafetes();
		unlistenRefresh = await listen('gafetes:refresh', loadGafetes);
	});

	onDestroy(() => unlistenRefresh?.());
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2 shadow-sm z-10">
		<div class="flex items-center justify-between gap-6">
			<div>
				<h2 class="text-xl font-semibold text-primary">Gestión de Gafetes</h2>
				<p class="mt-1 text-xs text-secondary">
					Administración de inventario, estado físico y alertas de seguridad
				</p>
			</div>

			<div class="flex items-center gap-4 bg-surface-3 border border-surface p-1 rounded-lg px-3">
				<div class="flex items-center gap-3 text-[10px] font-bold uppercase tracking-wider">
					<div class="flex items-center gap-1.5 text-emerald-400">
						<span
							class="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"
						></span>
						Disponibles: {gafetes.filter((g) => g.status === 'disponible').length}
					</div>
					<div class="w-px h-3 bg-white/10"></div>
					<div class="flex items-center gap-1.5 text-blue-400">
						<span class="w-1.5 h-1.5 rounded-full bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.5)]"
						></span>
						En Uso: {gafetes.filter((g) => g.status === 'en_uso').length}
					</div>
					<div class="w-px h-3 bg-white/10"></div>
					<div class="flex items-center gap-1.5 text-rose-400">
						<span class="w-1.5 h-1.5 rounded-full bg-rose-500 shadow-[0_0_8px_rgba(244,63,94,0.5)]"
						></span>
						Dañados: {gafetes.filter((g) => g.status === 'danado').length}
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Toolbar & Grid -->
	<GridToolbar
		bind:searchTerm
		hasSelection={selectedRows.length > 0}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={() => {
			showHeaderFilters = !showHeaderFilters;
			if (typeof window !== 'undefined') {
				localStorage.setItem('tabulator-header-filters', String(showHeaderFilters));
			}
			if (gridWrapper) {
				setTimeout(() => {
					gridWrapper.redraw(true);
				}, 50);
			}
		}}
		onAdvancedExport={() => {}}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<!-- Modo Selección -->
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200">
					{#if selectedRows.length === 1}
						<button
							class="flex items-center gap-2 px-3 py-1.5
							       bg-amber-600/10 hover:bg-amber-600/20
							       text-amber-400 hover:text-amber-300
							       border border-amber-500/20 hover:border-amber-500/30
							       rounded-md text-sm font-medium transition-all"
							onclick={() => {
								handleEdit(selectedRows[0]);
								gridWrapper?.deselectAll();
							}}
						>
							<svg
								class="w-4 h-4"
								xmlns="http://www.w3.org/2000/svg"
								width="24"
								height="24"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
								><path d="M12 20h9" /><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z" /></svg
							>
							<span>Editar</span>
						</button>
					{/if}

					<button
						class="flex items-center gap-2 px-3 py-1.5
						       bg-surface-3 hover:bg-surface-hover
						       text-secondary hover:text-primary
						       border border-surface
						       rounded-md text-sm font-medium transition-all"
						onclick={() => {
							gridWrapper?.deselectAll();
							selectedRows = [];
						}}
					>
						<X size={16} />
						<span>Cancelar</span>
					</button>
				</div>
			{:else}
				<!-- Acciones normales -->
				<button
					onclick={handleNew}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<Plus size={14} /> Nuevo
				</button>
				<button
					onclick={() => (showBulkModal = true)}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-hover hover:text-primary text-sm font-medium transition-colors"
				>
					<ListPlus size={14} /> Generar Lote
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<div
		class="flex-1 overflow-hidden relative bg-surface-1 {showHeaderFilters ? '' : 'hide-filters'}"
	>
		{#if loading && gafetes.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary opacity-20"></div>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={gafetes}
				{columns}
				class="h-full"
				persistenceID="gafete-list-v5"
				pagination={true}
				withCheckboxSelection={true}
				options={{
					...defaultTabulatorOptions,
					layout: 'fitData',
					placeholder: 'No se encontraron gafetes',
					selectableRowsCheck: (row) => {
						// Permitir selección solo si no está en estado perdido
						const data = row.getData() as GafeteResponse;
						return data.status !== 'perdido';
					}
				}}
				onRowSelectionChanged={(rows) => {
					selectedRows = rows as GafeteResponse[];
				}}
			/>
		{/if}
	</div>
</div>

<!-- Modals -->
{#if showModal}
	<GafeteFormModal
		show={showModal}
		gafete={selectedGafete}
		loading={formLoading}
		onSave={handleFormSubmit}
		onClose={() => {
			showModal = false;
			selectedGafete = null;
		}}
	/>
{/if}

{#if showBulkModal}
	<BulkCreateGafeteModal
		show={showBulkModal}
		loading={formLoading}
		onSave={handleBulkSubmit}
		onClose={() => (showBulkModal = false)}
	/>
{/if}

{#if showResolveModal && selectedAlertGafete}
	<ResolveAlertModal
		show={showResolveModal}
		gafeteNumero={selectedAlertGafete.numero.toString()}
		nombrePersona={selectedAlertGafete.quienPerdio || 'Desconocido'}
		fechaReporte={selectedAlertGafete.fechaPerdido || new Date().toISOString()}
		loading={formLoading}
		onResolve={handleResolveSubmit}
		onCancel={() => {
			showResolveModal = false;
			selectedAlertGafete = null;
		}}
	/>
{/if}

<style>
	/* Ocultar filtros de encabezado cuando se desactiven */
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
