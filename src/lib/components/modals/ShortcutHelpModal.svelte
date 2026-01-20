<script lang="ts">
	import { shortcutRegistry } from '$lib/logic/shortcuts/registry';
	import type { ShortcutDefinition } from '$lib/logic/shortcuts/types';
	import { X, Command, Keyboard } from 'lucide-svelte';
	import { slide, fade } from 'svelte/transition';
	import { createEventDispatcher, onMount } from 'svelte';

	export let isOpen = false;
	const dispatch = createEventDispatcher();

	// Podemos suscribirnos al store del registro si queremos actualizaciones en vivo
	// Pero como los atajos cargados son estáticos por ahora, basta con leerlos al montar o reactivamente al store.

	let shortcuts: ShortcutDefinition[] = [];

	// Suscribirse al store de atajos activos
	const unsubscribe = shortcutRegistry.activeShortcuts.subscribe((value) => {
		shortcuts = value;
	});

	function close() {
		dispatch('close');
	}

	// Agrupar por categoría
	$: grouped = {
		Sistema: shortcuts.filter((s) => s.category === 'system'),
		Navegación: shortcuts.filter((s) => s.category === 'navigation'),
		'Acciones Globales': shortcuts.filter((s) => s.category === 'action'),
		Edición: shortcuts.filter((s) => s.category === 'edit')
	};

	function formatKeys(keys: string): string[] {
		return keys.replace('$mod', 'Ctrl').split('+');
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
			class="w-full max-w-3xl bg-white dark:bg-gray-900 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-800 overflow-hidden max-h-[80vh] flex flex-col"
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
				<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
					{#each Object.entries(grouped) as [category, items]}
						{#if items.length > 0}
							<div class="space-y-4">
								<h3
									class="text-sm font-semibold text-gray-400 uppercase tracking-wider flex items-center gap-2"
								>
									{category}
								</h3>
								<div class="space-y-2">
									{#each items as item}
										<div class="flex items-center justify-between group">
											<div class="flex flex-col">
												<span
													class="text-sm font-medium text-gray-700 dark:text-gray-200 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors"
												>
													{item.label}
												</span>
												{#if item.description}
													<span class="text-xs text-gray-500">{item.description}</span>
												{/if}
											</div>
											<div class="flex items-center gap-1">
												{#each formatKeys(item.keys) as key}
													<kbd
														class="px-2 py-1 min-w-[1.5rem] text-center text-xs font-semibold text-gray-800 bg-gray-100 border border-gray-200 rounded-lg dark:bg-gray-800 dark:text-gray-100 dark:border-gray-700 shadow-sm"
													>
														{key === ' ' ? 'Space' : key}
													</kbd>
												{/each}
											</div>
										</div>
									{/each}
								</div>
							</div>
						{/if}
					{/each}
				</div>
			</div>

			<!-- Footer -->
			<div
				class="p-4 bg-gray-50 dark:bg-gray-950/50 border-t border-gray-200 dark:border-gray-800 text-center text-xs text-gray-500"
			>
				Tip: Puedes abrir Spotlight con <kbd class="font-bold">Ctrl+K</kbd> para buscar acciones rápidamente.
			</div>
		</div>
	</div>
{/if}
