<!-- src/lib/components/proveedor/ProveedorListView.svelte -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';
	import ProveedorFormModal from '$lib/components/proveedor/ProveedorFormModal.svelte';
	import {
		fetchAllProveedores,
		createProveedor,
		updateProveedor,
		deleteProveedor,
		changeStatus
	} from '$lib/logic/proveedor/proveedorService';
	import { ProveedorColumns } from '$lib/logic/proveedor/proveedorColumns';
	import { createCustomButton } from '$lib/config/agGridConfigs';
	import type {
		ProveedorResponse,
		CreateProveedorInput,
		UpdateProveedorInput
	} from '$lib/types/proveedor';
	import { toast } from 'svelte-5-french-toast';
	import { activeTabId } from '$lib/stores/tabs';
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import { selectedSearchStore } from '$lib/stores/searchStore';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	interface Props {
		tabId?: string;
	}
	let { tabId = 'proveedor-list' }: Props = $props();

	// Estado del Grid
	let proveedores = $state<ProveedorResponse[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	// Estado del Modal
	let showModal = $state(false);
	let selectedProveedor = $state<ProveedorResponse | null>(null);
	let modalLoading = $state(false);
	let isUpdatingStatus = false;

	// Selección
	let selectedRows = $state<ProveedorResponse[]>([]);

	// Suscripción a comandos de teclado centralizados
	let unsubscribeKeyboard: (() => void) | null = null;

	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			switch (event.command) {
				case 'create-new':
					if (!showModal) {
						openFormModal(null);
						clearCommand();
					}
					break;
				case 'edit':
					if (selectedRows.length === 1 && !showModal) {
						openFormModal(selectedRows[0]);
						clearCommand();
					}
					break;
				case 'delete':
					if (selectedRows.length === 1 && !showModal) {
						confirmDelete(selectedRows[0]);
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) {
						showModal = false;
						clearCommand();
					}
					break;
				case 'refresh':
					loadData();
					clearCommand();
					break;
			}
		});
	}

	// Carga inicial
	const loadData = async () => {
		loading = true;
		error = null;

		let res;
		res = await fetchAllProveedores();

		if (res.ok) {
			proveedores = res.data;
		} else {
			error = res.error;
			toast.error(res.error);
		}
		loading = false;
	};

	// Manejadores del Grid

	let isReadOnlyModal = $state(false);

	function openFormModal(proveedor: ProveedorResponse | null, readonly: boolean = false) {
		selectedProveedor = proveedor;
		isReadOnlyModal = readonly;
		showModal = true;
	}

	// Creación / Edición
	async function handleSave(data: CreateProveedorInput | UpdateProveedorInput) {
		modalLoading = true;
		let result;

		if (selectedProveedor) {
			result = await updateProveedor(selectedProveedor.id, data as UpdateProveedorInput);
		} else {
			result = await createProveedor(data as CreateProveedorInput);
		}

		modalLoading = false;

		if (result.ok) {
			toast.success(selectedProveedor ? 'Proveedor actualizado' : 'Proveedor creado');
			loadData();
			showModal = false;
			return true;
		} else {
			toast.error(result.error);
			return false;
		}
	}

	// Cambio de estado
	async function handleStatusChange(id: string, currentStatus: any) {
		if (isUpdatingStatus) return;
		isUpdatingStatus = true;

		const newStatus = currentStatus === 'ACTIVO' ? 'INACTIVO' : 'ACTIVO';
		const toastId = toast.loading(`Cambiando estado a ${newStatus}...`);

		try {
			const res = await changeStatus(id, newStatus);

			if (res.ok) {
				toast.success('Estado actualizado', { id: toastId });
				proveedores = proveedores.map((p) =>
					p.id === id
						? {
								...p,
								estado: res.data.estado,
								puedeIngresar: res.data.puedeIngresar
							}
						: p
				);
			} else {
				toast.error(res.error, { id: toastId });
			}
		} catch (e) {
			console.error(e);
			toast.error('Error al cambiar estado', { id: toastId });
		} finally {
			isUpdatingStatus = false;
		}
	}

	// Eliminar
	async function confirmDelete(proveedor: ProveedorResponse) {
		if (!confirm(`¿Estás seguro de eliminar al proveedor "${proveedor.nombre}"?`)) return;

		const res = await deleteProveedor(proveedor.id);
		if (res.ok) {
			toast.success('Proveedor eliminado');
			loadData();
		} else {
			toast.error(res.error);
		}
	}

	// Botones Custom
	const customButtons = $derived.by(() => {
		const selected = selectedRows[0];
		let defaultBtns = [];
		defaultBtns.push(createCustomButton.nuevo(() => openFormModal(null)));

		let singleSelectBtns = [];

		singleSelectBtns.push(
			createCustomButton.editar(() => {
				if (selected) openFormModal(selected);
			})
		);

		singleSelectBtns.push(
			createCustomButton.eliminar(() => {
				if (selected) confirmDelete(selected);
			})
		);

		return {
			default: defaultBtns,
			singleSelect: singleSelectBtns,
			multiSelect: []
		};
	});

	// Definición de columnas
	const columnDefs = $derived(ProveedorColumns.getColumns(handleStatusChange));

	// Datos filtrados por buscador
	const filteredData = $derived.by(() => {
		let filtered = proveedores;
		const _search = $selectedSearchStore;
		if (_search.result && _search.result.tipo === 'proveedor') {
			return filtered.filter((p) => p.id === _search.result!.id);
		}
		return filtered;
	});

	// Lifecycle
	onMount(() => {
		loadData();
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
			setActiveContext('proveedor-list');
		}
	});
</script>

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
	<!-- Header -->
	<div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-gray-100">"Lista de Proveedores"</h2>
				<p class="mt-1 text-sm text-gray-400">Gestión y visualización de proveedores registrados</p>
			</div>
			<div class="flex-1 max-w-md">
				<SearchBar placeholder="Buscar por nombre, cédula o empresa..." limit={10} />
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="flex-1 overflow-hidden relative bg-[#1e1e1e]">
		{#if loading}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<svg class="mx-auto h-8 w-8 animate-spin text-blue-500" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					<p class="mt-4 text-sm text-gray-400">Cargando proveedores...</p>
				</div>
			</div>
		{:else}
			<AGGridWrapper
				gridId="proveedor-list"
				persistenceKey="proveedor-list-columns-v5"
				rowData={filteredData}
				{columnDefs}
				{customButtons}
				onSelectionChanged={(rows) => {
					selectedRows = rows;
				}}
				getRowId={(params) => params.data.id}
			/>
		{/if}
	</div>
</div>

<ProveedorFormModal
	show={showModal}
	proveedor={selectedProveedor}
	readonly={isReadOnlyModal}
	loading={modalLoading}
	onSave={handleSave}
	onClose={() => (showModal = false)}
/>
