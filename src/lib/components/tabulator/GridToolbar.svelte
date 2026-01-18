<script lang="ts">
	import { Search, Download, Settings, X, FileText, Sheet } from 'lucide-svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		searchable?: boolean;
		downloadable?: boolean;
		onSearch?: (term: string) => void;
		onExport?: (type: 'csv' | 'json' | 'xlsx') => void;
		primaryActions?: Snippet;
		secondaryActions?: Snippet; // Left-aligned actions (e.g. Columns)
		CustomFilters?: Snippet; // Left-aligned filters
	}

	let {
		searchable = true,
		downloadable = false,
		onSearch,
		onExport,
		primaryActions,
		secondaryActions,
		CustomFilters
	}: Props = $props();

	let searchTerm = $state('');
</script>

<div
	class="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between p-4 bg-[#1e1e1e] border-b border-white/5"
>
	<!-- Left Section: Primary Actions + Secondary Actions + Filters -->
	<div class="flex flex-wrap items-center gap-3 w-full sm:w-auto">
		{#if primaryActions}
			{@render primaryActions()}
		{/if}

		{#if secondaryActions}
			<div class="h-6 w-px bg-white/10 mx-1 hidden sm:block"></div>
			{@render secondaryActions()}
		{/if}

		{#if CustomFilters}
			{@render CustomFilters()}
		{/if}
	</div>

	<!-- Right Section: Search + Exports -->
	<div class="flex items-center gap-3 w-full sm:w-auto justify-end">
		{#if searchable}
			<div class="relative group w-full sm:w-64">
				<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
					<Search
						class="h-4 w-4 text-gray-500 group-focus-within:text-blue-500 transition-colors"
					/>
				</div>
				<input
					type="text"
					bind:value={searchTerm}
					oninput={(e) => onSearch?.(e.currentTarget.value)}
					placeholder="Buscar en tabla..."
					class="block w-full pl-10 pr-3 py-1.5 border border-white/10 rounded-md
                               bg-[#2d2d2d] text-gray-300 placeholder-gray-500
                               focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500
                               text-sm transition-all"
				/>
				{#if searchTerm}
					<button
						class="absolute inset-y-0 right-0 pr-2 flex items-center text-gray-500 hover:text-white"
						onclick={() => {
							searchTerm = '';
							onSearch?.('');
						}}
					>
						<X size={14} />
					</button>
				{/if}
			</div>
		{/if}

		<!-- Botón de Configuración -->
		<button
			class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
			onclick={() => {
				// TODO: Implementar lógica de configuración
				console.log('Configuración clickeada');
			}}
			title="Configuración"
		>
			<Settings class="h-4 w-4" />
		</button>
	</div>
</div>

<style>
	/*
     * Styles for slot buttons are handled by the parent component using Tailwind utility classes
     * to avoid @apply issues in Svelte/Vite.
     */
</style>
