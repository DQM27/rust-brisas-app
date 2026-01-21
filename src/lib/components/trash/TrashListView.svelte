<script lang="ts">
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { RotateCcw, AlertCircle } from 'lucide-svelte';
	import { TabulatorWrapper } from '$lib/components/tabulator';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';

	import type { TrashService, TrashItem } from '$lib/logic/trash/trashService';
	import TrashFormModal from './TrashFormModal.svelte';

	interface Props<T extends TrashItem> {
		title?: string;
		service: TrashService<T>;
		columnDefs: any[]; // Tabulator definition
		gridId: string;
		onBack: () => void;
		rowIdField?: string;
		entityName?: string;
	}

	let {
		service,
		columnDefs,
		gridId,
		rowIdField = 'id',
		entityName = 'Elemento'
	}: Props<any> = $props();

	// State
	let items = $state<any[]>([]);
	let error = $state('');
	let loading = $state(false);
	let selectedRows = $state<any[]>([]);
	let searchTerm = $state('');
	let gridWrapper: any = $state();
	let toolbarColumns = $state<
		{ field: string; title: string; visible: boolean; frozen: boolean }[]
	>([]);

	// Modal State
	let showModal = $state(false);
	let modalLoading = $state(false);
	let modalAction = $state<'restore' | 'delete' | null>(null);
	let itemToProcess = $state<any | null>(null);

	// Actions
	async function loadArchived() {
		error = '';
		loading = true;
		try {
			const result = await service.getArchived();
			if (result.ok) {
				items = result.data;
				if (gridWrapper) {
					gridWrapper.replaceData(items);
				}
			} else {
				error = result.error || 'Error desconocido';
			}
		} catch (err) {
			console.error(err);
			error = 'Error al cargar elementos eliminados';
		}
		loading = false;
	}

	function confirmRestore(rows: any[]) {
		if (!rows || rows.length === 0) return;
		// For now take the first one, or handle bulk
		itemToProcess = rows[0];
		modalAction = 'restore';
		showModal = true;
	}

	async function handleConfirmAction() {
		if (!itemToProcess || !modalAction) return;

		const id = itemToProcess[rowIdField];
		modalLoading = true;

		try {
			if (modalAction === 'restore') {
				const result = await service.restore(id);
				if (result.ok) {
					toast.success(`${entityName} restaurado`);
					await loadArchived();
					selectedRows = [];
					gridWrapper?.deselectAll();
					showModal = false;
					itemToProcess = null;
				} else {
					toast.error(result.error || 'Error al restaurar');
				}
			}
		} catch (_e) {
			toast.error('Error inesperado');
		}

		modalLoading = false;
	}

	onMount(() => {
		loadArchived();
	});
</script>

<div class="h-full flex flex-col">
	<div class="flex-1 overflow-hidden relative flex flex-col">
		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else}
			<GridToolbar
				bind:searchTerm
				searchable={true}
				onSearch={(term) => {
					gridWrapper?.getTable()?.setFilter('nombreCompleto', 'like', term);
				}}
				onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
				onFitColumns={() => gridWrapper?.fitColumns()}
				onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
				onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
				columns={toolbarColumns}
				hasSelection={selectedRows.length > 0}
			>
				{#snippet primaryActions()}
					{#if selectedRows.length > 0}
						<button
							class="flex items-center gap-2 px-3 py-1.5 bg-green-600/10 hover:bg-green-600/20 text-green-400 border border-green-500/20 rounded-md text-sm font-medium transition-all"
							onclick={() => confirmRestore(selectedRows)}
						>
							<RotateCcw size={16} />
							<span>Restaurar ({selectedRows.length})</span>
						</button>
					{/if}
				{/snippet}
			</GridToolbar>

			<div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
				<TabulatorWrapper
					bind:this={gridWrapper}
					bind:toolbarColumns
					columns={columnDefs}
					data={items}
					withCheckboxSelection={true}
					persistenceID={gridId}
					pagination={true}
					onRowSelectionChanged={(data) => (selectedRows = data)}
					options={{
						layout: 'fitData',
						placeholder: 'No hay elementos en la papelera'
					}}
				/>
			</div>
		{/if}
	</div>
</div>

<TrashFormModal
	show={showModal}
	item={itemToProcess}
	action={modalAction}
	{entityName}
	loading={modalLoading}
	onConfirm={handleConfirmAction}
	onClose={() => (showModal = false)}
/>



