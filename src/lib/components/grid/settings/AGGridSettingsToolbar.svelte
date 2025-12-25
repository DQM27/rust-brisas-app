<script lang="ts">
  // @ts-nocheck - Svelte 5 runes are not recognized by TS
  import type {
    GridId,
    ToolbarContext,
    ToolbarButtonDefinition,
  } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { getGridConfig } from "$lib/config/agGridConfigs";
  import {
    Eye,
    EyeOff,
    GripVertical,
    RotateCcw,
    AlertCircle,
    ChevronDown,
    Layout,
    MousePointerClick,
    Tally5,
    Info,
  } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
    customButtons?: {
      default?: any[];
      singleSelect?: any[];
      multiSelect?: any[];
    };
  }

  let { gridId, gridApi, customButtons }: Props = $props();

  // Estado
  let selectedContext = $state<ToolbarContext>("default");
  let draggedIndex = $state<number | null>(null);
  let showContextMenu = $state(false);

  // Configuración
  const gridConfig = $derived(getGridConfig(gridId));

  // Labels de contexto
  const contextLabels: Record<ToolbarContext, string> = {
    default: "Sin selección",
    singleSelect: "Selección simple",
    multiSelect: "Selección múltiple",
  };

  const contextDescriptions: Record<ToolbarContext, string> = {
    default: "Botones visibles cuando no hay filas seleccionadas",
    singleSelect: "Botones visibles cuando hay una fila seleccionada",
    multiSelect: "Botones visibles cuando hay múltiples filas seleccionadas",
  };

  // Límite de botones
  const buttonLimit = $derived(agGridSettings.getButtonLimit(selectedContext));

  // Botones disponibles para el contexto
  const availableButtons = $derived.by(() => {
    const baseButtons = gridConfig.availableButtons[selectedContext] || [];
    const customForContext = customButtons?.[selectedContext] || [];

    const customAsDefinitions = customForContext.map((btn) => ({
      id: btn.id,
      label: btn.label,
      icon: btn.icon,
      variant: btn.variant,
      tooltip: btn.tooltip,
      category: "custom" as const,
    }));

    return [...baseButtons, ...customAsDefinitions];
  });

  // Config actual de botones
  const buttonsConfig = $derived(
    agGridSettings.getButtonsConfig(gridId, selectedContext),
  );

  // Lista de botones ordenada
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

    // Botones en orden + los que no están en orden
    const orderedIds = [...order];
    availableButtons.forEach((btn) => {
      if (!orderSet.has(btn.id) && !hiddenSet.has(btn.id)) {
        orderedIds.push(btn.id);
      }
    });

    // Agregar hidden al final
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
          visible: !hiddenSet.has(id),
        } as ButtonItem;
      })
      .filter((b): b is ButtonItem => b !== null);
  });

  // Contadores
  const visibleCount = $derived(buttons.filter((b) => b.visible).length);
  const isAtLimit = $derived(visibleCount >= buttonLimit);
  const isOverLimit = $derived(visibleCount > buttonLimit);

  // Toggle visibilidad
  function toggleVisibility(buttonId: string) {
    const button = buttons.find((b) => b.id === buttonId);
    if (!button) return;

    const currentHidden = [...buttonsConfig.hidden];

    if (button.visible) {
      // Ocultar
      agGridSettings.setHiddenButtons(gridId, selectedContext, [
        ...currentHidden,
        buttonId,
      ]);
    } else {
      // Mostrar - verificar límite
      if (isAtLimit) return;
      agGridSettings.setHiddenButtons(
        gridId,
        selectedContext,
        currentHidden.filter((id) => id !== buttonId),
      );
    }
  }

  // Drag & Drop
  function handleDragStart(e: DragEvent, index: number) {
    // Required for native HTML5 drag-and-drop to work
    e.dataTransfer?.setData("text/plain", String(index));
    e.dataTransfer!.effectAllowed = "move";
    draggedIndex = index;
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    e.dataTransfer!.dropEffect = "move";
    if (draggedIndex === null || draggedIndex === index) return;

    // Reordenar visualmente
    const currentOrder = buttons.map((b) => b.id);
    const [draggedId] = currentOrder.splice(draggedIndex, 1);
    currentOrder.splice(index, 0, draggedId);

    // Guardar nuevo orden
    agGridSettings.setButtonOrder(gridId, selectedContext, currentOrder);
    draggedIndex = index;
  }

  function handleDragEnd() {
    draggedIndex = null;
  }

  // Reset
  function resetToDefault() {
    agGridSettings.setButtonOrder(gridId, selectedContext, []);
    agGridSettings.setHiddenButtons(gridId, selectedContext, []);
  }

  // Cambiar contexto
  function handleContextChange(ctx: ToolbarContext) {
    selectedContext = ctx;
    showContextMenu = false;
  }
</script>

