<script lang="ts">
	import type { GridId } from '$lib/types/agGrid';
	import type { GridApi } from '@ag-grid-community/core';
	import { agGridSettings } from '$lib/stores/agGridSettings.svelte';
	import { Filter, Search, Trash2 } from 'lucide-svelte';
	import SelectDropdown from '$lib/components/shared/SelectDropdown.svelte';

	interface Props {
		gridId: GridId;
		gridApi: GridApi | null;
	}

	let { gridId, gridApi }: Props = $props();

	// Estado
	let paginationSize = $derived(agGridSettings.getPaginationSize(gridId));
	let showFloatingFilters = $derived(agGridSettings.getShowFloatingFilters(gridId));
	let enableQuickFilter = $derived(agGridSettings.getEnableQuickFilter(gridId));
	let rowSelectionMode = $derived(agGridSettings.getRowSelectionMode(gridId));
	let suppressRowClickSelection = $derived(agGridSettings.getSuppressRowClickSelection(gridId));
	let enterNavigation = $derived(agGridSettings.getEnterNavigation(gridId));
	let tabNavigation = $derived(agGridSettings.getTabNavigation(gridId));
	let autoSizeOnLoad = $derived(agGridSettings.getAutoSizeColumnsOnLoad(gridId));

	let quickFilterText = $state('');

	// Opciones de paginación
	const paginationOptions = [10, 20, 50, 100, 200, 500];

	// Typed options for SelectDropdown
	const selectionOptions: {
		value: 'single' | 'multiple' | 'none';
		label: string;
	}[] = [
		{ value: 'multiple', label: 'Múltiple (Checkbox)' },
		{ value: 'single', label: 'Simple (Una fila)' },
		{ value: 'none', label: 'Desactivado' }
	];

	const enterOptions: { value: 'down' | 'next' | 'none'; label: string }[] = [
		{ value: 'down', label: 'Mover abajo' },
		{ value: 'next', label: 'Mover siguiente celda' },
		{ value: 'none', label: 'Nada' }
	];

	// Handlers
	function handlePaginationChange(size: number) {
		agGridSettings.setPaginationSize(gridId, size);
		if (gridApi) gridApi.setGridOption('paginationPageSize', size);
	}

	function handleSelectionModeChange(value: 'single' | 'multiple' | 'none') {
		agGridSettings.setRowSelectionMode(gridId, value);
	}

	function handleSuppressRowClickChange(checked: boolean) {
		agGridSettings.setSuppressRowClickSelection(gridId, checked);
	}

	function handleEnterNavChange(value: 'down' | 'next' | 'none') {
		agGridSettings.setEnterNavigation(gridId, value);
	}

	function handleTabNavChange(checked: boolean) {
		agGridSettings.setTabNavigation(gridId, checked);
	}

	function handleAutoSizeChange(checked: boolean) {
		agGridSettings.setAutoSizeColumnsOnLoad(gridId, checked);
	}

	function handleFloatingFiltersChange(checked: boolean) {
		agGridSettings.setShowFloatingFilters(gridId, checked);
		if (gridApi) {
			const currentDefaultColDef = gridApi.getGridOption('defaultColDef');
			gridApi.setGridOption('defaultColDef', {
				...currentDefaultColDef,
				floatingFilter: checked
			});
			gridApi.refreshHeader();
		}
	}

	function handleQuickFilterToggle(checked: boolean) {
		agGridSettings.setEnableQuickFilter(gridId, checked);
		if (!checked && gridApi) {
			gridApi.setGridOption('quickFilterText', '');
			quickFilterText = '';
		}
	}

	function handleQuickFilterChange() {
		if (gridApi && enableQuickFilter) {
			gridApi.setGridOption('quickFilterText', quickFilterText);
		}
	}

	function clearAllFilters() {
		if (gridApi) {
			gridApi.setFilterModel(null);
			gridApi.setGridOption('quickFilterText', '');
			quickFilterText = '';
		}
	}

	function clearSort() {
		if (gridApi) {
			gridApi.applyColumnState({ defaultState: { sort: null } });
		}
	}

	const activeFiltersCount = $derived.by(() => {
		if (!gridApi) return 0;
		const filterModel = gridApi.getFilterModel();
		return filterModel ? Object.keys(filterModel).length : 0;
	});

	const sectionClass = 'space-y-4 p-1';
	const labelClass = 'block text-xs font-medium text-zinc-400 mb-1.5 ml-0.5';
	const checkboxClass =
		'w-4 h-4 rounded bg-black/20 border-zinc-600 text-blue-600 focus:ring-blue-600 focus:ring-offset-0 transition-all checked:bg-blue-600 checked:border-blue-600 cursor-pointer';
</script>

