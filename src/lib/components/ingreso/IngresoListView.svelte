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
	import PersonDetailModal from '../shared/PersonDetailModal.svelte';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId, openTab } from '$lib/stores/tabs';
	import { statusBarInfo } from '$lib/stores/ui';
	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';
	import { getAvailableFormats } from '$lib/api/export';
	import { searchByType } from '$lib/api/searchService';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'ingreso-list', data }: Props = $props();

	// Effect to handle external actions
	$effect(() => {
		if (data?.openCreateModal) {
			if (data.initialPersonId) {
				// Si viene con ID de persona, cargamos y abrimos el modal directamente
				const personId = data.initialPersonId;
				// Buscamos en el store local si ya lo tenemos o invocamos servicio
				// Para simplificar, abrimos el modal de ingreso con un mock mínimo,
				// el propio IngresoFormModal se encarga de re-validar al recibir initialPerson.
				setTimeout(() => {
					personForIngreso = { id: personId, tipo: 'contratista' };
					showModal = true;
				}, 100);
			} else {
				// Comportamiento por defecto: abrir buscador rápido
				setTimeout(() => {
					if (!showQuickEntry) {
						showQuickEntry = true;
					}
				}, 100);
			}
		}
	});

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
	// Date Range & Filters: Mostrar últimos 30 días por defecto en historial
	const today = new Date().toLocaleDateString('en-CA');
	const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toLocaleDateString('en-CA');
	let dateRange = $state({
		start: thirtyDaysAgo,
		end: today
	});
	let hideActive = $state(false);
	let searchTerm = $state('');

	// Export State
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);
	let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>([]);
	let exportRowsSnapshot = $state<Record<string, any>[]>([]);

	// Metadata para el Toolbar (visibilidad y fijado)
	let toolbarColumns = $state<
		{ field: string; title: string; visible: boolean; frozen: boolean }[]
	>([]);

	// Filters visibility
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Grouping State
	let groupByField = $state<string | undefined>(undefined);

	// Detail Modal State
	let showDetailModal = $state(false);
	let selectedPersonForDetail = $state<IngresoResponse | null>(null);

	// Context Menu for Rows
	const rowContextMenu = [
		{
			label: 'Copiar Nombre',
			action: (e: any, row: any) => {
				const data = row.getData();
				navigator.clipboard.writeText(data.nombreCompleto);
				toast.success('Nombre copiado');
			}
		},
		{
			label: 'Ver Detalles del Contratista',
			action: (e: any, row: any) => {
				const data = row.getData();
				selectedPersonForDetail = data;
				showDetailModal = true;
			}
		}
	];

	function handleRowDblClick(e: any, row: any) {
		const data = row.getData();
		selectedPersonForDetail = data;
		showDetailModal = true;
	}

	// Derived Data (Search + ViewMode + Filter)
	let filteredIngresos = $derived(ingresos);

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

			const rawData = data as IngresoResponse[];
			// Enriquecer datos con campos virtuales para fecha/hora (evita colisiones en persistencia/export)
			ingresos = rawData.map((i) => ({
				...i,
				fechaHoraIngreso_fecha: i.fechaHoraIngreso,
				fechaHoraIngreso_hora: i.fechaHoraIngreso,
				fechaHoraSalida_fecha: i.fechaHoraSalida,
				fechaHoraSalida_hora: i.fechaHoraSalida
			})) as any;

			if (gridWrapper) {
				gridWrapper.replaceData(ingresos);
			}

			// La actualización de toolbarColumns ahora la gestiona TabulatorWrapper automáticamente
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
		if (mode === 'actives') groupByField = undefined;
		loadIngresos();
	}

	function handleNuevoIngreso() {
		showQuickEntry = true;
	}

	function handleSalida(ingreso: IngresoResponse) {
		selectedIngreso = ingreso;
		showSalidaModal = true;
	}

	// Smart Search for Ingresos (Transactional)
	async function handleSearch(term: string) {
		searchTerm = term;

		// 1. Reset si está vacío
		if (!term || term.trim().length === 0) {
			if (gridWrapper) gridWrapper.replaceData(ingresos);
			return;
		}

		// Si es muy corto, filtro local simple para evitar llamadas rápidas al backend, pero seguro a tipos
		if (term.trim().length < 2) {
			const q = term.toLowerCase();
			const simpleFiltered = ingresos.filter((i) =>
				String(i.nombreCompleto || '')
					.toLowerCase()
					.includes(q)
			);
			if (gridWrapper) gridWrapper.replaceData(simpleFiltered);
			return;
		}

		try {
			// Estrategia Híbrida Robustecida

			// Paso A: Buscamos PERSONAS (Contratistas) en Tantivy para obtener sus IDs.
			// Esto cubre coincidencias difusas en Nombres, Apellidos y Cédulas de la persona.
			let matchedPersonIds = new Set<string>();
			try {
				const results = await searchByType(term, 'contratista', 100);
				matchedPersonIds = new Set(results.map((r) => r.id));
			} catch (err) {
				console.warn('Tantivy search unavailable, falling back to full local search', err);
			}

			const q = term.toLowerCase();

			const filtered = ingresos.filter((ingreso) => {
				// 1. Coincidencia por Entidad (Tantivy)
				const matchesPerson = ingreso.contratistaId
					? matchedPersonIds.has(ingreso.contratistaId)
					: false;

				// 2. Coincidencia Local (Campos Transaccionales y de Auditoría)
				// Usamos String() explicitamente para que valores numéricos (gafete) o undefined no rompan la app
				const matchesLocal =
					String(ingreso.gafeteNumero || '')
						.toLowerCase()
						.includes(q) ||
					String(ingreso.empresaNombre || '')
						.toLowerCase()
						.includes(q) ||
					String(ingreso.usuarioIngresoNombre || '')
						.toLowerCase()
						.includes(q) ||
					String(ingreso.usuarioSalidaNombre || '')
						.toLowerCase()
						.includes(q) ||
					String(ingreso.cedula || '')
						.toLowerCase()
						.includes(q) || // Redundancia por seguridad
					String(ingreso.nombreCompleto || '')
						.toLowerCase()
						.includes(q);

				return matchesPerson || matchesLocal;
			});

			if (gridWrapper) gridWrapper.replaceData(filtered);
		} catch (e) {
			console.error('Error crítico en búsqueda de ingresos:', e);
			// Fallback de emergencia
			if (gridWrapper) {
				gridWrapper.replaceData(ingresos);
				toast.error('Error al realizar la búsqueda');
			}
		}
	}

	// EXPORT LOGIC
	async function handleExportClick() {
		loading = true;
		try {
			availableFormats = await getAvailableFormats();

			const table = gridWrapper.getTable();
			const allCols = table.getColumns();

			// 1. Obtener definiciones de columnas para exportar
			exportColumns = allCols
				.map((col: any) => {
					const def = col.getDefinition();
					return {
						id: def.title || col.getField() || 'col',
						name: def.title || col.getField() || 'Columna',
						selected: col.isVisible()
					};
				})
				.filter((col: any) => col.id && !['Acciones', 'ag-Grid-ControlsColumn'].includes(col.name));

			// 2. Pre-formatear los datos para el exportador (esto arregla el preview y los valores)
			const isSelection = selectedRows.length > 0;
			const rawRows = isSelection ? $state.snapshot(selectedRows) : table.getData('active');

			exportRowsSnapshot = rawRows.map((row: any) => {
				const formattedRow: Record<string, any> = {};
				allCols.forEach((col: any) => {
					const def = col.getDefinition();
					const header = def.title || col.getField();
					if (!header || header === 'Acciones') return;

					let val = row[col.getField()];

					// Aplicar formateador si existe para limpiar HTML o dar formato
					if (def.formatter && typeof def.formatter === 'function') {
						try {
							// Simulamos un objeto de celda para el formateador
							const mockCell = {
								getValue: () => val,
								getData: () => row,
								getElement: () => ({})
							};
							const res = def.formatter(mockCell);
							if (typeof res === 'string') {
								// Limpiar etiquetas HTML de los formatters (como los spans de colores)
								val = res.replace(/<[^>]*>?/gm, '');
							} else {
								val = res;
							}
						} catch (e) {
							console.warn('Error formatting col', header, e);
						}
					}

					formattedRow[header] = val != null ? String(val) : '';
				});
				return formattedRow;
			});

			showExportModal = true;
		} catch (err) {
			console.error('Export preload error:', err);
			toast.error('Error al preparar exportación');
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
				// Buscar la columna por título primero, luego por campo
				const col = allCols.find((c: any) => (c.getDefinition().title || c.getField()) === id);
				if (col) {
					const field = col.getField();
					headers.push(col.getDefinition().title || id);
					fields.push(field);
				}
			});

			const rowsToExport = exportRowsSnapshot.map((row: any) => {
				const newRow: Record<string, any> = {};
				headers.forEach((header) => {
					newRow[header] = row[header] || '';
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

	$effect(() => {
		if ($activeTabId === tabId) {
			setActiveContext('ingreso-list');
			// Actualizar StatusBar con el conteo actual
			statusBarInfo.set({
				count: filteredIngresos.length,
				selectedCount: selectedRows.length,
				label: 'Registros',
				message: viewMode === 'actives' ? 'Visualizando Ingresos Activos' : 'Visualizando Historial'
			});
		}
	});

	// Limpiar StatusBar al salir si la pestaña se cierra o cambia
	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
		statusBarInfo.set({ count: undefined, label: '', message: '' });
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

	// Lifecycle
	onMount(() => {
		loadIngresos();
		setupKeyboardSubscription();

		// Update tab title
		// @ts-ignore
		import('$lib/stores/tabs').then(({ useTabState }) => {
			const { updateTitle } = useTabState(tabId);
			updateTitle('Ingresos Contratista');
		});
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

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
			title: 'Lista de Contratistas',
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
						{viewMode === 'actives' ? 'Ingresos Contratista' : 'Historial de Contratistas'}
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
		{searchTerm}
		onSearch={handleSearch}
		hasSelection={selectedRows.length > 0}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		onAdvancedExport={handleExportClick}
		columns={toolbarColumns}
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

		{#snippet secondaryActions()}
			{#if viewMode === 'history'}
				<div class="flex items-center gap-2 border-l border-white/5 pl-3">
					<!-- Multi-Grouping Menu -->
					<div class="flex items-center gap-1 bg-[#2d2d2d] border border-white/10 rounded-md p-0.5">
						<span class="text-[10px] text-gray-500 font-bold uppercase px-2">Agrupar:</span>
						{#each [{ id: undefined, label: 'Ninguno' }, { id: 'empresaNombre', label: 'Empresa' }, { id: 'tipoAutorizacionDisplay', label: 'Autorización' }, { id: 'modoIngresoDisplay', label: 'Modo' }] as opt}
							<button
								onclick={() => (groupByField = opt.id)}
								class="px-2 py-1 rounded text-[11px] font-medium transition-all {groupByField ===
								opt.id
									? 'bg-blue-500/20 text-blue-400'
									: 'text-gray-400 hover:text-white hover:bg-white/5'}"
							>
								{opt.label}
							</button>
						{/each}
					</div>
				</div>
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
				bind:toolbarColumns
				data={ingresos}
				{columns}
				withCheckboxSelection={true}
				groupBy={groupByField}
				columnCalculations={false}
				{rowContextMenu}
				options={{
					...defaultTabulatorOptions,
					layout: 'fitData',
					placeholder: 'No hay ingresos registrados'
				}}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				onRowDblClick={handleRowDblClick}
				persistenceID="ingreso-list-v4"
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

<PersonDetailModal bind:show={showDetailModal} person={selectedPersonForDetail} />

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
