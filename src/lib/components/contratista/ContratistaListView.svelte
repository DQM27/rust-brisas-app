<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
	import { onMount, onDestroy, tick, untrack } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
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
		X
	} from 'lucide-svelte';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId } from '$lib/stores/tabs';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	import { selectedSearchStore } from '$lib/stores/searchStore';
	import { getContratistaColumns } from '$lib/logic/contratista/contratistaColumns';
	// Services and Logic
	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	// Components
	import { TabulatorWrapper } from '$lib/components/tabulator';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import ContratistaFormModal from './ContratistaFormModal.svelte';
	import VehiculoManagerModal from '$lib/components/vehiculo/VehiculoManagerModal.svelte';

	import type { ContratistaResponse, EstadoContratista } from '$lib/types/contratista';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'contratista-list' }: Props = $props();

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	let contratistas = $state<ContratistaResponse[]>([]);
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

	// Filters
	let estadoFilter = $state<'todos' | 'activo' | 'inactivo' | 'suspendido'>('todos');
	let showEstadoDropdown = $state(false);
	let praindFilter = $state<'todos' | 'vigente' | 'vencido' | 'por-vencer'>('todos');
	let showPraindDropdown = $state(false);

	// Filter Buttons logic (for keyboard nav state reference mostly, actual filtering is in derived data)
	// NOTE: In Tabulator we pass the filtered data directly or use Filter API. Here we filter locally first.

	// ==========================================
	// COLUMN FORMATTERS (Tabulator Re-implementation)
	// ==========================================

	// Status Badge Formatter
	const statusFormatter = (cell: any) => {
		const estado = cell.getValue() as EstadoContratista;
		const baseClass =
			'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm cursor-pointer hover:opacity-80 transition-opacity';

		let badgeClass = '';
		if (estado === 'activo') {
			badgeClass =
				'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20';
		} else if (estado === 'suspendido') {
			badgeClass =
				'bg-red-500/10 text-red-600 border-red-500/20 dark:bg-red-500/10 dark:text-red-400 dark:border-red-500/20';
		} else {
			badgeClass =
				'bg-gray-500/10 text-gray-600 border-gray-500/20 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-700';
		}

		const displayText = estado ? estado.toUpperCase() : 'N/A';
		return `<button class="status-btn ${baseClass} ${badgeClass}">${displayText}</button>`;
	};

	// Praind Badge Formatter
	const praindFormatter = (cell: any) => {
		const row = cell.getData() as ContratistaResponse;
		const baseClass =
			'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';
		let badgeClass = '';
		let text = '';

		if (row.praindVencido) {
			badgeClass =
				'bg-red-500/10 text-red-600 border-red-500/20 dark:bg-red-500/10 dark:text-red-400 dark:border-red-500/20';
			text = 'Vencido';
		} else if (row.diasHastaVencimiento <= 30) {
			badgeClass =
				'bg-amber-500/10 text-amber-600 border-amber-500/20 dark:bg-amber-500/10 dark:text-amber-400 dark:border-amber-500/20';
			text = `${row.diasHastaVencimiento} días`;
		} else {
			badgeClass =
				'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20';
			text = 'Vigente';
		}
		return `<span class="${baseClass} ${badgeClass}">${text}</span>`;
	};

	// Access Badge Formatter
	const accessFormatter = (cell: any) => {
		const row = cell.getData() as ContratistaResponse;
		const baseClass =
			'inline-flex items-center px-2.5 py-0.5 rounded-full text-[10px] font-bold border uppercase tracking-widest leading-none shadow-sm';
		const redBadge =
			'bg-red-500/10 text-red-600 border-red-500/20 dark:bg-red-500/10 dark:text-red-400 dark:border-red-500/20';
		const greenBadge =
			'bg-emerald-500/10 text-emerald-600 border-emerald-500/20 dark:bg-emerald-500/10 dark:text-emerald-400 dark:border-emerald-500/20';

		if (row.estaBloqueado) return `<span class="${baseClass} ${redBadge}">Bloqueado</span>`;
		if (row.estado !== 'activo') return `<span class="${baseClass} ${redBadge}">Denegado</span>`;
		if (row.puedeIngresar) return `<span class="${baseClass} ${greenBadge}">Permitido</span>`;
		return `<span class="${baseClass} ${redBadge}">Denegado</span>`;
	};

	// Actions Formatter
	const actionsFormatter = (cell: any) => {
		// We can check permissions here if we had access to $currentUser inside this pure function context
		// Instead we'll render all buttons and handle clicks securely.
		return `
            <div class="flex gap-1 justify-center">
                <button class="action-btn edit-btn p-1 hover:bg-white/10 rounded text-blue-400" title="Editar"><svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9"/><path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z"/></svg></button>
                <button class="action-btn car-btn p-1 hover:bg-white/10 rounded text-amber-400" title="Vehículos"><svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"/><circle cx="7" cy="17" r="2"/><circle cx="17" cy="17" r="2"/></svg></button>
                <button class="action-btn delete-btn p-1 hover:bg-white/10 rounded text-red-400" title="Eliminar"><svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg></button>
            </div>
        `;
	};

	// Cell Click Handler
	// Column Definitions (Tabulator)
	// We use a derived state to ensure handlers are fresh if they depend on closure variables
	// although handlers defined as functions on the component instance are stable enough.
	let columns = $derived(
		getContratistaColumns({
			onStatusChange: handleStatusChange,
			onEdit: (data: any) => openModal(data),
			onDelete: (data: any) => handleDelete(data),
			onVehiculoClick: (data: any) => openVehiculoModal(data)
		})
	);

	// ==========================================
	// DATA LOADING
	// ==========================================
	let gridWrapper: any = $state(); // Reference to the TabulatorWrapper component

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

	async function loadContratistas() {
		loading = true;
		error = '';
		// We don't clear contratistas immediately to avoid flash if possible, or just let Tabulator handle it
		try {
			const result = await contratistaService.fetchAllContratistas();
			if (result.ok) {
				contratistas = result.data.contratistas;
				// Manual update needed as prop is not reactive in wrapper
				if (gridWrapper) {
					gridWrapper.replaceData(contratistas);
					// Force redraw to fix layout glitches
					setTimeout(() => gridWrapper.redraw(), 100);
				}
			} else {
				error = result.error;
			}
		} catch {
			console.error('Error al cargar contratistas');
			error = 'Error al cargar contratistas';
		}
		loading = false;
	}

	// Filter Application Logic
	function applyFilters() {
		if (!gridWrapper) return;

		let filtered = [...contratistas];

		// Search
		if ($selectedSearchStore.result) {
			filtered = filtered.filter((c) => c.id === $selectedSearchStore.result!.id);
		}

		// Apply to grid
		gridWrapper.replaceData(filtered);
	}

	// Subscribe to global search
	$effect(() => {
		const _ = $selectedSearchStore;
		applyFilters();
	});

	// ==========================================
	// HANDLERS
	// ==========================================

	// Status Change
	async function handleStatusChange(id: string, status: string) {
		if (isUpdatingStatus) return;
		if (!$currentUser || !can($currentUser, 'UPDATE_CONTRACTOR')) {
			toast.error('No tienes permisos para cambiar el estado.');
			return;
		}

		isUpdatingStatus = true;
		const newStatus = status === 'activo' ? 'inactivo' : 'activo';
		const toastId = toast.loading(`Cambiando a ${newStatus}...`);

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
				toast.success(`Estado actualizado`, { id: toastId });
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
				toast.error(res.error, { id: toastId });
			}
		} catch (e) {
			gridWrapper?.updateRow(id, { estado: status });
			console.error(e);
			toast.error('Error al cambiar estado', { id: toastId });
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
					toast.success('Contratista actualizado');
					closeModal();
					loadContratistas();
				} else {
					toast.error(res.error);
				}
			} else {
				const res = await contratistaService.createContratista(data);
				if (res.ok) {
					toast.success('Contratista creado');
					closeModal();
					loadContratistas();
				} else {
					toast.error(res.error);
				}
			}
		} catch (e) {
			console.error(e);
			toast.error('Error al guardar contratista');
		}
		modalLoading = false;
	}

	// Delete Contractor
	async function handleDelete(contratista: ContratistaResponse) {
		if (!$currentUser || !can($currentUser, 'DELETE_CONTRACTOR')) {
			toast.error('No tienes permisos para eliminar.');
			return;
		}

		if (
			!confirm(
				`¿Estás seguro de eliminar a ${contratista.nombreCompleto}? Se moverá a la papelera.`
			)
		)
			return;

		const toastId = toast.loading('Eliminando...');
		const result = await contratistaService.deleteContratista(contratista.id);

		if (result.ok) {
			toast.success('Contratista movido a papelera', { id: toastId });
			loadContratistas();
		} else {
			toast.error(result.error, { id: toastId });
		}
	}

	// Vehiculo Actions
	function openVehiculoModal(contratista: ContratistaResponse) {
		selectedContratistaForVehicles = contratista;
		showVehiculoModal = true;
	}
	function closeVehiculoModal() {
		showVehiculoModal = false;
		selectedContratistaForVehicles = null;
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

	// Keyboard Subscriptions
	let unsubscribeKeyboard: (() => void) | null = null;
	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			const canCreate = $currentUser && can($currentUser, 'CREATE_CONTRACTOR');

			switch (event.command) {
				case 'create-new':
					if (canCreate && !showModal && !showVehiculoModal) {
						openModal();
						clearCommand();
					}
					break;
				case 'refresh':
					loadContratistas();
					clearCommand();
					break;
				case 'escape':
					if (showEstadoDropdown) showEstadoDropdown = false;
					if (showPraindDropdown) showPraindDropdown = false;
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
			setActiveContext('contratista-list');
		}
	});
