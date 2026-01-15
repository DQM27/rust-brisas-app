<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { Users, History, FileText, Search, Plus, X, LogOut, Download } from 'lucide-svelte';
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';
	import IngresoVisitaFormModal from '$lib/components/ingreso/IngresoVisitaFormModal.svelte';
	import SalidaModal from '$lib/components/ingreso/SalidaModal.svelte';
	import ExportDialog from '$lib/components/export/ExportDialog.svelte';

	import { ingresoVisitaService } from '$lib/services/ingresoVisitaService';
	import { INGRESO_VISITA_COLUMNS } from '$lib/logic/visita/ingresoVisitaColumns';
	import type { IngresoVisita } from '$lib/types/ingreso-nuevos';
	import { toast } from 'svelte-5-french-toast';
	import type { ColDef, GridApi, ICellRendererParams } from '@ag-grid-community/core';
	import { currentUser } from '$lib/stores/auth';
	import { createCustomButton } from '$lib/config/agGridConfigs';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import {
		exportData,
		getAvailableFormats,
		extractGridData,
		extractSelectedRows
	} from '$lib/logic/export';
	import DateRangePicker from '$lib/components/shared/DateRangePicker.svelte';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'visitas-list' }: Props = $props();

	// Estado
	let ingresos = $state<IngresoVisita[]>([]);
	let loading = $state(false);
	let viewMode = $state<'actives' | 'history'>('actives');
	let selectedRows = $state<IngresoVisita[]>([]);

	// Modals
	let showIngresoModal = $state(false);
	let showSalidaModal = $state(false);
	let selectedPerson = $state<any>(null);
	let selectedIngreso = $state<IngresoVisita | null>(null);
	let salidaLoading = $state(false);

	// Estado para Exportación
	let gridApi = $state<GridApi<IngresoVisita> | null>(null);
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);
	let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>([]);
	let exportRows = $state<Record<string, any>[]>([]);

	// Rango de fechas por defecto: Hoy (Local)
	const today = new Date().toLocaleDateString('en-CA');
	let dateRange = $state({
		start: today,
		end: today
	});

	// Filtro local: Solo finalizados
	let hideActive = $state(false);

	let filteredIngresos = $derived(
		viewMode === 'history' && hideActive ? ingresos.filter((i) => i.fechaSalida) : ingresos
	);

	// Suscripción a comandos de teclado
	let unsubscribeKeyboard: (() => void) | null = null;

	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new':
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
				ingresos = await ingresoVisitaService.getHistorial();
			}
		} catch (_e: unknown) {
			console.error(_e);
			toast.error('Error cargando visitas');
		} finally {
			loading = false;
		}
	}

	function toggleViewMode(mode: 'actives' | 'history') {
		if (viewMode === mode) return;
		viewMode = mode;
		loadData();
	}

	function handleNuevoIngreso() {
		selectedPerson = null;
		showIngresoModal = true;
	}

	async function handleSalida(ingreso: IngresoVisita) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	async function handleSalidaConfirm(event: CustomEvent) {
		if (!selectedIngreso || !$currentUser) return;
		const { devolvioGafete, observaciones } = event.detail;

		try {
			salidaLoading = true;
			await ingresoVisitaService.registrarSalida(selectedIngreso.id, devolvioGafete, observaciones);
			toast.success('Salida registrada');
			showSalidaModal = false;
			selectedIngreso = null;
			loadData();
		} catch (e: any) {
			toast.error('Error: ' + e.message);
		} finally {
			salidaLoading = false;
		}
	}

	const columnDefs = $derived.by((): ColDef<IngresoVisita>[] => {
		const baseCols: ColDef<IngresoVisita>[] = [
			...INGRESO_VISITA_COLUMNS,
			{
				colId: 'actions',
				headerName: 'Acciones',
				width: 120,
				pinned: 'right',
				cellRenderer: (params: any) => {
					if (viewMode === 'history') return null;
					const button = document.createElement('button');
					button.className =
						'px-3 py-1 bg-red-600/10 text-red-500 border border-red-500/20 rounded-md text-xs hover:bg-red-600 hover:text-white transition-all';
					button.textContent = 'Salida';
					button.onclick = () => handleSalida(params.data);
					return button;
				}
			}
		];

		if (viewMode === 'history') {
			return baseCols.filter((c) => c.colId !== 'actions');
		}
		return baseCols;
	});

	const customButtons = $derived.by(() => {
		const selected = selectedRows[0];
		const defaultButtons: any[] = [createCustomButton.exportar(() => handleExportClick())];

		if (viewMode === 'actives') {
			// Botón "Nuevo Ingreso"
			defaultButtons.unshift(
				createCustomButton.nuevo(() => handleNuevoIngreso(), false, 'Nuevo Ingreso')
			);
		}

		return {
			default: defaultButtons,
			singleSelect: [
				createCustomButton.exportar(() => handleExportClick()),
				{
					id: 'check-out',
					label: 'Registrar Salida',
					icon: LogOut,
					category: 'action' as any,
					onClick: () => selected && handleSalida(selected),
					hide: viewMode === 'history',
					variant: 'danger' as any
				},
				{
					id: 'cancel-selection',
					label: 'Cancelar',
					icon: X,
					onClick: () => gridApi?.deselectAll(),
					variant: 'ghost' as any,
					tooltip: 'Cancelar selección'
				}
			],
			multiSelect: [
				createCustomButton.exportar(() => handleExportClick()),
				{
					id: 'cancel-selection',
					label: 'Cancelar',
					icon: X,
					onClick: () => gridApi?.deselectAll(),
					variant: 'ghost' as any,
					tooltip: 'Cancelar selección'
				}
			]
		};
	});

	// ==========================================
	// EXPORT
	// ==========================================
	async function handleExportClick() {
		if (!gridApi) return;
		availableFormats = await getAvailableFormats();
		const cols = gridApi.getAllGridColumns();
		exportColumns = cols
			.map((col: any) => ({
				id: col.getColId(),
				name: col.getColDef().headerName || col.getColId(),
				selected: col.isVisible()
			}))
			.filter((col: any) => col.id !== 'actions' && col.id !== 'selection');

		const isSelection = selectedRows.length > 0;
		const allColIds = exportColumns.map((c) => c.id);
		try {
			const extracted = isSelection
				? extractSelectedRows(gridApi, allColIds)
				: extractGridData(gridApi, allColIds);
			exportRows = extracted.rows;
		} catch (e) {
			console.error(e);
			exportRows = [];
		}
		showExportModal = true;
	}

	async function handleExport(format: any, options: any) {
		if (!gridApi) return;
		try {
			const isSelection = selectedRows.length > 0;
			const toastId = toast.loading(
				`Exportando ${isSelection ? 'selección' : 'todo'} a ${format.toUpperCase()}...`
			);
			await exportData(gridApi, format, options, isSelection);
			toast.success('Exportación completada', { id: toastId });
		} catch (err: any) {
			toast.error('Error: ' + err.message);
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
			setActiveContext('visita-list');
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="px-8 py-6 bg-surface-2 border-b border-surface">
		<div class="flex items-center justify-between gap-6">
			<div>
				<h2 class="text-xl font-semibold text-primary">
					{viewMode === 'actives' ? 'Visitas en Planta' : 'Historial de Visitas'}
				</h2>
				<p class="mt-1 text-xs text-secondary">
					{viewMode === 'actives'
						? 'Personas registradas actualmente en las instalaciones'
						: 'Registro histórico de accesos finalizados'}
				</p>
			</div>

			<!-- Center: SearchBar (handled by grid primarily, but we can put a global one if needed) -->
			<!-- Using the one inside AGGridWrapper is usually enough, but let's keep headers clean -->

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
						class="flex-1 flex items-center justify-center gap-2 px-6 py-2 rounded-md text-sm font-medium transition-colors z-10
							{viewMode === 'actives' ? 'text-primary dark:text-white' : 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('actives')}
					>
						<Users size={16} class={viewMode === 'actives' ? 'scale-110' : ''} />
						Activos
					</button>
					<button
						class="flex-1 flex items-center justify-center gap-2 px-6 py-2 rounded-md text-sm font-medium transition-colors z-10
							{viewMode === 'history' ? 'text-primary dark:text-white' : 'text-secondary hover:text-primary'}"
						onclick={() => toggleViewMode('history')}
					>
						<History size={16} class={viewMode === 'history' ? 'scale-110' : ''} />
						Historial
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-hidden relative bg-surface-1">
		{#snippet toolbarControls()}
			{#if viewMode === 'history'}
				<div class="flex items-center" transition:fade={{ duration: 150 }}>
					<DateRangePicker
						startDate={dateRange.start}
						endDate={dateRange.end}
						on:change={handleDateRangeChange}
					/>
				</div>
			{/if}
		{/snippet}

		{#snippet postToolbarControls()}
			{#if viewMode === 'history'}
				<div
					class="flex items-center gap-2 px-4 border-l border-surface"
					transition:fade={{ duration: 150 }}
				>
					<input
						type="checkbox"
						id="hideActiveVisita"
						bind:checked={hideActive}
						class="h-4 w-4 rounded border-white/10 bg-black/20 text-blue-500 focus:ring-blue-500/20"
					/>
					<label
						for="hideActiveVisita"
						class="text-xs text-secondary select-none cursor-pointer hover:text-primary transition-colors"
					>
						Solo Finalizados
					</label>
				</div>
			{/if}
		{/snippet}

		{#if loading && ingresos.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<div
						class="w-10 h-10 border-4 border-blue-500/10 border-t-blue-500 rounded-full animate-spin mb-4 mx-auto"
					></div>
					<p class="text-secondary font-medium animate-pulse">Cargando registros...</p>
				</div>
			</div>
		{:else}
			<AGGridWrapper
				gridId="visitas-list"
				rowData={filteredIngresos}
				{columnDefs}
				{customButtons}
				onSelectionChanged={(rows) => (selectedRows = rows)}
				getRowId={(params) => params.data.id}
				persistenceKey="visitas-list-state-v1"
				onRefresh={loadData}
				onGridReady={(api) => (gridApi = api)}
				customToolbarSlot={toolbarControls}
				customPostToolbarSlot={postToolbarControls}
			/>
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
		columns={exportColumns}
		rows={exportRows}
		{availableFormats}
		onExport={handleExport}
	/>
{/if}