<div class="space-y-4">
  <!-- Context Selector -->
  <div class="relative">
    <button
      onclick={() => (showContextMenu = !showContextMenu)}
      class="w-full flex items-center justify-between p-3 rounded-lg
        bg-[#252526] border border-white/10 hover:border-white/20 transition-colors"
    >
      <div class="text-left flex items-center gap-3">
        <div class="p-2 rounded-lg bg-blue-500/10">
          <MousePointerClick size={18} class="text-blue-400" />
        </div>
        <div>
          <p class="text-sm font-medium text-white">
            {contextLabels[selectedContext]}
          </p>
          <p class="text-xs text-gray-500">
            {contextDescriptions[selectedContext]}
          </p>
        </div>
      </div>
      <ChevronDown
        size={16}
        class="text-gray-400 transition-transform {showContextMenu
          ? 'rotate-180'
          : ''}"
      />
    </button>

    {#if showContextMenu}
      <div
        class="absolute top-full left-0 right-0 mt-1 z-10 rounded-lg
          bg-[#252526] border border-white/10 shadow-xl overflow-hidden"
      >
        {#each Object.entries(contextLabels) as [ctx, label]}
          <button
            onclick={() => handleContextChange(ctx as ToolbarContext)}
            class="w-full px-4 py-3 text-left hover:bg-white/5 transition-colors
              {selectedContext === ctx
              ? 'bg-blue-500/10 border-l-2 border-l-blue-500'
              : ''}"
          >
            <p class="text-sm text-white">{label}</p>
            <p class="text-xs text-gray-500">
              {contextDescriptions[ctx as ToolbarContext]}
            </p>
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <!-- Counter -->
  <div
    class="flex items-center justify-between p-3 rounded-lg
      {isOverLimit
      ? 'bg-red-500/10 border border-red-500/20'
      : 'bg-[#252526] border border-white/10'}"
  >
    <div class="flex items-center gap-3">
      <div
        class="p-1.5 rounded-md {isOverLimit
          ? 'bg-red-500/20'
          : isAtLimit
            ? 'bg-amber-500/20'
            : 'bg-green-500/20'}"
      >
        <Tally5
          size={16}
          class={isOverLimit
            ? "text-red-400"
            : isAtLimit
              ? "text-amber-400"
              : "text-green-400"}
        />
      </div>
      <span class="text-xs text-gray-300">
        Botones visibles:
        <span
          class="font-semibold
            {isOverLimit
            ? 'text-red-400'
            : isAtLimit
              ? 'text-amber-400'
              : 'text-green-400'}"
        >
          {visibleCount}/{buttonLimit}
        </span>
      </span>
    </div>
    {#if isOverLimit}
      <span class="text-xs text-red-100 bg-red-600 px-2 py-1 rounded">
        Oculta {visibleCount - buttonLimit}
      </span>
    {:else}
      <div class="flex items-center gap-1.5 opacity-40">
        <div class="flex gap-0.5">
          {#each Array(buttonLimit) as _, i}
            <div
              class="w-1 h-3 rounded-full {i < visibleCount
                ? 'bg-blue-400'
                : 'bg-gray-600'}"
            ></div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <!-- Button List -->
  <div class="space-y-1 max-h-72 overflow-y-auto pr-1" role="list">
    {#each buttons as button, index (button.id)}
      {@const canToggle = button.visible || !isAtLimit}

      <div
        draggable="true"
        ondragstart={(e) => handleDragStart(e, index)}
        ondragover={(e) => handleDragOver(e, index)}
        ondragend={handleDragEnd}
        class="group flex items-center gap-2 p-2.5 rounded-lg transition-all cursor-grab active:cursor-grabbing
          {draggedIndex === index
          ? 'opacity-50 scale-98 bg-blue-500/20 border border-blue-500/30'
          : 'bg-[#252526] border border-transparent hover:border-white/10'}
          {!canToggle && !button.visible ? 'opacity-50' : ''}"
        role="listitem"
      >
        <!-- Drag Handle -->
        <div class="text-gray-600 group-hover:text-gray-400 cursor-move">
          <GripVertical size={14} />
        </div>

        <!-- Visibility Toggle -->
        <button
          onclick={() => toggleVisibility(button.id)}
          disabled={!canToggle && !button.visible}
          class="p-1 rounded transition-colors
            {button.visible
            ? 'text-green-400 hover:bg-green-500/10'
            : 'text-gray-500 hover:bg-white/5'}
            disabled:cursor-not-allowed disabled:opacity-50"
        >
          {#if button.visible}
            <Eye size={14} />
          {:else}
            <EyeOff size={14} />
          {/if}
        </button>

        <!-- Icon -->
        {#if button.icon}
          {@const Icon = button.icon}
          <Icon size={14} class="text-gray-400" />
        {/if}

        <!-- Label -->
        <span
          class="flex-1 text-sm truncate
            {button.visible ? 'text-white' : 'text-gray-500'}"
        >
          {button.label}
        </span>

        <!-- Category Badge -->
        {#if button.category}
          <span
            class="px-1.5 py-0.5 text-[10px] text-gray-500 bg-white/5 rounded"
          >
            {button.category}
          </span>
        {/if}

        <!-- Order Number -->
        {#if button.visible}
          <span
            class="w-5 h-5 flex items-center justify-center text-[10px] text-gray-500 bg-white/5 rounded"
          >
            {index + 1}
          </span>
        {/if}
      </div>
    {/each}
  </div>

  <!-- Reset Button -->
  <button
    onclick={resetToDefault}
    class="w-full flex items-center justify-center gap-2 p-2.5 rounded-lg
      bg-[#252526] border border-white/10 text-sm text-gray-300
      hover:bg-white/5 hover:border-white/20 transition-colors"
  >
    <RotateCcw size={14} />
    Restaurar orden por defecto
  </button>

  <!-- Tips -->
  <div class="p-3 rounded-lg bg-blue-500/5 border border-blue-500/10">
    <p class="text-xs text-blue-400">
      <strong>Tips:</strong> Arrastra para reordenar • El orden se guarda por contexto
      • Los botones ocultos no aparecen en la toolbar
    </p>
  </div>
</div>
