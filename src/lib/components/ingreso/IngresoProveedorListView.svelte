<!-- src/lib/components/ingreso/IngresoProveedorListView.svelte -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toastService } from '$lib/services/toastService';
	import { AlertCircle, FileText, Users, History, X, LogIn, UserPlus } from '@lucide/svelte';

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
	import { get } from 'svelte/store';
	import {
		shortcutCommand,
		setActiveContext,
		clearCommand,
		shortcutRegistry
	} from '$lib/shortcuts';

	// Types
	import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'proveedor-ingreso-list', data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			if (data.initialPersonId) {
				// Cargar proveedor y abrir modal directamente
				setTimeout(() => {
					providerForIngreso = { id: data.initialPersonId, tipo: 'proveedor' };
					showIngresoModal = true;
				}, 100);
			} else {
				setTimeout(() => {
					if (!showQuickEntry) showQuickEntry = true;
				}, 100);
			}
		}
	});

	// Reactive wrapper to satisfy Svelte 5 linter regarding effect dependencies
	let activeTabIdValue = $derived(tabId);

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
	let showQuickExit = $state(false);
	let showDetailModal = $state(false);

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
	let groupByField = $state<string | undefined>(undefined);

	// Derived Data
	let filteredIngresos = $derived(
		viewMode === 'history' && hideActive ? ingresos.filter((i) => i.fechaSalida) : ingresos
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
					if (!showIngresoModal && !showSalidaModal && !showQuickEntry && !showProveedorModal) {
						showProveedorModal = true;
						clearCommand();
					}
					break;
				case 'quick-entry':
					if (!showIngresoModal && !showQuickEntry) {
						showQuickEntry = true;
						clearCommand();
					}
					break;
				case 'quick-exit':
					if (!showSalidaModal && !showQuickExit) {
						showQuickExit = true;
						clearCommand();
					}
					break;
				case 'scan-badge':
					toastService.info('Modo escaneo activado');
					clearCommand();
					break;
				case 'scan-badge':
					toastService.info('Modo escaneo activado');
					clearCommand();
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
				const startLocal = new Date(dateRange.start + 'T00:00:00');
				const endLocal = new Date(dateRange.end + 'T23:59:59.999');
				data = await ingresoProveedorService.getHistorial({
					fechaInicio: startLocal.toISOString(),
					fechaFin: endLocal.toISOString()
				});
			}
			ingresos = data;
			if (gridWrapper) {
				gridWrapper.replaceData(filteredIngresos);
			}
		} catch (err: any) {
			error = err.message || 'Error al cargar datos';
			toastService.error(error);
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
			toastService.success('Salida de proveedor registrada');
			showSalidaModal = false;
			selectedIngreso = null;
			loadIngresos();
		} catch (err: any) {
			toastService.error('Error al registrar salida: ' + err.message);
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

	function handleOpenListado() {
		toastService.info('Abriendo listado de proveedores...');
		openTab({
			componentKey: 'proveedor-list',
			title: 'Lista de Proveedores',
			id: 'proveedor-list',
			focusOnOpen: true
		});
	}

	// Tab State for title update
	import { useTabState } from '$lib/stores/tabs';
	// Lifecycle
	onMount(() => {
		loadIngresos();
		setupKeyboardSubscription();
		useTabState(tabId).updateTitle('Ingresos Proveedor');
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === activeTabIdValue) {
			shortcutRegistry.setScope('list');
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
				<div class="flex items-center gap-2 border-l border-white/10 pl-3 ml-1" transition:fade>
					<DateRangePicker
						startDate={dateRange.start}
						endDate={dateRange.end}
						on:change={handleDateRangeChange}
					/>
					<div class="flex items-center gap-2 ml-2">
						<input
							type="checkbox"
							id="hideActiveProv"
							bind:checked={hideActive}
							class="rounded border-surface bg-surface-3"
						/>
						<label for="hideActiveProv" class="text-xs text-secondary cursor-pointer select-none">
							Solo Finalizados
						</label>
					</div>
				</div>
			{/if}
		{/snippet}

		{#snippet secondaryActions()}
			{#if viewMode === 'history'}
				<!-- Separator moved to start of block if needed, but IngresoListView uses border-l inside the div -->
				<div class="flex items-center gap-2 border-l border-white/5 pl-3">
					<!-- Multi-Grouping Menu -->
					<div
						class="flex items-center gap-1 bg-surface-3 border border-surface rounded-md p-0.5"
						transition:fade
					>
						<span class="text-[10px] text-tertiary font-bold uppercase px-2">Agrupar:</span>
						{#each [{ label: 'Ninguno', value: undefined }, { label: 'Empresa', value: 'empresaNombre' }, { label: 'Área', value: 'areaVisitada' }, { label: 'Modo', value: 'modoIngreso' }] as option}
							<button
								onclick={() => (groupByField = option.value)}
								class="px-2 py-1 rounded text-[11px] font-medium transition-all {groupByField ===
								option.value
									? 'bg-blue-500/20 text-blue-400'
									: 'text-secondary hover:text-primary hover:bg-surface-hover'}"
							>
								{option.label}
							</button>
						{/each}
					</div>
				</div>
			{/if}
		{/snippet}

		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-hover hover:text-primary text-sm font-medium transition-colors"
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
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-hover hover:text-primary text-sm font-medium transition-colors"
					>
						<UserPlus size={14} /> Proveedor
					</button>
					<button
						onclick={handleOpenListado}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-hover hover:text-primary text-sm font-medium transition-colors"
					>
						<FileText size={14} /> Listado
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
			{#key viewMode}
				<TabulatorWrapper
					bind:this={gridWrapper}
					bind:toolbarColumns
					data={filteredIngresos}
					{columns}
					groupBy={groupByField}
					withCheckboxSelection={true}
					onRowSelectionChanged={(data) => (selectedRows = data)}
					persistenceID={`ingresos-proveedores-grid-${viewMode}-v5`}
					options={{
						...defaultTabulatorOptions,
						placeholder: 'No hay registros para mostrar'
					}}
				/>
			{/key}
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
				toastService.success('Proveedor creado');
				showProveedorModal = false;
				loadIngresos();
			} else {
				toastService.error(res.error);
			}
		} catch {
			toastService.error('Error al crear proveedor');
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