</script>

```
<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
	<!-- Header -->
	<div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-gray-100">Lista de Contratistas</h2>
				<p class="mt-1 text-sm text-gray-400">
					Gestión y visualización de contratistas registrados
				</p>
			</div>
			<!-- Filters removed from here, now in Toolbar -->
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 flex flex-col overflow-hidden relative bg-[#1e1e1e]">
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
				searchable={true}
				downloadable={true}
				onSearch={(term) => {
					if (gridWrapper) {
						gridWrapper.getTable()?.setFilter('nombreCompleto', 'like', term);
					}
				}}
				onExport={(type) => {
					if (gridWrapper) {
						gridWrapper.getTable()?.download(type, `contratistas.${type}`);
					}
				}}
			>
				{#snippet primaryActions()}
					{#if selectedRows.length > 0}
						<!-- Selection Mode Actions -->
						<div
							class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200"
						>
							<button
								class="flex items-center gap-2 px-3 py-1.5
                                       bg-red-600/10 hover:bg-red-600/20
                                       text-red-400 hover:text-red-300
                                       border border-red-500/20 hover:border-red-500/30
                                       rounded-md text-sm font-medium transition-all"
								onclick={() => {
									if (confirm(`¿Eliminar ${selectedRows.length} elementos seleccionados?`)) {
										// TODO: Implement bulk delete
										console.log('Deleting', selectedRows);
										// After delete, clear selection
										gridWrapper?.deselectAll();
									}
								}}
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
						<!-- Default Actions -->
						{#if $currentUser && can($currentUser, 'CREATE_CONTRACTOR')}
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
				{/snippet}
			</GridToolbar>

			<!-- Tabulator Component (Clean) -->
			<div class="flex-1 overflow-hidden p-4 relative bg-[#1e1e1e]">
				<TabulatorWrapper
					bind:this={gridWrapper}
					data={[]}
					columns={columns as any}
					searchable={false}
					downloadable={false}
					withCheckboxSelection={true}
					onRowSelectionChanged={(data, rows) => {
						selectedRows = data;
					}}
					options={{
						height: '100%', // Explicit height to separate from overflow container
						...defaultTabulatorOptions,
						rowHeight: 40,
						layout: 'fitDataFill' // Ensure we use the safe layout
					}}
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
