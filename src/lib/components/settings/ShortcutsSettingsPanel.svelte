<!-- src/lib/components/settings/ShortcutsSettingsPanel.svelte -->
<script lang="ts">
	import {
		Keyboard,
		Search,
		RotateCcw,
		Check,
		X,
		AlertTriangle,
		Settings2,
		Command,
		Globe,
		Layout,
		Grid3x3,
		Box,
		LogIn
	} from 'lucide-svelte';
	import { fade, slide } from 'svelte/transition';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-5-french-toast';

	import {
		ALL_SHORTCUTS,
		getOrderedCategories,
		shortcutRegistry,
		type ShortcutDefinition,
		type ShortcutCategory
	} from '$lib/shortcuts';
	import {
		getUserShortcuts,
		saveUserShortcut,
		deleteUserShortcut,
		resetUserShortcuts
	} from '$lib/shortcuts/userShortcutsService';

	// Estado
	let loading = $state(true);
	let saving = $state(false);
	let searchTerm = $state('');
	let editingShortcut = $state<string | null>(null);
	let recordingKeys = $state(false);
	let recordedKeys = $state('');
	let customizations = $state<Map<string, { keys: string; enabled: boolean }>>(new Map());
	let successMessage = $state('');
	let error = $state('');

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

	// Iconos por categoría
	const categoryIcons: Record<ShortcutCategory, any> = {
		system: Globe,
		spotlight: Command,
		modules: Layout,
		modals: Box,
		grids: Grid3x3,
		ingresos: LogIn
	};

	function showSuccess(msg: string) {
		successMessage = msg;
		setTimeout(() => (successMessage = ''), 3000);
	}

	async function loadCustomizations() {
		loading = true;
		error = '';
		try {
			const userShortcuts = await getUserShortcuts();
			customizations = new Map(
				userShortcuts.map((s) => [s.shortcutId, { keys: s.customKeys, enabled: s.enabled }])
			);
		} catch (e) {
			error = `Error cargando personalizaciones: ${e}`;
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
			error = `"${recordedKeys}" ya está asignado a "${collision.conflictingShortcut?.label}"`;
			return;
		}

		saving = true;
		error = '';

		const success = await saveUserShortcut(editingShortcut, recordedKeys, true);

		if (success) {
			customizations.set(editingShortcut, { keys: recordedKeys, enabled: true });
			customizations = new Map(customizations);
			showSuccess('Atajo guardado correctamente');

			// Rebind en el registro
			shortcutRegistry.rebind(editingShortcut, recordedKeys);
		} else {
			error = 'Error al guardar el atajo';
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
			showSuccess('Atajo restaurado');

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
			showSuccess('Todos los atajos restaurados');

			// Re-inicializar registry
			shortcutRegistry.init();
		}
		saving = false;
	}

	function cancelEdit() {
		editingShortcut = null;
		recordingKeys = false;
		recordedKeys = '';
		error = '';
	}

	function formatKeysDisplay(keys: string): string[] {
		return keys
			.replace(/ctrl/gi, 'Ctrl')
			.replace(/shift/gi, 'Shift')
			.replace(/alt/gi, 'Alt')
			.replace(/escape/gi, 'Esc')
			.replace(/delete/gi, 'Del')
			.split('+');
	}

	onMount(() => {
		loadCustomizations();
	});
</script>

<svelte:window on:keydown={handleKeyDown} />

