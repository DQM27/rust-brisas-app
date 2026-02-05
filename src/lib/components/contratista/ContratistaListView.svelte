<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
	import { onMount, onDestroy, tick, untrack } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toastService } from '$lib/services/toastService';
	import {
		Plus,
		Edit3,
		Trash2,
		RefreshCw,
		AlertCircle,
		Settings,
		ChevronDown, // Used in filter UI but kept here if needed for others
		Eye,
		Car,
		Edit, // Added from original imports
		Filter,
		Pencil,
		X,
		RotateCcw,
		History
	} from 'lucide-svelte';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId } from '$lib/stores/tabs';
	import { get } from 'svelte/store';
	import {
		shortcutCommand,
		setActiveContext,
		clearCommand,
		shortcutRegistry
	} from '$lib/shortcuts';

	import { selectedSearchStore } from '$lib/stores/searchStore';
	import { searchByType } from '$lib/api/searchService';
	import { getContratistaColumns } from '$lib/logic/contratista/contratistaColumns';
	// Services and Logic
	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	import { vehiculos } from '$lib/api/vehiculos';
	import { contratistaStore } from '$lib/stores/contratistaStore.svelte';
	import { openConfirm } from '$lib/stores/confirm.svelte';
	import { exportData } from '$lib/api/export';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ContratistaFormModal from '$lib/components/contratista/modal/ContratistaFormModal.svelte';
	import VehiculoManagerModal from '$lib/components/vehiculo/VehiculoManagerModal.svelte';
	import ExportDialog from '$lib/components/export/ExportDialog.svelte';
	import PersonDetailModal from '$lib/components/shared/PersonDetailModal.svelte';

	// Logic
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import { getAvailableFormats } from '$lib/logic/export/exportService';
	import { validarIngreso } from '$lib/logic/ingreso/ingresoService';

	// Types
	import type {
		ContratistaResponse,
		ContratistaListResponse,
		EstadoContratista
	} from '$lib/types/contratista';

	interface Props {
		tabId?: string;
		data?: any;
	}
	let { tabId = 'contratista-list', data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			setTimeout(() => {
				if (!showModal) openModal();
			}, 100);
		}

		if (data?.search) {
			// Small delay to ensure grid is ready if just opening
			setTimeout(() => {
				handleGridSearch(data.search);
			}, 200);
		}

		if (data?.openDetailId) {
			// Si viene un ID para detalle, buscamos info extendida y abrimos modal de detalle
			setTimeout(async () => {
				const id = data.openDetailId;
				try {
					// 1. Intentar validar ingreso para ver si está dentro
					const res = await validarIngreso('contratista', id);
					if (res.ingresoAbierto) {
						// Está adentro -> mostrar datos reales del ingreso
						selectedPersonForDetail = res.ingresoAbierto;
					} else {
						// No está adentro -> Construir objeto parcial con datos del contratista
						// Buscamos en la lista local o usamos los datos del res.persona (que viene del backend)
						const contratista = res.persona || contratistas.find((c) => c.id === id);

						if (contratista) {
							// Normalizar campos debido a diferencias entre ValidacionIngresoResult.persona y ContratistaResponse
							const c = contratista as any;
							const empresaNombre = c.empresa || c.empresaNombre || 'N/A';
							const isPraindVigente =
								c.praindVigente ?? (c.praindVencido !== undefined ? !c.praindVencido : false);

							selectedPersonForDetail = {
								...contratista,
								id: '',
								tipoIngreso: 'contratista',
								tipoAutorizacionDisplay: isPraindVigente ? 'PRAIND Vigente' : 'Sin Autorización',
								modoIngresoDisplay: '-',
								fechaHoraIngreso: '',
								usuarioIngresoNombre: '-',
								estaAdentro: false,
								empresa: empresaNombre,
								empresaNombre: empresaNombre
							} as any;
						} else {
							toastService.error('No se pudo cargar la información del contratista');
							return;
						}
					}

					showDetailModal = true;

					// Filtrar grid de fondo
					if (gridWrapper) {
						gridWrapper.getTable().setFilter('id', '=', id);
					}
				} catch (e) {
					console.error('Error opening detail', e);
					toastService.error('Error al cargar detalles');
				}
			}, 500);
		}
	});

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	// NOTA: Usamos un derivado del store para reaccionar a cambios globales
	let contratistas = $derived(contratistaStore.contratistas);
	let contratistasTreeData = $state<any[]>([]); // Data transformada para tree view

	// Transformación reactiva para Tree Data
	$effect(() => {
		if (!contratistas) {
			contratistasTreeData = [];
			return;
		}

		contratistasTreeData = contratistas.map((c: ContratistaResponse) => {
			const children =
				c.vehiculos && c.vehiculos.length > 0
					? c.vehiculos.map((v: any) => ({
							_parent: { ...c, vehiculos: undefined }, // Break circular reference!
							id: v.id,
							// Map vehicle fields to column matches
							nombreCompleto: '', // Clear name column for cleaner look
							vehiculoTipo:
								`${v.tipoVehiculo} - ${v.marca || ''} ${v.modelo || ''} ${v.color || ''}`.trim(),
							vehiculoPlaca: v.placa,
							// Empty fields for other columns
							cedula: '',
							empresaNombre: '',
							estado: null, // Avoid status badge
							praindVencido: null,
							puedeIngresar: null
						}))
					: undefined;

			return {
				...c,
				_children: children
			};
		});
	});

	let loading = $state(false);
	let error = $state('');
	let showColDropdown = $state(false);
	let isUpdatingStatus = false;
	let selectedRows = $state<any[]>([]); // Track selected rows

	// Modal States
	let showModal = $state(false);
	let editingContratista = $state<ContratistaResponse | null>(null);
	let modalLoading = $state(false);
	let isReadOnlyModal = $state(false);

	// Vehiculo Modal State
	let showVehiculoModal = $state(false);
	let selectedContratistaForVehicles = $state<ContratistaResponse | null>(null);

	// Detail Modal State
	let showDetailModal = $state(false);
	let selectedPersonForDetail = $state<any>(null);

	// Filters
	let estadoFilter = $state<'todos' | 'activo' | 'inactivo' | 'suspendido'>('todos');
	let showEstadoDropdown = $state(false);
	let praindFilter = $state<'todos' | 'vigente' | 'vencido' | 'por-vencer'>('todos');
	let showPraindDropdown = $state(false);
	// Toggle header filters visibility - persist in localStorage
	let showHeaderFilters = $state(
		typeof window !== 'undefined'
			? localStorage.getItem('tabulator-header-filters') === 'true'
			: false
	);

	// Export State
	let showExportModal = $state(false);
	let availableFormats = $state<string[]>([]);
	let exportColumns = $state<{ id: string; name: string; selected: boolean }[]>([]);
	let exportRows = $state<Record<string, any>[]>([]);
	let searchTerm = $state('');

	// Metadata para el Toolbar (visibilidad y fijado)
	let toolbarColumns = $state<
		{ field: string; title: string; visible: boolean; frozen: boolean }[]
	>([]);

	// Filter Buttons logic (for keyboard nav state reference mostly, actual filtering is in derived data)
	// NOTE: In Tabulator we pass the filtered data directly or use Filter API. Here we filter locally first.

	// ==========================================
	// COLUMN FORMATTERS (Tabulator Re-implementation)
	// ==========================================

	// Cell Click Handler
	// Column Definitions (Tabulator)
	// We use a derived state to ensure handlers are fresh if they depend on closure variables
	// although handlers defined as functions on the component instance are stable enough.
	let columns = $derived(
		getContratistaColumns({
			onStatusChange: handleStatusChange,
			onEdit: (c) => {
				if (!c.cedula && c._parent) {
					openVehiculoModal(c._parent);
				} else {
					openModal(c);
				}
			},
			onDelete: async (c) => {
				if (!c.cedula) {
					// Vehicle Deletion
					if (confirm('¿Eliminar vehículo?')) {
						try {
							await vehiculos.delete(c.id);
							loadContratistas();
						} catch (e) {
							toastService.error('Error al eliminar vehículo');
						}
					}
				} else {
					handleDelete(c);
				}
			},
			onVehiculoClick: (c) => openVehiculoModal(c)
		})
	);

	// ==========================================
	// DATA LOADING
	// ==========================================
	let gridWrapper: any = $state(null); // Reference to the TabulatorWrapper component

	// ==========================================
	// DERIVED DATA (Filtering)
	// ==========================================
	// const filteredData = $derived.by(() => {
	// 	let filtered = contratistas;
	// 	const _search = $selectedSearchStore;

	// 	// Search Store filter
	// 	if (_search.result) {
	// 		return filtered.filter((c) => c.id === _search.result!.id);
	// 	}

	// 	// Filtro de estado
	// 	if (estadoFilter !== 'todos') {
	// 		filtered = filtered.filter((c) => c.estado === estadoFilter);
	// 	}

	// 	// Filtro de PRAIND
	// 	if (praindFilter === 'vigente') {
	// 		filtered = filtered.filter((c) => !c.praindVencido && c.diasHastaVencimiento > 30);
	// 	} else if (praindFilter === 'vencido') {
	// 		filtered = filtered.filter((c) => c.praindVencido);
	// 	} else if (praindFilter === 'por-vencer') {
	// 		filtered = filtered.filter((c) => !c.praindVencido && c.diasHastaVencimiento <= 30);
	// 	}

	// 	return filtered;
	// });

	// ==========================================
	// DATA LOADING
	// ==========================================

	// Archive State
	let showArchived = $state(false);

	// ==========================================
	// DATA LOADING
	// ==========================================

	async function loadContratistas() {
		loading = true;
		error = '';
		try {
			// Select service method based on view mode
			// Usamos el store para cargar
			contratistaStore.refresh(showArchived);
			loading = contratistaStore.loading; // Sync local loading visual if needed, or bind directly

			// Wait for store update (optional, effect handles it)
			// But for safety regarding loading state:
			loading = false;

			// Logic moved to effect:
			// Normalize data is done in store
			// Mapping to tree data is done in derived effect
		} catch {
			console.error('Error al cargar contratistas');
			error = 'Error al cargar contratistas';
		}
		loading = false;
	}

	function handleToggleArchived() {
		showArchived = !showArchived;
		loadContratistas();
	}

	// Smart Search implementation using Tantivy
	async function handleGridSearch(term: string) {
		searchTerm = term;
		if (!term || term.trim().length < 2) {
			if (gridWrapper) gridWrapper.replaceData(contratistas);
			return;
		}

		try {
			// Usamos Tantivy para obtener los IDs que coinciden
			const results = await searchByType(term, 'contratista', 100);
			const matchedIds = new Set(results.map((r) => r.id));

			// Filtramos la lista local basándonos en los poderes de Tantivy
			const filtered = contratistas.filter((c) => matchedIds.has(c.id));

			if (gridWrapper) {
				gridWrapper.replaceData(filtered);
				// Si no hay resultados locales pero Tantivy encontró algo,
				// podríamos opcionalmente cargar esos registros específicos de la DB.
				// Por ahora, asumimos que 'contratistas' tiene el set completo.
			}
		} catch (e) {
			console.error('Error en búsqueda inteligente:', e);
			// Fallback al filtro local básico si falla el motor
			if (gridWrapper) {
				gridWrapper.getTable()?.setFilter('nombreCompleto', 'like', term);
			}
		}
	}

	// Subscribe to global search (old mechanism for dropdown selection)
	$effect(() => {
		const result = $selectedSearchStore.result;
		if (result && result.tipo === 'contratista') {
			const filtered = contratistas.filter((c) => c.id === result.id);
			if (gridWrapper) gridWrapper.replaceData(filtered);
		} else if (!result && searchTerm === '') {
			if (gridWrapper) gridWrapper.replaceData(contratistas);
		}
	});

	// ==========================================
	// HANDLERS
	// ==========================================

	// Status Change
	async function handleStatusChange(id: string, status: string) {
		if (isUpdatingStatus) return;
		if (!$currentUser || !can($currentUser, 'contratistas:update')) {
			toastService.error('No tienes permisos para cambiar el estado.');
			return;
		}

		isUpdatingStatus = true;
		const newStatus = status === 'activo' ? 'inactivo' : 'activo';
		const toastId = toastService.loading(`Cambiando a ${newStatus}...`);

		// Update Grid (Optimistic, Surgical)
		// No full redraw, just update the row
		gridWrapper?.updateRow(id, { estado: newStatus });

		// Update local Svelte state silently (for consistency if filters rerun)
		// We do this by mutating the specific item reference if possible, or just updating list
		// Caution: Updating 'contratistas' triggers the effect above -> full redraw.
		// We want to avoid that for simple status change.
		// Solution: untrack? Or just accept full redraw?
		// Tabulator handles full redraw fast, but 'updateRow' is smoother.
		// Let's rely on the effect for consistency, but the `updateRow` gave instant feedback.
		// If the effect runs after, it will just overwrite with same data (cloned), which is fine but maybe flicker.
		// Let's try JUST using updateRow and updating internal state.

		try {
			const res = await contratistaService.changeEstado(id, newStatus as any);
			if (res.ok) {
				toastService.dismiss(toastId);
				toastService.success(`Estado actualizado`);
				// Update source of truth
				const index = contratistas.findIndex((c) => c.id === id);
				if (index !== -1) {
					// Update Svelte state so filters remain correct
					// This WILL trigger the effect above and do a replaceData
					// This mimics the "disappearing" behavior if filter doesn't match new status
					// But that is correct behavior! If I filter "Activos" and change to "Inactivo", it SHOULD disappear.
					contratistas[index].estado = newStatus as any;
				}
			} else {
				// Revert
				gridWrapper?.updateRow(id, { estado: status });
				toastService.dismiss(toastId);
				toastService.error(res.error);
			}
		} catch (e) {
			gridWrapper?.updateRow(id, { estado: status });
			console.error(e);
			toastService.dismiss(toastId);
			toastService.error('Error al cambiar estado');
		} finally {
			isUpdatingStatus = false;
		}
	}

	// ... rest of handlers ...

	// in template:
	// bind:this={gridWrapper}
	// data={[]} // Initial data can be empty, we load in mount

	// Modal Operations
	function openModal(contratista: ContratistaResponse | null = null, readonly: boolean = false) {
		editingContratista = contratista;
		isReadOnlyModal = readonly;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingContratista = null;
	}

	// Save Contractor
	async function handleSaveContratista(data: any) {
		modalLoading = true;
		try {
			if (editingContratista) {
				const res = await contratistaService.updateContratista(editingContratista.id, data);
				if (res.ok) {
					toastService.success('Contratista actualizado');
					closeModal();
					loadContratistas();
				} else {
					toastService.error(res.error);
				}
			} else {
				const res = await contratistaService.createContratista(data);
				if (res.ok) {
					toastService.success('Contratista creado');
					closeModal();
					loadContratistas();
				} else {
					toastService.error(res.error);
				}
			}
		} catch (e) {
			console.error(e);
			toastService.error('Error al guardar contratista');
		}
		modalLoading = false;
	}

	// Restore

	// Delete Contractor

	// Restore
	function handleRestore(contratista: ContratistaResponse) {
		if (!$currentUser || !can($currentUser, 'contratistas:delete')) {
			toastService.error('No tienes permisos para restaurar.');
			return;
		}

		openConfirm({
			title: 'Restaurar Contratista',
			message: `¿Estás seguro de que deseas restaurar a ${contratista.nombreCompleto}? El elemento volverá a estar visible en la lista principal.`,
			type: 'info',
			confirmText: 'Restaurar',
			onConfirm: async () => {
				const toastId = toastService.loading('Restaurando...');
				const result = await contratistaService.restoreContratista(contratista.id);
				if (result.ok) {
					toastService.dismiss(toastId);
					toastService.success('Contratista restaurado');
					loadContratistas();
				} else {
					toastService.dismiss(toastId);
					toastService.error(result.error || 'Error desconocido');
				}
			}
		});
	}

	// Delete Contractor
	function handleDelete(contratista: ContratistaResponse) {
		if (!$currentUser || !can($currentUser, 'contratistas:delete')) {
			toastService.error('No tienes permisos para eliminar.');
			return;
		}

		openConfirm({
			title: 'Mover a Papelera',
			message: `¿Estás seguro de mover a "${contratista.nombreCompleto}" a la papelera? Podrás recuperarlo más tarde.`,
			type: 'danger',
			confirmText: 'Mover a Papelera',
			onConfirm: async () => {
				const toastId = toastService.loading('Eliminando...');
				const result = await contratistaService.deleteContratista(contratista.id);
				if (result.ok) {
					toastService.dismiss(toastId);
					toastService.success('Contratista movido a papelera');
					loadContratistas();
				} else {
					toastService.dismiss(toastId);
					toastService.error(result.error);
				}
			}
		});
	}

	// Bulk Delete
	function handleDeleteMultiple(selection: ContratistaResponse[]) {
		if (!$currentUser || !can($currentUser, 'contratistas:delete')) {
			toastService.error('No tienes permisos para eliminar.');
			return;
		}

		openConfirm({
			title: 'Eliminación Múltiple',
			message: `¿Estás seguro de mover ${selection.length} contratistas a la papelera?`,
			type: 'danger',
			confirmText: 'Mover a Papelera',
			onConfirm: async () => {
				const toastId = toastService.loading('Eliminando...');
				let errors = 0;
				for (const c of selection) {
					const res = await contratistaService.deleteContratista(c.id);
					if (!res.ok) errors++;
				}
				if (errors === 0) {
					toastService.dismiss(toastId);
					toastService.success(`${selection.length} contratistas enviados a papelera`);
				} else {
					toastService.dismiss(toastId);
					toastService.error(`Error en ${errors} registros`);
				}
				loadContratistas();
				gridWrapper?.deselectAll();
			}
		});
	}

	// Vehiculo Actions
	function openVehiculoModal(contratista: ContratistaResponse) {
		selectedContratistaForVehicles = contratista;
		showVehiculoModal = true;
	}
	function closeVehiculoModal() {
		showVehiculoModal = false;
		selectedContratistaForVehicles = null;
		loadContratistas();
	}

	// Filter Logic
	function handleEstadoSelect(value: 'todos' | 'activo' | 'inactivo' | 'suspendido') {
		estadoFilter = value;
		showEstadoDropdown = false;
	}
	function handlePraindSelect(value: 'todos' | 'vigente' | 'vencido' | 'por-vencer') {
		praindFilter = value;
		showPraindDropdown = false;
	}
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.filter-dropdown-container')) {
			showEstadoDropdown = false;
			showPraindDropdown = false;
		}
	}

	// ==========================================
	// EXPORT
	// ==========================================
	async function handleExportClick() {
		if (!gridWrapper) return;
		const table = gridWrapper.getTable();
		if (!table) return;

		availableFormats = await getAvailableFormats();

		const cols = table.getColumns();
		exportColumns = cols
			.map((col: any) => ({
				id: col.getField(),
				name: col.getDefinition().title || col.getField(),
				selected: col.isVisible()
			}))
			.filter((col: any) => col.id && col.name !== 'Acciones');

		const isSelection = selectedRows.length > 0;
		// Tabulator getData returns array of data objects
		// 'active' returns data currently filtered/sorted in the table
		const rowsData = isSelection ? selectedRows : table.getData('active');

		exportRows = rowsData;
		showExportModal = true;
	}

	async function handleExport(format: any, options: any) {
		let toastId = '';
		try {
			const isSelection = selectedRows.length > 0;
			toastId = toastService.loading(
				`Exportando ${isSelection ? 'selección' : 'todo'} a ${format.toUpperCase()}...`
			);

			// Preparar opciones
			const exportOptions = {
				...options,
				title: options.title || `Reporte ${new Date().toLocaleDateString()}`,
				generatedBy: $currentUser?.nombreCompleto || ''
				// Pasamos gridWrapper para q el servicio extraiga headers/rows si quiere,
				// O podemos pasar lo que ya tenemos.
				// El servicio exportData (api/export.ts) espera (table, format, options, onlySelected).
				// Pero aquí `exportRows` YA contiene los datos filtrados/seleccionados que el usuario vio en el modal.
				// Y `options.columnIds` tiene las columnas elegidas en el modal.
				// PERO `exportData` de `api/export.ts` está diseñada para extraer DE NUEVO de la tabla.
				// Si el usuario filtró columnas en el modal (ExportDialog), esa información está en options.columnIds.

				// Opción A: Usar exportData pasando la tabla y dejar que re-extraiga.
				// Opción B: Usar una función de `api/export` que acepte rows/headers ya procesados.

				// Dado que `exportData` en `api/export.ts` hace `extractTabulatorData`, usemos esa comodidad.
				// Solo necesitamos pasar la instancia de Tabulator.
			};

			// Llamada al servicio centralizado
			await exportData(
				gridWrapper.getTable(),
				format,
				exportOptions,
				isSelection // onlySelected
			);

			toastService.dismiss(toastId);
			toastService.success('Exportación completada');
		} catch (err: any) {
			// Cancelación por usuario no es error grave
			if (err.message?.includes('cancelada')) {
				toastService.dismiss(toastId);
				toastService.info('Exportación cancelada');
				return;
			}
			toastService.dismiss(toastId);
			toastService.error('Error: ' + err.message);
		}
	}

	// Keyboard Subscriptions
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = shortcutCommand.subscribe((event) => {
			if (!event) return;
			const current = get(activeTabId);
			if (current !== tabId) return;

			const canCreate = $currentUser && can($currentUser, 'contratistas:create');

			switch (event.command) {
				case 'create':
					if (canCreate && !showModal && !showVehiculoModal) {
						openModal();
						clearCommand();
					}
					break;
				case 'refresh':
					loadContratistas();
					clearCommand();
					break;
				case 'refresh':
					loadContratistas();
					clearCommand();
					break;
			}
		});
	}

	// Lifecycle
	onMount(async () => {
		loadContratistas();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});
	$effect(() => {
		if ($activeTabId === tabId) {
			shortcutRegistry.setScope('list');
			setActiveContext('contratista-list');
		}
	});
