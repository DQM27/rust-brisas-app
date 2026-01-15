<script lang="ts">
	import { createEventDispatcher, onMount, onDestroy } from 'svelte';
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { searchStore, selectedSearchStore, hasResults } from '$lib/stores/searchStore';
	import { performSearch } from '$lib/logic/search/performSearch';
	import type { SearchResult, SearchState } from '$lib/types/search.types';
	import { highlightMatches } from '$lib/logic/search/highlightMatches';
	import { gridState } from '$lib/stores/gridStateStore.svelte';

	let {
		placeholder = 'Buscar por nombre, cédula o empresa...',
		disabled = false,
		limit = 10,
		autofocus = false,
		searchFunction = null
	} = $props();

	let inputRef = $state<HTMLInputElement>();
	let query = $state('');
	let showDropdown = $state(false);
	let highlightedIndex = $state(-1);
	let debounceTimer: ReturnType<typeof setTimeout>;

	const dispatch = createEventDispatcher<{
		select: SearchResult;
		clear: void;
	}>();

	// Reactive state from store (still works but we use $derived for better Svelte 5 compatibility)
	const results = $derived(($searchStore as SearchState).results);
	const isLoading = $derived(($searchStore as SearchState).isLoading);
	const error = $derived(($searchStore as SearchState).error);
	const selectedResult = $derived(($selectedSearchStore as { result: SearchResult | null }).result);

	function handleInput() {
		clearTimeout(debounceTimer);

		// Prioridad 1: Si hay searchFunction explícita (ej. en modales), usarla
		if (searchFunction) {
			// Dejar pasar al debounce normal
		}
		// Prioridad 2: Si no hay searchFunction pero hay Grid Activo, filtrar grid
		else if (gridState.activeGridApi) {
			gridState.activeGridApi.setGridOption('quickFilterText', query);
			showDropdown = false;
			return;
		}

		if (query.trim().length < 2) {
			searchStore.clearResults();
			showDropdown = false;
			return;
		}

		debounceTimer = setTimeout(async () => {
			if (searchFunction) {
				// Custom local search
				searchStore.setLoading(true);
				try {
					const customResults = await searchFunction(query);
					searchStore.setResults(customResults);
				} catch (e) {
					console.error(e);
					searchStore.setError('Error en la búsqueda');
				} finally {
					searchStore.setLoading(false);
				}
			} else {
				// Default global search
				await performSearch(query, limit);
			}
			showDropdown = true;
			highlightedIndex = -1;
		}, 300);
	}

	function handleSelect(result: SearchResult) {
		selectedSearchStore.select(result);
		dispatch('select', result);
		query = '';
		showDropdown = false;
		highlightedIndex = -1;
		searchStore.clear();
	}

	export function clear() {
		query = '';

		if (gridState.activeGridApi) {
			gridState.activeGridApi.setGridOption('quickFilterText', '');
		}

		showDropdown = false;
		highlightedIndex = -1;
		searchStore.clear();
		selectedSearchStore.clear();
		dispatch('clear');
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (!showDropdown || results.length === 0) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				highlightedIndex = Math.min(highlightedIndex + 1, results.length - 1);
				break;
			case 'ArrowUp':
				event.preventDefault();
				highlightedIndex = Math.max(highlightedIndex - 1, -1);
				break;
			case 'Enter':
				event.preventDefault();
				if (highlightedIndex >= 0 && highlightedIndex < results.length) {
					handleSelect(results[highlightedIndex]);
				}
				break;
			case 'Escape':
				event.preventDefault();
				showDropdown = false;
				highlightedIndex = -1;
				break;
		}
	}

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as Node;
		if (inputRef && !inputRef.parentElement?.contains(target)) {
			showDropdown = false;
		}
	}

	export function focus() {
		if (inputRef) inputRef.focus();
	}

	onMount(() => {
		if (autofocus && inputRef) inputRef.focus();
		document.addEventListener('click', handleClickOutside);
	});

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
		clearTimeout(debounceTimer);
		searchStore.clear();
		selectedSearchStore.clear();
	});
</script>

