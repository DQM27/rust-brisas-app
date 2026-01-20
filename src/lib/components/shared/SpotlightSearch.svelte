<!-- src/lib/components/shared/SpotlightSearch.svelte -->
<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { onMount, onDestroy } from 'svelte';
	import { Search } from 'lucide-svelte';

	// Import logic
	import {
		buildSpotlightItems,
		filterItems,
		groupItems,
		flattenGroups,
		getCategoryLabel,
		type SpotlightItem
	} from '$lib/logic/spotlight/spotlightItems';

	import { showSpotlight } from '$lib/stores/ui';
	import { tabsStore } from '$lib/stores/tabs';

	// Props
	interface Props {
		// Eliminado props de control externo
	}

	let {}: Props = $props();

	// State
	let query = $state('');
	let inputRef = $state<HTMLInputElement>();
	let highlightedIndex = $state(0);

	// Store subscription
	let show = $derived($showSpotlight);

	// Build items reactively
	const allItems = $derived(buildSpotlightItems($tabsStore, handleClose));
	const filteredItemsList = $derived(query.trim() === '' ? [] : filterItems(allItems, query));
	const groupedItems = $derived(groupItems(filteredItemsList));
	const flatItems = $derived(flattenGroups(groupedItems));

	// Reset highlight when query changes
	$effect(() => {
		query;
		highlightedIndex = 0;
	});

	// Focus input when opened
	$effect(() => {
		if (show) {
			query = '';
			highlightedIndex = 0;
			setTimeout(() => inputRef?.focus(), 50);
		}
	});

	function handleClose() {
		showSpotlight.set(false);
		query = '';
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			highlightedIndex = Math.min(highlightedIndex + 1, flatItems.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			highlightedIndex = Math.max(highlightedIndex - 1, 0);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			const item = flatItems[highlightedIndex];
			if (item) {
				item.action();
			}
		}
	}

	function handleItemClick(item: SpotlightItem) {
		item.action();
	}

	// Función para scroll automático
	function scrollIntoView(index: number) {
		const el = document.getElementById(`spotlight-item-${index}`);
		if (el) {
			el.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
		}
	}

	$effect(() => {
		if (show && flatItems.length > 0) {
			scrollIntoView(highlightedIndex);
		}
	});

	// Determinar si mostrar cabecera
	function shouldShowHeader(index: number, item: SpotlightItem): boolean {
		if (index === 0) return true;
		const prevItem = flatItems[index - 1];
		return item.category !== prevItem.category;
	}
</script>

