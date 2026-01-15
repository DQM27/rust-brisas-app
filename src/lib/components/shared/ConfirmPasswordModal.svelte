<script lang="ts">
	import { Lock, TriangleAlert, Shield, Eye, EyeOff } from 'lucide-svelte';
	import { fade, scale } from 'svelte/transition';
	import { currentUser } from '$lib/stores/auth';
	import { auth as authApi } from '$lib/api/auth';
	import type { UserResponse } from '$lib/types/user';

	interface Props {
		show: boolean;
		title?: string;
		warningMessage: string;
		confirmButtonText?: string;
		variant?: 'warning' | 'danger';
		user?: UserResponse | null; // Optional user prop
		onConfirm: () => Promise<void> | void;
		onCancel: () => void;
	}

	let {
		show,
		title = 'Confirmar Acción',
		warningMessage,
		confirmButtonText = 'Confirmar',
		variant = 'warning',
		user = null,
		onConfirm,
		onCancel
	}: Props = $props();

	// Use provided user or fallback to store
	const activeUser = $derived(user || $currentUser);

	let password = $state('');
	let error = $state('');
	let loading = $state(false);
	let showPassword = $state(false);
	let inputRef: HTMLInputElement | null = $state(null);

	// Focus password input when modal opens
	$effect(() => {
		if (show && inputRef) {
			setTimeout(() => inputRef?.focus(), 150);
		}
	});

	// Reset state when modal closes
	$effect(() => {
		if (!show) {
			password = '';
			error = '';
			loading = false;
			showPassword = false;
		}
	});

	async function handleSubmit() {
		if (!activeUser?.email) {
			error = 'Usuario no identificado';
			return;
		}

		if (!password.trim()) {
			error = 'Ingresa tu contraseña';
			return;
		}

		error = '';
		loading = true;

		try {
			// Verify password by attempting login with current user credentials
			await authApi.login(activeUser.email, password);

			// Password correct - execute the confirmed action
			await onConfirm();
			password = '';
		} catch (err: any) {
			console.error('Password verification failed:', err);
			error = 'Contraseña incorrecta';
			password = '';
			inputRef?.focus();
		} finally {
			loading = false;
		}
	}

	function handleCancel() {
		password = '';
		error = '';
		onCancel();
	}

	// Color variants
	const colors = {
		warning: {
			shield: 'text-orange-400',
			shieldBg: 'from-orange-500/20 to-orange-600/5 ring-orange-500/20 shadow-orange-900/20',
			alertBg: 'bg-yellow-900/20 border-yellow-700/30',
			alertIcon: 'text-yellow-500',
			alertText: 'text-yellow-200/80',
			highlight: 'text-yellow-400',
			button: 'bg-orange-700 hover:bg-orange-600 shadow-orange-900/20 text-white',
			glow: 'bg-orange-500/10'
		},
		danger: {
			shield: 'text-red-400',
			shieldBg: 'from-red-500/20 to-red-600/5 ring-red-500/20 shadow-red-900/20',
			alertBg: 'bg-red-900/20 border-red-700/30',
			alertIcon: 'text-red-500',
			alertText: 'text-red-200/80',
			highlight: 'text-red-400',
			button: 'bg-red-700 hover:bg-red-600 shadow-red-900/20 text-white',
			glow: 'bg-red-500/10'
		}
	};

	const c = $derived(colors[variant]);
</script>