<div class="relative w-full">
	<!-- Input -->
	<div class="relative">
		<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
			{#if isLoading}
				<svg class="h-4 w-4 animate-spin text-gray-400" fill="none" viewBox="0 0 24 24">
					<circle
						class="opacity-25"
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="3"
					/>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					/>
				</svg>
			{:else}
				<svg class="h-4 w-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
			{/if}
		</div>

		<input
			bind:this={inputRef}
			bind:value={query}
			oninput={handleInput}
			onfocus={() => query.trim().length >= 2 && results.length > 0 && (showDropdown = true)}
			onkeydown={handleKeyDown}
			type="text"
			autocomplete="off"
			placeholder={!searchFunction && gridState.activeGridApi
				? 'Filtrar en tabla activa...'
				: selectedResult
					? `Filtrando: ${selectedResult.nombreCompleto || selectedResult.id}`
					: placeholder}
			{disabled}
			class="w-full rounded-lg border bg-[#1e1e1e] pl-10 pr-16 py-2.5 text-sm text-white
        placeholder:text-gray-500 transition-colors
        focus:outline-none focus:ring-1
        disabled:opacity-50 disabled:cursor-not-allowed
        {selectedResult
				? 'border-blue-500/50 focus:ring-blue-500/30'
				: 'border-white/10 hover:border-white/20 focus:border-white/30 focus:ring-white/10'}"
		/>

		<div class="absolute inset-y-0 right-0 flex items-center gap-2 pr-3">
			{#if selectedResult || query}
				<button
					type="button"
					onclick={clear}
					class="p-1 text-gray-500 hover:text-gray-300 transition-colors"
					title="Limpiar"
				>
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			{/if}

			{#if selectedResult}
				<span class="text-xs text-blue-400 font-medium">Filtrado</span>
			{:else if $hasResults && !isLoading && query.length >= 2}
				<span class="text-xs text-gray-500">{results.length}</span>
			{/if}
		</div>
	</div>

	<!-- Dropdown -->
	{#if showDropdown && query.trim().length >= 2}
		<div
			transition:slide={{ duration: 150, easing: cubicOut }}
			class="absolute z-50 mt-1 w-full rounded-lg border border-white/10 bg-[#1e1e1e] shadow-lg overflow-hidden"
		>
			{#if isLoading}
				<div class="px-4 py-6 text-center">
					<p class="text-sm text-gray-400">Buscando...</p>
				</div>
			{:else if error}
				<div class="px-4 py-3">
					<p class="text-sm text-red-400">{error}</p>
				</div>
			{:else if results.length === 0}
				<div class="px-4 py-6 text-center">
					<p class="text-sm text-gray-400">Sin resultados para "{query}"</p>
				</div>
			{:else}
				<div class="py-1">
					{#each results as result, index}
						{@const typedResult = result as SearchResult}
						<button
							type="button"
							onclick={() => handleSelect(typedResult)}
							onmouseenter={() => (highlightedIndex = index)}
							class="w-full px-3 py-1 text-left flex items-center gap-3 {highlightedIndex === index
								? 'bg-[#1f6feb]/5'
								: 'hover:bg-gray-700/20'}"
						>
							<!-- Icon (GitHub style) -->
							<div class="flex-shrink-0 text-gray-500">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									class="h-3.5 w-3.5"
									fill="none"
									viewBox="0 0 24 24"
									stroke="currentColor"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
									/>
								</svg>
							</div>

							<!-- Content with Text Highlight -->
							<div class="flex-1 min-w-0 flex items-center justify-between py-1">
								<span
									class="text-[13px] px-1.5 py-0.5 rounded transition-colors {highlightedIndex ===
									index
										? 'bg-[#1f6feb] text-white'
										: 'text-gray-300'} truncate"
								>
									<!-- eslint-disable-next-line svelte/no-at-html-tags -->
									{@html highlightMatches(
										typedResult.nombreCompleto || `ID: ${typedResult.id}`,
										query
									)}
								</span>

								<div class="flex items-center gap-2">
									<span
										class="text-[10px] px-1.5 py-0.5 rounded-md bg-gray-800 text-gray-400 border border-gray-700 font-mono"
									>
										{typedResult.tipo}
									</span>
								</div>
							</div>
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>
