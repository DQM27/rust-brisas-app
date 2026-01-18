<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, History, Users, FileText, UserPlus, LogIn, X } from 'lucide-svelte';
	import type { IngresoResponse } from '$lib/types/ingreso';

	// Components
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import IngresoFormModal from './IngresoFormModal.svelte';
	import SalidaModal from './SalidaModal.svelte';
	import QuickExitModal from './QuickExitModal.svelte';
	import QuickEntryModal from './QuickEntryModal.svelte';
	import DateRangePicker from '$lib/components/shared/DateRangePicker.svelte';
	import ContratistaFormModal from '$lib/components/contratista/ContratistaFormModal.svelte';
	import ExportDialog from '$lib/components/export/ExportDialog.svelte';

	// Logic
	import { invoke } from '@tauri-apps/api/core';
	import { save } from '@tauri-apps/plugin-dialog';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import { getIngresoColumns } from '$lib/logic/ingreso/ingresoColumns';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';
	import { getAvailableFormats } from '$lib/api/export';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'ingreso-list' }: Props = $props();

	// State
	let ingresos = $state<IngresoResponse[]>([]);
	let loading = $state(false);
	let error = $state('');
	let selectedRows = $state<IngresoResponse[]>([]);

	// Modals State
	let showModal = $state(false);
	let showContratistaModal = $state(false);
	let showSalidaModal = $state(false);
	let showQuickExit = $state(false);
	let showQuickEntry = $state(false);

	let personForIngreso = $state<any>(null);
	let selectedIngreso = $state<IngresoResponse | null>(null);
	let salidaLoading = $state(false);

	// Grid State
	let gridWrapper = $state<any>(null);

	// View Mode
	type ViewMode = 'actives' | 'history';
	let viewMode = $state<ViewMode>('actives');

	// Date Range & Filters
	const today = new Date().toLocaleDateString('en-CA');
	let dateRange = $state({
		start: today,
		end: today
	});
	let hideActive = $state(false);
	let searchTerm = $state('');

	// Export State
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);
	let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>([]);
	let exportRowsSnapshot = $state<Record<string, any>[]>([]);

	// Filters visibility
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Derived Data (Search + ViewMode + Filter)
	let filteredIngresos = $derived.by(() => {
		let data = ingresos;
		// 1. View Mode Filter
		if (viewMode === 'history' && hideActive) {
			data = data.filter((i) => i.fechaHoraSalida);
		}
		// 2. Search Filter
		if (searchTerm) {
			const q = searchTerm.toLowerCase();
			data = data.filter(
				(i) =>
					i.nombreCompleto?.toLowerCase().includes(q) ||
					i.cedula?.toLowerCase().includes(q) ||
					i.empresaNombre?.toLowerCase().includes(q) ||
					i.gafeteNumero?.toLowerCase().includes(q)
			);
		}
		return data;
	});

	let columns = $derived(
		getIngresoColumns(
			{
				onSalida: (ingreso) => handleSalida(ingreso)
			},
			viewMode
		)
	);

	// Keyboard Subscription
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new':
					if (!showModal && !showSalidaModal && !showQuickEntry) {
						showQuickEntry = true;
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) showModal = false;
					else if (showSalidaModal) showSalidaModal = false;
					else if (showQuickEntry) showQuickEntry = false;
					else if (showExportModal) showExportModal = false;
					clearCommand();
					break;
				case 'refresh':
					loadIngresos();
					clearCommand();
					break;
			}
		});
	}

	// Logic Handlers
	async function loadIngresos() {
		loading = true;
		error = '';
		try {
			let data;
			if (viewMode === 'actives') {
				data = await invoke('get_ingresos_contratistas_activos');
			} else {
				const startLocal = new Date(dateRange.start + 'T00:00:00');
				const endLocal = new Date(dateRange.end + 'T23:59:59.999');
				data = await invoke('get_ingresos_contratistas_historial', {
					fechaInicio: startLocal.toISOString(),
					fechaFin: endLocal.toISOString()
				});
			}
			ingresos = data as IngresoResponse[];

			if (gridWrapper) {
				gridWrapper.replaceData(ingresos);
			}
		} catch (err: any) {
			error = err.message || 'Error al cargar datos';
			toast.error(error);
			ingresos = [];
		} finally {
			loading = false;
		}
	}

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

	function handleNuevoIngreso() {
		showQuickEntry = true;
	}

	function handleSalida(ingreso: IngresoResponse) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	// EXPORT LOGIC
	async function handleExportClick() {
		if (!gridWrapper) return;
		const table = gridWrapper.getTable();
		if (!table) return;

		loading = true;
		try {
			availableFormats = await getAvailableFormats();

			const cols = table.getColumns();
			exportColumns = cols
				.map((col: any) => ({
					id: col.getField(),
					name: col.getDefinition().title || col.getField(),
					selected: col.isVisible()
				}))
				.filter((col: any) => col.id && !['Acciones', 'ag-Grid-ControlsColumn'].includes(col.name));

			const isSelection = selectedRows.length > 0;
			// Tabulator getData('active') returns currently filtered/sorted data
			const rowsData = isSelection ? $state.snapshot(selectedRows) : table.getData('active');

			exportRowsSnapshot = rowsData;
			showExportModal = true;
		} catch (err) {
			console.error('Export preload error:', err);
		} finally {
			loading = false;
		}
	}

	async function handleExport(format: 'pdf' | 'excel' | 'csv', options: any) {
		try {
			const isSelection = selectedRows.length > 0;
			const toastId = toast.loading(
				`Exportando ${isSelection ? 'selección' : 'todo'} a ${format.toUpperCase()}...`
			);

			const targetColIds = options.columnIds;
			const table = gridWrapper?.getTable();
			const allCols = table?.getColumns() || [];

			const headers: string[] = [];
			const fields: string[] = [];

			targetColIds.forEach((id: string) => {
				const col = allCols.find((c: any) => c.getField() === id);
				if (col) {
					headers.push(col.getDefinition().title || id);
					fields.push(id);
				}
			});

			const rowsToExport = exportRowsSnapshot.map((row: any) => {
				const newRow: Record<string, any> = {};
				fields.forEach((field, index) => {
					const header = headers[index];
					let val = row[field];
					if (val === null || val === undefined) val = '';
					newRow[header] = String(val);
				});
				return newRow;
			});

			const request: any = {
				format,
				headers,
				rows: rowsToExport,
				title: options.title || `Reporte Ingresos ${new Date().toLocaleDateString()}`,
				orientation: options.orientation || 'landscape',
				delimiter: options.delimiter || 'comma',
				includeBom: options.includeBom ?? true,
				showPreview: options.showPreview || false,
				generatedBy: $currentUser?.nombreCompleto || ''
			};

			let targetPath = null;
			if (!options.showPreview) {
				const defaultName = `${request.title.replace(/[^a-z0-9]/gi, '_')}.${format === 'excel' ? 'xlsx' : format}`;
				const fileExtension = format === 'excel' ? 'xlsx' : format;
				targetPath = await save({
					defaultPath: defaultName,
					filters: [{ name: format.toUpperCase(), extensions: [fileExtension] }]
				});
				if (!targetPath) throw new Error('Exportación cancelada');
				request.targetPath = targetPath;
			}

			await invoke('export_data', { request });
			toast.success('Exportación completada', { id: toastId });
			showExportModal = false;
		} catch (err: any) {
			toast.error('Error: ' + err.message);
		}
	}

	// Lifecycle
	onMount(() => {
		loadIngresos();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === tabId) setActiveContext('ingreso-list');
	});

	// Handlers from Modals
	function handleModalComplete() {
		showModal = false;
		personForIngreso = null;
		loadIngresos();
	}

	function handleQuickEntrySelect(person: any) {
		showQuickEntry = false;
		personForIngreso = person;
		setTimeout(() => {
			showModal = true;
		}, 100);
	}

	function handleQuickExitSelect(ingreso: any) {
		showQuickExit = false;
		setTimeout(() => {
			handleSalida(ingreso);
		}, 100);
	}

	async function handleSalidaConfirm(event: CustomEvent) {
		const { devolvioGafete, observaciones } = event.detail;
		if (!selectedIngreso || !$currentUser?.id) return;

		try {
			salidaLoading = true;
			const usuarioId = $currentUser.id;

			if (selectedIngreso.tipoIngreso === 'contratista') {
				await invoke('register_exit_contratista', {
					input: {
						ingresoId: selectedIngreso.id,
						devolvioGafete,
						usuarioSalidaId: usuarioId,
						observacionesSalida: observaciones
					},
					usuarioId
				});
			} else if (selectedIngreso.tipoIngreso === 'proveedor') {
				await invoke('registrar_salida_proveedor', {
					id: selectedIngreso.id,
					usuarioId,
					observaciones,
					devolvioGafete
				});
			} else {
				await invoke('registrar_salida_visita', {
					id: selectedIngreso.id,
					usuarioId,
					devolvioGafete,
					observaciones
				});
			}
			toast.success('Salida registrada');
			showSalidaModal = false;
			selectedIngreso = null;
			loadIngresos();
		} catch (err: any) {
			toast.error('Error al registrar salida: ' + err.message);
		} finally {
			salidaLoading = false;
		}
	}

	// Toolbar handlers
	function handleAutoSize() {
		const table = gridWrapper?.getTable();
		if (table) {
			const cols = table.getColumnDefinitions().map((col: any) => ({ ...col, width: undefined }));
			table.setColumns(cols);
			toast.success('Columnas ajustadas al contenido');
		} else {
			console.warn('AutoSize: Table not ready');
		}
	}

	function handleFitColumns() {
		const table = gridWrapper?.getTable();
		if (table) {
			const containerWidth = table.element.clientWidth;
			const columns = table.getColumns();
			if (columns.length === 0) return;
			const columnWidth = Math.floor(containerWidth / columns.length);
			const remainder = containerWidth - columnWidth * columns.length;

			const cols = table.getColumnDefinitions().map((col: any, index: number) => ({
				...col,
				width: index === columns.length - 1 ? columnWidth + remainder : columnWidth
			}));
			table.setColumns(cols);
			toast.success('Columnas ajustadas al ancho');
		} else {
			console.warn('FitColumns: Table not ready');
		}
	}

	function handleToggleColumn(field: string) {
		const table = gridWrapper?.getTable();
		const column = table?.getColumn(field);
		if (column) {
			column.isVisible() ? column.hide() : column.show();
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

	function handleOpenListado() {
		toast('Abriendo listado de contratistas...');
		openTab({
			componentKey: 'contratista-list',
			title: 'Lista Contratistas',
			id: 'contratista-list',
			focusOnOpen: true
		});
	}
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex flex-col gap-4">
			<div class="flex items-center justify-between">
				<div>
					<h2 class="text-xl font-semibold text-primary">
						{viewMode === 'actives' ? 'Ingresos Activos' : 'Historial de Salidas'}
					</h2>
					<p class="mt-1 text-sm text-secondary">
						{viewMode === 'actives'
							? 'Personas actualmente en planta'
							: 'Registro histórico de visitas'}
					</p>
				</div>
				<!-- View Segmented Control -->
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

	<!-- Tabulator Toolbar & Grid -->
	<GridToolbar
		onSearch={(term) => (searchTerm = term)}
		onAutoSizeColumns={handleAutoSize}
		onFitColumns={handleFitColumns}
		onToggleColumn={handleToggleColumn}
		onToggleFilters={handleToggleFilters}
		onAdvancedExport={handleExportClick}
		{columns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
						title="Cancelar selección"
					>
						<X size={14} /> Cancelar
					</button>
				</div>
			{:else if viewMode === 'actives'}
				<button
					onclick={handleNuevoIngreso}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<LogIn size={14} /> Nuevo
				</button>
				<button
					onclick={() => (showContratistaModal = true)}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
				>
					<UserPlus size={14} /> Contratista
				</button>
				<button
					onclick={handleOpenListado}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-[#2d2d2d] text-gray-400 border border-white/10 rounded-md hover:bg-white/5 hover:text-white text-sm font-medium transition-colors"
				>
					<FileText size={14} /> Listado
				</button>
			{/if}
		{/snippet}

		{#snippet CustomFilters()}
			{#if viewMode === 'history'}
				<div class="flex items-center gap-2 border-l border-white/10 pl-3 ml-1">
					<DateRangePicker
						startDate={dateRange.start}
						endDate={dateRange.end}
						on:change={handleDateRangeChange}
					/>
					<div class="flex items-center gap-2 ml-2">
						<input
							type="checkbox"
							id="hideActive"
							bind:checked={hideActive}
							class="rounded border-surface bg-surface-3"
						/>
						<label for="hideActive" class="text-xs text-secondary">Solo Finalizados</label>
					</div>
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
		{#if loading && (!ingresos || ingresos.length === 0)}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else if error}
			<div class="p-8 text-center text-red-400">{error}</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				data={filteredIngresos}
				{columns}
				withCheckboxSelection={true}
				options={{
					...defaultTabulatorOptions,
					layout: 'fitColumns',
					placeholder: 'No hay ingresos registrados'
				}}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				persistenceID="ingreso-list-v1"
			/>
		{/if}
	</div>
</div>

<!-- Modals -->
<IngresoFormModal
	bind:show={showModal}
	initialPerson={personForIngreso}
	on:complete={handleModalComplete}
/>

<ContratistaFormModal
	show={showContratistaModal}
	onClose={() => (showContratistaModal = false)}
	onSave={async (data) => {
		try {
			const res = await contratistaService.createContratista(data as any);
			if (res.ok) {
				toast.success('Contratista creado');
				showContratistaModal = false;
			} else {
				toast.error(res.error);
			}
		} catch (e) {
			toast.error('Error al crear contratista');
		}
	}}
/>

<SalidaModal
	bind:show={showSalidaModal}
	ingreso={selectedIngreso}
	loading={salidaLoading}
	on:confirm={handleSalidaConfirm}
/>

<QuickEntryModal
	bind:show={showQuickEntry}
	onSelect={handleQuickEntrySelect}
	onClose={() => (showQuickEntry = false)}
/>

<QuickExitModal
	bind:show={showQuickExit}
	activeEntries={ingresos}
	onSelect={handleQuickExitSelect}
	onClose={() => (showQuickExit = false)}
/>

{#if showExportModal}
	<ExportDialog
		onClose={() => (showExportModal = false)}
		onExport={handleExport}
		{availableFormats}
		columns={exportColumns}
		rows={exportRowsSnapshot}
	/>
{/if}

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