{#if show}
	<!-- Backdrop with blur -->
	<div
		class="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
		role="dialog"
		aria-modal="true"
		transition:fade={{ duration: 200 }}
	>
		<!-- Modal Container -->
		<div
			class="w-full max-w-[480px] bg-[#0d1117] rounded-xl shadow-2xl border border-white/10 overflow-hidden relative"
			transition:scale={{ duration: 200, start: 0.95 }}
		>
			<!-- Background Glow Effect -->
			<div
				class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-32 {c.glow} blur-[60px] rounded-full pointer-events-none"
			></div>

			<div class="p-8 relative z-10">
				<!-- Header -->
				<div class="flex items-start gap-4 mb-6">
					<div class="flex-none p-3 rounded-2xl bg-gradient-to-br ring-1 shadow-lg {c.shieldBg}">
						<Shield class="w-8 h-8 {c.shield}" strokeWidth={1.5} />
					</div>
					<div>
						<h2 class="text-xl font-bold text-white leading-tight">{title}</h2>
						<p class="text-gray-400 text-sm mt-1">Acción que requiere verificación</p>
					</div>
				</div>

				<!-- Warning Message Box -->
				<div class="{c.alertBg} border rounded-lg p-4 mb-6 flex items-start gap-3">
					<TriangleAlert class="w-5 h-5 {c.alertIcon} flex-none relative top-0.5" />
					<p class="text-sm {c.alertText} leading-relaxed">
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						{@html warningMessage.replace(
							'ADVERTENCIA:',
							`<span class="font-semibold ${c.highlight}">ADVERTENCIA:</span>`
						)}
					</p>
				</div>

				<!-- User Info Card -->
				{#if activeUser}
					<div class="bg-white/5 border border-white/5 rounded-lg p-4 mb-6 flex items-center gap-4">
						<div
							class="w-10 h-10 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold text-lg"
						>
							{activeUser.nombre?.charAt(0)?.toUpperCase()}
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-sm font-medium text-white truncate">
								{activeUser.nombre}
								{activeUser.apellido || ''}
							</p>
							<p class="text-xs text-gray-500 truncate">
								{activeUser.email}
							</p>
						</div>
					</div>
				{/if}

				<!-- Password Form -->
				<form
					onsubmit={(e) => {
						e.preventDefault();
						handleSubmit();
					}}
					class="space-y-6"
				>
					<!-- Password Input -->
					<div class="space-y-2">
						<label for="confirm-password" class="block text-sm font-medium text-gray-300">
							Ingresa tu contraseña para confirmar
						</label>
						<div class="relative group">
							<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
								<Lock
									class="h-5 w-5 text-gray-500 group-focus-within:text-blue-500 transition-colors"
								/>
							</div>
							<input
								id="confirm-password"
								bind:this={inputRef}
								type={showPassword ? 'text' : 'password'}
								bind:value={password}
								placeholder="••••••••"
								disabled={loading}
								autocomplete="current-password"
								class="block w-full pl-10 pr-10 py-3 bg-black/40 border border-white/10 rounded-lg text-white placeholder-gray-600 focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all text-sm tracking-wide sm:text-base outline-none {error
									? '!border-red-500/50 !ring-red-500/20'
									: ''}"
							/>
							<button
								type="button"
								onclick={() => (showPassword = !showPassword)}
								class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer text-gray-500 hover:text-gray-300 transition-colors"
								tabindex="-1"
							>
								{#if showPassword}
									<EyeOff class="h-5 w-5" />
								{:else}
									<Eye class="h-5 w-5" />
								{/if}
							</button>
						</div>
						{#if error}
							<p
								class="mt-2 text-sm text-red-400 flex items-center gap-1.5"
								transition:fade={{ duration: 150 }}
							>
								<TriangleAlert size={14} />
								{error}
							</p>
						{/if}
					</div>

					<!-- Action Buttons -->
					<div class="grid grid-cols-2 gap-3">
						<button
							type="button"
							onclick={handleCancel}
							disabled={loading}
							class="px-4 py-2.5 rounded-lg border border-white/10 text-gray-300 font-medium hover:bg-white/5 hover:text-white transition-all text-sm disabled:opacity-50"
						>
							Cancelar
						</button>
						<button
							type="submit"
							disabled={loading || !password.trim()}
							class="px-4 py-2.5 rounded-lg {c.button} font-medium transition-all text-sm disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{#if loading}
								<span class="flex items-center justify-center gap-2">
									<svg
										class="h-4 w-4 animate-spin"
										xmlns="http://www.w3.org/2000/svg"
										fill="none"
										viewBox="0 0 24 24"
									>
										<circle
											class="opacity-25"
											cx="12"
											cy="12"
											r="10"
											stroke="currentColor"
											stroke-width="4"
										></circle>
										<path
											class="opacity-75"
											fill="currentColor"
											d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
										></path>
									</svg>
									Verificando...
								</span>
							{:else}
								{confirmButtonText}
							{/if}
						</button>
					</div>

					<!-- Security Note -->
					<div class="flex items-center justify-center gap-2 text-xs text-gray-600 mt-2">
						<Lock class="w-3 h-3" />
						Tu contraseña se verifica localmente y no se almacena
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}
