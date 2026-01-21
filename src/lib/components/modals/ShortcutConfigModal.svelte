<script lang="ts">
	import { onMount } from 'svelte';
	import { X, Keyboard, RotateCcw, Check, AlertTriangle, Search } from 'lucide-svelte';
	import { slide, fade, fly } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';
	import { toast } from 'svelte-5-french-toast';

	import {
		ALL_SHORTCUTS,
		getOrderedCategories,
		shortcutRegistry,
		type ShortcutDefinition,
		type CategoryMetadata
	} from '$lib/shortcuts';
	import {
		getUserShortcuts,
		saveUserShortcut,
		deleteUserShortcut,
		resetUserShortcuts
	} from '$lib/shortcuts/userShortcutsService';

	interface Props {
		isOpen: boolean;
	}

	let { isOpen }: Props = $props();
	const dispatch = createEventDispatcher();

	// Estado
	let loading = $state(true);
	let saving = $state(false);
	let searchTerm = $state('');
	let editingShortcut = $state<string | null>(null);
	let recordingKeys = $state(false);
	let recordedKeys = $state('');
	let customizations = $state<Map<string, { keys: string; enabled: boolean }>>(new Map());
	let hasChanges = $state(false);

	// Clases de UI siguiendo ui-patterns.md
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';

	// Categorías y shortcuts filtrados
	const categories = $derived(getOrderedCategories());
	const filteredShortcuts = $derived(() => {
		let shortcuts = ALL_SHORTCUTS;
		if (searchTerm.trim()) {
			const term = searchTerm.toLowerCase();
			shortcuts = shortcuts.filter(
				(s) =>
					s.label.toLowerCase().includes(term) ||
					s.description?.toLowerCase().includes(term) ||
					s.keys.toLowerCase().includes(term)
			);
		}
		return shortcuts;
	});

	const categoriesWithShortcuts = $derived(() => {
		return categories
			.map((cat) => ({
				...cat,
				items: filteredShortcuts().filter((s) => s.category === cat.id)
			}))
			.filter((cat) => cat.items.length > 0);
	});

	function close() {
		if (hasChanges) {
			// TODO: Confirmar antes de cerrar
		}
		editingShortcut = null;
		recordingKeys = false;
		dispatch('close');
	}

	async function loadCustomizations() {
		loading = true;
		try {
			const userShortcuts = await getUserShortcuts();
			customizations = new Map(
				userShortcuts.map((s) => [s.shortcutId, { keys: s.customKeys, enabled: s.enabled }])
			);
		} catch (e) {
			console.error('Error loading customizations:', e);
		}
		loading = false;
	}

	function getDisplayKeys(shortcut: ShortcutDefinition): string {
		const custom = customizations.get(shortcut.id);
		return custom?.keys || shortcut.keys;
	}

	function isCustomized(shortcutId: string): boolean {
		return customizations.has(shortcutId);
	}

	function startEditing(shortcutId: string) {
		if (ALL_SHORTCUTS.find((s) => s.id === shortcutId)?.readonly) {
			toast.error('Este atajo no se puede personalizar');
			return;
		}
		editingShortcut = shortcutId;
		recordedKeys = '';
		recordingKeys = true;
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (!recordingKeys) return;

		e.preventDefault();
		e.stopPropagation();

		// Ignorar solo modificadores
		if (['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) {
			return;
		}

		// Construir combinación de teclas
		const parts: string[] = [];
		if (e.ctrlKey) parts.push('ctrl');
		if (e.altKey) parts.push('alt');
		if (e.shiftKey) parts.push('shift');

		// Normalizar key
		let key = e.key.toLowerCase();
		if (key === 'escape') key = 'escape';
		else if (key === 'delete') key = 'delete';
		else if (key === 'backspace') key = 'backspace';
		else if (key === 'enter') key = 'enter';
		else if (key === ' ') key = 'space';
		else if (e.code.startsWith('Key')) key = e.code.replace('Key', '').toLowerCase();
		else if (e.code.startsWith('Digit')) key = e.code.replace('Digit', '');
		else if (e.code.startsWith('F') && !isNaN(parseInt(e.code.slice(1))))
			key = e.code.toLowerCase();

		parts.push(key);
		recordedKeys = parts.join('+');
	}

	async function saveShortcut() {
		if (!editingShortcut || !recordedKeys) return;

		// Verificar colisión
		const collision = shortcutRegistry.checkCollision(
			recordedKeys,
			ALL_SHORTCUTS.find((s) => s.id === editingShortcut)?.scope || 'all'
		);

		if (collision.hasCollision && collision.conflictingShortcut?.id !== editingShortcut) {
			toast.error(`"${recordedKeys}" ya está asignado a "${collision.conflictingShortcut?.label}"`);
			return;
		}

		saving = true;
		const success = await saveUserShortcut(editingShortcut, recordedKeys, true);

		if (success) {
			customizations.set(editingShortcut, { keys: recordedKeys, enabled: true });
			customizations = new Map(customizations);
			hasChanges = true;
			toast.success('Atajo guardado');

			// Rebind en el registro
			shortcutRegistry.rebind(editingShortcut, recordedKeys);
		} else {
			toast.error('Error al guardar');
		}

		saving = false;
		editingShortcut = null;
		recordingKeys = false;
	}

	async function resetShortcut(shortcutId: string) {
		const original = ALL_SHORTCUTS.find((s) => s.id === shortcutId);
		if (!original) return;

		const success = await deleteUserShortcut(shortcutId);
		if (success) {
			customizations.delete(shortcutId);
			customizations = new Map(customizations);
			hasChanges = true;
			toast.success('Atajo restaurado');

			// Rebind al original
			shortcutRegistry.rebind(shortcutId, original.keys);
		}
	}

	async function resetAll() {
		if (!confirm('¿Restaurar todos los atajos a sus valores por defecto?')) return;

		saving = true;
		const success = await resetUserShortcuts();

		if (success) {
			customizations.clear();
			customizations = new Map(customizations);
			hasChanges = true;
			toast.success('Todos los atajos restaurados');

			// Re-inicializar registry
			shortcutRegistry.init();
		}
		saving = false;
	}

	function cancelEdit() {
		editingShortcut = null;
		recordingKeys = false;
		recordedKeys = '';
	}

	function formatKeysDisplay(keys: string): string[] {
		return keys
			.replace('ctrl', 'Ctrl')
			.replace('shift', 'Shift')
			.replace('alt', 'Alt')
			.replace('escape', 'Esc')
			.replace('delete', 'Del')
			.split('+');
	}

	onMount(() => {
		if (isOpen) loadCustomizations();
	});

	$effect(() => {
		if (isOpen) loadCustomizations();
	});
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if isOpen}
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		role="presentation"
		transition:fade={{ duration: 200 }}
		onclick={(e) => e.target === e.currentTarget && close()}
	>
		<div
			class="w-full max-w-[700px] max-h-[95vh] overflow-hidden flex flex-col bg-surface-2 shadow-2xl border border-surface rounded-xl"
			transition:fly={{ y: -20, duration: 250 }}
		>
			<!-- Header (siguiendo ui-patterns.md) -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-blue-500/10 rounded-lg">
						<Keyboard class="w-5 h-5 text-blue-400" />
					</div>
					<div>
						<h2 class="text-xl font-semibold text-primary">Configurar Atajos</h2>
						<p class="text-xs text-secondary">Personaliza tus atajos de teclado</p>
					</div>
				</div>
				<button
					onclick={close}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Search Bar -->
			<div class="px-6 py-3 border-b border-surface bg-surface-1">
				<div class="relative">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500" />
					<input
						type="text"
						placeholder="Buscar atajos..."
						bind:value={searchTerm}
						class="{inputClass} pl-9"
					/>
				</div>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-y-auto p-6">
				{#if loading}
					<div class="flex items-center justify-center h-32">
						<div class="loading loading-spinner loading-lg text-primary opacity-20"></div>
					</div>
				{:else}
					<div class="space-y-6">
						{#each categoriesWithShortcuts() as category}
							<div>
								<h3
									class="text-sm font-semibold text-secondary uppercase tracking-wider mb-3 flex items-center gap-2"
								>
									{category.label}
									<span class="text-xs font-normal text-gray-500">({category.items.length})</span>
								</h3>

								<div class="bg-surface-1 rounded-lg border border-surface divide-y divide-surface">
									{#each category.items as shortcut}
										{@const isEditing = editingShortcut === shortcut.id}
										{@const isCustom = isCustomized(shortcut.id)}

										<div
											class="flex items-center justify-between px-4 py-3 group hover:bg-white/[0.02] transition-colors"
										>
											<div class="flex-1 min-w-0">
												<div class="flex items-center gap-2">
													<span class="text-sm font-medium text-primary">
														{shortcut.label}
													</span>
													{#if shortcut.readonly}
														<span
															class="text-[10px] px-1.5 py-0.5 bg-gray-700/50 text-gray-400 rounded"
														>
															Sistema
														</span>
													{/if}
													{#if isCustom}
														<span
															class="text-[10px] px-1.5 py-0.5 bg-blue-500/20 text-blue-400 rounded"
														>
															Personalizado
														</span>
													{/if}
												</div>
												{#if shortcut.description}
													<p class="text-xs text-secondary truncate mt-0.5">
														{shortcut.description}
													</p>
												{/if}
											</div>

											<div class="flex items-center gap-2">
												{#if isEditing}
													<!-- Modo edición -->
													<div
														class="flex items-center gap-2"
														transition:slide={{ axis: 'x', duration: 150 }}
													>
														<div
															class="px-3 py-1.5 min-w-[100px] text-center bg-blue-500/10 border border-blue-500/30 rounded-lg text-sm text-blue-400 animate-pulse"
														>
															{recordedKeys || 'Presiona teclas...'}
														</div>
														<button
															onclick={saveShortcut}
															disabled={!recordedKeys || saving}
															class="p-1.5 rounded-lg bg-success/10 text-success hover:bg-success/20 disabled:opacity-50 transition-colors"
														>
															<Check size={16} />
														</button>
														<button
															onclick={cancelEdit}
															class="p-1.5 rounded-lg bg-error/10 text-error hover:bg-error/20 transition-colors"
														>
															<X size={16} />
														</button>
													</div>
												{:else}
													<!-- Modo normal -->
													<div class="flex items-center gap-1">
														{#each formatKeysDisplay(getDisplayKeys(shortcut)) as key}
															<kbd
																class="px-2 py-1 min-w-[1.5rem] text-center text-xs font-semibold text-gray-300 bg-gray-800 border border-gray-700 rounded-lg shadow-sm"
															>
																{key}
															</kbd>
														{/each}
													</div>

													{#if !shortcut.readonly}
														<div
															class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
														>
															<button
																onclick={() => startEditing(shortcut.id)}
																class="p-1 text-secondary hover:text-primary transition-colors"
																title="Editar atajo"
															>
																<Keyboard size={14} />
															</button>
															{#if isCustom}
																<button
																	onclick={() => resetShortcut(shortcut.id)}
																	class="p-1 text-secondary hover:text-warning transition-colors"
																	title="Restaurar default"
																>
																	<RotateCcw size={14} />
																</button>
															{/if}
														</div>
													{/if}
												{/if}
											</div>
										</div>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer (siguiendo ui-patterns.md) -->
			<div
				class="flex-none flex items-center justify-between gap-3 px-6 py-4 border-t border-surface bg-surface-1"
			>
				<button
					onclick={resetAll}
					disabled={saving || customizations.size === 0}
					class="px-4 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-warning/50 hover:text-warning text-sm disabled:opacity-50"
				>
					<span class="flex items-center gap-2">
						<RotateCcw size={14} />
						Restaurar Todo
					</span>
				</button>

				<div class="flex items-center gap-3">
					{#if hasChanges}
						<span class="text-xs text-amber-400 flex items-center gap-1">
							<AlertTriangle size={12} />
							Cambios aplicados
						</span>
					{/if}
					<button
						onclick={close}
						class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
					>
						Cerrar
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Autofill Fix (siguiendo ui-patterns.md) */
	input:-webkit-autofill {
		-webkit-text-fill-color: white !important;
		-webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
		transition: background-color 5000s ease-in-out 0s;
	}
</style>
