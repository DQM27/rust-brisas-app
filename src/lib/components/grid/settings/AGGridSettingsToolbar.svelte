<script lang="ts">
	import type { GridId, ToolbarContext } from '$lib/types/agGrid';
	import type { GridApi } from '@ag-grid-community/core';
	import { agGridSettings } from '$lib/stores/agGridSettings.svelte';
	import { getGridConfig } from '$lib/config/agGridConfigs';
	import { Eye, EyeOff, GripVertical, RotateCcw, Tally5 } from 'lucide-svelte';
	import SelectDropdown from '$lib/components/shared/SelectDropdown.svelte';

	interface Props {
		gridId: GridId;
		gridApi: GridApi | null;
		customButtons?: {
			default?: any[];
			singleSelect?: any[];
			multiSelect?: any[];
		};
	}

	let { gridId, customButtons }: Props = $props();

	// Estado
	let selectedContext = $state<ToolbarContext>('default');
	let draggedIndex = $state<number | null>(null);

	// Configuración
	const gridConfig = $derived(getGridConfig(gridId));

	const contextOptions: { value: ToolbarContext; label: string }[] = [
		{ value: 'default', label: 'Sin selección (Por defecto)' },
		{ value: 'singleSelect', label: 'Selección Simple (1 fila)' },
		{ value: 'multiSelect', label: 'Selección Múltiple (+1 fila)' }
	];

	const contextDescriptions: Record<ToolbarContext, string> = {
		default: 'Botones visibles cuando no hay filas seleccionadas',
		singleSelect: 'Botones visibles cuando hay exactamente una fila seleccionada',
		multiSelect: 'Botones visibles cuando hay múltiples filas seleccionadas'
	};

	// Límite de botones
	const buttonLimit = $derived(agGridSettings.getButtonLimit(selectedContext));

	// Botones disponibles
	const availableButtons = $derived.by(() => {
		const baseButtons = gridConfig.availableButtons[selectedContext] || [];
		const customForContext = customButtons?.[selectedContext] || [];

		const customAsDefinitions = customForContext.map((btn) => ({
			id: btn.id,
			label: btn.label,
			icon: btn.icon,
			variant: btn.variant,
			tooltip: btn.tooltip,
			category: 'custom' as const
		}));

		return [...baseButtons, ...customAsDefinitions];
	});

	// Configuración actual
	const buttonsConfig = $derived(agGridSettings.getButtonsConfig(gridId, selectedContext));

	interface ButtonItem {
		id: string;
		label: string;
		icon?: any;
		category?: string;
		visible: boolean;
	}

	const buttons = $derived.by((): ButtonItem[] => {
		const { order, hidden } = buttonsConfig;
		const orderSet = new Set(order);
		const hiddenSet = new Set(hidden);

		const orderedIds = [...order];
		availableButtons.forEach((btn) => {
			if (!orderSet.has(btn.id) && !hiddenSet.has(btn.id)) {
				orderedIds.push(btn.id);
			}
		});

		hidden.forEach((id) => {
			if (!orderedIds.includes(id)) {
				orderedIds.push(id);
			}
		});

		return orderedIds
			.map((id) => {
				const def = availableButtons.find((b) => b.id === id);
				if (!def) return null;
				return {
					id,
					label: def.label,
					icon: def.icon,
					category: def.category,
					visible: !hiddenSet.has(id)
				} as ButtonItem;
			})
			.filter((b): b is ButtonItem => b !== null);
	});

	const visibleCount = $derived(buttons.filter((b) => b.visible).length);
	const isAtLimit = $derived(visibleCount >= buttonLimit);
	const isOverLimit = $derived(visibleCount > buttonLimit);

	// Handlers
	function toggleVisibility(buttonId: string) {
		const button = buttons.find((b) => b.id === buttonId);
		if (!button) return;

		const currentHidden = [...buttonsConfig.hidden];

		if (button.visible) {
			agGridSettings.setHiddenButtons(gridId, selectedContext, [...currentHidden, buttonId]);
		} else {
			if (isAtLimit) return;
			agGridSettings.setHiddenButtons(
				gridId,
				selectedContext,
				currentHidden.filter((id) => id !== buttonId)
			);
		}
	}

	function handleDragStart(e: DragEvent, index: number) {
		e.dataTransfer?.setData('text/plain', String(index));
		e.dataTransfer!.effectAllowed = 'move';
		draggedIndex = index;
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		e.dataTransfer!.dropEffect = 'move';
		if (draggedIndex === null || draggedIndex === index) return;

		const currentOrder = buttons.map((b) => b.id);
		const [draggedId] = currentOrder.splice(draggedIndex, 1);
		currentOrder.splice(index, 0, draggedId);

		agGridSettings.setButtonOrder(gridId, selectedContext, currentOrder);
		draggedIndex = index;
	}

	function handleDragEnd() {
		draggedIndex = null;
	}

	function resetToDefault() {
		agGridSettings.setButtonOrder(gridId, selectedContext, []);
		agGridSettings.setHiddenButtons(gridId, selectedContext, []);
	}

	function handleContextChange(ctx: ToolbarContext) {
		selectedContext = ctx;
	}

	const sectionClass = 'space-y-4 p-1';
