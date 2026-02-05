<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { onMount } from 'svelte';
	import {
		Search,
		User,
		ShieldCheck,
		Lock,
		X,
		ArrowLeft,
		Loader2,
		UserRoundPen
	} from 'lucide-svelte';
	import { showQuickSwitch, quickSwitchTarget } from '$lib/stores/ui';
	import { invoke } from '@tauri-apps/api/core';

	import { auth } from '$lib/api/auth';
	import { login, logout, currentUser } from '$lib/stores/auth';
	import { toastService } from '$lib/services/toastService';

	// Types
	interface UserSearchResult {
		id: string;
		tipo: string;
		nombreCompleto: string;
		email: string;
		cedula?: string;
		roleName?: string;
	}

	// State
	let query = $state('');
	let results = $state<UserSearchResult[]>([]);
	let selectedUser = $state<UserSearchResult | null>(null);
	let password = $state('');
	let loading = $state(false);
	let searching = $state(false);
	let error = $state('');
	let inputRef = $state<HTMLInputElement>();
	let passwordRef = $state<HTMLInputElement>();
	let highlightedIndex = $state(0);

	// Derived
	let show = $derived($showQuickSwitch);

	// Effects
	$effect(() => {
		if (show && !selectedUser) {
			reset();
			setTimeout(() => inputRef?.focus(), 50);
		}
	});

	$effect(() => {
		const target = $quickSwitchTarget;
		if (target) {
			selectedUser = target;
			showQuickSwitch.set(true);
			quickSwitchTarget.set(null); // Consumir el target
		}
	});

	$effect(() => {
		if (selectedUser) {
			setTimeout(() => passwordRef?.focus(), 50);
		}
	});

	// Search logic
	$effect(() => {
		const q = query.trim();
		if (q.length >= 2 && !selectedUser) {
			searching = true;
			performSearch(q);
		} else if (!selectedUser) {
			results = [];
			searching = false;
		}
	});

	async function performSearch(q: string) {
		try {
			console.log('[QuickSwitch] Searching for:', q);
			const rawResults = await invoke<any[]>('search_global', {
				query: q,
				limit: 20
			});
			console.log('[QuickSwitch] Total results:', rawResults.length);

			// Filter users (case insensitive and allow 'usuario' or 'user')
			results = rawResults.filter((r) => {
				const type = String(r.tipo || '').toLowerCase();
				return type === 'user' || type === 'usuario';
			});

			console.log('[QuickSwitch] Filtered users:', results.length);
			if (results.length > 0) {
				console.log('[QuickSwitch] First user:', results[0].nombreCompleto, results[0].email);
			}

			highlightedIndex = 0;
		} catch (e) {
			console.error('Error searching users:', e);
		} finally {
			searching = false;
		}
	}

	function handleClose() {
		showQuickSwitch.set(false);
		setTimeout(() => reset(), 200);
	}

	function reset() {
		query = '';
		results = [];
		selectedUser = null;
		password = '';
		error = '';
		loading = false;
		highlightedIndex = 0;
	}

	function selectUser(user: UserSearchResult) {
		selectedUser = user;
		error = '';
	}

	function goBack() {
		selectedUser = null;
		password = '';
		error = '';
		setTimeout(() => inputRef?.focus(), 50);
	}

	async function handleSwitch() {
		if (!selectedUser || !password.trim()) return;

		loading = true;
		error = '';

		try {
			await logout('Logout (Quick Switch)');
			const userResponse = await auth.login(selectedUser.email, password);
			login(userResponse, 'Quick Switch Login');
			toastService.success(`Sesión cambiada a ${userResponse.nombre}`);
			handleClose();
		} catch (err: any) {
			console.error('Switch user failed:', err);
			error = 'Contraseña incorrecta';
			password = '';
		} finally {
			loading = false;
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (selectedUser) {
				goBack();
			} else {
				handleClose();
			}
		} else if (!selectedUser) {
			// Search navigation
			if (e.key === 'ArrowDown') {
				e.preventDefault();
				highlightedIndex = Math.min(highlightedIndex + 1, results.length - 1);
			} else if (e.key === 'ArrowUp') {
				e.preventDefault();
				highlightedIndex = Math.max(highlightedIndex - 1, 0);
			} else if (e.key === 'Enter' && results.length > 0) {
				e.preventDefault();
				selectUser(results[highlightedIndex]);
			}
		}
	}
