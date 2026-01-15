<!-- src/lib/components/export/ExportAdvancedDialog.svelte -->
<script lang="ts">
	// @ts-nocheck - Svelte 5 runes not recognized by TS
	import {
		X,
		FileText,
		Download,
		RefreshCw,
		Link2,
		Unlink,
		Lock,
		Unlock,
		ChevronDown,
		Check
	} from 'lucide-svelte';
	import type { ExportOptions } from '$lib/types/export';
	import {
		MARGIN_UNITS,
		BANNER_COLORS,
		FONT_VARIANTS,
		PAPER_SIZES,
		type MarginUnit
	} from '$lib/logic/export/exportConstants';
	import { marginToCm } from '$lib/logic/export/exportUtils';
	import { exportPreview } from '$lib/api/export';
	import { currentUser } from '$lib/stores/auth';
	import { fade, fly } from 'svelte/transition';
	import PdfViewer from './PdfViewer.svelte';

	interface Props {
		onExport: (options: ExportOptions) => Promise<void>;
		onClose: () => void;
		onBack: () => void;
		columns: { id: string; name: string; selected: boolean }[];
		initialOptions: Partial<ExportOptions>;
		rows: Record<string, any>[];
	}

	let { onExport, onClose, onBack, columns, initialOptions = {}, rows }: Props = $props();

	// Extraer valores iniciales inmediatamente
	// svelte-ignore state_referenced_locally
	const {
		title: initTitle = 'Reporte',
		orientation: initOrientation = 'landscape',
		fontSize: initFontSize = 10,
		fontFamily: initFontFamily = 'Inter'
	} = initialOptions;

	// Estado de columnas
	let columnSelection = $state<{ id: string; name: string; selected: boolean }[]>([]);

	$effect(() => {
		columnSelection = columns.map((c) => ({ ...c, selected: c.selected }));
	});

	// Configuración
	let title = $state(initTitle);
	let orientation = $state<'portrait' | 'landscape'>(initOrientation);
	let fontSize = $state(initFontSize);
	let fontFamily = $state(initFontFamily);
	let marginTop = $state(20);
	let marginBottom = $state(20);
	let marginLeft = $state(15);
	let marginRight = $state(15);
	let marginUnit = $state<MarginUnit>('mm');
	let paperSize = $state<'us-letter' | 'a4' | 'legal'>('us-letter');
	let bannerColor = $state('#059669');

	// --- LOGIC FOR FIXED DROPDOWNS ---
	let activeDropdown = $state<string | null>(null);
	let dropdownPos = $state({ top: 0, left: 0, width: 0 });

	function toggleDropdown(e: MouseEvent, type: string) {
		e.stopPropagation();

		if (activeDropdown === type) {
			activeDropdown = null;
			return;
		}

		const button = e.currentTarget as HTMLElement;
		const rect = button.getBoundingClientRect();

		dropdownPos = {
			top: rect.bottom + 4, // 4px gap
			left: rect.left,
			width: rect.width
		};
		activeDropdown = type;
	}

	function closeDropdowns() {
		activeDropdown = null;
	}

	// Estado de vinculación de márgenes
	let linkVertical = $state(false);
	let linkHorizontal = $state(false);
	let linkAll = $state(false);

	function updateMargin(type: 'top' | 'bottom' | 'left' | 'right', value: number) {
		if (linkAll) {
			marginTop = marginBottom = marginLeft = marginRight = value;
			return;
		}
		if (linkVertical && (type === 'top' || type === 'bottom')) {
			marginTop = marginBottom = value;
			return;
		}
		if (linkHorizontal && (type === 'left' || type === 'right')) {
			marginLeft = marginRight = value;
			return;
		}
		if (type === 'top') marginTop = value;
		if (type === 'bottom') marginBottom = value;
		if (type === 'left') marginLeft = value;
		if (type === 'right') marginRight = value;
	}

	const marginStep = $derived(marginUnit === 'mm' ? 1 : marginUnit === 'pt' ? 5 : 0.1);
	const marginMax = $derived(
		marginUnit === 'mm' ? 50 : marginUnit === 'pt' ? 150 : marginUnit === 'in' ? 2 : 5
	);

	let isExporting = $state(false);
	let isGeneratingPreview = $state(false);
	let previewData = $state<Uint8Array | null>(null);
	let previewError = $state<string | null>(null);
	let previewTimeout: ReturnType<typeof setTimeout>;

	async function generatePreview() {
		if (previewTimeout) clearTimeout(previewTimeout);

		previewTimeout = setTimeout(async () => {
			isGeneratingPreview = true;
			previewError = null;

			try {
				const selectedHeaders = columnSelection.filter((c) => c.selected).map((c) => c.name);
				if (!rows || rows.length === 0) throw new Error('No hay datos disponibles');
				if (selectedHeaders.length === 0) throw new Error('Selecciona al menos una columna');

				const previewRows = rows.slice(0, 10);
				const result = await exportPreview({
					format: 'pdf',
					headers: selectedHeaders,
					rows: previewRows,
					title,
					orientation,
					fontSize,
					fontFamily,
					marginTop: marginToCm(marginTop, marginUnit),
					marginBottom: marginToCm(marginBottom, marginUnit),
					marginLeft: marginToCm(marginLeft, marginUnit),
					marginRight: marginToCm(marginRight, marginUnit),
					bannerColor,
					generatedBy: $currentUser?.nombreCompleto || '',
					showPreview: true
				});

				if (result.bytes && result.bytes.length > 0) {
					previewData = new Uint8Array(result.bytes);
				} else {
					previewError = result.message || 'Error generando PDF';
				}
			} catch (e: any) {
				previewError = e.message;
			} finally {
				isGeneratingPreview = false;
			}
		}, 500);
	}

	$effect(() => {
		(title,
			orientation,
			fontSize,
			fontFamily,
			marginTop,
			marginLeft,
			marginRight,
			marginBottom,
			bannerColor,
			paperSize,
			columnSelection.filter((c) => c.selected).length);
		generatePreview();
	});

	async function handleExport() {
		isExporting = true;
		try {
			await onExport({
				title: title.trim() || 'Reporte',
				orientation,
				fontSize,
				fontFamily,
				showPreview: false,
				columnIds: columnSelection.filter((c) => c.selected).map((c) => c.id),
				marginTop: marginToCm(marginTop, marginUnit),
				marginBottom: marginToCm(marginBottom, marginUnit),
				marginLeft: marginToCm(marginLeft, marginUnit),
				marginRight: marginToCm(marginRight, marginUnit),
				bannerColor,
				generatedBy: $currentUser?.nombreCompleto || ''
			});
			onClose();
		} catch (error) {
			alert('Error al exportar: ' + (error as Error).message);
		} finally {
			isExporting = false;
		}
	}

	const inputClass =
		'w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all';
	const inputSmClass =
		'bg-black/20 border border-white/10 rounded-lg px-2 py-1 text-xs text-white focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50';
	const labelClass = 'block text-xs font-medium text-secondary mb-1';
