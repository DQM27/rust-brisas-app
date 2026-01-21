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
		DoorOpen,
		Zap
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
		ingresos: DoorOpen,
		'ingress-access': Zap
	};

	// Agrupar por categoría usando las nuevas categorías
	$: allCategories = getOrderedCategories()
		.map((category) => ({
			...category,
			items: shortcuts.filter((s) => s.category === category.id)
		}))
		.filter((cat) => cat.items.length > 0);

	$: globalCategories = allCategories.filter((cat) =>
		['system', 'ingress-access'].includes(cat.id)
	);
	$: contextualCategories = allCategories.filter(
		(cat) => !['system', 'ingress-access'].includes(cat.id)
	);

	function formatKeys(keys: string): string[] {
		return keys
			.replace(/ctrl/gi, 'Ctrl')
			.replace(/shift/gi, 'Shift')
			.replace(/alt/gi, 'Alt')
			.replace(/escape/gi, 'Esc')
			.replace(/delete/gi, 'Del')
			.replace(/pagedown/gi, 'PgDn')
			.replace(/pageup/gi, 'PgUp')
			.replace(/home/gi, 'Home')
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
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/80 backdrop-blur-md"
		role="presentation"
		transition:fade={{ duration: 200 }}
		on:click|self={close}
	>
		<div
			class="w-full max-w-5xl bg-[#1c2128] rounded-2xl shadow-2xl border border-white/10 overflow-hidden max-h-[90vh] flex flex-col"
			transition:slide={{ duration: 250, axis: 'y' }}
		>
			<!-- Header -->
			<div class="flex items-center justify-between p-8 border-b border-white/5 bg-white/[0.02]">
				<div class="flex items-center gap-4">
					<div class="p-3 bg-blue-500/10 rounded-xl border border-blue-500/20">
						<Keyboard class="w-7 h-7 text-blue-400" />
					</div>
					<div>
						<h2 class="text-2xl font-bold text-white tracking-tight">Atajos de Teclado</h2>
						<p class="text-sm text-gray-400 font-medium">Guía rápida de comandos disponibles</p>
					</div>
				</div>
				<button
					on:click={close}
					class="p-2 hover:bg-white/5 rounded-full transition-colors text-gray-500 hover:text-white"
				>
					<X class="w-6 h-6" />
				</button>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-y-auto p-8 custom-scrollbar">
				<div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
					<!-- Columna Izquierda: Globales -->
					<div class="lg:col-span-5 space-y-10">
						{#each globalCategories as category}
							{@const CategoryIcon = categoryIcons[category.id] || Settings}
							<div class="space-y-5">
								<div class="flex items-center gap-3 pb-2 border-b border-white/5">
									<div class="p-2 bg-white/5 rounded-lg">
										<CategoryIcon class="w-5 h-5 text-blue-400" />
									</div>
									<h3 class="text-sm font-bold text-white uppercase tracking-[0.2em]">
										{category.label}
									</h3>
								</div>

								<div class="space-y-1">
									{#each category.items as item}
										<div
											class="flex items-center justify-between group py-2 px-3 rounded-lg hover:bg-white/[0.03] transition-all"
										>
											<div class="flex flex-col gap-0.5">
												<span
													class="text-sm font-semibold text-gray-200 group-hover:text-blue-400 transition-colors"
												>
													{item.label}
												</span>
												{#if item.description}
													<span class="text-[11px] text-gray-500 font-medium leading-tight">
														{item.description}
													</span>
												{/if}
											</div>
											<div class="flex items-center gap-1.5 flex-shrink-0 ml-4">
												{#each formatKeys(item.keys) as key}
													<kbd
														class="px-2.5 py-1.5 min-w-[2.2rem] text-center text-[10px] font-bold text-gray-100 bg-[#0d1117] border border-[#30363d] rounded-lg shadow-[0_2px_0_0_#161b22]"
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

					<!-- Separador Vertical (Desktop) -->
					<div class="hidden lg:block lg:col-span-1 w-px bg-white/5 justify-self-center"></div>

					<!-- Columna Derecha: Contextuales -->
					<div class="lg:col-span-6 space-y-10">
						{#each contextualCategories as category}
							{@const CategoryIcon = categoryIcons[category.id] || Settings}
							<div class="space-y-5">
								<div class="flex items-center gap-3 pb-2 border-b border-white/5">
									<div class="p-2 bg-white/5 rounded-lg">
										<CategoryIcon class="w-5 h-5 text-purple-400" />
									</div>
									<h3 class="text-sm font-bold text-white uppercase tracking-[0.2em]">
										{category.label}
									</h3>
								</div>

								<div class="grid grid-cols-1 md:grid-cols-2 gap-x-6 gap-y-1">
									{#each category.items as item}
										<div
											class="flex items-center justify-between group py-2 px-3 rounded-lg hover:bg-white/[0.03] transition-all"
										>
											<div class="flex flex-col gap-0.5">
												<span
													class="text-sm font-semibold text-gray-200 group-hover:text-purple-400 transition-colors"
												>
													{item.label}
												</span>
												{#if item.description}
													<span class="text-[11px] text-gray-500 font-medium leading-tight">
														{item.description}
													</span>
												{/if}
											</div>
											<div class="flex items-center gap-1.5 flex-shrink-0 ml-3">
												{#each formatKeys(item.keys) as key}
													<kbd
														class="px-2 py-1.2 min-w-[2rem] text-center text-[9px] font-bold text-gray-200 bg-[#0d1117] border border-[#30363d] rounded-lg shadow-[0_2px_0_0_#161b22]"
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
