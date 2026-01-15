<!-- src/lib/components/contratista/ContratistaListView.svelte -->
<!-- Vista unificada: Lista de contratistas + Modal para CRUD -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { Eye, Car } from 'lucide-svelte';
	import { can } from '$lib/logic/permissions';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId } from '$lib/stores/tabs';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	import { AlertCircle } from 'lucide-svelte';
	import { selectedSearchStore } from '$lib/stores/searchStore';
	import * as contratistaService from '$lib/logic/contratista/contratistaService';
	import { ContratistaColumns } from '$lib/logic/contratista/contratistaColumns';
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';
	import ContratistaFormModal from './ContratistaFormModal.svelte';
	import VehiculoManagerModal from '$lib/components/vehiculo/VehiculoManagerModal.svelte';

	import type { ContratistaResponse } from '$lib/types/contratista';
	import type { ColDef } from '@ag-grid-community/core';
	import { createCustomButton } from '$lib/config/agGridConfigs';

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	let contratistas = $state<ContratistaResponse[]>([]);
	let loading = $state(false);
	let error = $state('');
	let isUpdatingStatus = false;

	// States specific to Active Grid
	let selectedRows = $state<ContratistaResponse[]>([]);
	let showModal = $state(false);
	let editingContratista = $state<ContratistaResponse | null>(null);
	let modalLoading = $state(false);

	interface Props {
		tabId?: string;
	}
	let { tabId = 'contratista-list' }: Props = $props();

	// Suscripción a comandos de teclado centralizados
	let unsubscribeKeyboard: (() => void) | null = null;

	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			const canCreate = $currentUser && can($currentUser, 'CREATE_CONTRACTOR');
			const canDelete = $currentUser && can($currentUser, 'DELETE_CONTRACTOR');

			switch (event.command) {
				case 'create-new':
					if (canCreate && !showModal && !showVehiculoModal) {
						openModal();
						clearCommand();
					}
					break;
				case 'edit':
					if (selectedRows.length === 1 && !showModal) {
						openModal(selectedRows[0]);
						clearCommand();
					}
					break;
				case 'delete':
					if (canDelete && selectedRows.length === 1 && !showModal) {
						handleDelete(selectedRows[0]);
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) {
						closeModal();
						clearCommand();
					} else if (showVehiculoModal) {
						closeVehiculoModal();
						clearCommand();
					} else {
						showEstadoDropdown = false;
						showPraindDropdown = false;
					}
					break;
				case 'refresh':
					loadContratistas();
					clearCommand();
					break;
			}
		});
	}

	// Vehiculo Modal State
	let showVehiculoModal = $state(false);
	let selectedContratistaForVehicles = $state<ContratistaResponse | null>(null);

	// Filters
	let estadoFilter = $state<'todos' | 'activo' | 'inactivo' | 'suspendido'>('todos');
	let showEstadoDropdown = $state(false);
	let praindFilter = $state<'todos' | 'vigente' | 'vencido' | 'por-vencer'>('todos');
	let showPraindDropdown = $state(false);

	// ==========================================
	// DERIVED STATE
	// ==========================================
	const filteredData = $derived.by(() => {
		let filtered = contratistas;

		const _search = $selectedSearchStore;
		if (_search.result) {
			return filtered.filter((c) => c.id === _search.result!.id);
		}

		// Filtro de estado
		if (estadoFilter !== 'todos') {
			filtered = filtered.filter((c) => c.estado === estadoFilter);
		}

		// Filtro de PRAIND
		if (praindFilter === 'vigente') {
			filtered = filtered.filter((c) => !c.praindVencido && c.diasHastaVencimiento > 30);
		} else if (praindFilter === 'vencido') {
			filtered = filtered.filter((c) => c.praindVencido);
		} else if (praindFilter === 'por-vencer') {
			filtered = filtered.filter((c) => !c.praindVencido && c.diasHastaVencimiento <= 30);
		}

		return filtered;
	});

	// ... (labels)

	// Columnas - Only for Active List now
	const columnDefs = $derived.by((): ColDef<ContratistaResponse>[] => {
		const cols = ContratistaColumns.getColumns(handleStatusChange);

		return cols.map(
			(col) =>
				({
					field: String(col.field) as any,
					headerName: col.headerName,
					width: col.width,
					minWidth: col.minWidth,
					flex: col.flex,
					sortable: col.sortable !== false,
					filter: true,
					resizable: true,
					cellRenderer: col.cellRenderer,
					valueFormatter: col.valueFormatter,
					cellStyle: col.cellStyle,
					onCellClicked: col.onCellClicked
				}) as ColDef<ContratistaResponse>
		);
	});

	// Custom buttons
	const customButtons = $derived.by(() => {
		const selected = selectedRows[0];
		const canCreate = $currentUser && can($currentUser, 'CREATE_CONTRACTOR');
		const canUpdate = $currentUser && can($currentUser, 'UPDATE_CONTRACTOR');
		const canDelete = $currentUser && can($currentUser, 'DELETE_CONTRACTOR');
		const canViewDetail = $currentUser && can($currentUser, 'VIEW_CONTRACTOR_DETAIL');

		// Default buttons
		let defaultBtns = [];
		if (canCreate) {
			defaultBtns.push(createCustomButton.nuevo(() => openModal()));
		}

		// Single select buttons
		let singleSelectBtns = [];

		// Si puede actualizar, mostramos botón Editar estándar
		if (canUpdate) {
			singleSelectBtns.push(
				createCustomButton.editar(() => {
					if (selected) openModal(selected);
				})
			);
		} else {
			// Si NO puede actualizar, mostramos botón "Ver Detalle" si tiene el permiso VIEW_CONTRACTOR_DETAIL
			if (canViewDetail) {
				singleSelectBtns.push({
					id: 'view-detail',
					label: 'Ver Detalle',
					icon: Eye,
					onClick: () => {
						if (selected) openModal(selected, true); // true = readonly
					},
					variant: 'default' as const,
					tooltip: 'Ver detalles del contratista'
				});
			}
		}

		// Botón Gestión Vehículos
		if (canUpdate || canViewDetail) {
			singleSelectBtns.push({
				id: 'manage-vehicles',
				label: 'Vehículos',
				icon: Car,
				onClick: () => {
					if (selected) openVehiculoModal(selected);
				},
				variant: 'default' as const,
				tooltip: 'Gestionar vehículos'
			});
		}

		if (canDelete) {
			singleSelectBtns.push(
				createCustomButton.eliminar(() => {
					if (selected) handleDelete(selected);
				})
			);
		}

		return {
			default: defaultBtns,
			singleSelect: singleSelectBtns,
			multiSelect: []
		};
	});

	// ==========================================
	// HANDLERS - DATA
	// ==========================================
	async function loadContratistas() {
		loading = true;
		error = '';
		contratistas = []; // Clear current
		try {
			const result = await contratistaService.fetchAllContratistas();
			if (result.ok) {
				contratistas = (result.data as any).contratistas;
			} else {
				error = result.error;
			}
		} catch {
			console.error('Error al cargar contratistas');
			error = 'Error al cargar contratistas';
		}
		loading = false;
	}

	function handleRowDoubleClick(contratista: ContratistaResponse) {
		if (!$currentUser) return;

		const canUpdate = can($currentUser, 'UPDATE_CONTRACTOR');
		if (canUpdate) {
			openModal(contratista);
		} else {
			const canView = can($currentUser, 'VIEW_CONTRACTOR_DETAIL');
			if (canView) {
				openModal(contratista, true);
			}
		}
	}

	// ... (modal handlers remain same)

	// ==========================================
	// HANDLERS - MODAL
	// ==========================================
	let isReadOnlyModal = $state(false);

	function openModal(contratista: ContratistaResponse | null = null, readonly: boolean = false) {
		editingContratista = contratista;
		isReadOnlyModal = readonly;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingContratista = null;
	}

	async function handleSaveContratista(data: any) {
		// Should be typed properly based on input
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

	// ==========================================
	// HANDLERS - ACTIONS
	// ==========================================

	async function handleStatusChange(id: string, status: string) {
		if (isUpdatingStatus) return;
		isUpdatingStatus = true;

		const newStatus = status === 'activo' ? 'inactivo' : 'activo';

		// Optimistic update - save backup and update immediately
		const oldContratistas = [...contratistas];
		contratistas = contratistas.map((c) => (c.id === id ? { ...c, estado: newStatus } : c));

		const toastId = toast.loading(`Cambiando a ${newStatus}...`);

		try {
			const res = await contratistaService.changeEstado(id, newStatus as any);
			if (res.ok) {
				toast.success(`Estado actualizado a ${newStatus}`, { id: toastId });
				// No need to reload - optimistic update already applied
			} else {
				// Revert on error
				contratistas = oldContratistas;
				toast.error(res.error, { id: toastId });
			}
		} catch (e) {
			// Revert on error
			contratistas = oldContratistas;
			console.error(e);
			toast.error('Error al cambiar estado', { id: toastId });
		} finally {
			isUpdatingStatus = false;
		}
	}

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

	async function handleDelete(contratista: ContratistaResponse) {
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

	// ==========================================
	// HANDLERS - VEHICULO MODAL
	// ==========================================
	function openVehiculoModal(contratista: ContratistaResponse) {
		selectedContratistaForVehicles = contratista;
		showVehiculoModal = true;
	}

	function closeVehiculoModal() {
		showVehiculoModal = false;
		selectedContratistaForVehicles = null;
	}

	// ==========================================
	// LIFECYCLE
	// ==========================================
	onMount(async () => {
		loadContratistas();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) {
			unsubscribeKeyboard();
		}
	});

	// Registrar contexto activo cuando esta pestaña está activa
	$effect(() => {
		if ($activeTabId === tabId) {
			setActiveContext('contratista-list');
		}
	});
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
	<!-- Header -->
	<div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-gray-100">"Lista de Contratistas"</h2>
				<p class="mt-1 text-sm text-gray-400">
					Gestión y visualización de contratistas registrados
				</p>
			</div>
			<div class="flex-1 max-w-md">
				<!-- Only show searchbar if in active mode or if trash view supports it (trash view has internal logic or we hide it) -->
				<!-- For now we hide searchbar in trash view to simplify as trash view component handles its own display -->
				<SearchBar placeholder="Buscar por nombre, cédula o empresa..." limit={10} />
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
		{#if error}
			<div class="p-6">
				<div
					class="flex items-center gap-3 rounded-lg border border-red-500/20 bg-red-500/10 p-4 text-red-400"
					transition:fade
				>
					<AlertCircle size={20} />
					<div>
						<div class="font-medium">Error al cargar contratistas</div>
						<div class="text-sm opacity-90">{error}</div>
					</div>
				</div>
			</div>
		{:else if loading}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<svg class="mx-auto h-8 w-8 animate-spin text-blue-500" fill="none" viewBox="0 0 24 24">
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						/>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						/>
					</svg>
					<p class="mt-4 text-sm text-gray-400">Cargando contratistas...</p>
				</div>
			</div>
		{:else}
			<!-- Siempre mostrar la grid, AG Grid muestra "No hay filas para mostrar" cuando está vacío -->
			<AGGridWrapper
				gridId="contratista-list"
				{columnDefs}
				rowData={filteredData}
				{customButtons}
				getRowId={(params) => params.data.id}
				persistenceKey="contratistas-list-columns"
				onSelectionChanged={(rows) => (selectedRows = rows)}
				onRowDoubleClicked={handleRowDoubleClick}
			/>
		{/if}

		<!-- Filter Dropdowns (Only active view) -->
		<div class="filter-dropdown-container">
			{#if showEstadoDropdown}
				<div
					class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
					transition:fade={{ duration: 150 }}
				>
					{#each [['todos', 'Todos los estados'], ['activo', 'Activos'], ['inactivo', 'Inactivos'], ['suspendido', 'Suspendidos']] as [value, label]}
						<button
							onclick={() => handleEstadoSelect(value as any)}
							class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {estadoFilter ===
							value
								? 'bg-blue-500/20 text-blue-400'
								: ''}"
						>
							{label}
						</button>
					{/each}
				</div>
			{/if}

			{#if showPraindDropdown}
				<div
					class="absolute top-16 left-52 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
					transition:fade={{ duration: 150 }}
				>
					{#each [['todos', 'Todos PRAIND'], ['vigente', 'Vigentes'], ['por-vencer', 'Por vencer (≤30 días)'], ['vencido', 'Vencidos']] as [value, label]}
						<button
							onclick={() => handlePraindSelect(value as any)}
							class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {praindFilter ===
							value
								? 'bg-blue-500/20 text-blue-400'
								: ''}"
						>
							{label}
						</button>
					{/each}
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Modal -->
<ContratistaFormModal
	show={showModal}
	contratista={editingContratista}
	readonly={isReadOnlyModal}
	loading={modalLoading}
	onSave={handleSaveContratista}
	onClose={closeModal}
/>

<!-- Modal Vehículos -->
{#if selectedContratistaForVehicles}
	<VehiculoManagerModal
		show={showVehiculoModal}
		propietarioId={selectedContratistaForVehicles.id}
		propietarioNombre={selectedContratistaForVehicles.nombreCompleto ||
			selectedContratistaForVehicles.nombre + ' ' + selectedContratistaForVehicles.apellido}
		onClose={closeVehiculoModal}
	/>
{/if}
