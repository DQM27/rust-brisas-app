<!-- src/lib/components/user/UserListView.svelte -->
<!-- Vista unificada: Lista de usuarios + Modal para CRUD -->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { toast } from 'svelte-5-french-toast';
	import { AlertCircle } from 'lucide-svelte';
	import type { ColDef } from '@ag-grid-community/core';

	// Components
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import AGGridWrapper from '$lib/components/grid/AGGridWrapper.svelte';
	import UserFormModal from './UserFormModal.svelte';
	import ConfirmPasswordModal from '$lib/components/shared/ConfirmPasswordModal.svelte';

	// Logic & Config
	import * as userService from '$lib/logic/user/userService';
	import { UserColumns } from '$lib/logic/user/userColumns';
	import { createCustomButton } from '$lib/config/agGridConfigs';

	// Types
	import type { UserResponse, CreateUserInput, UpdateUserInput } from '$lib/types/user';

	// Stores
	import { selectedSearchStore } from '$lib/stores/searchStore';
	import { currentUser } from '$lib/stores/auth';
	import { activeTabId } from '$lib/stores/tabs';
	import { can } from '$lib/logic/permissions';
	import { Eye } from 'lucide-svelte';
	import { keyboardCommand, setActiveContext, clearCommand } from '$lib/stores/keyboardCommands';

	interface Props {
		tabId: string;
		tabId: string;
	}

	let { tabId }: Props = $props();

	// ==========================================
	// ESTADO LOCAL
	// ==========================================
	let users = $state<UserResponse[]>([]);
	let loading = $state(false);
	let error = $state('');
	let isUpdatingStatus = false;

	// Estado de filtros (inline, sin clase externa)
	let roleFilter = $state<'todos' | 'admin' | 'supervisor' | 'guardia'>('todos');
	let estadoFilter = $state<'todos' | 'activo' | 'inactivo'>('todos');

	// Estado para selección en grid
	let selectedRows = $state<UserResponse[]>([]);

	// Estado para dropdowns de filtros
	let showRoleDropdown = $state(false);
	let showEstadoDropdown = $state(false);

	// Estado para modal
	let showModal = $state(false);
	let editingUser = $state<UserResponse | null>(null);
	let modalLoading = $state(false);

	// Estado para modal de confirmación de auto-desactivación
	let showSelfDeactivateModal = $state(false);
	let pendingSelfDeactivation = $state<{
		id: string;
		currentStatus: boolean;
	} | null>(null);

	// Suscripción a comandos de teclado centralizados
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
				case 'edit':
					if (selectedRows.length === 1 && !showModal) {
						openModal(selectedRows[0]);
						clearCommand();
					}
					break;
				case 'delete':
					if (canDelete && selectedRows.length > 0 && !showModal) {
						if (selectedRows.length === 1) {
							handleDeleteUser(selectedRows[0]);
						} else {
							handleDeleteMultiple(selectedRows);
						}
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
	// DERIVADOS
	// ==========================================

	// Datos filtrados (reemplaza UserListLogic.getFilteredData)
	let filteredData = $derived.by(() => {
		let filtered = users;

		// Filtro por búsqueda global (prioridad)
		const selectedSearch = $selectedSearchStore;
		if (selectedSearch.result) {
			return filtered.filter((u) => u.id === selectedSearch.result!.id);
		}

		// Filtro de rol
		if (roleFilter !== 'todos') {
			filtered = filtered.filter((u) => u.roleName === roleFilter);
		}

		// Filtro de estado
		if (estadoFilter !== 'todos') {
			const isActive = estadoFilter === 'activo';
			filtered = filtered.filter((u) => u.isActive === isActive);
		}

		return filtered;
	});

	// Columnas de AG Grid (estáticas para evitar re-render innecesarios)
	// NOTA: Se crea una vez y se guarda en $state para que la referencia no cambie
	const columnDefs: ColDef<UserResponse>[] = UserColumns.getColumns((id, currentStatus) => {
		handleStatusChange(id, currentStatus);
	});

	// Botones personalizados por contexto
	const customButtons = $derived.by(() => {
		const selected = selectedRows[0];
		const canCreate = $currentUser && can($currentUser, 'CREATE_USER');
		const canDelete = $currentUser && can($currentUser, 'DELETE_USER');
		const canViewDetail = $currentUser && can($currentUser, 'VIEW_USER_DETAIL');

		let defaultBtns = [];
		if (canCreate) {
			defaultBtns.push(createCustomButton.nuevo(() => openModal(null)));
		}

		// Botones custom para singleSelect - requieren handlers específicos de User
		let singleSelectBtns: any[] = [];

		const canUpdateSelected =
			selected && $currentUser && can($currentUser, 'UPDATE_USER_PROFILE', selected);

		if (canUpdateSelected) {
			singleSelectBtns.push(
				createCustomButton.editar(() => {
					if (selected) openModal(selected);
				})
			);
		} else if (canViewDetail && selected) {
			// Si no puede editar, mostrar Ver Detalle (readonly)
			singleSelectBtns.push({
				id: 'view-detail',
				label: 'Ver Detalle',
				icon: Eye,
				onClick: () => {
					if (selected) openModal(selected, true);
				},
				variant: 'default' as const,
				tooltip: 'Ver detalles del usuario (solo lectura)'
			});
		}

		if (canDelete && selected) {
			singleSelectBtns.push(
				createCustomButton.eliminar(() => {
					if (selected) handleDeleteUser(selected);
				})
			);
		}

		// Botones custom para multiSelect
		let multiSelectBtns: any[] = [];
		if (canDelete && selectedRows.length > 0) {
			multiSelectBtns.push(
				createCustomButton.eliminar(() => {
					handleDeleteMultiple(selectedRows);
				})
			);
		}

		return {
			default: defaultBtns,
			singleSelect: singleSelectBtns,
			multiSelect: multiSelectBtns
		};
	});

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

	let isReadOnlyModal = $state(false);

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
				// Modo edición
				const result = await userService.updateUser(editingUser.id, data as UpdateUserInput);
				if (result.ok) {
					toast.success('Usuario actualizado');
					users = users.map((u) => (u.id === editingUser!.id ? result.data : u));
					// closeModal(); // Dejamos que el modal controle el cierre
					return true;
				} else {
					toast.error(result.error);
					return false;
				}
			} else {
				// Modo creación
				const result = await userService.createUser(data as CreateUserInput);
				if (result.ok) {
					toast.success('Usuario creado');
					await loadUsers(); // Recargar para obtener el nuevo usuario
					// closeModal(); // Dejamos que el modal controle el cierre (para mostrar password)
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

		// ⚠️ AUTO-DESACTIVACIÓN: Mostrar modal de confirmación con contraseña
		if ($currentUser && id === $currentUser.id && currentStatus === true) {
			// Solo pedir confirmación si se va a DESACTIVAR (currentStatus es true = activo)
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

			// Actualización optimista
			const oldUsers = [...users];
			users = users.map((u) => (u.id === id ? { ...u, isActive: newStatus } : u));

			const toastId = toast.loading('Actualizando estado...');
			const result = await userService.changeStatus(id, newStatus);

			if (result.ok) {
				toast.success(newStatus ? 'Usuario activado' : 'Usuario desactivado', {
					id: toastId
				});
			} else {
				users = oldUsers;
				toast.error(result.error || 'Error al cambiar estado', { id: toastId });
			}
		} finally {
			isUpdatingStatus = false;
		}
	}

	async function confirmSelfDeactivation() {
		if (!pendingSelfDeactivation) return;

		await executeStatusChange(pendingSelfDeactivation.id, pendingSelfDeactivation.currentStatus);

		showSelfDeactivateModal = false;
		pendingSelfDeactivation = null;

		// Después de desactivarse, se cerrará la sesión automáticamente
		// o quedará bloqueado según la lógica del sistema
	}

	function cancelSelfDeactivation() {
		showSelfDeactivateModal = false;
		pendingSelfDeactivation = null;
	}

	// ==========================================
	// HANDLERS - DELETE
	// ==========================================

	async function handleDeleteUser(user: UserResponse) {
		// ⚠️ PROTECCIÓN: No permitir auto-eliminación
		if ($currentUser && user.id === $currentUser.id) {
			toast.error(
				'No puedes eliminar tu propia cuenta. Solicita a otro administrador que lo haga.',
				{
					duration: 5000,
					icon: '🚫'
				}
			);
			return;
		}

		if (!confirm(`¿Eliminar a ${user.nombre}?`)) return;
		const toastId = toast.loading('Eliminando...');
		const result = await userService.deleteUser(user.id);
		if (result.ok) {
			toast.success('Usuario eliminado', { id: toastId });
			users = users.filter((u) => u.id !== user.id);
		} else {
			toast.error(result.error, { id: toastId });
		}
	}

	async function handleDeleteMultiple(usersToDelete: UserResponse[]) {
		// ⚠️ PROTECCIÓN: Filtrar el usuario actual de la selección
		const selfIncluded = $currentUser && usersToDelete.some((u) => u.id === $currentUser.id);
		if (selfIncluded) {
			toast.error('No puedes eliminarte a ti mismo. Te he excluido de la selección.', {
				duration: 4000,
				icon: '⚠️'
			});
			usersToDelete = usersToDelete.filter((u) => u.id !== $currentUser!.id);
			if (usersToDelete.length === 0) return;
		}

		if (!confirm(`¿Eliminar ${usersToDelete.length} usuarios?`)) return;
		const toastId = toast.loading('Eliminando...');
		let errors = 0;
		for (const u of usersToDelete) {
			const res = await userService.deleteUser(u.id);
			if (!res.ok) errors++;
		}
		if (errors === 0) {
			toast.success('Usuarios eliminados', { id: toastId });
		} else {
			toast.error(`Errores: ${errors}`, { id: toastId });
		}
		loadUsers();
	}

	function handleRowDoubleClick(user: UserResponse) {
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

	// ==========================================
	// HANDLERS - FILTROS
	// ==========================================

	function handleRoleSelect(value: 'todos' | 'admin' | 'supervisor' | 'guardia') {
		roleFilter = value;
		showRoleDropdown = false;
	}

	function handleEstadoSelect(value: 'todos' | 'activo' | 'inactivo') {
		estadoFilter = value;
		showEstadoDropdown = false;
	}

	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('.filter-dropdown-container') && !target.closest('[data-filter-button]')) {
			showRoleDropdown = false;
			showEstadoDropdown = false;
		}
	}

	// ==========================================
	// LIFECYCLE
	// ==========================================

	// Sincronizar cambios del usuario actual (ej: edición desde Sidebar)
	$effect(() => {
		if ($currentUser && users.length > 0) {
			const index = users.findIndex((u) => u.id === $currentUser.id);
			if (index !== -1) {
				// Verificar si hay cambios reales para evitar reactividad innecesaria
				if (JSON.stringify(users[index]) !== JSON.stringify($currentUser)) {
					users[index] = $currentUser;
				}
			}
		}
	});

	onMount(() => {
		loadUsers();
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
			setActiveContext('users-list');
		}
	});
