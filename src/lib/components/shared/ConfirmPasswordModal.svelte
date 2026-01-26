<script lang="ts">
	import { Lock, TriangleAlert, Shield, Eye, EyeOff, X } from 'lucide-svelte';
	import { fade, fly } from 'svelte/transition';
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
		} catch (err: unknown) {
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

	// Clases estándar según ui-patterns.md
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg pl-10 pr-10 py-2 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';

	// Colores según variante (reactivos)
	const iconColor = $derived(variant === 'warning' ? 'text-orange-400' : 'text-red-400');
	const iconBgColor = $derived(
		variant === 'warning'
			? 'bg-orange-500/10 border-orange-500/20'
			: 'bg-red-500/10 border-red-500/20'
	);
	const alertBgColor = $derived(
		variant === 'warning'
			? 'bg-yellow-900/10 border-yellow-700/30'
			: 'bg-red-900/10 border-red-700/30'
	);
	const alertTextColor = $derived(variant === 'warning' ? 'text-yellow-200/90' : 'text-red-200/90');
	const highlightColor = $derived(variant === 'warning' ? 'text-yellow-400' : 'text-red-400');
</script>

{#if show}
	<div
		class="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop -->
		<div class="absolute inset-0" onclick={handleCancel} role="presentation"></div>

		<!-- Modal -->
		<div
			class="relative z-10 w-full max-w-[450px] max-h-[95vh] overflow-hidden bg-surface-2 shadow-2xl border border-surface rounded-xl flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div>
					<h2 class="text-lg font-semibold text-primary">{title}</h2>
					<p class="text-xs text-secondary">Acción que requiere verificación</p>
				</div>
				<button
					onclick={handleCancel}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Content -->
			<form
				onsubmit={(e) => {
					e.preventDefault();
					handleSubmit();
				}}
				class="flex-1 p-6 space-y-4 overflow-y-auto"
			>
				<!-- Warning Alert Card -->
				<div class="{alertBgColor} border rounded-lg p-4 flex gap-3">
					<TriangleAlert class="w-5 h-5 {iconColor} flex-none mt-0.5" />
					<div class="text-sm {alertTextColor} leading-relaxed">
						{@html warningMessage.replace(
							'ADVERTENCIA:',
							`<span class="font-semibold ${highlightColor}">ADVERTENCIA:</span>`
						)}
					</div>
				</div>

				<!-- User Card -->
				{#if activeUser}
					<div class="bg-surface-1 border border-surface rounded-lg p-4 flex items-center gap-3">
						<div
							class="w-10 h-10 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold text-base flex-none"
						>
							{activeUser.nombre?.charAt(0)?.toUpperCase()}
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-sm font-medium text-primary truncate">
								{activeUser.nombre}
								{activeUser.apellido || ''}
							</p>
							<p class="text-xs text-secondary truncate">
								{activeUser.email}
							</p>
						</div>
					</div>
				{/if}

				<!-- Password Input Card -->
				<div class="bg-surface-1 rounded-lg border border-surface p-4 space-y-3">
					<label for="confirm-password" class={labelClass}>
						Ingresa tu contraseña para confirmar <span class="text-red-500">*</span>
					</label>

					<div class="relative group">
						<!-- Lock Icon -->
						<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
							<Lock
								class="h-4 w-4 text-gray-500 group-focus-within:text-blue-400 transition-colors"
							/>
						</div>

						<!-- Input -->
						<input
							id="confirm-password"
							bind:this={inputRef}
							type={showPassword ? 'text' : 'password'}
							bind:value={password}
							placeholder="••••••••"
							disabled={loading}
							autocomplete="current-password"
							class="{inputClass} {error ? 'border-red-500 ring-1 ring-red-500/20' : ''}"
						/>

						<!-- Toggle Password Visibility -->
						<button
							type="button"
							onclick={() => (showPassword = !showPassword)}
							class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer text-gray-500 hover:text-gray-300 transition-colors"
							tabindex="-1"
						>
							{#if showPassword}
								<EyeOff class="h-4 w-4" />
							{:else}
								<Eye class="h-4 w-4" />
							{/if}
						</button>
					</div>

					<!-- Error Message -->
					{#if error}
						<p
							class="mt-2 text-sm text-red-400 flex items-center gap-1.5"
							transition:fade={{ duration: 150 }}
						>
							<TriangleAlert size={14} />
							{error}
						</p>
					{/if}

					<!-- Security Note -->
					<div class="flex items-center gap-2 text-xs text-secondary">
						<Lock class="w-3 h-3" />
						Tu contraseña se verifica localmente y no se almacena
					</div>
				</div>
			</form>

			<!-- Footer -->
			<div
				class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<!-- Cancelar -->
				<button
					type="button"
					onclick={handleCancel}
					disabled={loading}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm disabled:opacity-50"
				>
					Cancelar
				</button>

				<!-- Confirmar -->
				<button
					type="submit"
					disabled={loading || !password.trim()}
					onclick={handleSubmit}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 {variant ===
					'warning'
						? 'hover:border-orange-500 hover:text-orange-400'
						: 'hover:border-red-500 hover:text-red-400'} text-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
				>
					{#if loading}
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
					{:else}
						{confirmButtonText}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Autofill Fix (Evita fondo blanco de Chrome) */
	input:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}

	/* Focus Override Global */
	input:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}
</style>
