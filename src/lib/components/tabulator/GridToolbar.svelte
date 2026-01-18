<script lang="ts">
	import {
		Search,
		Download,
		Settings,
		X,
		FileText,
		Sheet,
		ScanText,
		MoveHorizontal,
		Eye,
		Filter
	} from 'lucide-svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		searchable?: boolean;
		downloadable?: boolean;
		onSearch?: (term: string) => void;
		onExport?: (type: 'csv' | 'json' | 'xlsx') => void;
		onAutoSizeColumns?: () => void;
		onFitColumns?: () => void;
		onToggleColumn?: (field: string) => void;
		onToggleFilters?: () => void;
		onAdvancedExport?: () => void;
		columns?: any[]; // Column definitions for visibility toggle
		primaryActions?: Snippet;
		secondaryActions?: Snippet; // Left-aligned actions (e.g. Columns)
		CustomFilters?: Snippet; // Left-aligned filters
	}

	let {
		searchable = true,
		downloadable = false,
		onSearch,
		onExport,
		onAutoSizeColumns,
		onFitColumns,
		onToggleColumn,
		onToggleFilters,
		onAdvancedExport,
		columns = [],
		primaryActions,
		secondaryActions,
		CustomFilters
	}: Props = $props();

	let searchTerm = $state('');
	let showColumnDropdown = $state(false);
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

		<!-- Botones de ajuste de columnas -->
		<div class="flex items-center gap-2">
			<button
				class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
				onclick={() => onAutoSizeColumns?.()}
				title="Ajustar columnas al contenido"
			>
				<ScanText class="h-4 w-4" />
			</button>
			<button
				class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
				onclick={() => onFitColumns?.()}
				title="Ajustar columnas al ancho"
			>
				<MoveHorizontal class="h-4 w-4" />
			</button>
			<!-- Column Visibility Button with Dropdown -->
			<div class="relative">
				<button
					class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
					onclick={() => (showColumnDropdown = !showColumnDropdown)}
					title="Visibilidad de columnas"
				>
					<Eye class="h-4 w-4" />
				</button>
				{#if showColumnDropdown && columns.length > 0}
					<div
						class="absolute right-0 mt-2 w-56 rounded-md shadow-xl bg-[#18181b] ring-1 ring-black ring-opacity-5 z-50 p-2 border border-white/10"
					>
						<div class="text-[10px] font-bold text-gray-500 mb-2 px-2 uppercase tracking-wider">
							Columnas Visibles
						</div>
						{#each columns as col}
							<label
								class="flex items-center px-2 py-1.5 hover:bg-white/5 rounded cursor-pointer group transition-colors"
							>
								<input
									type="checkbox"
									class="form-checkbox h-3.5 w-3.5 text-blue-500 rounded bg-[#27272a] border-gray-600 focus:ring-blue-500/20 focus:ring-offset-0 transition-colors"
									checked={col.visible !== false}
									onclick={() => onToggleColumn?.(col.field)}
								/>
								<span
									class="ml-2 text-xs text-gray-400 group-hover:text-gray-200 transition-colors"
								>
									{col.title}
								</span>
							</label>
						{/each}
					</div>
				{/if}
			</div>
			<!-- Toggle Filters Button -->
			<button
				class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
				onclick={() => onToggleFilters?.()}
				title="Mostrar/Ocultar filtros"
			>
				<Filter class="h-4 w-4" />
			</button>

			<!-- Advanced Export Button -->
			<button
				class="p-2 text-gray-400 hover:text-white hover:bg-white/5 rounded-md transition-colors border border-white/10 bg-[#2d2d2d]"
				onclick={() => onAdvancedExport?.()}
				title="Exportar datos"
			>
				<Download class="h-4 w-4" />
			</button>

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
</div>

<style>
	/*
     * Styles for slot buttons are handled by the parent component using Tailwind utility classes
     * to avoid @apply issues in Svelte/Vite.
     */
</style>
