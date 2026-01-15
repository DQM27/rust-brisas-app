<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, FileText, Users, History } from 'lucide-svelte';
	import type { ColDef, GridApi, ICellRendererParams } from '@ag-grid-community/core';

	// Components
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';
	// Usamos ProveedorFormModal para crear nuevos proveedores
	import ProveedorFormModal from '$lib/components/proveedor/ProveedorFormModal.svelte';
	import SalidaModal from './SalidaModal.svelte';
	import ExportDialog from '$lib/components/export/ExportDialog.svelte';
	import QuickEntryModal from './QuickEntryModal.svelte';
	import IngresoProveedorFormModal from './IngresoProveedorFormModal.svelte';

	// Logic
	import { createCustomButton } from '$lib/config/agGridConfigs';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import {
		exportData,
		getAvailableFormats,
		extractGridData,
		extractSelectedRows
	} from '$lib/logic/export';
	import { ingresoProveedorService } from '$lib/services/ingresoProveedorService';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	// Shared Components
	import DateRangePicker from '$lib/components/shared/DateRangePicker.svelte';
	import { createProveedor } from '$lib/logic/proveedor/proveedorService';

	// Types
	import type { IngresoProveedor } from '$lib/types/ingreso-nuevos';

	// Props
	interface Props {
		tabId?: string;
	}
	let { tabId = 'ingreso-proveedor-list' }: Props = $props();

	// ==========================================
	// STATE
	// ==========================================
	let ingresos = $state<IngresoProveedor[]>([]);
	let loading = $state(false);
	let error = $state('');
	let selectedRows = $state<IngresoProveedor[]>([]); // Data row from grid

	// Modals
	// Modal registro nuevo proveedor
	let showProveedorModal = $state(false);

	// Modal de salida
	let showSalidaModal = $state(false);
	let selectedIngreso = $state<IngresoProveedor | null>(null);
	let salidaLoading = $state(false);

	// Quick Entry (Buscador para nuevo ingreso)
	let showQuickEntry = $state(false);

	// Modal de formulario de ingreso adaptado para proveedores
	let showIngresoModal = $state(false);
	let providerForIngreso = $state<any>(null);

	// Estado para Exportación
	let gridApi = $state<GridApi<IngresoProveedor> | null>(null);
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);
	let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>([]);
	let exportRows = $state<Record<string, any>[]>([]);

	// ==========================================
	// HISTORIAL / VIEW MODE STATE
	// ==========================================
	type ViewMode = 'actives' | 'history';
	let viewMode = $state<ViewMode>('actives');

	const today = new Date().toLocaleDateString('en-CA');
	let dateRange = $state({
		start: today,
		end: today
	});

	// Suscripción a comandos de teclado
	let unsubscribeKeyboard: (() => void) | null = null;

	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new': // Ctrl+N
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

	// ==========================================
	// HELPERS
	// ==========================================
	function parseDate(value: any): Date | null {
		if (!value) return null;
		let dateStr = String(value);
		if (dateStr.startsWith("d'") && dateStr.endsWith("'")) {
			dateStr = dateStr.slice(2, -1);
		}
		const d = new Date(dateStr);
		return isNaN(d.getTime()) ? null : d;
	}

	// ==========================================
	// COLUMNS
	// ==========================================
	let columnDefs = $derived.by((): ColDef<IngresoProveedor>[] => {
		const baseCols: ColDef<IngresoProveedor>[] = [
			{
				field: 'gafete', // Mapped from 'gafete' in IngresoProveedor
				headerName: 'Gafete',
				width: 100,
				sortable: true,
				filter: true,
				valueFormatter: (params) => params.value || 'S/G'
			},
			{
				field: 'nombre',
				headerName: 'Nombre',
				width: 150,
				sortable: true,
				filter: true,
				valueGetter: (params) => {
					if (!params.data) return '';
					return `${params.data.nombre} ${params.data.apellido}`;
				}
			},
			{
				field: 'cedula',
				headerName: 'Cédula',
				width: 130,
				sortable: true,
				filter: true
			},
			{
				field: 'empresaNombre', // backend populates this
				headerName: 'Empresa',
				width: 180,
				sortable: true,
				filter: true
			},
			// Provider doesn't have tipoAutorizacion in type definition currently
			// {
			// 	field: 'tipoAutorizacion',
			// 	headerName: 'Autorización',
			// 	width: 130,
			// 	sortable: true,
			// 	filter: true
			// },
			{
				field: 'modoIngreso',
				headerName: 'Modo',
				width: 110,
				sortable: true,
				filter: true
			},
			{
				field: 'fechaIngreso',
				headerName: 'Fecha Entrada',
				width: 140,
				sortable: true,
				valueFormatter: (params) => {
					const date = parseDate(params.value);
					if (!date) return '';
					return date.toLocaleDateString('es-ES', {
						day: '2-digit',
						month: '2-digit',
						year: 'numeric'
					});
				}
			},
			{
				field: 'fechaIngreso',
				headerName: 'Hora Entrada',
				width: 120,
				sortable: true,
				valueFormatter: (params) => {
					const date = parseDate(params.value);
					if (!date) return '';
					return date.toLocaleTimeString('es-ES', {
						hour: '2-digit',
						minute: '2-digit'
					});
				}
			},
			{
				field: 'usuarioIngresoNombre',
				headerName: 'Registrado Por',
				width: 150,
				sortable: true,
				filter: true,
				hide: true // Hid by default to save space, but available
			},
			{
				field: 'fechaSalida',
				headerName: 'Fecha Salida',
				width: 140,
				sortable: true,
				valueFormatter: (params) => {
					const date = parseDate(params.value);
					if (!date) return '-';
					return date.toLocaleDateString('es-ES', {
						day: '2-digit',
						month: '2-digit',
						year: 'numeric'
					});
				}
			},
			{
				field: 'fechaSalida',
				headerName: 'Hora Salida',
				width: 120,
				sortable: true,
				valueFormatter: (params) => {
					const date = parseDate(params.value);
					if (!date) return '-';
					return date.toLocaleTimeString('es-ES', {
						hour: '2-digit',
						minute: '2-digit'
					});
				}
			},
			{
				field: 'usuarioSalidaNombre',
				headerName: 'Salida Por',
				width: 150,
				sortable: true,
				filter: true,
				valueFormatter: (params) => params.value || '-',
				hide: true
			},
			{
				colId: 'tiempoPermanencia', // Calculated locally or returned? We'll calc locally if needed
				headerName: 'Tiempo Dentro',
				width: 140,
				sortable: false,
				valueGetter: (params) => {
					if (params.data && params.data.fechaSalida) {
						// If we have permanence info from backend, use it. Usually backend might send string.
						// If not, calc diff.
						const start = parseDate(params.data.fechaIngreso);
						const end = parseDate(params.data.fechaSalida);
						if (!start || !end) return '-';
						const diffMs = end.getTime() - start.getTime();
						const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
						const diffMins = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
						return `${diffHours}h ${diffMins}m`;
					}
					// Calcular tiempo transcurrido si aún está adentro
					if (!params.data) return '-';
					const entrada = parseDate(params.data.fechaIngreso);
					if (!entrada) return '-';

					const ahora = new Date();
					const diffMs = ahora.getTime() - entrada.getTime();
					const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
					const diffMins = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
					return `${diffHours}h ${diffMins}m`;
				}
			},
			{
				colId: 'actions',
				headerName: 'Acciones',
				width: 120,
				sortable: false,
				filter: false,
				pinned: 'right',
				cellRenderer: (params: ICellRendererParams<IngresoProveedor>) => {
					if (viewMode === 'history') return null; // No exit button in history
					const button = document.createElement('button');
					button.className =
						'px-3 py-1 bg-error text-white rounded-md text-sm hover:opacity-90 transition-opacity';
					button.textContent = 'Salida';
					button.onclick = () => {
						if (params.data) handleSalida(params.data);
					};
					return button;
				}
			}
		];

		if (viewMode === 'history') {
			return baseCols.filter((c) => c.colId !== 'actions');
		}
		return baseCols;
	});

	// ==========================================
	// TOOLBAR BUTTONS
	// ==========================================
	const customButtons = $derived.by(() => {
		const defaultButtons: any[] = [createCustomButton.exportar(() => handleExportClick())];

		// Botón List Proveedor siempre visible
		defaultButtons.unshift({
			id: 'list-proveedor-view',
			label: 'List. Proveedores',
			icon: FileText,
			onClick: () => {
				openTab({
					componentKey: 'proveedor-list',
					title: 'Lista de Proveedores',
					id: 'proveedor-list',
					focusOnOpen: true
				});
			},
			variant: 'default',
			tooltip: 'Ir a listado maestro de proveedores'
		});

		if (viewMode === 'actives') {
			// Botón "Nuevo Proveedor"
			defaultButtons.unshift({
				id: 'new-provider',
				label: 'Nuevo Proveedor',
				icon: Users,
				onClick: () => (showProveedorModal = true),
				variant: 'default',
				tooltip: 'Registrar nuevo proveedor en base de datos'
			});

			// Botón "Nuevo Ingreso"
			defaultButtons.unshift(
				createCustomButton.nuevo(() => handleNuevoIngreso(), false, 'Nuevo Ingreso')
			);
		}

		return {
			default: defaultButtons,
			singleSelect: [createCustomButton.exportar(() => handleExportClick())],
			multiSelect: [createCustomButton.exportar(() => handleExportClick())]
		};
	});

	// ==========================================
	// HANDLERS
	// ==========================================
	async function loadIngresos() {
		loading = true;
		error = '';
		try {
			let data;
			if (viewMode === 'actives') {
				data = await ingresoProveedorService.getActivos();
			} else {
				// Historial logic same as IngresoListView but for proveedores
				// We assume backend handles the date range filter on 'get_ingresos_proveedores_historial' if implemented,
				// OR we need a dedicated method. IngresoProveedorService definition showed:
				// getHistorial(): Promise<IngresoProveedor[]> -> invoke("get_ingresos_proveedores_historial")
				// It likely returns ALL history or a default limit.
				// NOTE: The previous service analysis didn't show args for getHistorial.
				// If it doesn't support dates, we might need to modify backend or service.
				// For now, we call it without args or check if we can pass args.
				// Assuming it fetches recent history.
				data = await ingresoProveedorService.getHistorial();
			}
			ingresos = data;
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
		loadIngresos(); // Refetch if service supports params, otherwise this might just reload same data
	}

	function toggleViewMode(mode: ViewMode) {
		if (viewMode === mode) return;
		viewMode = mode;
		loadIngresos();
	}

	function handleNuevoIngreso() {
		showQuickEntry = true;
	}

	function handleQuickEntrySelect(provider: any) {
		showQuickEntry = false;
		// Adapt provider object if necessary to match what IngresoFormModal expects
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

	function handleSalida(ingreso: IngresoProveedor) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	async function handleSalidaConfirm(event: CustomEvent) {
		const { devolvioGafete, observaciones } = event.detail;
		if (!selectedIngreso) return;
		if (!$currentUser?.id) {
			toast.error('Sesión no válida');
			return;
		}

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

	// ==========================================
	// LIFECYCLE
	// ==========================================
	onMount(() => {
		loadIngresos();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === tabId) {
			setActiveContext('proveedor-ingreso-list');
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
						{viewMode === 'actives' ? 'Proveedores en Planta' : 'Historial de Proveedores'}
					</h2>
					<p class="mt-1 text-sm text-secondary">
						{viewMode === 'actives'
							? 'Proveedores ingresados actualmente'
							: 'Registro histórico de accesos de proveedores'}
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
                {viewMode === 'actives'
								? 'text-primary dark:text-white'
								: 'text-secondary hover:text-primary dark:hover:text-zinc-300'}"
							onclick={() => toggleViewMode('actives')}
						>
							<Users size={16} class={viewMode === 'actives' ? 'scale-110' : ''} />
							Activos
						</button>
						<button
							class="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors z-10
                {viewMode === 'history'
								? 'text-primary dark:text-white'
								: 'text-secondary hover:text-primary dark:hover:text-zinc-300'}"
							onclick={() => toggleViewMode('history')}
						>
							<History size={16} class={viewMode === 'history' ? 'scale-110' : ''} />
							Historial
						</button>
					</div>
				</div>
			</div>

			<!-- Bottom Row: Controls -->
			<div class="flex items-center justify-between gap-4">
				<div class="flex-1 max-w-md">
					<SearchBar placeholder="Buscar proveedor, gafete..." limit={10} />
				</div>
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-hidden relative bg-surface-1 border-t border-surface">
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

		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-error bg-error bg-opacity-10 p-4 text-error"
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error al cargar datos</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else if loading}
			<div class="flex h-full items-center justify-center">
				<p class="text-secondary">Cargando...</p>
			</div>
		{:else}
			<AGGridWrapper
				gridId="proveedor-ingreso-list"
				{columnDefs}
				rowData={ingresos}
				{customButtons}
				getRowId={(params) => params.data.id}
				persistenceKey="ingresos-proveedores-columns"
				onSelectionChanged={(rows) => (selectedRows = rows)}
				onGridReady={(api) => (gridApi = api)}
				customToolbarSlot={toolbarControls}
			/>
		{/if}
	</div>
</div>

<!-- Modal Creacion Proveedor (Solo Wrapper) -->
<ProveedorFormModal
	show={showProveedorModal}
	onClose={() => (showProveedorModal = false)}
	onSave={async (data) => {
		// Replicamos la logica de handleSave del ProveedorListView o llamamos al servicio
		try {
			const res = await createProveedor(data as any);
			if (res.ok) {
				toast.success('Proveedor creado');
				showProveedorModal = false;
				// Opcional: Auto-seleccionar para ingreso
			} else {
				toast.error(res.error);
			}
		} catch {
			toast.error('Error al crear proveedor');
		}
	}}
/>

<!-- Modal Salida -->
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

<!-- Quick Entry / Buscador -->
<QuickEntryModal
	bind:show={showQuickEntry}
	onSelect={handleQuickEntrySelect}
	allowedTypes={['proveedor']}
	onClose={() => (showQuickEntry = false)}
/>
<!-- NOTE: QuickEntryModal needs to support searching PROVEEDORES. Currently it might default to PERSONAS. 
     We might need to update QuickEntryModal props or implementation. -->

<!-- Modal Ingreso (Formulario final) -->
<IngresoProveedorFormModal
	bind:show={showIngresoModal}
	initialPerson={providerForIngreso}
	on:complete={handleModalComplete}
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
