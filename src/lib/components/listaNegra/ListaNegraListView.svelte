<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { toast } from 'svelte-5-french-toast';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { Plus, RotateCw, Lock, LockOpen, UserPlus, Pencil, Ban } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ListaNegraFormModal from './ListaNegraFormModal.svelte';
	import BlacklistConfirmModal from './blacklistForm/BlacklistConfirmModal.svelte';

	// Logic & Services
	import * as listaNegraService from '$lib/logic/listaNegra/listaNegraService';
	import { getListaNegraColumns } from '$lib/logic/listaNegra/listaNegraColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import { currentUser } from '$lib/stores/auth';
	import { can } from '$lib/logic/permissions';
	import type { ListaNegraResponse, AddToListaNegraInput } from '$lib/types/listaNegra';
	import { activeTabId } from '$lib/stores/tabs';
	import { shortcutCommand, setActiveContext, clearCommand } from '$lib/shortcuts';
	import { searchByType } from '$lib/api/searchService';

	interface Props {
		tabId: string;
		data?: any;
	}

	let { tabId, data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			setTimeout(() => {
				if (!showFormModal) openFormModal(null);
			}, 100);
		}
	});

	// State
	let bloqueados = $state<ListaNegraResponse[]>([]);
	let loading = $state(false);
	let searchTerm = $state('');
	let selectedRows = $state<ListaNegraResponse[]>([]);

	// Grid Controller
	let gridWrapper = $state<any>(null);
	let toolbarColumns = $state<any[]>([]);

	// Modales
	let showFormModal = $state(false);
	let editingBloqueado = $state<ListaNegraResponse | null>(null);
	let formLoading = $state(false);
	let showConfirmModal = $state(false);
	let confirmMotivo = $state('');
	let confirmActionType = $state<'unblock' | 'reblock'>('unblock');
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Keyboard handling
	let unsubscribeKeyboard: (() => void) | null = null;

	const canManage = $derived(can($currentUser, 'MANAGE_BLACKLIST'));
	const columns = $derived(getListaNegraColumns());

	async function loadListaNegra() {
		loading = true;
		const result = await listaNegraService.fetchAll();
		if (result.ok) {
			bloqueados = result.data.bloqueados;
		} else {
			toast.error(result.error);
		}
		loading = false;
	}

	function openFormModal(bloqueado: ListaNegraResponse | null) {
		editingBloqueado = bloqueado;
		showFormModal = true;
	}

	async function handleSaveForm(input: AddToListaNegraInput): Promise<boolean> {
		formLoading = true;
		let result;
		if (editingBloqueado) {
			result = await listaNegraService.update(editingBloqueado.id, {
				nivelSeveridad: input.nivelSeveridad,
				motivoBloqueo: input.motivoBloqueo,
				empresaId: input.empresaId,
				empresaNombre: input.empresaNombre
			});
		} else {
			result = await listaNegraService.add(input);
		}

		if (result.ok) {
			toast.success(editingBloqueado ? 'Información actualizada' : 'Persona bloqueada');
			showFormModal = false;
			loadListaNegra();
			return true;
		} else {
			toast.error(result.error);
			return false;
		}
	}

	function openConfirmModal(bloqueado: ListaNegraResponse, type: 'unblock' | 'reblock') {
		editingBloqueado = bloqueado;
		confirmActionType = type;
		confirmMotivo = '';
		showConfirmModal = true;
	}

	async function handleSearch(term: string) {
		searchTerm = term;
		if (!term || term.trim().length < 2) {
			if (gridWrapper) gridWrapper.replaceData(bloqueados);
			return;
		}
		try {
			const results = await searchByType(term, 'lista_negra', 100);
			const matchedIds = new Set(results.map((r) => r.id));
			const filtered = bloqueados.filter((b) => matchedIds.has(b.id));
			if (gridWrapper) gridWrapper.replaceData(filtered);
		} catch (e) {
			console.error('Error en búsqueda inteligente:', e);
			if (gridWrapper) gridWrapper.getTable()?.setFilter('nombreCompleto', 'like', term);
		}
	}

	async function handleConfirmAction() {
		if (!editingBloqueado) return;
		formLoading = true;
		const result =
			confirmActionType === 'unblock'
				? await listaNegraService.unblock(editingBloqueado.id)
				: await listaNegraService.reblock(editingBloqueado.id);

		if (result.ok) {
			toast.success(
				confirmActionType === 'unblock' ? 'Persona desbloqueada' : 'Persona re-bloqueada'
			);
			showConfirmModal = false;
			loadListaNegra();
		} else {
			toast.error(result.error);
		}
		formLoading = false;
	}

	onMount(() => {
		loadListaNegra();
		unsubscribeKeyboard = shortcutCommand.subscribe((event) => {
			if (!event || $activeTabId !== tabId) return;
			if (event.command === 'create' && canManage) openFormModal(null);
			if (event.command === 'edit' && canManage && selectedRows.length === 1)
				openFormModal(selectedRows[0]);
			if (event.command === 'refresh') loadListaNegra();
			if (event.command === 'escape') {
				showFormModal = false;
				showConfirmModal = false;
			}
		});
	});

	onDestroy(() => unsubscribeKeyboard?.());

	$effect(() => {
		if ($activeTabId === tabId) setActiveContext('lista-negra');
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2 shadow-sm z-10">
		<div class="flex items-center justify-between gap-6">
			<div class="flex items-center gap-3">
				<div>
					<h2 class="text-xl font-semibold text-primary">Lista Negra</h2>
					<p class="mt-0.5 text-xs text-secondary">
						Control de accesos denegados y restricciones de seguridad
					</p>
				</div>
			</div>

			<div class="flex items-center gap-4 bg-surface-3 border border-surface p-1.5 rounded-lg px-4">
				<div class="flex items-center gap-4 text-[10px] font-bold uppercase tracking-wider">
					<div class="flex items-center gap-2 text-rose-400">
						<span
							class="w-1.5 h-1.5 rounded-full bg-rose-500 animate-pulse shadow-[0_0_8px_rgba(244,63,94,0.5)]"
						></span>
						Bloqueados: {bloqueados.filter((b) => b.isActive).length}
					</div>
					<div class="w-px h-3 bg-white/10"></div>
					<div class="flex items-center gap-2 text-emerald-400">
						<span class="w-1.5 h-1.5 rounded-full bg-emerald-500/50"></span>
						Histórico: {bloqueados.filter((b) => !b.isActive).length}
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Toolbar & Grid -->
	<GridToolbar
		{searchTerm}
		onSearch={handleSearch}
		hasSelection={selectedRows.length > 0}
		selectionCount={selectedRows.length}
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
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if canManage && selectedRows.length === 0}
				<button
					onclick={() => openFormModal(null)}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
				>
					<UserPlus size={14} /> Bloquear
				</button>
			{/if}
		{/snippet}

		{#snippet selectionActions()}
			{#if selectedRows.length === 1 && canManage}
				<button
					onclick={() => openFormModal(selectedRows[0])}
					class="flex items-center gap-2 px-3 py-1.5 bg-amber-600/10 hover:bg-amber-600/20 text-amber-400 hover:text-amber-300 border border-amber-500/20 hover:border-amber-500/30 rounded-md text-sm font-medium transition-all"
				>
					<Pencil size={16} /> Editar
				</button>

				{#if selectedRows[0].isActive}
					<button
						onclick={() => openConfirmModal(selectedRows[0], 'unblock')}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-md hover:bg-emerald-500/20 text-sm font-medium transition-colors"
					>
						<LockOpen size={14} /> Desbloquear
					</button>
				{:else}
					<button
						onclick={() => openConfirmModal(selectedRows[0], 'reblock')}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded-md hover:bg-amber-500/20 text-sm font-medium transition-colors"
					>
						<Lock size={14} /> Re-bloquear
					</button>
				{/if}
			{/if}
		{/snippet}
	</GridToolbar>

	<div
		class="flex-1 overflow-hidden relative bg-surface-1 {showHeaderFilters ? '' : 'hide-filters'}"
	>
		{#if loading && bloqueados.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary opacity-20"></div>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={bloqueados}
				{columns}
				persistenceID="lista-negra-v1"
				onRowSelectionChanged={(data) => (selectedRows = data)}
				withCheckboxSelection={true}
				class="h-full"
				pagination={true}
				options={{
					...defaultTabulatorOptions,
					layout: 'fitColumns',
					placeholder: 'No se encontraron registros en lista negra'
				}}
			/>
		{/if}
	</div>
</div>

<!-- Modals -->
<ListaNegraFormModal
	show={showFormModal}
	bloqueado={editingBloqueado}
	loading={formLoading}
	onSave={handleSaveForm}
	onClose={() => {
		showFormModal = false;
		editingBloqueado = null;
	}}
/>

<BlacklistConfirmModal
	show={showConfirmModal}
	contratistaName={editingBloqueado?.nombreCompleto || ''}
	motivo={confirmMotivo}
	onConfirm={handleConfirmAction}
	onCancel={() => {
		showConfirmModal = false;
		editingBloqueado = null;
	}}
	onMotivoChange={(v) => (confirmMotivo = v)}
/>

<style>
	/* Ocultar filtros de encabezado cuando se desactiven */
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