<div class={sectionClass}>
	<!-- Grid Layout -->
	<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
		<div class="space-y-4">
			<!-- Paginación -->
			<div>
				<span class={labelClass}>Registros por página</span>
				<div class="grid grid-cols-3 gap-2" role="group" aria-label="Registros por página">
					{#each paginationOptions as size}
						<button
							type="button"
							onclick={() => handlePaginationChange(size)}
							class="py-1.5 px-2 text-xs font-medium rounded-md transition-all border
                {paginationSize === size
								? 'bg-blue-600/20 text-blue-400 border-blue-500/50 ring-1 ring-blue-500/20'
								: 'bg-black/20 text-zinc-400 border-white/10 hover:border-white/20 hover:text-white'}"
						>
							{size}
						</button>
					{/each}
				</div>
			</div>

			<!-- Selección -->
			<SelectDropdown
				label="Modo de Selección"
				value={rowSelectionMode}
				options={selectionOptions}
				onSelect={handleSelectionModeChange}
			/>

			<!-- Enter Navigation -->
			<SelectDropdown
				label="Tecla Enter"
				value={enterNavigation}
				options={enterOptions}
				onSelect={handleEnterNavChange}
			/>
		</div>

		<div class="space-y-5 pt-0.5">
			<!-- Toggles Grid -->
			<div class="space-y-3 p-3 rounded-lg bg-black/10 border border-white/5">
				{#if rowSelectionMode !== 'none'}
					<label class="flex items-center gap-3 cursor-pointer group">
						<input
							type="checkbox"
							checked={suppressRowClickSelection}
							onchange={(e) => handleSuppressRowClickChange(e.currentTarget.checked)}
							class={checkboxClass}
						/>
						<div class="flex flex-col">
							<span class="text-xs text-zinc-200 group-hover:text-white">Solo Checkbox</span>
							<span class="text-[10px] text-zinc-500"
								>Evitar selección al hacer clic en la fila</span
							>
						</div>
					</label>
				{/if}

				<label class="flex items-center gap-3 cursor-pointer group">
					<input
						type="checkbox"
						checked={tabNavigation}
						onchange={(e) => handleTabNavChange(e.currentTarget.checked)}
						class={checkboxClass}
					/>
					<span class="text-xs text-zinc-200 group-hover:text-white">Navegación con Tab</span>
				</label>

				<label class="flex items-center gap-3 cursor-pointer group">
					<input
						type="checkbox"
						checked={autoSizeOnLoad}
						onchange={(e) => handleAutoSizeChange(e.currentTarget.checked)}
						class={checkboxClass}
					/>
					<div class="flex flex-col">
						<span class="text-xs text-zinc-200 group-hover:text-white">Auto-ajustar Columnas</span>
						<span class="text-[10px] text-zinc-500">Ajustar ancho al cargar datos</span>
					</div>
				</label>

				<label class="flex items-center gap-3 cursor-pointer group">
					<input
						type="checkbox"
						checked={showFloatingFilters}
						onchange={(e) => handleFloatingFiltersChange(e.currentTarget.checked)}
						class={checkboxClass}
					/>
					<div class="flex flex-col">
						<span class="text-xs text-zinc-200 group-hover:text-white">Filtros Flotantes</span>
						<span class="text-[10px] text-zinc-500">Input de filtro bajo cada columna</span>
					</div>
				</label>
			</div>

			<!-- Quick Filter -->
			<div class="space-y-2">
				<div class="flex items-center justify-between">
					<label class={labelClass} for="quickFilter">Búsqueda Rápida</label>
					<label class="flex items-center gap-2 cursor-pointer">
						<span class="text-[10px] text-zinc-500">
							{enableQuickFilter ? 'Activo' : 'Inactivo'}
						</span>
						<div
							class="relative inline-flex h-4 w-7 items-center rounded-full transition-colors {enableQuickFilter
								? 'bg-blue-600'
								: 'bg-zinc-700'}"
						>
							<input
								type="checkbox"
								checked={enableQuickFilter}
								onchange={(e) => handleQuickFilterToggle(e.currentTarget.checked)}
								class="sr-only"
							/>
							<span
								class="inline-block h-3 w-3 transform rounded-full bg-white transition-transform {enableQuickFilter
									? 'translate-x-3.5'
									: 'translate-x-0.5'}"
							></span>
						</div>
					</label>
				</div>

				{#if enableQuickFilter}
					<div class="relative">
						<Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500" />
						<input
							id="quickFilter"
							type="text"
							bind:value={quickFilterText}
							oninput={handleQuickFilterChange}
							placeholder="Filtrar en todo..."
							class="w-full bg-black/20 border border-white/10 rounded-lg pl-9 pr-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:border-blue-500/50 focus:ring-1 focus:ring-blue-500/20 transition-all"
						/>
						{#if quickFilterText}
							<button
								onclick={() => {
									quickFilterText = '';
									handleQuickFilterChange();
								}}
								class="absolute right-3 top-1/2 -translate-y-1/2 text-zinc-500 hover:text-white"
							>
								×
							</button>
						{/if}
					</div>
				{/if}
			</div>
		</div>
	</div>

	<div class="h-px bg-white/5 my-4"></div>

	<!-- Actions -->
	<div class="grid grid-cols-2 gap-3">
		<button
			onclick={clearAllFilters}
			class="flex items-center justify-center gap-2 py-2 rounded-md
        bg-red-500/10 border border-red-500/20 text-xs text-red-500 font-medium
        hover:bg-red-500/20 transition-colors h-[34px]"
		>
			<Filter size={14} />
			Limpiar filtros
			{#if activeFiltersCount > 0}
				<span class="px-1.5 py-0.5 text-[10px] bg-red-500/30 rounded-full">
					{activeFiltersCount}
				</span>
			{/if}
		</button>
		<button
			onclick={clearSort}
			class="flex items-center justify-center gap-2 py-2 rounded-md
        bg-black/20 border border-white/10 text-xs text-zinc-400 font-medium
        hover:bg-white/5 hover:text-white transition-colors h-[34px]"
		>
			<Trash2 size={14} />
			Resetaer orden
		</button>
	</div>
</div>