</script>

{#if show}
	<div
		class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[1000] flex items-start justify-center pt-[30vh] p-4"
		transition:fade={{ duration: 150 }}
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		onkeydown={handleKeyDown}
		role="button"
		tabindex="-1"
	>
		<div
			class="bg-surface-2 border border-surface w-full max-w-lg rounded-xl shadow-2xl overflow-hidden flex flex-col"
			transition:scale={{ duration: 200, start: 0.95 }}
			onclick={(e) => e.stopPropagation()}
			role="presentation"
		>
			{#if !selectedUser}
				<!-- Search Input Section (Spotlight Minimalist) -->
				<div
					class="px-4 py-3 bg-surface-2"
					class:border-b={query.trim().length > 0}
					class:border-surface={query.trim().length > 0}
				>
					<div
						class="relative flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-indigo-500/50 transition-all"
					>
						<UserRoundPen class="absolute left-3 text-gray-500" size={16} />
						<input
							bind:this={inputRef}
							bind:value={query}
							type="text"
							placeholder="Buscar usuario..."
							class="w-full bg-transparent pl-10 pr-12 py-3 text-[16px] text-white focus:outline-none placeholder:text-gray-600"
							autocomplete="off"
						/>
						<div class="absolute right-3 flex items-center gap-2">
							{#if searching}
								<Loader2 size={16} class="text-indigo-500 animate-spin" />
							{/if}
							<kbd
								class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[9px] text-gray-500 font-medium"
								>ESC</kbd
							>
						</div>
					</div>
				</div>
				{#if query.trim().length > 0}
					<div
						class="max-h-[40vh] overflow-y-auto p-2 bg-surface-2/50"
						transition:fade={{ duration: 100 }}
					>
						{#if results.length > 0}
							<div class="space-y-1">
								{#each results as user, i}
									<button
										class="w-full text-left px-3 py-2 rounded-lg flex items-center gap-3 transition-colors relative
                                        {i === highlightedIndex
											? 'bg-indigo-600 text-white shadow-lg'
											: 'hover:bg-white/5 text-gray-400'}"
										onclick={() => selectUser(user)}
										onmouseenter={() => (highlightedIndex = i)}
									>
										<div class="flex-shrink-0">
											<ShieldCheck
												size={16}
												class={i === highlightedIndex ? 'text-white' : 'text-indigo-400'}
											/>
										</div>
										<div class="flex-1 min-w-0">
											<div class="font-medium text-[14px] truncate flex items-center gap-2">
												<span class="truncate">{user.nombreCompleto || user.id}</span>
												{#if i === highlightedIndex}
													<span
														class="text-[8.5px] px-1 py-0 rounded border leading-none font-bold uppercase tracking-tight bg-white/20 text-white border-white/30"
													>
														SELECCIONAR
													</span>
												{/if}
											</div>
											<div
												class="text-[11px] truncate {i === highlightedIndex
													? 'text-white/70'
													: 'text-gray-500'}"
											>
												{user.email || 'Sin email'}
												{user.cedula ? ` • ${user.cedula}` : ''}
											</div>
										</div>
									</button>
								{/each}
							</div>
						{:else if query.length >= 2 && !searching}
							<div class="py-12 text-center text-gray-500">
								<User size={40} class="mx-auto mb-3 opacity-10" />
								<p class="text-sm">No se encontraron usuarios activos con "{query}"</p>
							</div>
						{:else if query.length < 2}
							<div class="py-14 text-center text-gray-500">
								<div class="relative inline-block mb-4">
									<Search size={48} class="text-gray-600/20" />
								</div>
								<p class="text-sm font-medium text-gray-400">
									Escribe al menos 2 caracteres para buscar...
								</p>
								<p class="text-[10px] mt-2 text-gray-600 font-bold uppercase tracking-[0.2em]">
									SESIÓN ACTUAL: {$currentUser?.nombre}
								</p>
							</div>
						{/if}
					</div>
				{/if}
			{:else}
				<!-- Password View -->
				<div class="p-6">
					<div class="flex items-center gap-3 mb-6">
						<button
							onclick={goBack}
							class="p-2 -ml-2 hover:bg-white/5 rounded-full transition-colors text-gray-400 hover:text-white"
						>
							<ArrowLeft size={20} />
						</button>
						<h2 class="text-xl font-bold text-white">Validar Identidad</h2>
					</div>

					<div
						class="mb-8 p-4 rounded-xl bg-blue-500/5 border border-blue-500/20 flex items-center gap-4"
					>
						<div
							class="w-12 h-12 rounded-full bg-blue-500/10 flex items-center justify-center text-blue-400"
						>
							<User size={24} />
						</div>
						<div>
							<div class="font-bold text-white text-lg">
								{selectedUser.nombreCompleto || selectedUser.id}
							</div>
							<div class="text-sm text-gray-400">{selectedUser.email || 'Sin email'}</div>
						</div>
					</div>

					<form
						onsubmit={(e) => {
							e.preventDefault();
							handleSwitch();
						}}
						class="space-y-4"
					>
						<div class="space-y-2">
							<label
								for="sw-password"
								class="text-xs font-bold text-gray-500 uppercase tracking-widest"
							>
								Contraseña de {(selectedUser.nombreCompleto || selectedUser.id).split(' ')[0]}
							</label>

							<div class="relative flex items-center">
								<Lock class="absolute left-3 text-gray-500" size={18} />
								<input
									id="sw-password"
									bind:this={passwordRef}
									bind:value={password}
									type="password"
									placeholder="••••••••"
									disabled={loading}
									class="w-full bg-black/20 border border-white/10 rounded-xl pl-10 pr-4 py-3 text-white focus:outline-none focus:border-blue-500/50 transition-all
                                        {error ? 'border-red-500/50 ring-2 ring-red-500/10' : ''}"
								/>
							</div>
							{#if error}
								<p class="text-xs text-red-400 font-medium" transition:fade>{error}</p>
							{/if}
						</div>

						<div class="flex gap-3 pt-4">
							<button
								type="button"
								onclick={goBack}
								disabled={loading}
								class="flex-1 px-4 py-2.5 rounded-lg border-2 border-white/10 text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm disabled:opacity-50"
							>
								Cambiar Usuario
							</button>
							<button
								type="submit"
								disabled={loading || !password.trim()}
								class="flex-[1.5] px-6 py-2.5 rounded-lg border-2 border-white/10 text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 flex items-center justify-center gap-2"
							>
								{#if loading}
									<Loader2 size={18} class="animate-spin" />
									Iniciando...
								{:else}
									Confirmar Cambio
								{/if}
							</button>
						</div>
					</form>
				</div>
			{/if}

			{#if selectedUser || query.trim().length > 0}
				<!-- Footer -->
				<div
					class="px-4 py-3 bg-surface-1 border-t border-surface flex justify-between items-center"
					transition:fade={{ duration: 100 }}
				>
					<div class="text-[10px] text-gray-600 font-bold uppercase tracking-widest">
						Ctrl + Shift + U
					</div>
					<div class="flex gap-4">
						<div class="flex items-center gap-1.5">
							<kbd
								class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[9px] text-gray-500"
								>ESC</kbd
							>
							<span class="text-[10px] text-gray-600 font-medium">atrás</span>
						</div>
						<div class="flex items-center gap-1.5">
							<kbd
								class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[9px] text-gray-500"
								>↵</kbd
							>
							<span class="text-[10px] text-gray-600 font-medium">confirmar</span>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	:global(body:has(.quick-user-switch-modal)) {
		overflow: hidden;
	}

	input::placeholder {
		color: #4b5563;
	}
</style>
