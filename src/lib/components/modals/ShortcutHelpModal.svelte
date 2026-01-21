<script lang="ts">
	import { activeShortcuts, getOrderedCategories, shortcutRegistry } from '$lib/shortcuts';
	import type { ShortcutDefinition, CategoryMetadata } from '$lib/shortcuts';
	import {
		X,
		Keyboard,
		Settings,
		Search,
		LayoutList,
		Square,
		Table,
		DoorOpen
	} from 'lucide-svelte';
	import { slide, fade } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';

	export let isOpen = false;
	const dispatch = createEventDispatcher();

	let shortcuts: ShortcutDefinition[] = [];

	// Cuando cambian los atajos activos (triggers update), obtenemos TODOS los atajos
	// para mostrarlos en la guía, no solo los del scope actual.
	// También se actualiza al montar si isOpen cambia.
	$: if ($activeShortcuts || isOpen) {
		shortcuts = shortcutRegistry.getAllShortcuts();
	}

	function close() {
		dispatch('close');
	}

	// Obtener ícono por categoría
	const categoryIcons: Record<string, typeof Settings> = {
		system: Settings,
		modules: LayoutList,
		modals: Square,
		grids: Table,
		ingresos: DoorOpen
	};

	// Agrupar por categoría usando las nuevas categorías
	$: categoriesWithShortcuts = getOrderedCategories()
		.map((category) => ({
			...category,
			items: shortcuts.filter((s) => s.category === category.id)
		}))
		.filter((cat) => cat.items.length > 0);

	function formatKeys(keys: string): string[] {
		return keys
			.replace('ctrl', 'Ctrl')
			.replace('shift', 'Shift')
			.replace('alt', 'Alt')
			.replace('escape', 'Esc')
			.replace('delete', 'Del')
			.replace('pagedown', 'PgDn')
			.replace('pageup', 'PgUp')
			.replace('home', 'Home')
			.split('+');
	}
</script>

<svelte:window
	on:keydown={(e) => {
		if (isOpen && e.key === 'Escape') {
			e.preventDefault();
			close();
		}
	}}
/>

{#if isOpen}
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		role="presentation"
		transition:fade={{ duration: 200 }}
		on:click|self={close}
		on:keydown={(e) => {
			if (e.key === 'Escape') close();
		}}
	>
		<div
			class="w-full max-w-4xl bg-white dark:bg-gray-900 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-800 overflow-hidden max-h-[85vh] flex flex-col"
			transition:slide={{ duration: 250, axis: 'y' }}
		>
			<!-- Header -->
			<div
				class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-900/50"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
						<Keyboard class="w-6 h-6 text-blue-600 dark:text-blue-400" />
					</div>
					<div>
						<h2 class="text-xl font-bold text-gray-900 dark:text-white">Atajos de Teclado</h2>
						<p class="text-sm text-gray-500 dark:text-gray-400">
							Guía rápida de comandos disponibles
						</p>
					</div>
				</div>
				<button
					on:click={close}
					class="p-2 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-full transition-colors text-gray-500"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-y-auto p-6">
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each categoriesWithShortcuts as category}
						{@const CategoryIcon = categoryIcons[category.id] || Settings}
						<div class="space-y-3">
							<div class="flex items-center gap-2">
								<div class="p-1.5 bg-gray-100 dark:bg-gray-800 rounded-md">
									<CategoryIcon class="w-4 h-4 text-gray-500 dark:text-gray-400" />
								</div>
								<h3
									class="text-sm font-semibold text-gray-600 dark:text-gray-300 uppercase tracking-wider"
								>
									{category.label}
								</h3>
							</div>

							<div class="space-y-2 pl-1">
								{#each category.items as item}
									<div class="flex items-center justify-between group py-1">
										<div class="flex flex-col">
											<span
												class="text-sm font-medium text-gray-700 dark:text-gray-200 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors"
											>
												{item.label}
											</span>
											{#if item.description}
												<span class="text-xs text-gray-500 dark:text-gray-500 line-clamp-1">
													{item.description}
												</span>
											{/if}
										</div>
										<div class="flex items-center gap-1 flex-shrink-0 ml-2">
											{#each formatKeys(item.keys) as key}
												<kbd
													class="px-2 py-1 min-w-[1.5rem] text-center text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded-lg dark:bg-gray-800 dark:text-gray-100 dark:border-gray-700 shadow-sm"
												>
													{key}
												</kbd>
											{/each}
										</div>
									</div>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Footer -->
			<div
				class="p-4 bg-gray-50 dark:bg-gray-950/50 border-t border-gray-200 dark:border-gray-800 text-center text-xs text-gray-500"
			>
				<div class="flex items-center justify-center gap-4">
					<span>
						Tip: Abre Spotlight con
						<kbd class="font-bold px-1.5 py-0.5 bg-gray-200 dark:bg-gray-800 rounded">
							{formatKeys(shortcutRegistry.getShortcut('toggle-spotlight')?.keys || 'Ctrl+K').join(
								'+'
							)}
						</kbd>
					</span>
					<span class="text-gray-400">•</span>
					<span> Los atajos pueden personalizarse en Configuración </span>
				</div>
			</div>
		</div>
	</div>
{/if}
