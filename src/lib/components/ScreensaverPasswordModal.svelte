<script lang="ts">
	import { Lock } from 'lucide-svelte';
	import { preventDefault } from 'svelte/legacy';
	import { currentUser } from '$lib/stores/auth';
	import {
		exitScreensaver,
		cancelScreensaverPassword,
		sessionMode
	} from '$lib/stores/sessionStore';
	import { auth as authApi } from '$lib/api/auth';

	let password = $state('');
	let error = $state('');
	let loading = $state(false);
	let inputRef: HTMLInputElement | null = $state(null);

	$effect(() => {
		if (inputRef) {
			inputRef.focus();
		}
	});

	async function handleSubmit() {
		if (!$currentUser?.email) {
			error = 'Usuario no identificado';
			return;
		}

		error = '';
		loading = true;

		try {
			// Verify password by attempting login with current user credentials
			await authApi.login($currentUser.email, password);

			// Password correct - exit screensaver
			exitScreensaver();
			password = '';
		} catch (err: unknown) {
			console.error('Screensaver password verification failed:', err);
			error = 'Contraseña incorrecta';
			password = '';
		} finally {
			loading = false;
		}
	}

	async function handleCancel() {
		// Cancel screensaver and perform full logout (exits fullscreen first)
		await cancelScreensaverPassword();
	}
</script>

<div
	class="fixed inset-0 z-[10000] flex items-center justify-center bg-black/60 backdrop-blur-md animate-fade-in"
	role="dialog"
	aria-modal="true"
	aria-labelledby="screensaver-password-title"
>
	<div
		class="w-full max-w-md mx-4 rounded-lg bg-surface-2 p-8 shadow-2xl border border-emphasis animate-scale-in"
	>
		<!-- Header -->
		<div class="flex items-center gap-3 mb-6">
			<div class="flex items-center justify-center w-12 h-12 rounded-full bg-accent/10">
				<Lock class="text-accent" size={24} />
			</div>
			<div>
				<h2 id="screensaver-password-title" class="text-xl font-bold text-primary">
					{$sessionMode === 'locked' ? 'Bloqueo por Inactividad' : 'Protector de Pantalla'}
				</h2>
				<p class="text-sm text-tertiary">
					{$sessionMode === 'locked'
						? 'La aplicación se ha bloqueado por seguridad'
						: 'Ingresa tu contraseña para continuar'}
				</p>
			</div>
		</div>

		<!-- User Info -->
		{#if $currentUser}
			<div class="mb-6 p-3 rounded-lg bg-surface-1 border border-emphasis">
				<div class="text-sm text-secondary">Usuario</div>
				<div class="font-medium text-primary">{$currentUser.nombre}</div>
				<div class="text-xs text-tertiary">{$currentUser.email}</div>
			</div>
		{/if}

		<!-- Password Form -->
		<form onsubmit={preventDefault(handleSubmit)} class="flex flex-col gap-4">
			<!-- Password Input -->
			<div class="flex flex-col gap-1.5">
				<label for="screensaver-password" class="text-sm font-medium text-secondary">
					Contraseña
				</label>
				<input
					id="screensaver-password"
					bind:this={inputRef}
					type="password"
					bind:value={password}
					placeholder="••••••••"
					disabled={loading}
					class="w-full rounded border bg-surface-1 px-3 py-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent transition-all {error
						? 'border-red-500 focus:border-red-500 focus:ring-red-500/20'
						: 'border-emphasis focus:border-accent'}"
				/>
				{#if error}
					<span class="text-xs text-red-500 animate-fade-in">{error}</span>
				{/if}
			</div>

			<!-- Actions -->
			<div class="flex gap-3 mt-2">
				<button
					type="button"
					onclick={handleCancel}
					disabled={loading}
					class="flex-1 py-2.5 px-4 rounded-lg border border-surface-tertiary text-secondary font-medium hover:bg-surface-3 transition-colors text-sm disabled:opacity-50"
				>
					Cerrar Sesión
				</button>
				<button
					type="submit"
					disabled={loading || !password.trim()}
					class="flex-1 rounded-lg bg-accent px-4 py-2.5 font-medium text-white transition-all hover:bg-accent-hover hover:shadow-lg hover:shadow-accent/20 disabled:cursor-not-allowed disabled:opacity-60 active:scale-[0.98]"
				>
					{#if loading}
						<span class="flex items-center justify-center gap-2">
							<svg
								class="h-4 w-4 animate-spin text-white"
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
							...
						</span>
					{:else}
						Continuar
					{/if}
				</button>
			</div>
		</form>

		<!-- Hint -->
		<div class="mt-4 text-center">
			<p class="text-xs text-tertiary">💡 O cierra sesión completamente si has terminado</p>
		</div>
	</div>
</div>

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes scale-in {
		from {
			transform: scale(0.95);
			opacity: 0;
		}
		to {
			transform: scale(1);
			opacity: 1;
		}
	}

	.animate-fade-in {
		animation: fade-in 0.2s ease-out;
	}

	.animate-scale-in {
		animation: scale-in 0.2s ease-out;
	}
</style>
