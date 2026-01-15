<!-- src/lib/components/export/ExportDialog.svelte -->
<script lang="ts">
	// @ts-nocheck - Svelte 5 runes not recognized by TS
	import {
		X,
		FileText,
		Table2,
		FileSpreadsheet,
		Download,
		Settings,
		Columns,
		ChevronDown,
		Check
	} from 'lucide-svelte';
	import type { ExportOptions } from '$lib/types/export';
	import { currentUser } from '$lib/stores/auth';
	import { slide, fade, fly } from 'svelte/transition';
	import ExportAdvancedDialog from './ExportAdvancedDialog.svelte';

	interface Props {
		onExport: (format: 'pdf' | 'excel' | 'csv', options: ExportOptions) => Promise<void>;
		onClose: () => void;
		availableFormats?: string[];
		columns?: { id: string; name: string; selected: boolean }[];
		rows?: Record<string, any>[];
		headers?: string[];
	}

	let {
		onExport,
		onClose,
		availableFormats = ['pdf', 'excel', 'csv'],
		columns = [],
		rows = [],
		headers = []
	}: Props = $props();

	// Estado de columnas
	let columnSelection = $state<{ id: string; name: string; selected: boolean }[]>([]);

	$effect(() => {
		columnSelection = columns
			.filter((c) => c.id !== 'ag-Grid-ControlsColumn')
			.map((c) => ({ ...c, selected: c.selected }));
	});

	let showColumnSelector = $state(false);
	let showAdvancedDialog = $state(false);

	let allColumnsSelected = $derived(columnSelection.every((c) => c.selected));

	function toggleAllColumns() {
		const newState = !allColumnsSelected;
		columnSelection = columnSelection.map((c) => ({
			...c,
			selected: newState
		}));
	}

	// Estado principal
	let selectedFormat = $state<'pdf' | 'excel' | 'csv'>('pdf');
	let title = $state('Reporte');
	let orientation = $state<'portrait' | 'landscape'>('landscape');
	let delimiter = $state<'comma' | 'semicolon' | 'tab' | 'pipe'>('comma');
	let includeBom = $state(true);
	let isExporting = $state(false);

	// Dropdown States
	let showOrientationDropdown = $state(false);
	let showDelimiterDropdown = $state(false);

	// Formatos disponibles
	const formats = $derived([
		{
			id: 'pdf' as const,
			label: 'PDF',
			icon: FileText,
			available: availableFormats.includes('pdf')
		},
		{
			id: 'excel' as const,
			label: 'Excel',
			icon: FileSpreadsheet,
			available: availableFormats.includes('excel')
		},
		{
			id: 'csv' as const,
			label: 'CSV',
			icon: Table2,
			available: true
		}
	]);

	async function handleExport() {
		isExporting = true;

		try {
			const options: ExportOptions = {
				title: title.trim() || 'Reporte',
				orientation: selectedFormat === 'pdf' ? orientation : undefined,
				delimiter: selectedFormat === 'csv' ? delimiter : undefined,
				includeBom: selectedFormat === 'csv' ? includeBom : undefined,
				showPreview: false,
				columnIds: columnSelection.filter((c) => c.selected).map((c) => c.id),
				generatedBy: $currentUser?.nombreCompleto || ''
			};

			await onExport(selectedFormat, options);
			onClose();
		} catch (error) {
			console.error('Error exportando:', error);
			alert('Error al exportar: ' + (error as Error).message);
		} finally {
			isExporting = false;
		}
	}

	async function handleAdvancedExport(options: ExportOptions) {
		await onExport('pdf', options);
	}

	// Standard styles from patterns
	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
</script>