</script>

<div class={sectionClass}>
	<!-- Context Selection -->
	<div class="space-y-2">
		<SelectDropdown
			label="Contexto de Toolbar"
			value={selectedContext}
			options={contextOptions}
			onSelect={handleContextChange}
		/>
		<p class="text-[10px] text-zinc-500 px-1">
			{contextDescriptions[selectedContext]}
		</p>
	</div>

	<!-- Counter & Status -->
	<div
		class="flex items-center justify-between p-2.5 rounded-lg border
      {isOverLimit ? 'bg-red-500/10 border-red-500/20' : 'bg-black/20 border-white/5'}"
	>
		<div class="flex items-center gap-3">
			<div class="p-1 rounded-md {isOverLimit ? 'bg-red-500/20' : 'bg-green-500/20'}">
				<Tally5
					size={14}
					class={isOverLimit ? 'text-red-500' : isAtLimit ? 'text-yellow-500' : 'text-green-500'}
				/>
			</div>
			<div>
				<span class="text-xs text-zinc-400 block">Botones visibles</span>
				<span class="text-xs font-medium text-white">
					{visibleCount} de {buttonLimit} permitidos
				</span>
			</div>
		</div>

		<div class="flex gap-0.5">
			{#each Array(buttonLimit) as _, i}
				<div class="w-1 h-3 rounded-full {i < visibleCount ? 'bg-blue-500' : 'bg-zinc-700'}"></div>
			{/each}
		</div>
	</div>

	<!-- Draggable List -->
	<div>
		<h3 class="text-xs font-medium text-zinc-400 mb-2 ml-0.5">Orden y Visibilidad</h3>
		<div class="space-y-1 max-h-[300px] overflow-y-auto pr-1 custom-scrollbar" role="list">
			{#each buttons as button, index (button.id)}
				{@const canToggle = button.visible || !isAtLimit}

				<div
					draggable="true"
					ondragstart={(e) => handleDragStart(e, index)}
					ondragover={(e) => handleDragOver(e, index)}
					ondragend={handleDragEnd}
					class="group flex items-center gap-2 p-2 rounded-md transition-all cursor-grab active:cursor-grabbing border
            {draggedIndex === index
						? 'opacity-50 border-blue-500/50 bg-blue-500/10'
						: 'bg-black/20 border-white/5 hover:border-white/10'}
            {!canToggle && !button.visible ? 'opacity-50' : ''}"
					role="listitem"
				>
					<!-- Grip -->
					<div class="text-zinc-600 group-hover:text-zinc-400 cursor-move">
						<GripVertical size={12} />
					</div>

					<!-- Icon -->
					<div class="w-6 flex justify-center">
						{#if button.icon}
							{@const Icon = button.icon}
							<Icon size={14} class="text-zinc-400" />
						{/if}
					</div>

					<!-- Label -->
					<span
						class="flex-1 text-xs truncate
                {button.visible ? 'text-zinc-200' : 'text-zinc-500'}"
					>
						{button.label}
					</span>

					<!-- Toggle -->
					<button
						onclick={() => toggleVisibility(button.id)}
						disabled={!canToggle && !button.visible}
						class="p-1 rounded transition-colors
                {button.visible
							? 'text-green-500 hover:bg-green-500/10'
							: 'text-zinc-600 hover:bg-zinc-700'}
                disabled:cursor-not-allowed"
					>
						{#if button.visible}
							<Eye size={14} />
						{:else}
							<EyeOff size={14} />
						{/if}
					</button>
				</div>
			{/each}
		</div>
	</div>

	<!-- Reset -->
	<button
		onclick={resetToDefault}
		class="w-full flex items-center justify-center gap-2 py-2 rounded-md
      bg-black/20 border border-white/10 text-xs text-zinc-400 font-medium
      hover:bg-white/5 hover:text-white transition-colors h-[34px]"
	>
		<RotateCcw size={14} />
		Restaurar orden
	</button>
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.1);
		border-radius: 2px;
	}
</style>