{#if show}
	<!-- Overlay -->
	<div
		class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100] flex items-center justify-center p-4 outline-none"
		transition:fade={{ duration: 150 }}
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		role="button"
		tabindex="-1"
		onkeydown={handleKeyDown}
	>
		<!-- Modal -->
		<div
			class="spotlight-modal bg-[#1e1e1e] border border-white/10 w-full max-w-lg rounded-xl shadow-2xl overflow-hidden flex flex-col outline-none"
			transition:scale={{ duration: 200, start: 0.95 }}
		>
			<!-- Search Input Section -->
			<div class="px-4 py-3 border-b border-white/5 bg-[#252526]">
				<div
					class="search-container relative flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 transition-all outline-none"
				>
					<Search class="absolute left-3 text-gray-500" size={18} />
					<input
						bind:this={inputRef}
						bind:value={query}
						type="text"
						placeholder="Buscar en Megabrisas..."
						class="w-full bg-transparent pl-10 pr-4 py-3 text-[15px] text-white focus:outline-none outline-none border-none placeholder:text-gray-600 appearance-none ring-0"
						autocomplete="off"
						onkeydown={handleKeyDown}
					/>
					<div class="absolute right-3 flex gap-2">
						<kbd
							class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[10px] text-gray-500 font-mono"
						>
							ESC
						</kbd>
					</div>
				</div>
			</div>

			<!-- Results Section (Only if query is not empty) -->
			{#if query.trim() !== ''}
				<div
					class="results-container max-h-[50vh] overflow-y-auto p-2 scrollbar-thin shadow-inner bg-[#1e1e1e]/50"
				>
					{#if flatItems.length === 0}
						<div class="p-8 text-center text-gray-500">
							<Search size={32} class="mx-auto mb-3 opacity-20" />
							<p class="text-sm">No se encontraron resultados para "{query}"</p>
						</div>
					{:else}
						<div class="space-y-0.5">
							{#each flatItems as item, i (item.id)}
								<!-- Header Categoría -->
								{#if shouldShowHeader(i, item)}
									<div
										class="px-2 py-1.5 mt-2 first:mt-0 text-[10px] font-semibold text-gray-500 uppercase tracking-wider bg-transparent"
									>
										{getCategoryLabel(item.category)}
									</div>
								{/if}

								<button
									id="spotlight-item-{i}"
									onclick={() => handleItemClick(item)}
									onmouseenter={() => (highlightedIndex = i)}
									class="w-full text-left px-3 py-2 rounded-lg flex items-center gap-3 transition-colors relative scroll-mt-10
                                        {i === highlightedIndex
										? item.subCategory === 'master'
											? 'bg-emerald-600 text-white shadow-lg'
											: item.subCategory === 'transaction'
												? 'bg-amber-600 text-white shadow-lg'
												: item.subCategory === 'settings'
													? 'bg-indigo-600 text-white shadow-lg'
													: 'bg-blue-600 text-white shadow-lg'
										: 'hover:bg-white/5 text-gray-400'}"
								>
									<div class="flex-shrink-0">
										<item.icon
											size={18}
											class={i === highlightedIndex
												? 'text-white'
												: item.subCategory === 'master'
													? 'text-emerald-500'
													: item.subCategory === 'transaction'
														? 'text-amber-500'
														: item.subCategory === 'settings'
															? 'text-indigo-400'
															: item.category === 'tab'
																? 'text-blue-400'
																: 'text-gray-500'}
										/>
									</div>
									<div class="flex-1 min-w-0">
										<div class="font-medium text-[14px] truncate flex items-center gap-2">
											<span class="truncate">{item.label}</span>
											<div class="flex gap-1 items-center">
												{#if item.subCategory === 'master'}
													<span
														class="text-[8px] px-1 py-0 rounded border leading-none font-bold uppercase tracking-tight
                                                        {i === highlightedIndex
															? 'bg-white/20 text-white border-white/30'
															: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20'}"
													>
														CREAR
													</span>
												{:else if item.subCategory === 'transaction'}
													<span
														class="text-[8px] px-1 py-0 rounded border leading-none font-bold uppercase tracking-tight
                                                        {i === highlightedIndex
															? 'bg-white/20 text-white border-white/30'
															: 'bg-amber-500/10 text-amber-400 border-amber-500/20'}"
													>
														NUEVO
													</span>
												{:else if item.subCategory === 'settings'}
													<span
														class="text-[8px] px-1 py-0 rounded border leading-none font-bold uppercase tracking-tight
                                                        {i === highlightedIndex
															? 'bg-white/20 text-white border-white/30'
															: 'bg-indigo-500/10 text-indigo-400 border-indigo-500/20'}"
													>
														AJUSTE
													</span>
												{/if}

												{#if item.isOpen}
													<span
														class="text-[8px] px-1 py-0 rounded border leading-none font-bold uppercase tracking-tight
                                                        {i === highlightedIndex
															? 'bg-white/20 text-white border-white/30'
															: 'bg-blue-500/10 text-blue-400 border-blue-500/20'}"
													>
														ABIERTO
													</span>
												{/if}
											</div>
										</div>
										{#if item.description}
											<div
												class="text-[11px] truncate {i === highlightedIndex
													? 'text-white/70'
													: 'text-gray-500'}"
											>
												{item.description}
											</div>
										{/if}
									</div>
								</button>
							{/each}
						</div>
					{/if}
				</div>

				<!-- Footer Info (Only if query is not empty) -->
				<div
					class="px-4 py-2 bg-black/20 border-t border-white/5 flex justify-between items-center bg-[#252526]"
				>
					<div class="text-[9px] text-gray-500 font-medium uppercase tracking-wider">
						{flatItems.length} resultados
					</div>
					<div class="flex items-center gap-3">
						<span class="text-[9px] text-gray-600 flex items-center gap-1">
							<kbd class="px-1 py-0.5 bg-black/40 rounded border border-white/10 text-[8px]">↑↓</kbd
							>
							navegar
						</span>
						<span class="text-[9px] text-gray-600 flex items-center gap-1">
							<kbd class="px-1 py-0.5 bg-black/40 rounded border border-white/10 text-[8px]">⏎</kbd>
							seleccionar
						</span>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	:global(body:has(.spotlight-modal)) {
		overflow: hidden;
	}

	.search-container,
	.search-container *:focus {
		outline: none !important;
		box-shadow: none !important;
	}

	.search-container:focus-within {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
	}

	/* Scrollbar styling */
	.results-container::-webkit-scrollbar {
		width: 6px;
	}

	.results-container::-webkit-scrollbar-track {
		background: transparent;
	}

	.results-container::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 3px;
	}

	.results-container::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.2);
	}
</style>
