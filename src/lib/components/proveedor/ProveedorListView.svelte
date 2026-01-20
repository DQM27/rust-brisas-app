<!-- src/lib/components/proveedor/ProveedorListView.svelte -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { Plus, Pencil, Trash2, X } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ProveedorFormModal from '$lib/components/proveedor/ProveedorFormModal.svelte';

	// Logic & Services
	import {
		fetchAllProveedores,
		createProveedor,
		updateProveedor,
		deleteProveedor,
		changeStatus
	} from '$lib/logic/proveedor/proveedorService';
	import { getProveedorColumns } from '$lib/logic/proveedor/proveedorColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	// Types
	import type {
		ProveedorResponse,
		CreateProveedorInput,
		UpdateProveedorInput,
		EstadoProveedor
	} from '$lib/types/proveedor';
	import { searchByType } from '$lib/api/searchService';

	// Stores
	import { activeTabId } from '$lib/stores/tabs';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'proveedor-list', data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			setTimeout(() => {
				if (!showModal) openFormModal(null);
			}, 200);
		}

		if (data?.search) {
			setTimeout(() => {
				handleSearch(data.search);
			}, 300);
		}
	});

	// State
	let proveedores = $state<ProveedorResponse[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let selectedRows = $state<ProveedorResponse[]>([]);
	let searchTerm = $state('');

	// Modals State
	let showModal = $state(false);
	let selectedProveedor = $state<ProveedorResponse | null>(null);
	let isReadOnlyModal = $state(false);
	let modalLoading = $state(false);
	let isUpdatingStatus = false;

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
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new':
					if (!showModal) {
						openFormModal(null);
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) {
						showModal = false;
						clearCommand();
					}
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
			const res = await fetchAllProveedores();
			if (res.ok) {
				proveedores = res.data;
				if (gridWrapper) {
					gridWrapper.replaceData(proveedores);
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
	let columns = $derived(
		getProveedorColumns({
			onStatusToggle: (id, currentStatus) => handleStatusChange(id, currentStatus)
		})
	);

	// Modal Handlers
	function openFormModal(proveedor: ProveedorResponse | null, readonly: boolean = false) {
		selectedProveedor = proveedor;
		isReadOnlyModal = readonly;
		showModal = true;
	}

	async function handleSave(data: CreateProveedorInput | UpdateProveedorInput) {
		modalLoading = true;
		try {
			let result;
			if (selectedProveedor) {
				result = await updateProveedor(selectedProveedor.id, data as UpdateProveedorInput);
			} else {
				result = await createProveedor(data as CreateProveedorInput);
			}

			if (result.ok) {
				toast.success(selectedProveedor ? 'Proveedor actualizado' : 'Proveedor creado');
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

	// Status & Delete Handlers
	async function handleStatusChange(id: string, currentStatus: string) {
		if (isUpdatingStatus) return;
		isUpdatingStatus = true;
		const newStatus: EstadoProveedor = currentStatus === 'ACTIVO' ? 'INACTIVO' : 'ACTIVO';
		const toastId = toast.loading(`Cambiando estado a ${newStatus}...`);
		try {
			const res = await changeStatus(id, newStatus);
			if (res.ok) {
				toast.success('Estado actualizado', { id: toastId });
				loadData();
			} else {
				toast.error(res.error, { id: toastId });
			}
		} catch (e) {
			toast.error('Error al cambiar estado', { id: toastId });
		} finally {
			isUpdatingStatus = false;
		}
	}

	async function confirmDelete(proveedor: ProveedorResponse) {
		if (!confirm(`¿Estás seguro de eliminar al proveedor "${proveedor.nombre}"?`)) return;
		const res = await deleteProveedor(proveedor.id);
		if (res.ok) {
			toast.success('Proveedor eliminado');
			loadData();
		} else {
			toast.error(res.error);
		}
	}

	async function handleDeleteMultiple(selection: ProveedorResponse[]) {
		if (!confirm(`¿Eliminar ${selection.length} proveedores?`)) return;
		const toastId = toast.loading('Eliminando...');
		let errors = 0;
		for (const p of selection) {
			const res = await deleteProveedor(p.id);
			if (!res.ok) errors++;
		}
		if (errors === 0) {
			toast.success('Proveedores eliminados', { id: toastId });
		} else {
			toast.error(`Errores: ${errors}`, { id: toastId });
		}
		loadData();
		gridWrapper?.deselectAll();
	}

	function handleRowDblClick(e: any, row: any) {
		const data = row.getData();
		openFormModal(data);
	}

	async function handleSearch(term: string) {
		searchTerm = term;
		if (!term || term.trim().length < 2) {
			if (gridWrapper) gridWrapper.replaceData(proveedores);
			return;
		}
		try {
			const results = await searchByType(term, 'proveedor', 100);
			const matchedIds = new Set(results.map((r) => r.id));
			const filtered = proveedores.filter((p) => matchedIds.has(p.id));
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
			setActiveContext('proveedor-list');
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary">Lista de Proveedores</h2>
				<p class="mt-1 text-sm text-secondary">
					Gestión y visualización de proveedores registrados
				</p>
			</div>
		</div>
	</div>

	<!-- Toolbar -->
	<GridToolbar
		{searchTerm}
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
					<Plus size={14} /> Nuevo
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
		{#if loading && proveedores.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else if error}
			<div class="p-8 text-center text-red-400">{error}</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={proveedores}
				{columns}
				withCheckboxSelection={true}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				onRowDblClick={handleRowDblClick}
				persistenceID="proveedor-list-v1"
				options={{
					...defaultTabulatorOptions,
					placeholder: 'No hay proveedores registrados'
				}}
			/>
		{/if}
	</div>
</div>

<ProveedorFormModal
	show={showModal}
	proveedor={selectedProveedor}
	readonly={isReadOnlyModal}
	loading={modalLoading}
	onSave={handleSave}
	onClose={() => (showModal = false)}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
