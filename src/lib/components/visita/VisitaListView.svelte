<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		Users,
		History,
		Plus,
		X,
		LogOut,
		FileText,
		UserPlus,
		LogIn,
		CalendarClock
	} from '@lucide/svelte';
	import { scale } from 'svelte/transition';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import IngresoVisitaFormModal from '$lib/components/ingreso/IngresoVisitaFormModal.svelte';
	import PreRegistroListView from '$lib/components/visita/PreRegistroListView.svelte';

	import SalidaModal from '$lib/components/ingreso/SalidaModal.svelte';
	import ExportDialog from '$lib/components/export/ExportDialog.svelte';
	import DateRangePicker from '$lib/components/shared/DateRangePicker.svelte';

	// Logic & Services
	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { getIngresoVisitaColumns } from '$lib/logic/visita/ingresoVisitaColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import { createGridBadge } from '$lib/components/tabulator/gridBadge';
	import type { IngresoVisita } from '$lib/types/ingreso-nuevos';
	import { toastService } from '$lib/services/toastService';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import {
		shortcutCommand,
		setActiveContext,
		clearCommand,
		shortcutRegistry
	} from '$lib/shortcuts';
	import { getAvailableFormats, exportData } from '$lib/logic/export';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'visitas-list', data }: Props = $props();

	$effect(() => {
		if (data?.openCreateModal) {
			if (data.initialPersonId) {
				setTimeout(() => {
					selectedPerson = { id: data.initialPersonId, tipo: 'visita' };
					showIngresoModal = true;
				}, 100);
			} else {
				setTimeout(() => {
					handleNuevoIngreso();
				}, 100);
			}
		}
	});

	// Estado
	let ingresos = $state<IngresoVisita[]>([]);
	let loading = $state(false);
	let viewMode = $state<'actives' | 'history' | 'expected'>('actives');
	let selectedRows = $state<IngresoVisita[]>([]);
	let searchTerm = $state('');

	// Modals
	let showIngresoModal = $state(false);
	let showPreRegistrosModal = $state(false);
	let showSalidaModal = $state(false);
	let selectedPerson = $state<any>(null);
	let selectedIngreso = $state<IngresoVisita | null>(null);
	let salidaLoading = $state(false);
	let preRegistroList = $state<any>(null); // Use any for now to avoid TS issues with Svelte 5 exports in bind:this

	// Grid State
	let gridWrapper = $state<any>(null);
	let toolbarColumns = $state<any[]>([]);
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Estado para Exportación
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);

	// Rango de fechas por defecto: Hoy (Local)
	const today = new Date().toLocaleDateString('en-CA');
	let dateRange = $state({
		start: today,
		end: today
	});

	// Filtro local: Solo finalizados
	let hideActive = $state(false);

	// Grouping
	let groupByField = $state<string | undefined>(undefined);

	function setGrouping(field: string | undefined) {
		groupByField = field;
	}

	let filteredIngresos = $derived.by(() => {
		let data = ingresos;
		if (viewMode === 'history' && hideActive) {
			data = data.filter((i: any) => i.fechaHoraSalida);
		}
		return data;
	});

	// Columnas dinámicas
	const columns = $derived.by(() => {
		let cols = getIngresoVisitaColumns(viewMode);

		// Añadir columna de acciones solo en modo activos
		if (viewMode === 'actives') {
			cols.push({
				title: 'Acciones',
				width: 100,
				headerSort: false,
				hozAlign: 'center',
				formatter: () =>
					createGridBadge({
						text: 'Salida',
						color: 'red',
						isButton: true,
						className: 'salida-btn'
					}),
				cellClick: (e, cell) => {
					const target = e.target as HTMLElement;
					if (target.classList.contains('salida-btn')) {
						handleSalida(cell.getRow().getData() as IngresoVisita);
					}
				}
			});
		}

		return cols;
	});

	// Suscripción a comandos de teclado
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = shortcutCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create':
					handleNuevoIngreso();
					clearCommand();
					break;
				case 'refresh':
					loadData();
					clearCommand();
					break;
			}
		});
	}

	// Carga de datos
	async function loadData() {
		loading = true;
		try {
			if (viewMode === 'actives') {
				ingresos = await ingresoVisitaService.getActivos();
			} else {
				// Convert to full ISO string for backend filtering
				// Convert to full ISO string for backend filtering (adding Z for UTC/Datetime compatibility)
				const startISO = `${dateRange.start}T00:00:00Z`;
				const endISO = `${dateRange.end}T23:59:59Z`;
				ingresos = await ingresoVisitaService.getHistorial({ start: startISO, end: endISO });
			}
		} catch (_e: unknown) {
			console.error(_e);
			toastService.error('Error cargando visitas');
		} finally {
			loading = false;
		}
	}

	function toggleViewMode(mode: 'actives' | 'history' | 'expected') {
		if (viewMode === mode) return;
		viewMode = mode;
		if (mode !== 'expected') {
			loadData();
		}
	}

	function handleNuevoIngreso() {
		selectedPerson = null;
		showIngresoModal = true;
	}

	function handleSalida(ingreso: IngresoVisita) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	async function handleSalidaConfirm(event: CustomEvent) {
		if (!selectedIngreso || !$currentUser) return;
		const { devolvioGafete, observaciones } = event.detail;

		try {
			salidaLoading = true;
			await ingresoVisitaService.registrarSalida(selectedIngreso.id, devolvioGafete, observaciones);
			toastService.success('Salida registrada');
			showSalidaModal = false;
			selectedIngreso = null;
			loadData();
		} catch (e: any) {
			toastService.error('Error: ' + e.message);
		} finally {
			salidaLoading = false;
		}
	}

	function handleOpenListado() {
		toastService.info('Abriendo catálogo de visitantes...');
		openTab({
			componentKey: 'visitante-list',
			title: 'Catálogo de Visitantes',
			id: 'visitante-list',
			focusOnOpen: true
		});
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

	async function handleExportClick() {
		availableFormats = await getAvailableFormats();
		showExportModal = true;
	}

	async function handleExport(format: any, options: any) {
		if (!gridWrapper) return;
		try {
			const table = gridWrapper.getTable();
			if (!table) return;

			const isSelection = selectedRows.length > 0;
			const toastId = toastService.loading(`Exportando ${isSelection ? 'selección' : 'todo'}...`);

			// Nota: La lógica de exportación puede necesitar adaptación si dependía íntimamente de AG Grid
			// Pero exportData parece estar diseñado para manejar la abstracción si se le pasan los datos
			await exportData(table, format, options, isSelection);

			toastService.success('Exportación completada', { id: toastId });
			showExportModal = false;
		} catch (err: any) {
			toastService.error('Error: ' + err.message);
		}
	}

	function handleDateRangeChange(event: CustomEvent<{ startDate: string; endDate: string }>) {
		dateRange.start = event.detail.startDate;
		dateRange.end = event.detail.endDate;
		loadData();
	}

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
			setActiveContext('visita-list');
		}
	});

	$effect(() => {
		if (viewMode === 'expected' && preRegistroList) {
			preRegistroList.setSearchTerm(searchTerm);
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex flex-col gap-4">
			<div class="flex items-center justify-between">
				<div>
					<h2 class="text-xl font-semibold text-primary">
						{#if viewMode === 'actives'}
							Visitas en Planta
						{:else if viewMode === 'history'}
							Historial de Visitas
						{:else}
							Visitas Esperadas
						{/if}
					</h2>
					<p class="mt-1 text-sm text-secondary">
						{#if viewMode === 'actives'}
							Personas registradas actualmente
						{:else if viewMode === 'history'}
							Registro histórico de accesos
						{:else}
							Gestión de pre-registros y agenda
						{/if}
					</p>
				</div>

				<!-- View Toggle -->
				<div class="relative flex items-center bg-surface-3 p-1 rounded-lg">
					<button
						class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors {viewMode ===
						'actives'
							? 'bg-surface-1 text-primary shadow-sm'
							: 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('actives')}
					>
						<Users size={16} /> Activos
					</button>
					<button
						class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors {viewMode ===
						'expected'
							? 'bg-surface-1 text-primary shadow-sm'
							: 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('expected')}
					>
						<CalendarClock size={16} /> Esperadas
					</button>
					<button
						class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors {viewMode ===
						'history'
							? 'bg-surface-1 text-primary shadow-sm'
							: 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('history')}
					>
						<History size={16} /> Historial
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Toolbar & Grid -->
	{#if viewMode !== 'expected'}
		<GridToolbar
			bind:searchTerm
			hasSelection={selectedRows.length > 0}
			selectionCount={selectedRows.length}
			onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
			onFitColumns={() => gridWrapper?.fitColumns()}
			onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
			onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
			onToggleFilters={() => handleToggleFilters()}
			onAdvancedExport={handleExportClick}
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
					{#if selectedRows.length === 1 && viewMode === 'actives'}
						<button
							onclick={() => handleSalida(selectedRows[0])}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
						>
							<LogOut size={14} /> Salida
						</button>
					{/if}
				{:else}
					<button
						onclick={handleNuevoIngreso}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
					>
						<LogIn size={14} /> Nuevo
					</button>

					<button
						onclick={handleOpenListado}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-hover hover:text-primary text-sm font-medium transition-colors"
					>
						<FileText size={14} /> Listado
					</button>
				{/if}
			{/snippet}

			{#snippet secondaryActions()}
				{#if viewMode === 'history'}
					<!-- Grouping Controls -->
					<div class="flex items-center gap-2 border-l border-white/5 pl-3">
						<!-- Multi-Grouping Menu -->
						<div
							class="flex items-center gap-1 bg-surface-3 border border-surface rounded-md p-0.5"
						>
							<span class="text-[10px] text-tertiary font-bold uppercase px-2">Agrupar:</span>
							{#each [{ id: undefined, label: 'Ninguno' }, { id: 'empresaNombre', label: 'Empresa' }, { id: 'anfitrion', label: 'Anfitrión' }, { id: 'areaVisitada', label: 'Área' }] as opt}
								<button
									onclick={() => setGrouping(opt.id)}
									class="px-2 py-1 rounded text-[11px] font-medium transition-all {groupByField ===
									opt.id
										? 'bg-blue-500/20 text-blue-400'
										: 'text-secondary hover:text-primary hover:bg-surface-hover'}"
								>
									{opt.label}
								</button>
							{/each}
						</div>
					</div>

					<!-- Date & Filter Controls -->
					<div class="flex items-center gap-2 border-l border-white/10 pl-3 ml-1">
						<DateRangePicker
							startDate={dateRange.start}
							endDate={dateRange.end}
							on:change={handleDateRangeChange}
						/>
						<div class="flex items-center gap-2 ml-2 cursor-pointer">
							<input
								type="checkbox"
								id="hideActiveVisita"
								bind:checked={hideActive}
								class="rounded border-surface bg-surface-3 cursor-pointer"
							/>
							<label
								for="hideActiveVisita"
								class="text-xs text-secondary cursor-pointer hover:text-white transition-colors"
								>Solo Finalizados</label
							>
						</div>
					</div>
				{/if}
			{/snippet}
		</GridToolbar>
	{/if}

	<div
		class="flex-1 overflow-hidden relative bg-surface-1 {viewMode !== 'expected'
			? 'border-t border-surface'
			: ''} {viewMode !== 'expected' && !showHeaderFilters ? 'hide-filters' : ''}"
	>
		{#if viewMode === 'expected'}
			<PreRegistroListView bind:this={preRegistroList} showHeader={true} showTitle={false} />
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
					class="h-full"
					withCheckboxSelection={true}
					groupBy={groupByField}
					onRowSelectionChanged={(data) => (selectedRows = data)}
					persistenceID={`visitas-grid-${viewMode}-v5`}
					pagination={true}
					options={{
						...defaultTabulatorOptions,
						layout: 'fitColumns',
						placeholder: 'No se encontraron registros',
						groupHeader: (value, count, data, group) => {
							const field = group.getField();
							let displayValue = value;

							if (!value || value === 'undefined' || value === 'null') {
								if (field === 'empresaNombre') displayValue = 'Particular';
								else displayValue = 'Sin especificar';
							}

							return `<span class="text-accent font-bold">${displayValue}</span> <span class="text-secondary ml-2">(${count} ${count === 1 ? 'registro' : 'registros'})</span>`;
						}
					}}
				/>
			{/key}
		{/if}
	</div>
</div>

<IngresoVisitaFormModal
	bind:show={showIngresoModal}
	initialPerson={selectedPerson}
	onComplete={loadData}
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

{#if showExportModal}
	<ExportDialog
		onClose={() => (showExportModal = false)}
		{availableFormats}
		onExport={handleExport}
	/>
{/if}

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
