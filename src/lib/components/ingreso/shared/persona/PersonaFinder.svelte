<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import type { SearchResult } from '$lib/types/search.types';

	// Props
	interface Props {
		scope?: 'all' | 'contratista' | 'proveedor' | 'visita';
		autoFocus?: boolean;
	}

	let { scope = 'all', autoFocus = true }: Props = $props();

	const dispatch = createEventDispatcher();

	async function handleSearch(q: string): Promise<SearchResult[]> {
		try {
			const rawResults: any[] = await invoke('search_global', {
				query: q,
				limit: 10
			});

			// Filter by scope if needed
			if (scope !== 'all') {
				return rawResults.filter((r) => r.tipo.toLowerCase() === scope);
			}
			return rawResults;
		} catch (e: any) {
			console.error('Error searching in PersonaFinder:', e);
			throw e;
		}
	}

	let searchBarRef = $state<any>();

	export function focus() {
		if (searchBarRef) searchBarRef.focus();
	}

	function handleSelect(event: CustomEvent<SearchResult>) {
		const item = event.detail;
		dispatch('select', {
			id: item.id,
			type: item.tipo.toLowerCase(),
			data: item
		});
	}
</script>

<div class="w-full">
	<SearchBar
		bind:this={searchBarRef}
		placeholder="Buscar por nombre, cédula o empresa..."
		autofocus={autoFocus}
		searchFunction={handleSearch}
		on:select={handleSelect}
	/>
</div>
