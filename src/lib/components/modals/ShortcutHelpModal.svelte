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
	onkeydown={(e) => {
		if (isOpen && e.key === 'Escape') {
			e.preventDefault();
			close();
		}
	}}
/>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
		role="presentation"
		transition:fade={{ duration: 200 }}
		onclick={(e) => e.target === e.currentTarget && close()}
	>
		<!-- Modal Container -->
		<div
			class="w-full max-w-[750px] bg-surface-2 shadow-2xl border border-surface rounded-xl overflow-hidden max-h-[95vh] flex flex-col"
			transition:slide={{ duration: 250, axis: 'y' }}
		>
			<!-- Header Estándar -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 bg-surface-2 border-b border-surface"
			>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-primary/10 rounded-lg">
						<Keyboard class="w-6 h-6 text-primary" />
					</div>
					<h2 class="text-xl font-semibold text-primary">Atajos de Teclado</h2>
				</div>
				<button
					onclick={close}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={20} />
				</button>
			</div>

			<!-- Content Section -->
			<div class="flex-1 overflow-y-auto p-6 space-y-8 custom-scrollbar bg-surface-1/30">
				<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
					{#each allCategories as category}
						{@const CategoryIcon = categoryIcons[category.id] || Settings}
						<div class="space-y-4">
							<!-- Categoría Title -->
							<div class="flex items-center gap-2 pb-1">
								<CategoryIcon size={16} class="text-secondary" />
								<h3 class="text-xs font-bold text-secondary uppercase tracking-widest">
									{category.label}
								</h3>
							</div>

							<!-- Atajos List (Structured Grouping) -->
							<div class="bg-surface-1 border border-surface rounded-xl overflow-hidden">
								{#each category.items as item}
									<div
										class="flex items-center justify-between p-3.5 hover:bg-primary/5 transition-colors group"
									>
										<div class="flex flex-col gap-0.5 min-w-0">
											<span
												class="text-sm font-medium text-primary group-hover:text-primary transition-colors line-clamp-1"
											>
												{item.label}
											</span>
											{#if item.description}
												<span class="text-[11px] text-secondary/60 line-clamp-1">
													{item.description}
												</span>
											{/if}
										</div>
										<div class="flex items-center gap-1.5 ml-4 flex-none">
											{#each formatKeys(item.keys) as key}
												<kbd
													class="px-2 py-1 min-w-[2rem] text-center text-[10px] font-bold text-primary bg-surface-3 border border-surface rounded shadow-sm group-hover:border-primary/30 transition-colors"
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

			<!-- Footer Estándar -->
			<div
				class="flex-none flex items-center justify-between px-6 py-4 border-t border-surface bg-surface-1"
			>
				<div class="flex items-center gap-2 text-[11px] text-secondary">
					<Search size={14} />
					<span
						>Usa <kbd class="font-bold px-1.5 py-0.5 bg-surface-3 rounded border border-surface"
							>Ctrl+K</kbd
						> para búsqueda rápida</span
					>
				</div>
				<button
					onclick={close}
					class="px-6 py-2 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm"
				>
					Entendido
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