</script>

<svelte:window onclick={handleClickOutside} />

<PersonDetailModal
	bind:show={showDetailModal}
	person={selectedPersonForDetail}
	onClose={() => {
		showDetailModal = false;
		selectedPersonForDetail = null;
		// Opcional: limpiar filtro al cerrar? Mejor dejarlo para contexto
	}}
/>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary">Lista de Contratistas</h2>
				<p class="mt-1 text-sm text-secondary">
					Gestión y visualización de contratistas registrados
				</p>
			</div>
			<!-- Filters removed from here, now in Toolbar -->
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 flex flex-col overflow-hidden relative bg-surface-1">
		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error al cargar contratistas</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else}
			<!-- New Independent Toolbar -->
			<GridToolbar
				{searchTerm}
				onSearch={handleGridSearch}
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
				onAdvancedExport={handleExportClick}
				columns={toolbarColumns}
				hasSelection={selectedRows.length > 0}
			>
				{#snippet primaryActions()}
					{#if selectedRows.length > 0}
						<!-- Selection Mode Actions -->
						<div
							class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200"
						>
							{#if showArchived && selectedRows.length === 1}
								<button
									class="flex items-center gap-2 px-3 py-1.5
										   bg-teal-600/10 hover:bg-teal-600/20
										   text-teal-400 hover:text-teal-300
										   border border-teal-500/20 hover:border-teal-500/30
										   rounded-md text-sm font-medium transition-all"
									onclick={() => handleRestore(selectedRows[0])}
								>
									<RotateCcw size={16} />
									<span>Restaurar</span>
								</button>
							{/if}

							{#if !showArchived}
								<button
									class="flex items-center gap-2 px-3 py-1.5
										   bg-red-600/10 hover:bg-red-600/20
										   text-red-400 hover:text-red-300
										   border border-red-500/20 hover:border-red-500/30
										   rounded-md text-sm font-medium transition-all"
									onclick={() => handleDeleteMultiple(selectedRows)}
								>
									<Trash2 size={16} />
									<span>Eliminar ({selectedRows.length})</span>
								</button>

								{#if selectedRows.length === 1}
									<button
										class="flex items-center gap-2 px-3 py-1.5
											   bg-amber-600/10 hover:bg-amber-600/20
											   text-amber-400 hover:text-amber-300
											   border border-amber-500/20 hover:border-amber-500/30
											   rounded-md text-sm font-medium transition-all"
										onclick={() => openModal(selectedRows[0])}
									>
										<Pencil size={16} />
										<span>Editar</span>
									</button>
								{/if}
							{/if}

							<button
								class="flex items-center gap-2 px-3 py-1.5
									   bg-[#27272a] hover:bg-[#3f3f46]
									   text-gray-400 hover:text-white
									   border border-white/10
									   rounded-md text-sm font-medium transition-all"
								onclick={() => gridWrapper?.deselectAll()}
							>
								<X size={16} />
								<span>Cancelar</span>
							</button>
						</div>
					{:else}
						<button
							onclick={handleToggleArchived}
							class="flex items-center gap-1.5 px-3 py-1.5 {showArchived
								? 'bg-amber-500/20 text-amber-400 border-amber-500/40'
								: 'bg-surface-3 text-secondary border-surface'} border rounded-md hover:bg-surface-4 text-sm font-medium transition-colors"
							title={showArchived ? 'Ver Activos' : 'Ver Archivados'}
						>
							{#if showArchived}
								<History size={14} /> Ver Activos
							{:else}
								<Trash2 size={14} /> Papelera
							{/if}
						</button>

						{#if !showArchived && $currentUser && can($currentUser, 'contratistas:create')}
							<button
								class="flex items-center gap-2 px-3 py-1.5
                                       bg-blue-600/10 hover:bg-blue-600/20
                                       text-blue-400 hover:text-blue-300
                                       border border-blue-500/20 hover:border-blue-500/30
                                       rounded-md text-sm font-medium transition-all"
								onclick={() => openModal()}
							>
								<Plus size={16} />
								<span>Nuevo</span>
							</button>
						{/if}
					{/if}
				{/snippet}

				{#snippet secondaryActions()}
					<!-- Column Visibility Toggle -->
					<!-- Temporarily disabled for debugging reactivity issues
					<div class="relative inline-block text-left">
						<button
							class="px-3 py-1.5
                                   bg-[#18181b] hover:bg-[#27272a]
                                   border border-white/10 hover:border-white/20
                                   rounded-md text-sm text-gray-400 hover:text-gray-200
                                   flex items-center gap-2 transition-all shadow-sm"
							onclick={() => (showColDropdown = !showColDropdown)}
						>
							<Settings size={14} />
							<span>Ajustar Columnas</span>
						</button>

						{#if showColDropdown}
							<div
								class="absolute right-0 mt-2 w-56 rounded-md shadow-xl bg-[#18181b] ring-1 ring-black ring-opacity-5 focus:outline-none z-50 p-2 border border-white/10"
							>
								<div class="text-[10px] font-bold text-gray-500 mb-2 px-2 uppercase tracking-wider">
									Columnas Visibles
								</div>
								{#each columns as col}
									<label
										class="flex items-center px-2 py-1.5 hover:bg-white/5 rounded cursor-pointer group transition-colors"
									>
										<input
											type="checkbox"
											class="form-checkbox h-3.5 w-3.5 text-blue-500 rounded bg-[#27272a] border-gray-600 focus:ring-blue-500/20 focus:ring-offset-0 transition-colors"
											checked={col.visible}
											onclick={() => {
												col.visible = !col.visible; // Update local state for checkbox
												if (gridWrapper) {
													// Toggle in Tabulator
													if (col.visible) {
														gridWrapper.getTable()?.showColumn(col.field);
													} else {
														gridWrapper.getTable()?.hideColumn(col.field);
													}
													gridWrapper.redraw();
												}
											}}
										/>
										<span
											class="ml-2 text-xs text-gray-400 group-hover:text-gray-200 transition-colors"
											>{col.title}</span
										>
									</label>
								{/each}
							</div>
						{/if}
					</div>
					-->
				{/snippet}
			</GridToolbar>

			<!-- Tabulator Component (Clean) -->
			<div
				class="flex-1 overflow-hidden relative bg-[#1e1e1e] {showHeaderFilters
					? ''
					: 'hide-filters'}"
			>
				<TabulatorWrapper
					bind:this={gridWrapper}
					bind:toolbarColumns
					data={contratistas}
					{columns}
					withCheckboxSelection={true}
					persistenceID="contratista-list-grid-v5"
					options={{
						...defaultTabulatorOptions,
						dataTree: true,
						dataTreeChildField: '_children',
						dataTreeStartExpanded: false,
						dataTreeElementColumn: 'vehiculoTipo',
						layout: 'fitData',
						rowDblClick: (e: any, row: any) => {
							const data = row.getData();
							if (!data.cedula && data._parent) {
								openVehiculoModal(data._parent);
							} else if (data.cedula) {
								openModal(data);
							}
						}
					}}
					onRowSelectionChanged={(data) => (selectedRows = data)}
				/>
			</div>
		{/if}
	</div>
</div>

<!-- Modales -->
{#if showModal}
	<ContratistaFormModal
		show={showModal}
		contratista={editingContratista}
		readonly={isReadOnlyModal}
		loading={modalLoading}
		onSave={handleSaveContratista}
		onClose={closeModal}
	/>
{/if}

{#if showVehiculoModal && selectedContratistaForVehicles}
	<VehiculoManagerModal
		show={showVehiculoModal}
		propietarioId={selectedContratistaForVehicles.id}
		propietarioNombre={selectedContratistaForVehicles.nombreCompleto ||
			selectedContratistaForVehicles.nombre + ' ' + selectedContratistaForVehicles.apellido}
		onClose={closeVehiculoModal}
	/>
{/if}

{#if showExportModal}
	<ExportDialog
		onClose={() => (showExportModal = false)}
		columns={exportColumns}
		rows={exportRows}
		{availableFormats}
		onExport={handleExport}
	/>
{/if}

<style>
	/* Hide header filters when toggled off */
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
