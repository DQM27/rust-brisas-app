<!-- src/lib/components/layout/sidebar/Sidebar.svelte -->
<script lang="ts">
	import { activeView } from '$lib/stores/ui';
	import { logout, currentUser } from '$lib/stores/auth';
	import { openTab } from '$lib/stores/tabs';
	import { modulesStore } from '$lib/stores/modules';
	import { onMount } from 'svelte';

	// Icons
	import {
		User,
		FileText,
		Package,
		Calendar,
		BadgeCheck,
		LogIn,
		Ban,
		Users,
		Zap,
		Truck
	} from 'lucide-svelte';

	// Components
	import SidebarIcon from './SidebarIcon.svelte';
	import SidebarPanel from './SidebarPanel.svelte';
	import ProfileMenu from './ProfileMenu.svelte';
	import SettingsMenu from './SettingsMenu.svelte';

	// Store & types
	import { activePanel, openView } from '$lib/stores/sidebar';
	import type { SidebarItem } from '../../../types/Sidebar';
	import { can } from '$lib/logic/permissions';
	import { ROLE_ADMIN_ID } from '$lib/types/role';

	// Modals
	import UserFormModal from '$lib/components/user/UserFormModal.svelte';
	import UpdateModal from '$lib/components/settings/modals/UpdateModal.svelte';
	import AboutModal from '$lib/components/settings/modals/AboutModal.svelte';
	import ProfileModal from '$lib/components/user/ProfileModal.svelte';
	import ConfirmModal from '$lib/components/shared/ConfirmModal.svelte';

	// Services
	import * as userService from '$lib/logic/user/userService';
	import type { CreateUserInput, UpdateUserInput } from '$lib/types/user';
	import { toast } from 'svelte-5-french-toast';
	import { reindexGlobalSearch } from '$lib/api/searchService';

	// Sidebar items configuration
	const allSidebarItems: SidebarItem[] = [
		{
			id: 'users',
			icon: User,
			label: 'Usuarios',
			action: () => {
				openTab({
					componentKey: 'user-list',
					title: 'Lista de Usuarios',
					id: 'users-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_USER_DETAIL'
		},
		{
			id: 'proveedores',
			icon: Package,
			label: 'Proveedores',
			action: () => {
				openTab({
					componentKey: 'proveedor-list',
					title: 'Lista de Proveedores',
					id: 'proveedores-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_PROVIDER_DETAIL'
		},
		{
			id: 'visitantes',
			icon: Users,
			label: 'Visitantes',
			action: () => {
				openTab({
					componentKey: 'visitante-list',
					title: 'Lista de Visitantes',
					id: 'visitante-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_VISITOR_LIST'
		},
		{
			id: 'blacklist',
			icon: Ban,
			label: 'Lista Negra',
			action: () => {
				openTab({
					componentKey: 'lista-negra-list',
					title: 'Lista Negra',
					id: 'lista-negra-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_BLACKLIST'
		},
		{
			id: 'citas',
			icon: Calendar,
			label: 'Visitas',
			action: () => {
				openTab({
					componentKey: 'citas-view',
					title: 'Pre-registro Visitas',
					id: 'citas-view',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_APPOINTMENT_LIST'
		},
		{
			id: 'gafetes',
			icon: BadgeCheck,
			label: 'Gafetes',
			action: () => {
				openTab({
					componentKey: 'gafete-list',
					title: 'Gestión de Gafetes',
					id: 'gafete-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_GAFETE_LIST'
		},
		{
			id: 'ingresos',
			icon: LogIn,
			label: 'Ingresos',
			action: () => {
				openTab({
					componentKey: 'ingreso-list',
					title: 'Control de Ingresos',
					id: 'ingreso-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_ENTRY_LIST'
		},
		{
			id: 'ingreso-proveedores',
			icon: Truck,
			label: 'Ingreso Prov.',
			action: () => {
				openTab({
					componentKey: 'proveedor-ingreso-list',
					title: 'Ingreso Proveedores',
					id: 'proveedor-ingreso-list',
					focusOnOpen: true
				});
			},
			permission: 'VIEW_ENTRY_LIST'
		},
		{
			id: 'logs',
			icon: FileText,
			label: 'Logs',
			roleId: [ROLE_ADMIN_ID]
		}
	];

	// Menu state
	let showSettingsMenu = $state(false);
	let showUpdateModal = $state(false);
	let showAboutModal = $state(false);
	let showProfileMenu = $state(false);

	function handleWindowClick() {
		if (showSettingsMenu) showSettingsMenu = false;
		if (showProfileMenu) showProfileMenu = false;
	}

	// Filter items by permissions
	const sidebarItems = $derived(
		allSidebarItems.filter((item) => {
			if (!item) return false;
			// @ts-ignore
			if (item.permission && !can($currentUser, item.permission)) return false;
			// @ts-ignore
			if (item.roleId && $currentUser && !item.roleId.includes($currentUser.roleId)) return false;
			return true;
		})
	);

	const currentActivePanel = $derived($activePanel);
	const activeItem = $derived(sidebarItems.find((item) => item.id === currentActivePanel));

	// Module status mapping
	const MODULE_KEY_MAP: Record<string, string> = {
		users: 'users',
		contractors: 'contractors',
		proveedores: 'providers',
		visitantes: 'visits',
		blacklist: 'access_control',
		citas: 'visits',
		gafetes: 'access_control',
		ingresos: 'access_control',
		logs: 'reports'
	};

	function handleItemSelect(item: SidebarItem) {
		const moduleKey = MODULE_KEY_MAP[item.id] || item.id;
		const status = modulesStore.getStatus(moduleKey, $modulesStore);

		if ((status === 'development' || status === 'maintenance') && !$currentUser?.isSuperuser) {
			openTab({
				componentKey: 'under-construction',
				title: item.label,
				id: `locked-${item.id}`,
				data: { type: status, moduleName: item.label },
				focusOnOpen: true
			});
			return;
		}

		activeView.set(item.id);

		if (item.action) {
			item.action();
			activePanel.set(null);
		} else if (item.panelComponent) {
			activePanel.set($activePanel === item.id ? null : item.id);
		}
	}

	function handlePanelClose() {
		activePanel.set(null);
	}

	// User derived values
	const userInitials = $derived(
		$currentUser
			? `${$currentUser.nombre?.[0] || ''}${$currentUser.apellido?.[0] || ''}`.toUpperCase()
			: ''
	);

	const userName = $derived(
		$currentUser
			? `${$currentUser.nombre || ''} ${$currentUser.apellido || ''}`.trim() || 'Usuario'
			: 'Usuario'
	);

	// Avatar
	let avatarUrl = $state<string | null>(null);

	async function loadUserAvatar(userId: string) {
		const result = await userService.getUserAvatar(userId);
		avatarUrl = result.ok ? `data:image/webp;base64,${result.data}` : null;
	}

	$effect(() => {
		if ($currentUser) {
			loadUserAvatar($currentUser.id);
		} else {
			avatarUrl = null;
		}
	});

	onMount(async () => {
		if ($currentUser) {
			try {
				const res = await userService.fetchUserById($currentUser.id);
				if (res.ok) {
					currentUser.set(res.data);
					loadUserAvatar(res.data.id);
				}
			} catch (e) {
				console.error('Error refreshing session:', e);
			}
		}
	});

	// Profile Modal Logic
	let showProfileModal = $state(false);
	let showProfileViewModal = $state(false);
	let profileLoading = $state(false);

	function openProfile() {
		showProfileViewModal = true;
	}

	function handleEditProfile() {
		showProfileViewModal = false;
		showProfileModal = true;
	}

	async function handleSaveProfile(data: CreateUserInput | UpdateUserInput): Promise<boolean> {
		if (!$currentUser) return false;

		profileLoading = true;
		try {
			const result = await userService.updateUser($currentUser.id, data as UpdateUserInput);

			if (result.ok) {
				toast.success('Perfil actualizado correctamente');
				return true;
			} else {
				toast.error(result.error);
				return false;
			}
		} catch (err) {
			console.error(err);
			toast.error('Error al guardar perfil');
			return false;
		} finally {
			profileLoading = false;
		}
	}

	// Reindex Logic
	let showReindexConfirm = $state(false);
	let reindexLoading = $state(false);

	function requestReindex() {
		showReindexConfirm = true;
	}

	async function confirmReindex() {
		reindexLoading = true;
		const toastId = toast.loading('Reindexando base de datos...');

		try {
			await reindexGlobalSearch();
			toast.success('Reindexado completado correctamente', { id: toastId });
			showReindexConfirm = false;
		} catch (e: any) {
			console.error(e);
			toast.error(e.message || 'Error al reindexar', { id: toastId });
		} finally {
			reindexLoading = false;
		}
	}
</script>

<svelte:window onclick={handleWindowClick} />

<div class="flex h-full">
	<!-- Sidebar Icons -->
	<div class="sidebar-icons">
		<div class="flex flex-col gap-1.5">
			{#each sidebarItems as item}
				<SidebarIcon {item} isActive={currentActivePanel === item.id} onSelect={handleItemSelect} />
			{/each}
		</div>

		<!-- GOD MODE Icon -->
		{#if $currentUser?.isSuperuser}
			<div class="mt-auto mb-2 flex justify-center">
				<button
					class="sidebar-icon-btn group text-yellow-500 hover:text-yellow-400"
					title="Modo Ingeniería (GOD)"
					onclick={() => openView('dev-settings', 'Modo Ingeniería')}
				>
					<Zap
						size={24}
						strokeWidth={2.5}
						class="transition-all duration-300 group-hover:scale-110 group-hover:drop-shadow-[0_0_8px_rgba(250,204,21,0.5)]"
					/>
					<span class="sidebar-icon-tooltip">Ingeniería</span>
				</button>
			</div>
		{/if}

		<div class="sidebar-bottom-actions">
			<!-- Profile Menu -->
			<ProfileMenu
				bind:show={showProfileMenu}
				{userName}
				{userInitials}
				{avatarUrl}
				currentUser={$currentUser}
				onOpenProfile={openProfile}
				onLogout={logout}
				onClose={() => (showProfileMenu = false)}
			/>

			<!-- Settings Menu -->
			<SettingsMenu
				bind:show={showSettingsMenu}
				currentUser={$currentUser}
				onOpenUpdate={() => (showUpdateModal = true)}
				onOpenAbout={() => (showAboutModal = true)}
				onRequestReindex={requestReindex}
				onClose={() => (showSettingsMenu = false)}
			/>
		</div>
	</div>

	<!-- Side Panel -->
	{#if activeItem}
		<SidebarPanel item={activeItem} onClose={handlePanelClose} />
	{/if}

	<!-- Profile Modals -->
	{#if $currentUser}
		<ProfileModal
			show={showProfileViewModal}
			user={$currentUser}
			onClose={() => (showProfileViewModal = false)}
			onEdit={handleEditProfile}
		/>
		<UserFormModal
			show={showProfileModal}
			user={$currentUser}
			loading={profileLoading}
			onSave={handleSaveProfile}
			onClose={() => (showProfileModal = false)}
			isSelfEdit={true}
		/>
	{/if}

	<!-- Reindex Confirm Modal -->
	<ConfirmModal
		show={showReindexConfirm}
		title="Reindexar Búsqueda"
		message="Esta acción reconstruirá completamente el índice de búsqueda. Puede tomar varios segundos dependiendo de la cantidad de datos. ¿Deseas continuar?"
		confirmText="Sí, Reindexar"
		type="warning"
		loading={reindexLoading}
		onConfirm={confirmReindex}
		onClose={() => (showReindexConfirm = false)}
	/>

	<!-- System Modals -->
	<UpdateModal show={showUpdateModal} onClose={() => (showUpdateModal = false)} />
	<AboutModal show={showAboutModal} onClose={() => (showAboutModal = false)} />
</div>