</script>

<svelte:window onclick={handleClickOutside} />

<div class="flex h-full flex-col relative bg-[#1e1e1e]">
	<!-- Header -->
	<div class="border-b border-white/10 px-6 py-4 bg-[#252526]">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="text-xl font-semibold text-gray-100">Lista de Usuarios</h2>
				<p class="mt-1 text-sm text-gray-400">
					Gestión y visualización de todos los usuarios del sistema
				</p>
			</div>
			<div class="flex-1 max-w-md">
				<SearchBar placeholder="Buscar por nombre, cédula o email..." limit={10} />
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
						<div class="font-medium">Error al cargar usuarios</div>
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
					<p class="mt-4 text-sm text-gray-400">Cargando usuarios...</p>
				</div>
			</div>
		{:else if users.length === 0}
			<div class="flex h-full items-center justify-center">
				<div class="text-center">
					<AlertCircle size={48} class="mx-auto text-gray-400" />
					<p class="mt-4 text-lg font-medium text-gray-300">No hay usuarios</p>
					<p class="mt-2 text-sm text-gray-400">Crea el primer usuario para comenzar</p>
					{#if $currentUser && can($currentUser, 'CREATE_USER')}
						<button
							onclick={() => openModal(null)}
							class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
						>
							Nuevo Usuario
						</button>
					{/if}
				</div>
			</div>
		{:else}
			<AGGridWrapper
				gridId="users-list"
				{columnDefs}
				rowData={filteredData}
				{customButtons}
				getRowId={(params) => params.data.id}
				persistenceKey="users-list-columns"
				onSelectionChanged={(rows) => (selectedRows = rows)}
				onRowDoubleClicked={handleRowDoubleClick}
			/>
		{/if}
	</div>

	<!-- Dropdowns de Filtros -->
	<div class="filter-dropdown-container">
		{#if showRoleDropdown}
			<div
				class="absolute top-16 left-6 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
				transition:fade={{ duration: 150 }}
			>
				{#each [['todos', 'Todos los roles'], ['admin', 'Administradores'], ['supervisor', 'Supervisores'], ['guardia', 'Guardias']] as [value, label]}
					<button
						onclick={() => handleRoleSelect(value as 'todos' | 'admin' | 'supervisor' | 'guardia')}
						class="w-full px-4 py-2 text-left text-sm text-white hover:bg-white/5 transition-colors {roleFilter ===
						value
							? 'bg-blue-500/20 text-blue-400'
							: ''}"
					>
						{label}
					</button>
				{/each}
			</div>
		{/if}

		{#if showEstadoDropdown}
			<div
				class="absolute top-16 left-44 z-50 bg-[#252526] border border-white/10 rounded-lg shadow-2xl py-2 min-w-[200px]"
				transition:fade={{ duration: 150 }}
			>
				{#each [['todos', 'Todos'], ['activo', 'Activos'], ['inactivo', 'Inactivos']] as [value, label]}
					<button
						onclick={() => handleEstadoSelect(value as 'todos' | 'activo' | 'inactivo')}
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
	</div>
</div>

<!-- Modal para Crear/Editar -->
<UserFormModal
	show={showModal}
	user={editingUser}
	readonly={isReadOnlyModal}
	loading={modalLoading}
	onSave={handleSaveUser}
	onClose={closeModal}
/>

<!-- Modal de Confirmación para Auto-Desactivación -->
<ConfirmPasswordModal
	show={showSelfDeactivateModal}
	title="Desactivar Tu Cuenta"
	warningMessage="⚠️ ADVERTENCIA: Estás a punto de desactivar tu propia cuenta. Una vez desactivada, NO podrás iniciar sesión hasta que otro administrador te reactive. ¿Estás seguro de que deseas continuar?"
	confirmButtonText="Sí, Desactivar Mi Cuenta"
	user={$currentUser}
	onConfirm={confirmSelfDeactivation}
	onCancel={cancelSelfDeactivation}
/>
