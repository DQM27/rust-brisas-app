<!-- src/lib/components/ingreso/IngresoProveedorListView.svelte -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, FileText, Users, History, X, LogIn, UserPlus } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ProveedorFormModal from '$lib/components/proveedor/ProveedorFormModal.svelte';
	import SalidaModal from './SalidaModal.svelte';
	import QuickEntryModal from './QuickEntryModal.svelte';
	import IngresoProveedorFormModal from './IngresoProveedorFormModal.svelte';
	import DateRangePicker from '$lib/components/shared/DateRangePicker.svelte';

	// Logic & Services
	import { ingresoProveedorService } from '$lib/services/ingresoProveedorService';
	import { createProveedor } from '$lib/logic/proveedor/proveedorService';
	import { getIngresoProveedorColumns } from '$lib/logic/ingreso/ingresoProveedorColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	// Stores
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	// Types
	import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'proveedor-ingreso-list' }: Props = $props();

	// State
	let ingresos = $state<IngresoProveedor[]>([]);
	let loading = $state(false);
	let error = $state('');
	let selectedRows = $state<IngresoProveedor[]>([]);
	let searchTerm = $state('');

	// View Mode
	type ViewMode = 'actives' | 'history';
	let viewMode = $state<ViewMode>('actives');
	const today = new Date().toLocaleDateString('en-CA');
	let dateRange = $state({ start: today, end: today });
	let hideActive = $state(false);

	// Modals State
	let showProveedorModal = $state(false);
	let showSalidaModal = $state(false);
	let selectedIngreso = $state<IngresoProveedor | null>(null);
	let salidaLoading = $state(false);
	let showQuickEntry = $state(false);
	let showIngresoModal = $state(false);
	let providerForIngreso = $state<any>(null);

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

	// Derived Data
	let filteredIngresos = $derived(
		viewMode === 'history' && hideActive ? ingresos.filter((i) => i.fechaSalida) : ingresos
	);

	// Keyboard Subscription
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new':
					if (!showIngresoModal && !showSalidaModal && !showQuickEntry) {
						showQuickEntry = true;
						clearCommand();
					}
					break;
				case 'escape':
					if (showIngresoModal) {
						showIngresoModal = false;
						clearCommand();
					} else if (showSalidaModal) {
						showSalidaModal = false;
						selectedIngreso = null;
						clearCommand();
					} else if (showQuickEntry) {
						showQuickEntry = false;
						clearCommand();
					}
					break;
				case 'refresh':
					loadIngresos();
					clearCommand();
					break;
			}
		});
	}

	// Data Loading
	async function loadIngresos() {
		loading = true;
		error = '';
		try {
			let data;
			if (viewMode === 'actives') {
				data = await ingresoProveedorService.getActivos();
			} else {
				data = await ingresoProveedorService.getHistorial();
			}
			ingresos = data;
			if (gridWrapper) {
				gridWrapper.replaceData(filteredIngresos);
			}
		} catch (err: any) {
			error = err.message || 'Error al cargar datos';
			toast.error(error);
			ingresos = [];
		} finally {
			loading = false;
		}
	}

	// Columns Definition
	let columns = $derived(
		getIngresoProveedorColumns(
			{
				onSalida: (ingreso) => handleSalida(ingreso)
			},
			viewMode
		)
	);

	// Handlers
	function handleDateRangeChange(event: CustomEvent<{ startDate: string; endDate: string }>) {
		dateRange.start = event.detail.startDate;
		dateRange.end = event.detail.endDate;
		loadIngresos();
	}

	function toggleViewMode(mode: ViewMode) {
		if (viewMode === mode) return;
		viewMode = mode;
		loadIngresos();
	}

	function handleSalida(ingreso: IngresoProveedor) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	async function handleSalidaConfirm(event: CustomEvent) {
		const { devolvioGafete, observaciones } = event.detail;
		if (!selectedIngreso || !$currentUser?.id) return;
		try {
			salidaLoading = true;
			await ingresoProveedorService.registrarSalida(
				selectedIngreso.id,
				$currentUser.id,
				observaciones,
				devolvioGafete
			);
			toast.success('Salida de proveedor registrada');
			showSalidaModal = false;
			selectedIngreso = null;
			loadIngresos();
		} catch (err: any) {
			toast.error('Error al registrar salida: ' + err.message);
		} finally {
			salidaLoading = false;
		}
	}

	function handleQuickEntrySelect(provider: any) {
		showQuickEntry = false;
		providerForIngreso = provider;
		setTimeout(() => {
			showIngresoModal = true;
		}, 100);
	}

	function handleModalComplete() {
		showIngresoModal = false;
		providerForIngreso = null;
		loadIngresos();
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

	function handleGoToCatalog() {
		openTab({
			componentKey: 'proveedor-list',
			title: 'Lista de Proveedores',
			id: 'proveedor-list',
			focusOnOpen: true
		});
	}

	// Tab State for title update
	import { useTabState } from '$lib/stores/tabs';
	const { updateTitle } = useTabState(tabId);

	// Lifecycle
	onMount(() => {
		loadIngresos();
		setupKeyboardSubscription();
		updateTitle('Ingresos Proveedor');
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === tabId) {
			setActiveContext('proveedor-ingreso-list');
		}
	});

	$effect(() => {
		if (gridWrapper && filteredIngresos) {
			gridWrapper.replaceData(filteredIngresos);
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2 shadow-sm z-10">
		<div class="flex items-center justify-between gap-6">
			<div>
				<h2 class="text-xl font-semibold text-primary">
					{viewMode === 'actives' ? 'Ingresos Proveedor' : 'Historial de Proveedores'}
				</h2>
				<p class="mt-1 text-xs text-secondary">
					{viewMode === 'actives'
						? 'Proveedores ingresados actualmente'
						: 'Registro histórico de accesos'}
				</p>
			</div>

			<div class="flex items-center gap-4">
				<div class="relative flex items-center bg-surface-3 p-1 rounded-lg isolate">
					<div
						class="absolute top-1 bottom-1 rounded-md bg-white dark:bg-zinc-700 shadow-sm transition-all duration-300 ease-in-out z-[-1]"
						style="
								left: {viewMode === 'actives' ? '4px' : '50%'};
								right: {viewMode === 'actives' ? '50%' : '4px'};
								width: calc(50% - 6px);
							"
					></div>
					<button
						class="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors z-10
							{viewMode === 'actives' ? 'text-primary dark:text-white' : 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('actives')}
					>
						<Users size={16} />
						Activos
					</button>
					<button
						class="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors z-10
							{viewMode === 'history' ? 'text-primary dark:text-white' : 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('history')}
					>
						<History size={16} />
						Historial
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Toolbar -->
	<GridToolbar
		bind:searchTerm
		onSearch={(term) => {
			if (gridWrapper) {
				gridWrapper.getTable()?.setFilter('nombre', 'like', term);
			}
		}}
		hasSelection={selectedRows.length > 0}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		columns={toolbarColumns}
	>
		{#snippet CustomFilters()}
			{#if viewMode === 'history'}
				<div class="flex items-center border-r border-surface pr-4 mr-2" transition:fade>
					<DateRangePicker
						startDate={dateRange.start}
						endDate={dateRange.end}
						on:change={handleDateRangeChange}
					/>
				</div>
			{/if}
		{/snippet}

		{#snippet secondaryActions()}
			{#if viewMode === 'history'}
				<div class="flex items-center gap-2 px-3 py-1 border-x border-surface" transition:fade>
					<input
						type="checkbox"
						id="hideActiveProv"
						bind:checked={hideActive}
						class="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary"
					/>
					<label for="hideActiveProv" class="text-sm text-secondary cursor-pointer select-none">
						Solo Finalizados
					</label>
				</div>
			{/if}
			<button
				onclick={handleGoToCatalog}
				class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
			>
				<FileText size={14} /> Listado
			</button>
		{/snippet}

		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
					>
						<X size={14} /> Cancelar
					</button>
					{#if viewMode === 'actives' && selectedRows.length === 1}
						<button
							onclick={() => handleSalida(selectedRows[0])}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
						>
							<History size={14} /> Salida
						</button>
					{/if}
				</div>
			{:else if viewMode === 'actives'}
				<div class="flex items-center gap-2">
					<button
						onclick={() => (showQuickEntry = true)}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
					>
						<LogIn size={14} /> Nuevo
					</button>
					<button
						onclick={() => (showProveedorModal = true)}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
					>
						<UserPlus size={14} /> Proveedor
					</button>
				</div>
			{/if}
		{/snippet}
	</GridToolbar>

	<!-- Content -->
	<div
		class="flex-1 overflow-hidden relative bg-surface-1 border-t border-surface {showHeaderFilters
			? ''
			: 'hide-filters'}"
	>
		{#if error}
			<div class="p-8 text-center">
				<div
					class="inline-flex items-center gap-3 rounded-lg border border-error bg-error/10 p-4 text-error"
				>
					<AlertCircle size={20} />
					<div class="text-left">
						<div class="font-medium">Error al cargar datos</div>
						<div class="text-xs opacity-80">{error}</div>
					</div>
				</div>
			</div>
		{:else if loading && ingresos.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={filteredIngresos}
				{columns}
				withCheckboxSelection={true}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				persistenceID="ingresos-proveedores-v1"
				options={{
					...defaultTabulatorOptions,
					placeholder: 'No hay registros para mostrar'
				}}
			/>
		{/if}
	</div>
</div>

<ProveedorFormModal
	show={showProveedorModal}
	onClose={() => (showProveedorModal = false)}
	onSave={async (data) => {
		try {
			const res = await createProveedor(data as any);
			if (res.ok) {
				toast.success('Proveedor creado');
				showProveedorModal = false;
				loadIngresos();
			} else {
				toast.error(res.error);
			}
		} catch {
			toast.error('Error al crear proveedor');
		}
	}}
/>

<SalidaModal
	bind:show={showSalidaModal}
	ingreso={selectedIngreso}
	loading={salidaLoading}
	on:confirm={handleSalidaConfirm}
	on:close={() => {
		showSalidaModal = false;
		selectedIngreso = null;
	}}
/>

<QuickEntryModal
	bind:show={showQuickEntry}
	onSelect={handleQuickEntrySelect}
	allowedTypes={['proveedor']}
	onClose={() => (showQuickEntry = false)}
/>

<IngresoProveedorFormModal
	bind:show={showIngresoModal}
	initialPerson={providerForIngreso}
	on:complete={handleModalComplete}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
