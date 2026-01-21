<!-- src/lib/components/gafete/modals/BulkCreateGafeteModal.svelte -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X, Plus, ChevronDown, Check } from 'lucide-svelte';
	import type { CreateGafeteRangeInput } from '$lib/types/gafete';

	interface Props {
		show: boolean;
		loading?: boolean;
		onSave: (data: CreateGafeteRangeInput) => Promise<void> | void;
		onClose: () => void;
	}

	let { show, loading = false, onSave, onClose }: Props = $props();

	let start = $state(1);
	let end = $state(50);
	let tipo = $state<'contratista' | 'proveedor' | 'visita' | 'otro'>('contratista');
	let showTipoDropdown = $state(false);

	// Referencias y posición del dropdown
	let triggerButton = $state<HTMLButtonElement>();
	let dropdownTop = $state(0);
	let dropdownLeft = $state(0);
	let dropdownWidth = $state(0);

	// Calcular vista previa
	const previewStart = $derived(start.toString().padStart(2, '0'));
	const previewEnd = $derived(end.toString().padStart(2, '0'));
	const totalCount = $derived(Math.max(0, end - start + 1));

	// Opciones de tipo
	const tipoOptions = [
		{ value: 'contratista', label: 'Contratista' },
		{ value: 'proveedor', label: 'Proveedor' },
		{ value: 'visita', label: 'Visita' },
		{ value: 'otro', label: 'Otro' }
	] as const;

	const tipoLabel = $derived(
		tipoOptions.find((opt) => opt.value === tipo)?.label ?? 'Seleccionar...'
	);

	// Función para toggle del dropdown y calcular posición
	function handleTipoDropdownToggle() {
		showTipoDropdown = !showTipoDropdown;
		if (showTipoDropdown && triggerButton) {
			const rect = triggerButton.getBoundingClientRect();
			dropdownTop = rect.bottom + 4;
			dropdownLeft = rect.left;
			dropdownWidth = rect.width;
		}
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		if (start > end) {
			return alert('El inicio debe ser menor o igual al fin');
		}

		await onSave({
			start,
			end,
			prefix: '', // Sin prefijo
			padding: 2, // Padding fijo de 2 dígitos
			tipo
		});
	}

	// Clases estándar según ui-patterns.md
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		transition:fade={{ duration: 150 }}
	>
		<div class="absolute inset-0" onclick={onClose} role="presentation"></div>

		<div
			class="relative z-10 w-full max-w-[450px] max-h-[95vh] overflow-hidden bg-surface-2 shadow-2xl border border-surface rounded-xl flex flex-col"
			transition:fly={{ y: 20, duration: 200 }}
		>
			<!-- Header -->
			<div
				class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
			>
				<h2 class="text-xl font-semibold text-primary flex items-center gap-2">
					<Plus size={20} class="text-accent" />
					Generar Gafetes en Lote
				</h2>
				<button
					onclick={onClose}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Content -->
			<form onsubmit={handleSubmit} class="flex-1 p-6 space-y-4 overflow-y-auto">
				<!-- Card de Inputs -->
				<div class="bg-surface-1 rounded-lg border border-surface p-6 space-y-4">
					<!-- Rango -->
					<div class="grid grid-cols-2 gap-4">
						<div>
							<label for="start" class={labelClass}>Inicio</label>
							<input
								type="number"
								id="start"
								bind:value={start}
								min="1"
								disabled={loading}
								class={inputClass}
							/>
						</div>
						<div>
							<label for="end" class={labelClass}>Fin</label>
							<input
								type="number"
								id="end"
								bind:value={end}
								min="1"
								disabled={loading}
								class={inputClass}
							/>
						</div>
					</div>

					<!-- Tipo (Custom Dropdown) -->
					<div class="relative">
						<label class={labelClass} for="tipo-gafete-bulk">Tipo de Gafete</label>

						<!-- Trigger -->
						<button
							id="tipo-gafete-bulk"
							type="button"
							bind:this={triggerButton}
							onclick={handleTipoDropdownToggle}
							disabled={loading}
							class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
							class:!border-blue-500={showTipoDropdown}
						>
							<span class="truncate">{tipoLabel}</span>
							<ChevronDown size={16} class="text-secondary" />
						</button>

						{#if showTipoDropdown}
							<!-- Backdrop -->
							<div
								class="fixed inset-0 z-[60]"
								onclick={() => (showTipoDropdown = false)}
								role="presentation"
							></div>

							<!-- Menú Fixed -->
							<div
								style="top: {dropdownTop}px; left: {dropdownLeft}px; width: {dropdownWidth}px;"
								class="fixed z-[70] bg-[#1c2128] border border-white/10 rounded-lg shadow-xl max-h-[200px] overflow-y-auto p-1"
								transition:fly={{ y: -5, duration: 200 }}
							>
								{#each tipoOptions as option}
									<button
										type="button"
										onclick={() => {
											tipo = option.value;
											showTipoDropdown = false;
										}}
										class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
									>
										<span>{option.label}</span>
										{#if tipo === option.value}
											<Check size={14} class="text-white" />
										{/if}
									</button>
								{/each}
							</div>
						{/if}
					</div>
				</div>

				<!-- Preview -->
				<div class="p-4 bg-blue-900/10 rounded-lg border border-blue-900/30 text-sm space-y-2">
					<p class="text-blue-400 font-medium text-xs">Resumen de generación:</p>
					<div class="flex justify-between items-end">
						<div>
							<p class="text-gray-500 text-xs">Rango:</p>
							<p class="font-mono text-gray-200 text-sm">
								{previewStart} → {previewEnd}
							</p>
						</div>
						<div class="text-right">
							<p class="text-gray-500 text-xs">Total:</p>
							<p class="font-bold text-blue-400 text-base">
								{totalCount} gafetes
							</p>
						</div>
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
					onclick={onClose}
					disabled={loading}
					class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm disabled:opacity-50"
				>
					Cancelar
				</button>

				<!-- Generar -->
				<button
					type="submit"
					disabled={loading || totalCount <= 0}
					onclick={handleSubmit}
					class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
				>
					{#if loading}
						<svg
							class="animate-spin h-4 w-4"
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
						Generando...
					{:else}
						Generar Gafetes
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