<div class="h-full flex flex-col bg-surface-1 p-6 relative overflow-hidden" in:fade>
	<div class="flex flex-col h-full">
		<!-- Header -->
		<div class="mb-8 flex flex-col md:flex-row md:items-center justify-between gap-4">
			<div>
				<h2 class="text-2xl font-bold text-white flex items-center gap-3">
					<Keyboard class="text-primary-400" />
					Atajos de Teclado
				</h2>
				<p class="text-gray-400 mt-1">Personaliza las combinaciones de teclas del sistema.</p>
			</div>

			<div class="flex items-center gap-3">
				<!-- Search -->
				<div class="relative">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500" />
					<input
						type="text"
						bind:value={searchTerm}
						placeholder="Buscar atajo..."
						class="bg-surface-2 border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-white focus:outline-none focus:border-primary-500 transition-all w-64"
					/>
				</div>

				<!-- Reset All -->
				<button
					onclick={resetAll}
					disabled={saving || customizations.size === 0}
					class="px-4 py-2 bg-amber-500/10 hover:bg-amber-500/20 text-amber-400 rounded-lg font-medium transition-all flex items-center gap-2 border border-amber-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					<RotateCcw size={18} />
					Restaurar Todo
				</button>
			</div>
		</div>

		<!-- Loading/Error/Success -->
		{#if loading}
			<div class="flex-1 flex items-center justify-center">
				<div class="flex flex-col items-center gap-4">
					<div class="relative w-12 h-12">
						<div class="absolute inset-0 border-4 border-primary-500/30 rounded-full"></div>
						<div
							class="absolute inset-0 border-4 border-primary-500 border-t-transparent rounded-full animate-spin"
						></div>
					</div>
					<p class="text-gray-400 font-medium">Cargando atajos...</p>
				</div>
			</div>
		{:else}
			{#if error}
				<div
					class="p-4 mb-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 flex items-center gap-3"
				>
					<AlertTriangle />
					{error}
					<button onclick={() => (error = '')} class="ml-auto p-1 hover:bg-white/10 rounded">
						<X size={16} />
					</button>
				</div>
			{/if}
			{#if successMessage}
				<div
					class="p-4 mb-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 flex items-center gap-3"
					in:slide
				>
					<Check />
					{successMessage}
				</div>
			{/if}

			<!-- Categories Grid -->
			<div class="flex-1 overflow-y-auto custom-scrollbar space-y-6 pb-4">
				{#each categoriesWithShortcuts() as category}
					<div class="bg-surface-2 border border-white/5 rounded-xl overflow-hidden">
						<!-- Category Header -->
						<div class="px-5 py-4 bg-surface-3 border-b border-white/5 flex items-center gap-3">
							<div class="p-2 rounded-lg bg-primary-500/10">
								<svelte:component
									this={categoryIcons[category.id] || Settings2}
									size={20}
									class="text-primary-400"
								/>
							</div>
							<div>
								<h3 class="text-lg font-semibold text-white">{category.label}</h3>
								{#if category.description}
									<p class="text-xs text-gray-400">{category.description}</p>
								{/if}
							</div>
							<span class="ml-auto text-xs text-gray-500 font-mono bg-surface-2 px-2 py-1 rounded">
								{category.items.length} atajos
							</span>
						</div>

						<!-- Shortcuts List -->
						<div class="divide-y divide-white/5">
							{#each category.items as shortcut}
								{@const isEditing = editingShortcut === shortcut.id}
								{@const isCustom = isCustomized(shortcut.id)}

								<div
									class="flex items-center justify-between px-5 py-4 group hover:bg-white/[0.02] transition-colors"
								>
									<!-- Info -->
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2">
											<span class="text-sm font-medium text-white">
												{shortcut.label}
											</span>
											{#if shortcut.readonly}
												<span
													class="text-[10px] px-1.5 py-0.5 bg-gray-700/50 text-gray-400 rounded font-medium"
												>
													Sistema
												</span>
											{/if}
											{#if isCustom}
												<span
													class="text-[10px] px-1.5 py-0.5 bg-blue-500/20 text-blue-400 rounded font-medium"
												>
													Personalizado
												</span>
											{/if}
										</div>
										{#if shortcut.description}
											<p class="text-xs text-gray-500 truncate mt-0.5">
												{shortcut.description}
											</p>
										{/if}
									</div>

									<!-- Key Display / Editor -->
									<div class="flex items-center gap-3">
										{#if isEditing}
											<!-- Modo edición -->
											<div
												class="flex items-center gap-2"
												transition:slide={{ axis: 'x', duration: 150 }}
											>
												<div
													class="px-4 py-2 min-w-[120px] text-center bg-blue-500/10 border border-blue-500/30 rounded-lg text-sm text-blue-400 animate-pulse font-mono"
												>
													{recordedKeys || 'Presiona teclas...'}
												</div>
												<button
													onclick={saveShortcut}
													disabled={!recordedKeys || saving}
													class="p-2 rounded-lg bg-green-500/10 text-green-400 hover:bg-green-500/20 disabled:opacity-50 transition-colors"
													title="Guardar"
												>
													<Check size={18} />
												</button>
												<button
													onclick={cancelEdit}
													class="p-2 rounded-lg bg-red-500/10 text-red-400 hover:bg-red-500/20 transition-colors"
													title="Cancelar"
												>
													<X size={18} />
												</button>
											</div>
										{:else}
											<!-- Modo normal -->
											<div class="flex items-center gap-1.5">
												{#each formatKeysDisplay(getDisplayKeys(shortcut)) as key}
													<kbd
														class="px-2.5 py-1.5 min-w-[2rem] text-center text-xs font-semibold text-gray-200 bg-[#0d1117] border border-[#30363d] rounded-lg shadow-sm"
													>
														{key}
													</kbd>
												{/each}
											</div>

											{#if !shortcut.readonly}
												<div
													class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity ml-2"
												>
													<button
														onclick={() => startEditing(shortcut.id)}
														class="p-1.5 text-gray-500 hover:text-primary-400 hover:bg-white/5 rounded transition-colors"
														title="Editar atajo"
													>
														<Keyboard size={16} />
													</button>
													{#if isCustom}
														<button
															onclick={() => resetShortcut(shortcut.id)}
															class="p-1.5 text-gray-500 hover:text-amber-400 hover:bg-white/5 rounded transition-colors"
															title="Restaurar default"
														>
															<RotateCcw size={16} />
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
</div>

<style>
	.custom-scrollbar {
		scrollbar-width: thin;
		scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
	}
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background-color: rgba(255, 255, 255, 0.1);
		border-radius: 20px;
	}
</style>
