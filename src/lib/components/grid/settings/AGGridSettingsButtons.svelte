<!-- src/lib/components/grid/settings/AGGridSettingsButtons.svelte -->
<script lang="ts">
  import type { GridId, ToolbarContext } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { getGridConfig } from "$lib/config/agGridConfigs";
  import { RotateCcw, Eye, EyeOff, AlertCircle } from "lucide-svelte";

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

  // Configuración
  const gridConfig = getGridConfig(gridId);
  const buttonLimit = $derived(agGridSettings.getButtonLimit(selectedContext));

  // Botones disponibles para el contexto seleccionado (incluyendo custom)
  const availableButtons = $derived.by(() => {
    const baseButtons = gridConfig.availableButtons[selectedContext];
    const customForContext = customButtons?.[selectedContext] || [];

    // Convertir custom buttons al formato ToolbarButtonDefinition
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

  // Configuración actual de botones
  const buttonsConfig = $derived(
    agGridSettings.getButtonsConfig(gridId, selectedContext) || {
      order: [],
      hidden: [],
    },
  );

  // Todos los botones (order + no en order + hidden)
  const allButtonIds = $derived.by(() => {
    const orderSet = new Set(buttonsConfig.order);
    const hiddenSet = new Set(buttonsConfig.hidden);
    const inOrderOrHidden = new Set([...orderSet, ...hiddenSet]);

    // Botones que están en availableButtons pero no en order ni hidden
    const notInConfig = availableButtons
      .map((b) => b.id)
      .filter((id) => !inOrderOrHidden.has(id));

    return [...buttonsConfig.order, ...notInConfig, ...buttonsConfig.hidden];
  });

  // Lista de botones con info completa
  const buttons = $derived.by(() => {
    return allButtonIds.map((id) => {
      const def = availableButtons.find((b) => b.id === id);
      const isHidden = buttonsConfig.hidden.includes(id);
      const isInOrder = buttonsConfig.order.includes(id);

      return {
        id,
        label: def?.label || id,
        icon: def?.icon,
        category: def?.category || "custom",
        visible: !isHidden,
        inOrder: isInOrder,
      };
    });
  });

  // Contar visibles
  const visibleCount = $derived(buttons.filter((b) => b.visible).length);
  const isAtLimit = $derived(visibleCount >= buttonLimit);
  const isOverLimit = $derived(visibleCount > buttonLimit);

  // Color del contador
  const counterColor = $derived.by(() => {
    if (isOverLimit) return "text-red-400";
    if (visibleCount >= buttonLimit - 1) return "text-amber-400";
    return "text-green-400";
  });

  // Togglear visibilidad
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
      if (isAtLimit) {
        return;
      }
      agGridSettings.setHiddenButtons(
        gridId,
        selectedContext,
        currentHidden.filter((id) => id !== buttonId),
      );
    }
  }

  // Cambiar contexto
  function handleContextChange(context: ToolbarContext) {
    selectedContext = context;
  }

  // Reset a default
  function resetToDefault() {
    agGridSettings.setButtonOrder(gridId, selectedContext, []);
    agGridSettings.setHiddenButtons(gridId, selectedContext, []);
  }

  // Obtener info de botón
  function getButtonInfo(buttonId: string) {
    return availableButtons.find((b) => b.id === buttonId);
  }

  const contextLabels: Record<ToolbarContext, string> = {
    default: "Sin Selección",
    singleSelect: "Selección Simple",
    multiSelect: "Selección Múltiple",
  };
</script>

<div class="space-y-4">
  <!-- Selector de contexto -->
  <div>
    <label
      for="context-select"
      class="block text-sm font-medium text-gray-400 mb-2"
    >
      Contexto de Toolbar
    </label>
    <select
      id="context-select"
      bind:value={selectedContext}
      onchange={(e) =>
        handleContextChange(e.currentTarget.value as ToolbarContext)}
      class="w-full px-3 py-2 bg-[#252526] border border-white/10 rounded-lg text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      {#each Object.entries(contextLabels) as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </div>

  <!-- Contador con límite -->
  <div
    class="flex items-center justify-between p-3 bg-[#252526] border border-white/10 rounded-lg"
  >
    <div class="flex items-center gap-2">
      {#if isOverLimit}
        <AlertCircle size={18} class="text-red-400" />
      {/if}
      <span class="text-sm text-gray-400">
        Botones visibles:
        <span class="font-semibold {counterColor}">
          {visibleCount}/{buttonLimit}
        </span>
      </span>
    </div>
    {#if isAtLimit || isOverLimit}
      <span class="text-xs text-amber-400">
        {isOverLimit ? "⚠️ Límite excedido" : "⚠️ Límite alcanzado"}
      </span>
    {/if}
  </div>

  <!-- Mensaje de ayuda -->
  {#if isAtLimit && !isOverLimit}
    <div class="p-3 bg-amber-500/10 border border-amber-500/20 rounded-lg">
      <p class="text-xs text-amber-400">
        <strong>Límite casi alcanzado.</strong> Elige sabiamente qué acciones usas
        con más frecuencia.
      </p>
    </div>
  {:else if isOverLimit}
    <div class="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
      <p class="text-xs text-red-400">
        <strong>⚠️ Límite excedido ({visibleCount}/{buttonLimit}).</strong>
        Desmarca {visibleCount - buttonLimit} botón{visibleCount - buttonLimit >
        1
          ? "es"
          : ""} para continuar.
      </p>
    </div>
  {/if}

  <!-- Lista de botones -->
  <div class="space-y-1 max-h-80 overflow-y-auto">
    {#each buttons as button (button.id)}
      {@const info = getButtonInfo(button.id)}
      {@const canToggle = button.visible || !isAtLimit}

      <label
        class="flex items-center gap-3 p-3 bg-[#252526] border border-white/10 rounded-lg hover:border-white/20 transition-all
        {!canToggle ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
      >
        <input
          type="checkbox"
          checked={button.visible}
          disabled={!canToggle}
          onchange={() => toggleVisibility(button.id)}
          class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 rounded focus:ring-blue-500 focus:ring-2
            disabled:opacity-50 disabled:cursor-not-allowed"
        />

        {#if info?.icon}
          {@const Icon = info.icon}
          <Icon size={16} class="text-gray-400" />
        {/if}

        <span class="flex-1 text-sm text-white">
          {button.label}
        </span>

        {#if info?.category}
          <span class="text-xs text-gray-500 px-2 py-0.5 bg-white/5 rounded">
            {info.category}
          </span>
        {/if}

        {#if button.visible}
          <Eye size={16} class="text-green-400" />
        {:else}
          <EyeOff size={16} class="text-gray-500" />
        {/if}
      </label>
    {/each}
  </div>

  <!-- Botón de reset -->
  <button
    onclick={resetToDefault}
    class="w-full flex items-center justify-center gap-2 px-4 py-2.5 bg-[#252526] border border-white/10 rounded-lg text-sm font-medium text-white hover:bg-white/5 hover:border-white/20 transition-colors"
  >
    <RotateCcw size={16} />
    Restaurar por defecto
  </button>
</div>