</script>

<svelte:window onresize={closeDropdowns} onclick={closeDropdowns} />

<div
	class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4"
	transition:fade={{ duration: 150 }}
	role="presentation"
	tabindex="-1"
>
	<!-- Modal Container -->
	<div
		class="bg-surface-2 rounded-xl border border-surface shadow-2xl flex flex-row w-[1100px] max-w-[98vw] h-[750px] max-h-[95vh] relative"
		transition:fly={{ y: 20, duration: 200 }}
		onclick={(e) => e.stopPropagation()}
		role="dialog"
		aria-modal="true"
	>
		<!-- Panel Izquierdo: Configuración -->
		<div class="w-[340px] flex flex-col border-r border-surface bg-surface-2">
			<!-- Header -->
			<div class="px-5 py-4 border-b border-surface flex items-center justify-between">
				<div class="flex items-center gap-2">
					<button
						onclick={onBack}
						disabled={isExporting}
						class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
					>
						←
					</button>
					<h2 class="text-base font-semibold text-primary">Configuración Avanzada</h2>
				</div>
				<button
					onclick={onClose}
					disabled={isExporting}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<X size={18} />
				</button>
			</div>

			<!-- Contenido scrolleable -->
			<div class="flex-1 overflow-y-auto p-5 space-y-5">
				<!-- Título -->
				<div>
					<label for="adv-title" class={labelClass}> Título </label>
					<input
						id="adv-title"
						type="text"
						bind:value={title}
						disabled={isExporting}
						class={inputClass}
					/>
				</div>

				<!-- Paper & Orientation -->
				<div class="grid grid-cols-2 gap-4">
					<div class="relative">
						<label for="adv-paper" class={labelClass}> Papel </label>
						<button
							type="button"
							onclick={(e) => toggleDropdown(e, 'paper')}
							disabled={isExporting}
							class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
							class:!border-blue-500={activeDropdown === 'paper'}
						>
							<span class="truncate"
								>{PAPER_SIZES.find((p) => p.id === paperSize)?.label || paperSize}</span
							>
							<ChevronDown size={14} class="text-secondary" />
						</button>
					</div>

					<div class="relative">
						<label for="adv-orientation" class={labelClass}> Orientación </label>
						<button
							type="button"
							onclick={(e) => toggleDropdown(e, 'orientation')}
							disabled={isExporting}
							class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
							class:!border-blue-500={activeDropdown === 'orientation'}
						>
							<span>{orientation === 'landscape' ? 'Horizontal' : 'Vertical'}</span>
							<ChevronDown size={14} class="text-secondary" />
						</button>
					</div>
				</div>

				<!-- Fuente -->
				<div class="relative">
					<label for="adv-font" class={labelClass}> Fuente </label>
					<button
						type="button"
						onclick={(e) => toggleDropdown(e, 'font')}
						disabled={isExporting}
						class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
						class:!border-blue-500={activeDropdown === 'font'}
					>
						<span>{fontFamily}</span>
						<ChevronDown size={14} class="text-secondary" />
					</button>
				</div>

				<!-- Color del Banner -->
				<div>
					<label for="adv-banner-color" class={labelClass}> Color del banner </label>
					<div class="flex items-center gap-3">
						<div class="relative w-[34px] h-[34px]">
							<div
								class="w-full h-full rounded-lg border border-surface"
								style="background-color: {bannerColor}"
							></div>
							<input
								type="color"
								bind:value={bannerColor}
								disabled={isExporting}
								class="absolute inset-0 opacity-0 cursor-pointer w-full h-full"
							/>
						</div>

						<div class="relative flex-1">
							<button
								type="button"
								onclick={(e) => toggleDropdown(e, 'banner')}
								disabled={isExporting}
								class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left"
								class:!border-blue-500={activeDropdown === 'banner'}
							>
								<div class="flex items-center gap-2">
									<div
										class="w-3 h-3 rounded-full border border-white/20"
										style="background-color: {bannerColor}"
									></div>
									<span class="truncate">
										{BANNER_COLORS.find((c) => c.id === bannerColor)?.label || 'Personalizado'}
									</span>
								</div>
								<ChevronDown size={14} class="text-secondary" />
							</button>
						</div>
					</div>
				</div>

				<!-- Tamaño de texto -->
				<div>
					<label for="adv-fontsize" class={labelClass}>
						Tamaño de texto ({fontSize}pt)
					</label>
					<input
						id="adv-fontsize"
						type="range"
						min="8"
						max="20"
						bind:value={fontSize}
						disabled={isExporting}
						class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-600"
					/>
				</div>

				<!-- Márgenes -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<span class={labelClass}> Márgenes </span>
						<div class="relative flex items-center gap-2">
							<span class="text-xs text-secondary">Unidad:</span>
							<button
								type="button"
								onclick={(e) => toggleDropdown(e, 'marginUnit')}
								class="px-2 py-0.5 text-xs rounded border border-surface bg-black/20 text-white focus:outline-none flex items-center gap-1 min-w-[50px] justify-between"
							>
								<span>{marginUnit}</span>
								<ChevronDown size={10} class="text-secondary" />
							</button>
						</div>
					</div>

					<div class="flex flex-col gap-2">
						<!-- Row 1: Vertical -->
						<div class="flex items-center justify-between gap-2">
							<div class="flex items-center gap-1 flex-1">
								<span class="text-xs text-secondary w-8">Arr:</span>
								<input
									type="number"
									step={marginStep}
									min="0"
									max={marginMax}
									bind:value={marginTop}
									oninput={(e) => updateMargin('top', +e.currentTarget.value)}
									class={inputSmClass}
								/>
							</div>
							<button
								class="p-1 rounded text-secondary hover:bg-surface-3 transition-colors"
								class:text-accent={linkVertical && !linkAll}
								class:opacity-30={linkAll}
								disabled={linkAll}
								onclick={() => (linkVertical = !linkVertical)}
							>
								{#if linkVertical && !linkAll}
									<Link2 size={14} />
								{:else}
									<Unlink size={14} />
								{/if}
							</button>
							<div class="flex items-center gap-1 flex-1">
								<span class="text-xs text-secondary w-8">Aba:</span>
								<input
									type="number"
									step={marginStep}
									min="0"
									max={marginMax}
									bind:value={marginBottom}
									oninput={(e) => updateMargin('bottom', +e.currentTarget.value)}
									class={inputSmClass}
								/>
							</div>
						</div>

						<!-- Row 2: Link All -->
						<div class="flex justify-center -my-1 relative z-10">
							<button
								class="p-1 rounded-full bg-surface-2 border border-surface text-secondary hover:text-white hover:border-white/50 transition-colors"
								class:text-accent={linkAll}
								class:border-accent={linkAll}
								onclick={() => {
									linkAll = !linkAll;
									if (linkAll) updateMargin('top', marginTop);
								}}
							>
								{#if linkAll}
									<Lock size={12} />
								{:else}
									<Unlock size={12} />
								{/if}
							</button>
						</div>

						<!-- Row 3: Horizontal -->
						<div class="flex items-center justify-between gap-2">
							<div class="flex items-center gap-1 flex-1">
								<span class="text-xs text-secondary w-8">Izq:</span>
								<input
									type="number"
									step={marginStep}
									min="0"
									max={marginMax}
									bind:value={marginLeft}
									oninput={(e) => updateMargin('left', +e.currentTarget.value)}
									class={inputSmClass}
								/>
							</div>
							<button
								class="p-1 rounded text-secondary hover:bg-surface-3 transition-colors"
								class:text-accent={linkHorizontal && !linkAll}
								class:opacity-30={linkAll}
								disabled={linkAll}
								onclick={() => (linkHorizontal = !linkHorizontal)}
							>
								{#if linkHorizontal && !linkAll}
									<Link2 size={14} />
								{:else}
									<Unlink size={14} />
								{/if}
							</button>
							<div class="flex items-center gap-1 flex-1">
								<span class="text-xs text-secondary w-8">Der:</span>
								<input
									type="number"
									step={marginStep}
									min="0"
									max={marginMax}
									bind:value={marginRight}
									oninput={(e) => updateMargin('right', +e.currentTarget.value)}
									class={inputSmClass}
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- Columnas -->
				<div>
					<span class="block text-xs font-medium text-secondary mb-2">
						Columnas ({columnSelection.filter((c) => c.selected).length}/{columnSelection.length})
					</span>
					<div
						class="max-h-32 overflow-y-auto bg-black/20 border border-surface rounded-lg p-2 space-y-1"
					>
						{#each columnSelection as col}
							<label
								class="flex items-center gap-2 text-xs cursor-pointer hover:bg-white/5 p-1 rounded"
							>
								<input
									type="checkbox"
									bind:checked={col.selected}
									class="w-3.5 h-3.5 rounded border-white/20 bg-transparent text-accent focus:ring-accent"
								/>
								<span class="text-white truncate">{col.name}</span>
							</label>
						{/each}
					</div>
				</div>
			</div>

			<!-- Footer -->
			<div class="px-5 py-4 border-t border-surface flex gap-3 bg-surface-1">
				<button
					onclick={onBack}
					disabled={isExporting}
					class="flex-1 px-3 py-2.5 text-sm font-medium rounded-lg border-2 border-surface text-secondary hover:border-white/60 hover:text-white/80 transition-all"
				>
					Cancelar
				</button>
				<button
					onclick={handleExport}
					disabled={isExporting || columnSelection.filter((c) => c.selected).length === 0}
					class="flex-1 px-3 py-2.5 text-sm font-medium rounded-lg border-2 border-surface text-secondary hover:border-accent hover:text-accent transition-all flex items-center justify-center gap-2 disabled:opacity-50"
				>
					{#if isExporting}
						<div
							class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
						></div>
					{:else}
						<Download size={16} />
					{/if}
					Exportar
				</button>
			</div>
		</div>

		<!-- Panel Derecho: Vista Previa -->
		<div class="flex-1 min-w-0 flex flex-col bg-black/30 overflow-hidden isolate">
			<!-- Header Preview -->
			<div class="px-5 py-3 border-b border-surface flex items-center justify-between bg-surface-2">
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium text-primary">Vista Previa</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-surface-3 text-secondary"
						>10 registros max</span
					>
				</div>
				<button
					onclick={generatePreview}
					disabled={isGeneratingPreview}
					class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
				>
					<RefreshCw size={16} class={isGeneratingPreview ? 'animate-spin' : ''} />
				</button>
			</div>

			<!-- PdfViewer -->
			<div class="flex-1 relative overflow-hidden bg-[#2e3136]">
				{#if previewData}
					<PdfViewer pdfData={previewData} onError={(err) => (previewError = err)} />
				{:else if isGeneratingPreview}
					<div class="absolute inset-0 flex items-center justify-center">
						<div class="text-center">
							<div
								class="w-8 h-8 border-2 border-surface border-t-accent rounded-full animate-spin mx-auto mb-2"
							></div>
							<p class="text-xs text-secondary">Generando preview...</p>
						</div>
					</div>
				{:else if previewError}
					<div class="absolute inset-0 flex items-center justify-center">
						<div class="text-center">
							<p class="text-sm text-red-500 font-medium">{previewError}</p>
							<button onclick={generatePreview} class="mt-2 text-xs text-accent hover:underline">
								Reintentar
							</button>
						</div>
					</div>
				{:else}
					<div class="absolute inset-0 flex items-center justify-center text-center text-secondary">
						<div>
							<FileText size={48} class="mx-auto mb-2 opacity-30" />
							<p class="text-sm font-medium">Vista previa del documento</p>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- DROPDOWNS: FIXED POSITIONING LAYER -->
	{#if activeDropdown}
		<div
			class="fixed z-[100] bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-y-auto flex flex-col"
			style="top: {dropdownPos.top}px; left: {dropdownPos.left}px; width: {dropdownPos.width}px; max-height: 300px;"
			transition:fly={{ y: -5, duration: 200 }}
			onclick={(e) => e.stopPropagation()}
		>
			{#if activeDropdown === 'paper'}
				<div class="p-1">
					{#each PAPER_SIZES as size}
						<button
							class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
							onclick={() => {
								paperSize = size.id;
								closeDropdowns();
							}}
						>
							<span>{size.label}</span>
							{#if paperSize === size.id}
								<Check size={14} class="text-white" />
							{/if}
						</button>
					{/each}
				</div>
			{:else if activeDropdown === 'orientation'}
				<div class="p-1">
					{#each [{ value: 'landscape', label: 'Horizontal' }, { value: 'portrait', label: 'Vertical' }] as opt}
						<button
							class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
							onclick={() => {
								orientation = opt.value as any;
								closeDropdowns();
							}}
						>
							<span>{opt.label}</span>
							{#if orientation === opt.value}
								<Check size={14} class="text-white" />
							{/if}
						</button>
					{/each}
				</div>
			{:else if activeDropdown === 'font'}
				<div class="p-1">
					{#each Object.entries(FONT_VARIANTS) as [family, variants]}
						<div
							class="px-3 py-1.5 text-xs text-white/50 bg-black/20 font-semibold uppercase tracking-wider rounded-md mb-0.5 mt-1 first:mt-0"
						>
							{family}
						</div>
						{#each variants as variant}
							<button
								class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group pl-4"
								onclick={() => {
									fontFamily = variant;
									closeDropdowns();
								}}
							>
								<span>{variant}</span>
								{#if fontFamily === variant}
									<Check size={14} class="text-white" />
								{/if}
							</button>
						{/each}
					{/each}
				</div>
			{:else if activeDropdown === 'banner'}
				<div class="p-1">
					{#each BANNER_COLORS as color}
						<button
							class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
							onclick={() => {
								bannerColor = color.id;
								closeDropdowns();
							}}
						>
							<div class="flex items-center gap-2">
								<div
									class="w-3 h-3 rounded-full border border-white/20"
									style="background-color: {color.id}"
								></div>
								<span>{color.label}</span>
							</div>
							{#if bannerColor === color.id}
								<Check size={14} class="text-white" />
							{/if}
						</button>
					{/each}
					<div class="h-px bg-white/10 my-1"></div>
					<div class="px-3 py-1 text-xs text-white/50 text-center">
						Usa el selector de color para personalizar
					</div>
				</div>
			{:else if activeDropdown === 'marginUnit'}
				<div class="p-1">
					{#each MARGIN_UNITS as unit}
						<button
							class="w-full text-left px-3 py-1.5 text-xs text-gray-300 hover:bg-white/10 rounded-md transition-colors"
							onclick={() => {
								marginUnit = unit.id;
								closeDropdowns();
							}}
						>
							{unit.id}
						</button>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	/* Focus Override Global */
	select:focus,
	input:focus {
		border-color: rgba(59, 130, 246, 0.5) !important;
		box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
		outline: none !important;
	}
</style>
