<!-- src/lib/components/layout/sidebar/ProfileMenu.svelte -->
<script lang="ts">
	import type { UserResponse } from '$lib/types/user';

	interface Props {
		show: boolean;
		userName: string;
		userInitials: string;
		avatarUrl: string | null;
		currentUser: UserResponse | null;
		onOpenProfile: () => void;
		onLogout: () => void;
		onClose: () => void;
	}

	let {
		show = $bindable(),
		userName,
		userInitials,
		avatarUrl,
		currentUser,
		onOpenProfile,
		onLogout,
		onClose
	}: Props = $props();

	function handleAvatarClick(e: MouseEvent) {
		e.stopPropagation();
		show = !show;
	}
</script>

<div class="relative w-full flex items-center justify-center">
	<button
		class="user-avatar overflow-hidden flex items-center justify-center p-0"
		title={userName}
		onclick={handleAvatarClick}
	>
		{#if avatarUrl}
			<img src={avatarUrl} alt="Avatar" class="w-full h-full object-cover" />
		{:else}
			{userInitials}
		{/if}
	</button>

	{#if show}
		<div
			class="profile-menu"
			onclick={(e) => e.stopPropagation()}
			role="menu"
			tabindex="-1"
			onkeydown={(e) => e.key === 'Escape' && onClose()}
		>
			<!-- User Info Header -->
			<div class="px-3 py-2 border-b border-[#454545] mb-1">
				<p class="text-xs font-semibold text-white">{userName}</p>
				<p class="text-[10px] text-gray-400">{currentUser?.email}</p>
				{#if currentUser?.roleName}
					{#if currentUser.isSuperuser}
						<div
							class="mt-1 inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-purple-500/30 text-purple-300 uppercase tracking-wide"
						>
							⚡ GOD MODE
						</div>
					{:else}
						<div
							class="mt-1 inline-flex items-center px-1.5 py-0.5 rounded text-[10px] font-medium bg-[#2da44e]/20 text-[#2da44e] uppercase tracking-wide"
						>
							{currentUser.roleName}
						</div>
					{/if}
				{/if}
			</div>

			<button
				class="profile-menu-item"
				onclick={() => {
					onOpenProfile();
					show = false;
				}}
			>
				Ver Perfil
			</button>

			<div class="profile-menu-separator"></div>

			<button
				class="profile-menu-item text-red-400 hover:text-red-300"
				onclick={() => {
					onLogout();
					show = false;
				}}
			>
				Cerrar Sesión
			</button>
		</div>
	{/if}
</div>

<style>
	.profile-menu {
		position: absolute;
		bottom: 48px;
		left: 52px;
		z-index: 2000;
		min-width: 220px;
		padding: 4px 0;
		background: #1e1e1e;
		border: 1px solid #454545;
		border-radius: 4px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.profile-menu-item {
		display: block;
		width: calc(100% - 8px);
		margin: 2px 4px;
		padding: 6px 12px;
		text-align: left;
		font-size: 13px;
		color: #cccccc;
		background: transparent;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-family: 'Segoe UI', system-ui, sans-serif;
		transition: all 0.15s ease;
	}

	.profile-menu-item:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.profile-menu-separator {
		height: 1px;
		background-color: #454545;
		margin: 4px 0;
	}
</style>
