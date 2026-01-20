<!-- src/lib/components/user/UserListView.svelte -->
<!-- Vista unificada: Lista de usuarios + Modal para CRUD -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle, UserPlus, Pencil, Trash2, X } from 'lucide-svelte';

	// Components
	import TabulatorWrapper from '$lib/components/tabulator/TabulatorWrapper.svelte';
	import GridToolbar from '$lib/components/tabulator/GridToolbar.svelte';
	import UserFormModal from './UserFormModal.svelte';
	import ConfirmPasswordModal from '$lib/components/shared/ConfirmPasswordModal.svelte';

	// Logic & Config
	import * as userService from '$lib/logic/user/userService';
	import { getUserColumns } from '$lib/logic/user/userColumns';
	import { defaultTabulatorOptions } from '$lib/logic/tabulator/tabulatorController';

	// Types
	import type { UserResponse, CreateUserInput, UpdateUserInput } from '$lib/types/user';
	import { searchByType } from '$lib/api/searchService';

	// Stores
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId } from '$lib/stores/tabs';
	import { can } from '$lib/logic/permissions';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	interface Props {
		tabId: string;
		data?: any;
	}

	let { tabId, data }: Props = $props();

	// Effect to handle external actions (like from Spotlight)
	$effect(() => {
		if (data?.openCreateModal) {
			// Usamos un timeout pequeño para asegurar que el componente esté listo
			setTimeout(() => {
				if (!showModal) {
					openModal(null);
				}
			}, 100);
			// Consumir el flag para evitar re-aperturas no deseadas (opcional, pero buena práctica)
			data.openCreateModal = false;
		}
	});

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	let users = $state<UserResponse[]>([]);
	let loading = $state(false);
	let error = $state('');
	let isUpdatingStatus = false;
	let selectedRows = $state<UserResponse[]>([]);
	let searchTerm = $state('');

	// Modals
	let showModal = $state(false);
	let editingUser = $state<UserResponse | null>(null);
	let isReadOnlyModal = $state(false);
	let modalLoading = $state(false);

	let showSelfDeactivateModal = $state(false);
	let pendingSelfDeactivation = $state<{ id: string; currentStatus: boolean } | null>(null);

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

	// Keyboard Subscription
	let unsubscribeKeyboard: (() => void) | null = null;

	function setupKeyboardSubscription() {
		unsubscribeKeyboard = keyboardCommand.subscribe((event) => {
			if (!event) return;
			if ($activeTabId !== tabId) return;

			const canCreate = $currentUser && can($currentUser, 'CREATE_USER');
			const canDelete = $currentUser && can($currentUser, 'DELETE_USER');

			switch (event.command) {
				case 'create-new':
					if (canCreate && !showModal) {
						openModal(null);
						clearCommand();
					}
					break;
				case 'escape':
					if (showModal) {
						closeModal();
						clearCommand();
					}
					break;
				case 'refresh':
					loadUsers();
					clearCommand();
					break;
			}
		});
	}

	// ==========================================
	// COLUMNS
	// ==========================================
	let columns = $derived(
		getUserColumns({
			onStatusToggle: (id, currentStatus) => handleStatusChange(id, currentStatus)
		})
	);

	// ==========================================
	// HANDLERS - DATA
	// ==========================================

	async function loadUsers() {
		loading = true;
		error = '';
		try {
			const result = await userService.fetchAllUsers();
			if (result.ok) {
				users = result.data;
				if (gridWrapper) {
					gridWrapper.replaceData(users);
				}
			} else {
				error = result.error;
			}
		} catch (_err) {
			error = 'Error al cargar usuarios';
		}
		loading = false;
	}

	// ==========================================
	// HANDLERS - MODAL
	// ==========================================

	function openModal(user: UserResponse | null, readonly: boolean = false) {
		editingUser = user;
		isReadOnlyModal = readonly;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingUser = null;
	}

	async function handleSaveUser(data: CreateUserInput | UpdateUserInput): Promise<boolean> {
		modalLoading = true;
		try {
			if (editingUser) {
				const result = await userService.updateUser(editingUser.id, data as UpdateUserInput);
				if (result.ok) {
					toast.success('Usuario actualizado');
					loadUsers();
					return true;
				} else {
					toast.error(result.error);
					return false;
				}
			} else {
				const result = await userService.createUser(data as CreateUserInput);
				if (result.ok) {
					toast.success('Usuario creado');
					await loadUsers();
					return true;
				} else {
					toast.error(result.error);
					return false;
				}
			}
		} catch (e) {
			console.error(e);
			toast.error('Error inesperado');
			return false;
		} finally {
			modalLoading = false;
		}
	}

	// ==========================================
	// HANDLERS - STATUS
	// ==========================================

	async function handleStatusChange(id: string, currentStatus: boolean) {
		if (loading || isUpdatingStatus) return;

		if ($currentUser && id === $currentUser.id && currentStatus === true) {
			pendingSelfDeactivation = { id, currentStatus };
			showSelfDeactivateModal = true;
			return;
		}

		await executeStatusChange(id, currentStatus);
	}

	async function executeStatusChange(id: string, currentStatus: boolean) {
		try {
			isUpdatingStatus = true;
			const newStatus = !currentStatus;

			const toastId = toast.loading('Actualizando estado...');
			const result = await userService.changeStatus(id, newStatus);

			if (result.ok) {
				toast.success(newStatus ? 'Usuario activado' : 'Usuario desactivado', {
					id: toastId
				});
				loadUsers();
			} else {
				toast.error(result.error || 'Error al cambiar estado', { id: toastId });
			}
		} finally {
			isUpdatingStatus = false;
		}
	}

	// ==========================================
	// HANDLERS - DELETE
	// ==========================================

	async function handleDeleteUser(user: UserResponse) {
		if ($currentUser && user.id === $currentUser.id) {
			toast.error('No puedes eliminar tu propia cuenta.', { icon: '🚫' });
			return;
		}

		if (!confirm(`¿Eliminar a ${user.nombre}?`)) return;
		const toastId = toast.loading('Eliminando...');
		const result = await userService.deleteUser(user.id);
		if (result.ok) {
			toast.success('Usuario eliminado', { id: toastId });
			loadUsers();
		} else {
			toast.error(result.error, { id: toastId });
		}
	}

	async function handleDeleteMultiple(selection: UserResponse[]) {
		const selfIncluded = $currentUser && selection.some((u) => u.id === $currentUser.id);
		let toDelete = selection;
		if (selfIncluded) {
			toast.error('No puedes eliminarte a ti mismo. Excluido de la selección.', { icon: '⚠️' });
			toDelete = selection.filter((u) => u.id !== $currentUser!.id);
			if (toDelete.length === 0) return;
		}

		if (!confirm(`¿Eliminar ${toDelete.length} usuarios?`)) return;
		const toastId = toast.loading('Eliminando...');
		let errors = 0;
		for (const u of toDelete) {
			const res = await userService.deleteUser(u.id);
			if (!res.ok) errors++;
		}
		if (errors === 0) {
			toast.success('Usuarios eliminados', { id: toastId });
		} else {
			toast.error(`Errores: ${errors}`, { id: toastId });
		}
		loadUsers();
		gridWrapper?.deselectAll();
	}

	function handleRowDoubleClick(e: any, row: any) {
		const user = row.getData() as UserResponse;
		if (!$currentUser) return;

		const canUpdate = can($currentUser, 'UPDATE_USER_PROFILE', user);
		if (canUpdate) {
			openModal(user);
		} else {
			const canView = can($currentUser, 'VIEW_USER_DETAIL');
			if (canView) {
				openModal(user, true);
			}
		}
	}

	async function handleSearch(term: string) {
		searchTerm = term;
		if (!term || term.trim().length < 2) {
			if (gridWrapper) gridWrapper.replaceData(users);
			return;
		}
		try {
			const results = await searchByType(term, 'user', 100);
			const matchedIds = new Set(results.map((r) => r.id));
			const filtered = users.filter((u) => matchedIds.has(u.id));
			if (gridWrapper) gridWrapper.replaceData(filtered);
		} catch (e) {
			console.error('Error en búsqueda inteligente:', e);
			if (gridWrapper) gridWrapper.getTable()?.setFilter('nombre', 'like', term);
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

	// Lifecycle
	onMount(() => {
		loadUsers();
		setupKeyboardSubscription();
	});

	onDestroy(() => {
		if (unsubscribeKeyboard) unsubscribeKeyboard();
	});

	$effect(() => {
		if ($activeTabId === tabId) {
			setActiveContext('users-list');
		}
	});
</script>

<div class="flex h-full flex-col relative bg-surface-1">
	<!-- Header -->
	<div class="border-b border-surface px-6 py-4 bg-surface-2">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-primary">Lista de Usuarios</h2>
				<p class="mt-1 text-sm text-secondary">
					Gestión y visualización de todos los usuarios del sistema
				</p>
			</div>
		</div>
	</div>

	<!-- Toolbar -->
	<GridToolbar
		{searchTerm}
		onSearch={handleSearch}
		hasSelection={selectedRows.length > 0}
		onAutoSizeColumns={() => gridWrapper?.autoSizeColumns()}
		onFitColumns={() => gridWrapper?.fitColumns()}
		onToggleColumn={(field) => gridWrapper?.toggleColumn(field)}
		onToggleFreeze={(field) => gridWrapper?.toggleFreeze(field)}
		onToggleFilters={handleToggleFilters}
		columns={toolbarColumns}
	>
		{#snippet primaryActions()}
			{#if selectedRows.length > 0}
				<div class="flex items-center gap-2 animate-in fade-in slide-in-from-left-2 duration-200">
					<button
						onclick={() => gridWrapper?.deselectAll()}
						class="flex items-center gap-1.5 px-3 py-1.5 bg-surface-3 text-secondary border border-surface rounded-md hover:bg-surface-4 hover:text-primary text-sm font-medium transition-colors"
						title="Cancelar selección"
					>
						<X size={14} /> Cancelar
					</button>

					{#if selectedRows.length === 1}
						{@const selected = selectedRows[0]}
						{#if $currentUser && can($currentUser, 'UPDATE_USER_PROFILE', selected)}
							<button
								onclick={() => openModal(selected)}
								class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500/10 text-amber-400 border border-amber-500/20 rounded-md hover:bg-amber-500/20 text-sm font-medium transition-colors"
							>
								<Pencil size={14} /> Editar
							</button>
						{/if}
					{/if}

					{#if $currentUser && can($currentUser, 'DELETE_USER')}
						<button
							onclick={() => handleDeleteMultiple(selectedRows)}
							class="flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 text-red-400 border border-red-500/20 rounded-md hover:bg-red-500/20 text-sm font-medium transition-colors"
						>
							<Trash2 size={14} /> Eliminar ({selectedRows.length})
						</button>
					{/if}
				</div>
			{:else if $currentUser && can($currentUser, 'CREATE_USER')}
				<button
					onclick={() => openModal(null)}
					class="flex items-center gap-1.5 px-3 py-1.5 bg-blue-500/10 text-blue-400 border border-blue-500/20 rounded-md hover:bg-blue-500/20 text-sm font-medium transition-colors"
				>
					<UserPlus size={14} /> Nuevo Usuario
				</button>
			{/if}
		{/snippet}
	</GridToolbar>

	<!-- Content -->
	<div
		class="flex-1 overflow-hidden relative bg-surface-1 border-t border-surface {showHeaderFilters
			? ''
			: 'hide-filters'}"
	>
		{#if loading && users.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="loading loading-spinner loading-lg text-primary"></div>
			</div>
		{:else if error}
			<div class="p-8 text-center text-red-400">{error}</div>
		{:else}
			<TabulatorWrapper
				bind:this={gridWrapper}
				bind:toolbarColumns
				data={users}
				{columns}
				withCheckboxSelection={true}
				onRowSelectionChanged={(data) => (selectedRows = data)}
				onRowDblClick={handleRowDoubleClick}
				persistenceID="users-list-v1"
				options={{
					...defaultTabulatorOptions,
					placeholder: 'No hay usuarios registrados'
				}}
			/>
		{/if}
	</div>
</div>

<!-- Modals -->
<UserFormModal
	show={showModal}
	user={editingUser}
	readonly={isReadOnlyModal}
	loading={modalLoading}
	onSave={handleSaveUser}
	onClose={closeModal}
/>

<ConfirmPasswordModal
	show={showSelfDeactivateModal}
	title="Desactivar Tu Cuenta"
	warningMessage="⚠️ ADVERTENCIA: Estás a punto de desactivar tu propia cuenta. Una vez desactivada, NO podrás iniciar sesión hasta que otro administrador te reactive."
	confirmButtonText="Sí, Desactivar Mi Cuenta"
	user={$currentUser}
	onConfirm={() => {
		if (pendingSelfDeactivation)
			executeStatusChange(pendingSelfDeactivation.id, pendingSelfDeactivation.currentStatus);
		showSelfDeactivateModal = false;
	}}
	onCancel={() => (showSelfDeactivateModal = false)}
/>

<style>
	:global(.hide-filters .tabulator-header-filter) {
		display: none !important;
	}
</style>
