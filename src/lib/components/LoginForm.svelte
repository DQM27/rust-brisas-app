<script lang="ts">
	import { preventDefault } from 'svelte/legacy';
	import { onMount } from 'svelte';
	import { exitApp } from '$lib/services/keyringService';
	import { loginStore } from '$lib/stores/loginStore.svelte';
	import { validateLoginForm } from '$lib/logic/auth/loginValidation';

	interface Props {
		loading?: boolean;
		onSubmit: (data: { email: string; password: string }) => void;
	}

	let { loading = false, onSubmit }: Props = $props();

	let email = $state('');
	let password = $state('');
	let errors = $state<Record<string, string>>({});
	let rememberMe = $state(false);
	let rememberPassword = $state(false);

	let emailInput = $state<HTMLInputElement>();
	let passwordInput = $state<HTMLInputElement>();
	let mounted = $state(false);

	onMount(() => {
		// Restaurar hidratación explícita para asegurar carga inicial
		if (loginStore.hasRememberedEmail) {
			email = loginStore.rememberedEmail;
			rememberMe = true;

			// Si hay contraseña recordada, cargarla también
			if (loginStore.hasRememberedPassword) {
				password = loginStore.rememberedPassword;
				rememberPassword = true;
			}

			setTimeout(() => passwordInput?.focus(), 100);
		} else {
			setTimeout(() => emailInput?.focus(), 100);
		}

		// Pequeño delay para marcar como montado y evitar que el effect limpie todo al inicio
		setTimeout(() => {
			mounted = true;
		}, 200);
	});

	// Limpieza: Solo si el usuario explícitamente desmarca rememberMe DESPUÉS de montar
	$effect(() => {
		if (mounted && !rememberMe && email) {
			// Solo limpiamos si había algo escrito, para evitar loops raros
			loginStore.clearRememberedEmail();
		}
	});

	// Sincronización Reactiva: Si el store cambia (ej. carga asíncrona tardía), actualizamos UI
	$effect(() => {
		// 1. Sync Email (si llega tarde)
		if (loginStore.rememberedEmail && !email) {
			email = loginStore.rememberedEmail;
			rememberMe = true;
		}

		// 2. Sync Password (si llega tarde)
		if (loginStore.rememberedPassword && !password) {
			password = loginStore.rememberedPassword;
			rememberPassword = true;
		}
	});

	function handleRememberPasswordChange() {
		if (!rememberPassword) {
			loginStore.clearRememberedPassword();
			password = '';
		}
	}

	async function handleSubmit() {
		const result = validateLoginForm(email, password);

		if (!result.valid) {
			errors = result.errors;
			return;
		}

		errors = {};

		if (rememberMe) {
			await loginStore.setRememberedEmail(email);
			if (rememberPassword) {
				await loginStore.setRememberedPassword(password);
			} else {
				await loginStore.clearRememberedPassword();
			}
		} else {
			await loginStore.clearRememberedEmail();
		}

		onSubmit({ email, password });
	}

	export async function reset() {
		await loginStore.reload();
		email = loginStore.rememberedEmail;
		password = '';
		errors = {};
		rememberMe = loginStore.hasRememberedEmail;

		// Reset focus logic
		if (rememberMe) {
			setTimeout(() => passwordInput?.focus(), 100);
		} else {
			setTimeout(() => emailInput?.focus(), 100);
		}
	}
</script>

<div class="w-full flex flex-col justify-center pt-1 pb-14 px-8 bg-[#1e1e1e] rounded-xl shadow-2xl">
	<form onsubmit={preventDefault(handleSubmit)} class="flex flex-col gap-4">
		<div class="text-center flex flex-col items-center mb-5">
			<img
				src="/icono-brisas.png"
				alt="Logo"
				class="w-20 h-20 active:scale-95 transition-transform mb-2"
			/>
			<h1 class="text-3xl font-bold text-primary">MegaBrisas</h1>
		</div>

		<!-- Email -->
		<div class="flex flex-col gap-1.5">
			<label for="email" class="text-sm font-medium text-secondary"> Correo Electrónico </label>
			<div
				class="input-container bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all {errors.email
					? 'border-red-500'
					: ''}"
			>
				<input
					id="email"
					bind:this={emailInput}
					type="email"
					bind:value={email}
					placeholder="ejemplo@correo.com"
					disabled={loading}
					class="w-full bg-transparent px-3 py-2.5 text-white placeholder:text-gray-500 focus:outline-none outline-none border-none appearance-none ring-0"
				/>
			</div>
			{#if errors.email}
				<span class="text-xs text-red-500 animate-fade-in">{errors.email}</span>
			{/if}
		</div>

		<!-- Password -->
		<div class="flex flex-col gap-1.5">
			<div class="flex justify-between items-center">
				<label for="password" class="text-sm font-medium text-secondary"> Contraseña </label>
			</div>
			<div
				class="input-container bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all {errors.password
					? 'border-red-500'
					: ''}"
			>
				<input
					id="password"
					bind:this={passwordInput}
					type="password"
					bind:value={password}
					placeholder="••••••••"
					disabled={loading}
					class="w-full bg-transparent px-3 py-2.5 text-white placeholder:text-gray-500 focus:outline-none outline-none border-none appearance-none ring-0"
				/>
			</div>
			{#if errors.password}
				<span class="text-xs text-red-500 animate-fade-in">{errors.password}</span>
			{/if}
		</div>

		<!-- Options -->
		<div class="flex flex-row gap-6 items-center justify-center">
			<label
				class="flex items-center gap-2 cursor-pointer text-sm text-secondary hover:text-primary transition-colors select-none"
			>
				<input
					type="checkbox"
					bind:checked={rememberMe}
					disabled={loading}
					class="rounded border-surface-tertiary text-accent focus:ring-accent w-4 h-4 cursor-pointer"
				/>
				Recordar usuario
			</label>

			{#if rememberMe}
				<label
					class="flex items-center gap-2 cursor-pointer text-sm text-secondary hover:text-primary transition-colors select-none animate-fade-in"
				>
					<input
						type="checkbox"
						bind:checked={rememberPassword}
						onchange={handleRememberPasswordChange}
						disabled={loading}
						class="rounded border-surface-tertiary text-accent focus:ring-accent w-4 h-4 cursor-pointer"
					/>
					Recordar contraseña
				</label>
			{/if}
		</div>

		<!-- Acciones -->
		<div class="flex gap-3 mt-4">
			<button
				type="button"
				onclick={exitApp}
				class="flex-1 flex items-center justify-center py-2.5 px-4 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 disabled:opacity-50"
			>
				Cancelar
			</button>
			<button
				type="submit"
				disabled={loading}
				class="flex-1 flex items-center justify-center py-2.5 px-4 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success disabled:opacity-50"
			>
				{#if loading}
					<span class="flex items-center justify-center gap-2">
						<svg
							class="h-4 w-4 animate-spin text-current"
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
					Ingresar
				{/if}
			</button>
		</div>
	</form>
</div>

<style>
	/* Input container - mismo estilo que GafeteInput */
	.input-container,
	.input-container *:focus {
		outline: none !important;
		box-shadow: none !important;
	}

	.input-container:focus-within {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
	}
</style>
