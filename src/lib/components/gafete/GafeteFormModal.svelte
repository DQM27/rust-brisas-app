<!-- src/lib/components/gafete/GafeteFormModal.svelte -->
<script lang="ts">
	import { fade, fly } from 'svelte/transition';
	import { X, ChevronDown, Check } from 'lucide-svelte';
	import { shortcutRegistry } from '$lib/shortcuts';
	import type { GafeteResponse, CreateGafeteInput, UpdateGafeteInput } from '$lib/types/gafete';
	import { gafeteSchema } from '$lib/schemas/gafeteSchema';

	interface Props {
		show: boolean;
		gafete?: GafeteResponse | null;
		loading?: boolean;
		onSave: (data: CreateGafeteInput | UpdateGafeteInput) => Promise<void> | void;
		onClose: () => void;
	}

	let { show, gafete = null, loading = false, onSave, onClose }: Props = $props();

	// Modo derivado
	const isEditMode = $derived(!!gafete);
	const modalTitle = $derived(isEditMode ? 'Editar Gafete' : 'Nuevo Gafete');

	// Estado del formulario
	let numero = $state('');
	let tipo = $state<'contratista' | 'proveedor' | 'visita' | 'otro'>('contratista');
	let errors = $state<Record<string, string>>({});
	let showTipoDropdown = $state(false);

	// Referencias y posición del dropdown
	let triggerButton = $state<HTMLButtonElement>();
	let dropdownTop = $state(0);
	let dropdownLeft = $state(0);
	let dropdownWidth = $state(0);

	// Opciones de tipo
	const tipoOptions = [
		{ value: 'contratista', label: 'Contratista' },
		{ value: 'proveedor', label: 'Proveedor' },
		{ value: 'visita', label: 'Visita' },
		{ value: 'otro', label: 'Otro' }
	] as const;

	// Label derivado para el dropdown
	const tipoLabel = $derived(
		tipoOptions.find((opt) => opt.value === tipo)?.label ?? 'Seleccionar...'
	);

	// Función para toggle del dropdown y calcular posición
	function handleTipoDropdownToggle() {
		showTipoDropdown = !showTipoDropdown;
		if (showTipoDropdown && triggerButton) {
			const rect = triggerButton.getBoundingClientRect();
			dropdownTop = rect.bottom + 4; // 4px de margen
			dropdownLeft = rect.left;
			dropdownWidth = rect.width;
		}
	}

	// Activar scope modal
	$effect(() => {
		if (show) {
			shortcutRegistry.pushScope('modal');
			return () => shortcutRegistry.popScope();
		}
	});

	// Cargar datos iniciales
	$effect(() => {
		if (show && gafete) {
			numero = String(gafete.numero);
			tipo = gafete.tipo;
			errors = {};
		} else if (show) {
			numero = '';
			tipo = 'contratista';
			errors = {};
		}
	});

	async function handleSubmit(event: Event) {
		event.preventDefault();
		errors = {};

		// Validar con Zod
		const result = gafeteSchema.safeParse({ numero: numero.trim(), tipo });

		if (!result.success) {
			// Convertir errores de Zod a nuestro formato
			result.error.issues.forEach((err) => {
				if (err.path[0]) {
					errors[err.path[0] as string] = err.message;
				}
			});
			return;
		}

		try {
			await onSave(result.data);
		} catch (err: any) {
			errors.form = err.message || 'Error al guardar';
		}
	}

	// Función helper para clases de validación
	function getFieldStateClass(field: string) {
		if (errors[field]) {
			return 'is-error';
		}
		return '';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!show || loading) return;
		if (e.key === 'Escape' && !showTipoDropdown) {
			onClose();
		}
		if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
			e.preventDefault();
			if (numero.trim()) {
				handleSubmit(e);
			}
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

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
				<h2 class="text-xl font-semibold text-primary">
					{modalTitle}
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
					<!-- Número de Gafete -->
					<div>
						<label for="numero" class="form-label">
							Número de Gafete <span class="text-error">*</span>
						</label>
						<input
							type="number"
							id="numero"
							name="numero"
							bind:value={numero}
							disabled={isEditMode || loading}
							class="form-input {getFieldStateClass('numero')}"
							placeholder="Ej: 101"
							min="1"
							step="1"
						/>
						{#if isEditMode}
							<p class="mt-1 text-xs text-tertiary">
								El número no se puede cambiar una vez creado.
							</p>
						{/if}
						{#if errors.numero}
							<p class="form-error">{errors.numero}</p>
						{/if}
					</div>

					<!-- Tipo de Gafete (Custom Dropdown) -->
					<div class="relative">
						<label class="form-label" for="tipo-gafete">
							Tipo <span class="text-error">*</span>
						</label>

						<!-- Trigger -->
						<button
							id="tipo-gafete"
							type="button"
							bind:this={triggerButton}
							onclick={handleTipoDropdownToggle}
							disabled={loading}
							class="form-select flex items-center justify-between cursor-pointer w-full text-left {getFieldStateClass(
								'tipo'
							)}"
							class:border-accent={showTipoDropdown}
							class:ring-1={showTipoDropdown}
							class:ring-accent-subtle={showTipoDropdown}
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
								class="fixed z-[70] form-dropdown p-1"
								transition:fly={{ y: -5, duration: 200 }}
							>
								{#each tipoOptions as option}
									<button
										type="button"
										onclick={() => {
											tipo = option.value;
											showTipoDropdown = false;
										}}
										class="form-dropdown-item flex items-center justify-between group"
									>
										<span>{option.label}</span>
										{#if tipo === option.value}
											<Check size={14} class="text-primary" />
										{/if}
									</button>
								{/each}
							</div>
						{/if}

						{#if errors.tipo}
							<p class="form-error">{errors.tipo}</p>
						{/if}
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
					class="form-btn-outline-secondary"
				>
					Cancelar
				</button>

				<!-- Guardar -->
				<button
					type="submit"
					disabled={loading || !numero.trim()}
					onclick={handleSubmit}
					class="form-btn-outline-success inline-flex items-center gap-2"
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
						Guardando...
					{:else}
						{isEditMode ? 'Actualizar' : 'Crear Gafete'}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Autofill Fix handled globally in theme.css or reset.css, removing duplicate here */
</style>
