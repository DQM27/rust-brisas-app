<script lang="ts">
	import { Shield, TriangleAlert, Lock, Eye, EyeOff, X } from 'lucide-svelte';
	import { fade, fly } from 'svelte/transition';
	import { currentUser } from '$lib/stores/auth';

	interface Props {
		isOpen: boolean;
		onConfirm: (password: string) => void;
		onCancel: () => void;
	}

	let { isOpen, onConfirm, onCancel }: Props = $props();

	let password = $state('');
	let showPassword = $state(false);
	let inputRef = $state<HTMLInputElement>();

	$effect(() => {
		if (isOpen && inputRef) {
			inputRef.focus();
		}
	});

	function handleSubmit() {
		if (password) {
			onConfirm(password);
			password = ''; // Reset
		}
	}

	function handleCancel() {
		password = '';
		onCancel();
	}

	// Clases estándar según ui-patterns.md
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg pl-10 pr-10 py-2 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<!-- Backdrop -->
		<div class="absolute inset-0" onclick={handleCancel} role="presentation"></div>

		<!-- Modal -->
		<div
			class="relative z-10 w-full max-w-[500px] max-h-[95vh] overflow-hidden bg-surface-2 shadow-2xl border border-surface rounded-xl flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 rounded-lg bg-orange-500/10 border border-orange-500/20">
						<Shield class="w-5 h-5 text-orange-400" />
					</div>
					<div>
						<h2 class="text-lg font-semibold text-primary">Desactivar Tu Cuenta</h2>
						<p class="text-xs text-secondary">Acción que requiere verificación</p>
					</div>
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
				<div class="bg-yellow-900/10 border border-yellow-700/30 rounded-lg p-4 flex gap-3">
					<TriangleAlert class="w-5 h-5 text-yellow-500 flex-none mt-0.5" />
					<div class="text-sm text-yellow-200/90 leading-relaxed">
						<span class="font-semibold text-yellow-400">ADVERTENCIA:</span> Estás a punto de desactivar
						tu propia cuenta. Una vez desactivada, NO podrás iniciar sesión hasta que otro administrador
						te reactive.
					</div>
				</div>

				<!-- User Card -->
				{#if $currentUser}
					<div class="bg-surface-1 border border-surface rounded-lg p-4 flex items-center gap-3">
						<div
							class="w-10 h-10 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold text-base flex-none"
						>
							{$currentUser.nombre[0].toUpperCase()}
						</div>
						<div class="flex-1 min-w-0">
							<p class="text-sm font-medium text-primary truncate">
								{$currentUser.nombre}
								{$currentUser.apellido}
							</p>
							<p class="text-xs text-secondary truncate">
								{$currentUser.email}
							</p>
						</div>
					</div>
				{/if}

				<!-- Password Input Card -->
				<div class="bg-surface-1 rounded-lg border border-surface p-4 space-y-3">
					<label for="admin-pass" class={labelClass}>
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
							bind:this={inputRef}
							id="admin-pass"
							type={showPassword ? 'text' : 'password'}
							bind:value={password}
							class={inputClass}
							placeholder="••••••••"
						/>

						<!-- Toggle Password Visibility -->
						<button
							type="button"
							class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer text-gray-500 hover:text-gray-300 transition-colors"
							onclick={() => (showPassword = !showPassword)}
						>
							{#if showPassword}
								<EyeOff class="h-4 w-4" />
							{:else}
								<Eye class="h-4 w-4" />
							{/if}
						</button>
					</div>

					<!-- Security Note -->
					<div class="flex items-center gap-2 text-xs text-secondary mt-2">
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
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
				>
					Cancelar
				</button>

				<!-- Confirmar -->
				<button
					type="submit"
					disabled={!password}
					onclick={handleSubmit}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-red-500 hover:text-red-400 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
				>
					Sí, Desactivar Mi Cuenta
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



