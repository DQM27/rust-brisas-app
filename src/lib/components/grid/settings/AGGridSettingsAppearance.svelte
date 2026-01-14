<script lang="ts">
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import type {
    GridId,
    AGGridTheme,
    AGGridFont,
    RowHeight,
    ToolbarPosition,
  } from "$lib/types/agGrid";
  import SelectDropdown from "$lib/components/shared/SelectDropdown.svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Load initial settings
  let theme = $state<AGGridTheme>(agGridSettings.getTheme(gridId));
  let font = $state<AGGridFont>(agGridSettings.getFont(gridId));
  let rowHeight = $state<RowHeight>(agGridSettings.getRowHeight(gridId));
  let headerHeight = $state<number>(agGridSettings.getHeaderHeight(gridId));
  let toolbarPosition = $state<ToolbarPosition>(
    agGridSettings.getToolbarPosition(gridId),
  );
  let animateRows = $state<boolean>(agGridSettings.getAnimateRows(gridId));
  let cellTextSelection = $state<boolean>(
    agGridSettings.getCellTextSelection(gridId),
  );

  // Handlers
  function handleThemeChange(newTheme: AGGridTheme) {
    theme = newTheme;
    agGridSettings.setTheme(gridId, newTheme);
  }

  function handleFontChange(newFont: AGGridFont) {
    font = newFont;
    agGridSettings.setFont(gridId, newFont);
  }

  function handleRowHeightChange(newHeight: RowHeight) {
    rowHeight = newHeight;
    agGridSettings.setRowHeight(gridId, newHeight);
    gridApi?.resetRowHeights();
  }

  function handleHeaderHeightChange(newHeight: number) {
    headerHeight = newHeight;
    agGridSettings.setHeaderHeight(gridId, newHeight);
    gridApi?.setGridOption("headerHeight", newHeight);
  }

  function handleToolbarPosChange(newPos: ToolbarPosition) {
    toolbarPosition = newPos;
    agGridSettings.setToolbarPosition(gridId, newPos);
  }

  function handleAnimateChange(checked: boolean) {
    animateRows = checked;
    agGridSettings.setAnimateRows(gridId, checked);
    gridApi?.setGridOption("animateRows", checked);
  }

  function handleTextSelectionChange(checked: boolean) {
    cellTextSelection = checked;
    agGridSettings.setCellTextSelection(gridId, checked);
    gridApi?.setGridOption("enableCellTextSelection", checked);
  }

  // Options
  const themes: { value: AGGridTheme; label: string }[] = [
    { value: "ag-theme-quartz-dark", label: "Quartz Dark" },
    { value: "ag-theme-quartz", label: "Quartz Light" },
    { value: "ag-theme-alpine-dark", label: "Alpine Dark" },
    { value: "ag-theme-alpine", label: "Alpine Light" },
    { value: "ag-theme-balham", label: "Balham" },
  ];

  const fonts: { value: AGGridFont; label: string }[] = [
    { value: "system", label: "Sistema" },
    { value: "inter", label: "Inter" },
    { value: "roboto", label: "Roboto" },
    { value: "source-sans", label: "Source Sans" },
  ];

  const densities: { value: RowHeight; label: string }[] = [
    { value: "compact", label: "Compacto" },
    { value: "normal", label: "Normal" },
    { value: "comfortable", label: "Cómodo" },
  ];

  const toolbarPositions: { value: ToolbarPosition; label: string }[] = [
    { value: "top", label: "Superior" },
    { value: "bottom", label: "Inferior" },
  ];

  const labelClass = "block text-xs font-medium text-zinc-400 mb-1.5 ml-0.5";
  const sectionClass = "space-y-4 p-1";
</script>

<div class={sectionClass}>
  <!-- Grid Layout for Compactness -->
  <div class="grid grid-cols-2 gap-4">
    <!-- Theme -->
    <SelectDropdown
      label="Tema Visual"
      value={theme}
      options={themes}
      onSelect={handleThemeChange}
    />

    <!-- Font -->
    <SelectDropdown
      label="Tipografía"
      value={font}
      options={fonts}
      onSelect={handleFontChange}
    />
  </div>

  <div class="grid grid-cols-2 gap-4">
    <!-- Toolbar Position -->
    <SelectDropdown
      label="Barra de Herramientas"
      value={toolbarPosition}
      options={toolbarPositions}
      onSelect={handleToolbarPosChange}
    />

    <!-- Row Density (Segmented Control) -->
    <div>
      <label class={labelClass}>Densidad de Filas</label>
      <div
        class="flex bg-black/20 p-1 rounded-lg border border-white/10 h-[34px]"
      >
        {#each densities as d}
          <button
            class="flex-1 text-xs font-medium rounded-md transition-all flex items-center justify-center {rowHeight ===
            d.value
              ? 'bg-blue-600/20 text-blue-400 ring-1 ring-blue-500/50'
              : 'text-zinc-500 hover:text-zinc-300'}"
            onclick={() => handleRowHeightChange(d.value)}
          >
            {d.label}
          </button>
        {/each}
      </div>
    </div>
  </div>

  <!-- Header Height Slider -->
  <div class="pt-2">
    <div class="flex justify-between items-center mb-2">
      <label class={labelClass}>Altura de Cabecera</label>
      <span
        class="text-xs font-mono text-blue-400 bg-blue-900/20 px-1.5 py-0.5 rounded border border-blue-500/20"
      >
        {headerHeight}px
      </span>
    </div>
    <div class="px-1">
      <input
        type="range"
        min="30"
        max="60"
        step="2"
        value={headerHeight}
        oninput={(e) => handleHeaderHeightChange(Number(e.currentTarget.value))}
        class="w-full h-1.5 bg-zinc-700/50 rounded-lg appearance-none cursor-pointer focus:outline-none focus:ring-2 focus:ring-blue-600/50 accent-blue-600"
      />
      <div
        class="flex justify-between text-[10px] text-zinc-600 mt-1.5 font-medium px-0.5"
      >
        <span>Compacta (30px)</span>
        <span>Amplia (60px)</span>
      </div>
    </div>
  </div>

  <div class="h-px bg-white/5 my-2"></div>

  <!-- Additional Options -->
  <div class="grid grid-cols-2 gap-4">
    <label class="flex items-center gap-3 cursor-pointer group">
      <input
        type="checkbox"
        checked={animateRows}
        onchange={(e) => handleAnimateChange(e.currentTarget.checked)}
        class="w-4 h-4 rounded bg-black/20 border-zinc-600 text-blue-600 focus:ring-blue-600 focus:ring-offset-0 transition-all checked:bg-blue-600 checked:border-blue-600"
      />
      <span class="text-sm text-zinc-400 group-hover:text-zinc-200"
        >Animar filas</span
      >
    </label>

    <label class="flex items-center gap-3 cursor-pointer group">
      <input
        type="checkbox"
        checked={cellTextSelection}
        onchange={(e) => handleTextSelectionChange(e.currentTarget.checked)}
        class="w-4 h-4 rounded bg-black/20 border-zinc-600 text-blue-600 focus:ring-blue-600 focus:ring-offset-0 transition-all checked:bg-blue-600 checked:border-blue-600"
      />
      <span class="text-sm text-zinc-400 group-hover:text-zinc-200"
        >Selección de texto</span
      >
    </label>
  </div>
</div>
