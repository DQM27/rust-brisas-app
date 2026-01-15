<!-- BlacklistContratistaSearch.svelte -->
<script lang="ts">
	import SearchBar from '$lib/components/shared/SearchBar.svelte';
	import type { SearchResult } from '$lib/types/search.types';
	import { listaNegra } from '$lib/api/listaNegra';

	interface Props {
		loading?: boolean;
		onSelect: (result: SearchResult) => Promise<void>;
		onClear: () => void;
	}

	let { loading = false, onSelect, onClear }: Props = $props();

	let checkingBlock = $state(false);

	async function handleSelect(event: CustomEvent<SearchResult>) {
		const result = event.detail;
		try {
			await listaNegra.checkIsBlocked(result.cedula || '');
		} catch (error) {
			console.error('Error al verificar bloqueo:', error);
		}

		await onSelect(result);
	}

	function handleClear() {
		onClear();
	}
</script>

<div class="space-y-1.5">
	<div class="text-sm font-medium text-primary mb-1">Buscar Contratista</div>
	<SearchBar
		placeholder="Buscar por nombre, cédula o empresa..."
		disabled={loading}
		limit={10}
		on:select={handleSelect}
		on:clear={handleClear}
	/>

	{#if checkingBlock}
		<div class="card-base bg-surface-2 p-3 mt-2">
			<div class="flex items-center gap-2 text-sm text-secondary">
				<svg
					class="animate-spin h-4 w-4"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
				>
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					></path>
				</svg>
				Verificando estado del contratista...
			</div>
		</div>
	{/if}
</div>
