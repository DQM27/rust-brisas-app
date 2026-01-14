<!-- src/lib/components/grid/AGGridToolbar.svelte -->
<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import type {
    ToolbarContext,
    CustomToolbarButton,
    GridId,
    ToolbarButtonDefinition,
    ButtonState,
    ButtonVariant,
  } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { getGridConfig } from "$lib/config/agGridConfigs";
  import { Settings } from "lucide-svelte";
  import AGGridToolbarButton from "./AGGridToolbarButton.svelte";

  interface Props {
    gridId: GridId;
    context: ToolbarContext;
    selectedRows: any[];
    gridApi: GridApi | null;
    onOpenSettings: () => void;
    customButtons?: {
      default?: CustomToolbarButton[];
      singleSelect?: CustomToolbarButton[];
      multiSelect?: CustomToolbarButton[];
    };
    customToolbarSlot?: import("svelte").Snippet;
  }

  let {
    gridId,
    context,
    selectedRows,
    gridApi,
    onOpenSettings,
    customButtons = {},
    customToolbarSlot,
  }: Props = $props();

  // Obtener configuración de la grid
  const gridConfig = $derived(getGridConfig(gridId));

  // Proveer valor por defecto si no existe configuración
  const buttonsConfig = $derived(
    agGridSettings.getButtonsConfig(gridId, context) || {
      order: [],
      hidden: [],
    },
  );
  const buttonLimit = $derived(agGridSettings.getButtonLimit(context));

  // Obtener botones disponibles para el contexto actual
  const availableButtons = $derived(gridConfig.availableButtons[context]);

  // Obtener custom buttons para el contexto actual
  const contextCustomButtons = $derived(customButtons[context] || []);

  // Tipo unificado para botones
  interface UnifiedButton {
    id: string;
    label: string;
    icon?: any;
    variant?: ButtonVariant;
    tooltip?: string;
    category?: string;
    custom: boolean;
    onClick?: () => void | Promise<void>;
    disabled?: boolean;
    state?: ButtonState;
  }

  // Combinar botones comunes + custom
  const allButtons = $derived.by((): UnifiedButton[] => {
    const common: UnifiedButton[] = availableButtons.map((def) => ({
      id: def.id,
      label: def.label,
      icon: def.icon,
      variant: def.variant,
      tooltip: def.tooltip,
      category: def.category,
      custom: false,
      disabled: false,
    }));

    const custom: UnifiedButton[] = contextCustomButtons.map((btn) => ({
      id: btn.id,
      label: btn.label,
      icon: btn.icon,
      variant: btn.variant,
      tooltip: btn.tooltip,
      category: "custom",
      custom: true, // Always true for buttons passed via customButtons prop
      onClick: btn.onClick,
      disabled: btn.disabled,
      state: btn.state,
    }));

    return [...common, ...custom];
  });

  // Filtrar botones visibles según configuración
  const visibleButtons = $derived.by(() => {
    const { order, hidden } = buttonsConfig;

    // Si no hay orden configurado, usar todos los disponibles
    if (order.length === 0) {
      return allButtons.filter((btn) => !hidden.includes(btn.id));
    }

    // Ordenar según configuración y filtrar ocultos
    return order
      .map((id) => allButtons.find((btn) => btn.id === id))
      .filter(
        (btn): btn is UnifiedButton =>
          btn !== undefined && !hidden.includes(btn.id),
      );
  });

  // Agrupar botones por categoría
  const groupedButtons = $derived.by(() => {
    const groups: Record<string, UnifiedButton[]> = {};

    visibleButtons.forEach((btn) => {
      const category = btn.category || "custom";
      if (!groups[category]) {
        groups[category] = [];
      }
      groups[category].push(btn);
    });

    return groups;
  });

  // Handlers para botones comunes
  function handleCommonButtonClick(buttonId: string) {
    if (!gridApi) return;

    switch (buttonId) {
      case "autosize-all":
        gridApi.autoSizeAllColumns();
        break;
      case "autosize-selected":
        const selectedCols = gridApi
          .getColumnState()
          ?.filter((col) => col.hide === false)
          .map((col) => col.colId);
        if (selectedCols) {
          gridApi.autoSizeColumns(selectedCols);
        }
        break;
      case "size-to-fit":
        gridApi.sizeColumnsToFit();
        break;
      case "reset-columns":
        gridApi.resetColumnState();
        break;
      case "export-csv":
        gridApi.exportDataAsCsv({
          fileName: `${gridId}-${new Date().toISOString().split("T")[0]}.csv`,
        });
        break;
      case "export-excel":
        console.warn("Excel export requires AG Grid Enterprise");
        alert("La exportación a Excel requiere AG Grid Enterprise");
        break;
      case "export-json":
        const data = [] as any[];
        gridApi.forEachNodeAfterFilterAndSort((node) => {
          if (node.data) data.push(node.data);
        });
        const blob = new Blob([JSON.stringify(data, null, 2)], {
          type: "application/json",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `${gridId}-${new Date().toISOString().split("T")[0]}.json`;
        a.click();
        URL.revokeObjectURL(url);
        break;
      case "select-all":
        gridApi.selectAllFiltered();
        break;
      case "deselect-all":
      case "deselect":
        gridApi.deselectAll();
        break;
      case "copy-selected":
        const selected = gridApi.getSelectedRows();
        if (selected.length > 0) {
          const text = JSON.stringify(selected, null, 2);
          navigator.clipboard.writeText(text);
        }
        break;
      case "toggle-filters":
        const isFiltersVisible = agGridSettings.getShowFloatingFilters(gridId);
        agGridSettings.setShowFloatingFilters(gridId, !isFiltersVisible);
        break;
      case "toggle-sidebar":
        console.warn("Sidebar requires AG Grid Enterprise");
        alert("El panel lateral requiere AG Grid Enterprise");
        break;
      case "refresh":
        gridApi.refreshCells();
        break;
      case "clear-filters":
        gridApi.setFilterModel(null);
        break;
      case "clear-sort":
        gridApi.applyColumnState({ defaultState: { sort: null } });
        break;
      case "expand-groups":
        gridApi.forEachNode((node) => {
          if (
            node.group ||
            (node.childrenAfterGroup && node.childrenAfterGroup.length > 0)
          ) {
            node.setExpanded(true);
          }
        });
        break;
      case "collapse-groups":
        gridApi.forEachNode((node) => {
          if (
            node.group ||
            (node.childrenAfterGroup && node.childrenAfterGroup.length > 0)
          ) {
            node.setExpanded(false);
          }
        });
        break;
      case "export-selection":
        gridApi.exportDataAsCsv({
          fileName: `${gridId}-selection-${new Date().toISOString().split("T")[0]}.csv`,
          onlySelected: true,
        });
        break;
    }
  }

  function handleButtonClick(button: UnifiedButton) {
    if (button.custom && button.onClick) {
      button.onClick();
    } else {
      handleCommonButtonClick(button.id);
    }
  }

  // Ordenar grupos para display
  const categoryOrder = [
    "custom",
    "columns",
    "export",
    "selection",
    "data",
    "ui",
  ];
  const orderedGroups = $derived.by(() => {
    return categoryOrder
      .map((cat) => ({ category: cat, buttons: groupedButtons[cat] || [] }))
      .filter((group) => group.buttons.length > 0);
  });
</script>

<div
  class="flex justify-between items-center gap-3 p-4 bg-[#252526] border-b border-white/10"
>
  <!-- Sección izquierda: Botones agrupados -->
  <div class="flex items-center gap-2">
    {#each orderedGroups as group, idx}
      {#if idx > 0}
        <div class="w-px h-6 bg-white/10"></div>
      {/if}

      {#each group.buttons as button}
        <div
          data-filter-button={button.id.startsWith("filter-")
            ? "true"
            : undefined}
        >
          <AGGridToolbarButton
            {button}
            disabled={button.disabled || !gridApi}
            onclick={() => handleButtonClick(button)}
          />
        </div>
      {/each}
    {/each}

    <!-- Divider antes de componentes personalizados -->
    {#if visibleButtons.length > 0 && customToolbarSlot}
      <div class="w-px h-6 bg-white/10"></div>
    {/if}

    <!-- Slot personalizado para componentes adicionales (ej: DateRangePicker) -->
    {#if customToolbarSlot}
      <div class="flex items-center">
        {@render customToolbarSlot()}
      </div>
    {/if}

    <!-- Divider antes de Settings -->
    {#if visibleButtons.length > 0 || customToolbarSlot}
      <div class="w-px h-6 bg-white/10"></div>
    {/if}

    <!-- Settings (siempre visible) -->
    <button
      onclick={onOpenSettings}
      class="flex items-center gap-1.5 px-3 py-2 bg-[#1e1e1e] border border-white/10 rounded-md text-white text-xs font-medium cursor-pointer transition-colors hover:bg-white/5 hover:border-white/20"
      title="Configuración de Grid"
    >
      <Settings size={16} />
      Configuración
    </button>
  </div>

  <!-- Sección derecha: Info de selección -->
  {#if selectedRows.length > 0}
    <div
      class="flex items-center gap-2 bg-blue-500/10 border border-blue-500/20 rounded-md px-3 py-2"
    >
      <span class="text-blue-400 text-xs font-medium">
        {selectedRows.length} seleccionada(s)
      </span>
    </div>
  {/if}
</div>