{#if showAdvancedDialog}
	<ExportAdvancedDialog
		onExport={handleAdvancedExport}
		{onClose}
		onBack={() => (showAdvancedDialog = false)}
		columns={columnSelection}
		initialOptions={{ title, orientation }}
		{rows}
		{headers}
	/>
{:else}
	<div
		class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
		transition:fade={{ duration: 150 }}
		role="presentation"
		tabindex="-1"
	>
		<div
			class="bg-surface-2 rounded-xl border border-surface shadow-2xl flex flex-row overflow-hidden max-h-[85vh]"
			transition:fly={{ y: 20, duration: 200 }}
			onclick={(e) => e.stopPropagation()}
			onkeydown={() => {}}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<!-- Main Panel -->
			<div class="w-full max-w-sm flex flex-col">
				<!-- Header -->
				<div
					class="px-5 py-4 border-b border-surface flex items-center justify-between bg-surface-2"
				>
					<div>
						<h2 class="text-base font-semibold text-primary">Exportar</h2>
						<p class="text-xs text-secondary mt-0.5">
							{columnSelection.filter((c) => c.selected).length} columnas
						</p>
					</div>
					<button
						onclick={onClose}
						disabled={isExporting}
						class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors disabled:opacity-50"
						aria-label="Cerrar"
					>
						<X size={18} />
					</button>
				</div>

				<!-- Content -->
				<div class="flex-1 overflow-y-auto p-5 space-y-5">
					<!-- Formato -->
					<div>
						<span class="block text-xs font-medium text-secondary mb-2 uppercase tracking-wide">
							Formato
						</span>
						<div class="flex gap-2">
							{#each formats as format}
								{@const Icon = format.icon}
								<button
									onclick={() => (selectedFormat = format.id)}
									disabled={!format.available || isExporting}
									class="flex-1 px-3 py-2.5 rounded-lg border-2 transition-all flex items-center justify-center gap-2 font-medium text-sm
                    {selectedFormat === format.id
										? 'border-accent bg-accent/10 text-accent'
										: 'border-surface text-secondary hover:border-surface-3 hover:text-primary'}
                    {!format.available ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}
                    disabled:opacity-50"
								>
									<Icon size={16} />
									<span>{format.label}</span>
								</button>
							{/each}
						</div>
					</div>

					<!-- Título -->
					<div>
						<label for="export-title" class={labelClass}> Título del documento </label>
						<input
							id="export-title"
							type="text"
							bind:value={title}
							disabled={isExporting}
							placeholder="Ej: Reporte Mensual"
							class={inputClass}
						/>
					</div>

					<!-- Opciones PDF -->
					{#if selectedFormat === 'pdf'}
						<div
							class="space-y-4 p-4 bg-black/20 border border-white/5 rounded-lg"
							transition:slide={{ duration: 150 }}
						>
							<div class="relative">
								<label for="orientation" class={labelClass}> Orientación </label>
								<button
									id="orientation"
									type="button"
									onclick={() => (showOrientationDropdown = !showOrientationDropdown)}
									disabled={isExporting}
									class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
									class:!border-blue-500={showOrientationDropdown}
								>
									<span>{orientation === 'landscape' ? 'Horizontal' : 'Vertical'}</span>
									<ChevronDown size={16} class="text-secondary" />
								</button>

								{#if showOrientationDropdown}
									<div
										class="fixed inset-0 z-40"
										onclick={() => (showOrientationDropdown = false)}
										role="presentation"
									></div>
									<div
										class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
										transition:fly={{ y: -5, duration: 200 }}
									>
										{#each [{ value: 'landscape', label: 'Horizontal' }, { value: 'portrait', label: 'Vertical' }] as opt}
											<button
												type="button"
												onclick={() => {
													orientation = opt.value as any;
													showOrientationDropdown = false;
												}}
												class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
											>
												<span>{opt.label}</span>
												{#if orientation === opt.value}
													<Check size={14} class="text-white" />
												{/if}
											</button>
										{/each}
									</div>
								{/if}
							</div>

							<!-- Botón Avanzado -->
							<button
								onclick={() => (showAdvancedDialog = true)}
								class="w-full flex items-center justify-center gap-2 px-3 py-2 text-sm font-medium rounded-lg border border-surface bg-surface-1 text-secondary hover:text-primary hover:border-white/20 transition-all"
							>
								<Settings size={14} />
								Configuración Avanzada
							</button>
						</div>
					{/if}

					<!-- Opciones CSV -->
					{#if selectedFormat === 'csv'}
						<div
							class="space-y-4 p-4 bg-black/20 border border-white/5 rounded-lg"
							transition:slide={{ duration: 150 }}
						>
							<div class="relative">
								<label for="delimiter" class={labelClass}> Delimitador </label>
								<button
									id="delimiter"
									type="button"
									onclick={() => (showDelimiterDropdown = !showDelimiterDropdown)}
									disabled={isExporting}
									class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
									class:!border-blue-500={showDelimiterDropdown}
								>
									<span>
										{delimiter === 'comma'
											? 'Coma (,)'
											: delimiter === 'semicolon'
												? 'Punto y coma (;)'
												: delimiter === 'tab'
													? 'Tabulación'
													: 'Barra (|)'}
									</span>
									<ChevronDown size={16} class="text-secondary" />
								</button>

								{#if showDelimiterDropdown}
									<div
										class="fixed inset-0 z-40"
										onclick={() => (showDelimiterDropdown = false)}
										role="presentation"
									></div>
									<div
										class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
										transition:fly={{ y: -5, duration: 200 }}
									>
										{#each [{ value: 'comma', label: 'Coma (,)' }, { value: 'semicolon', label: 'Punto y coma (;)' }, { value: 'tab', label: 'Tabulación' }, { value: 'pipe', label: 'Barra (|)' }] as opt}
											<button
												type="button"
												onclick={() => {
													delimiter = opt.value as any;
													showDelimiterDropdown = false;
												}}
												class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
											>
												<span>{opt.label}</span>
												{#if delimiter === opt.value}
													<Check size={14} class="text-white" />
												{/if}
											</button>
										{/each}
									</div>
								{/if}
							</div>

							<label class="flex items-center gap-3 cursor-pointer">
								<div class="relative flex items-center">
									<input
										type="checkbox"
										bind:checked={includeBom}
										disabled={isExporting}
										class="peer sr-only"
									/>
									<div
										class="w-9 h-5 bg-surface-3 rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"
									></div>
								</div>
								<span class="text-sm text-secondary">UTF-8 BOM (Excel compatible)</span>
							</label>
						</div>
					{/if}

					<!-- Toggle columnas -->
					{#if columns.length > 0}
						<button
							onclick={() => (showColumnSelector = !showColumnSelector)}
							class="w-full flex items-center justify-between px-3 py-2.5 bg-surface-1 border border-surface rounded-lg hover:border-white/20 transition-colors"
						>
							<div class="flex items-center gap-2">
								<Columns size={16} class="text-secondary" />
								<span class="text-sm text-primary font-medium">Selección de Columnas</span>
							</div>
							<span
								class="text-xs font-medium px-2 py-0.5 rounded-full bg-surface-3 text-secondary"
							>
								{columnSelection.filter((c) => c.selected).length}/{columnSelection.length}
							</span>
						</button>
					{/if}
				</div>

				<!-- Footer -->
				<div
					class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
				>
					<button
						onclick={onClose}
						disabled={isExporting}
						class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm disabled:opacity-50"
					>
						Cancelar
					</button>
					<button
						onclick={handleExport}
						disabled={isExporting || columnSelection.filter((c) => c.selected).length === 0}
						class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-accent hover:text-accent text-sm disabled:opacity-50 flex items-center gap-2"
					>
						{#if isExporting}
							<div
								class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
							></div>
							Exportando...
						{:else}
							<Download size={16} />
							Exportar
						{/if}
					</button>
				</div>
			</div>

			<!-- Column Selector Panel -->
			{#if showColumnSelector}
				<div
					class="w-72 border-l border-surface flex flex-col bg-surface-2"
					transition:fly={{ x: -20, duration: 150 }}
				>
					<div class="px-4 py-3 border-b border-surface flex items-center justify-between">
						<span class="text-xs font-medium text-secondary uppercase tracking-wide">
							Columnas Disponibles
						</span>
						<button
							onclick={toggleAllColumns}
							disabled={isExporting}
							class="text-xs font-medium text-accent hover:text-accent/80 transition-colors disabled:opacity-50"
						>
							{allColumnsSelected ? 'Deseleccionar todo' : 'Seleccionar todo'}
						</button>
					</div>

					<div class="flex-1 overflow-y-auto p-2">
						{#each columnSelection as col}
							<button
								onclick={() => (col.selected = !col.selected)}
								disabled={isExporting}
								class="group w-full px-3 py-2 text-left text-sm transition-all disabled:opacity-50 flex items-center gap-3 rounded-lg hover:bg-surface-3"
							>
								<div
									class="flex-shrink-0 w-4 h-4 rounded border transition-all flex items-center justify-center
                      {col.selected
										? 'bg-accent border-accent'
										: 'border-white/30 group-hover:border-white/50'}"
								>
									{#if col.selected}
										<svg
											class="w-3 h-3 text-white"
											viewBox="0 0 16 16"
											fill="none"
											stroke="currentColor"
											stroke-width="3"
											stroke-linecap="round"
											stroke-linejoin="round"
										>
											<polyline points="3,8 6,11 13,4" />
										</svg>
									{/if}
								</div>
								<span class="{col.selected ? 'text-primary' : 'text-secondary'} truncate"
									>{col.name}</span
								>
							</button>
						{/each}
					</div>

					{#if columnSelection.filter((c) => c.selected).length === 0}
						<div class="p-4 border-t border-surface bg-yellow-500/10">
							<p class="text-xs text-yellow-500 text-center font-medium">
								⚠️ Selecciona al menos una columna
							</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	/* Focus Override Global */
	select:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}
</style>
