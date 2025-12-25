<script lang="ts">
  // @ts-nocheck
  import type {
    GridId,
    AGGridTheme,
    AGGridFont,
    RowHeight,
  } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { Check } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Estado local sincronizado con store
  let theme = $derived(agGridSettings.getTheme(gridId));
  let font = $derived(agGridSettings.getFont(gridId));
  let rowHeight = $derived(agGridSettings.getRowHeight(gridId));
  let toolbarPosition = $derived(agGridSettings.getToolbarPosition(gridId));
  let animateRows = $derived(agGridSettings.getAnimateRows(gridId));
  let cellTextSelection = $derived(agGridSettings.getCellTextSelection(gridId));

  // Opciones
  const themes: { value: AGGridTheme; label: string; dark: boolean }[] = [
    { value: "ag-theme-quartz-dark", label: "Quartz Dark", dark: true },
    { value: "ag-theme-quartz", label: "Quartz Light", dark: false },
    { value: "ag-theme-alpine-dark", label: "Alpine Dark", dark: true },
    { value: "ag-theme-alpine", label: "Alpine Light", dark: false },
    { value: "ag-theme-balham", label: "Balham", dark: false },
  ];

  const fonts: { value: AGGridFont; label: string; preview: string }[] = [
    { value: "system", label: "Sistema", preview: "Aa" },
    { value: "inter", label: "Inter", preview: "Aa" },
    { value: "roboto", label: "Roboto", preview: "Aa" },
    { value: "source-sans", label: "Source Sans", preview: "Aa" },
  ];

  const densities: { value: RowHeight; label: string; px: number }[] = [
    { value: "compact", label: "Compacto", px: 32 },
    { value: "normal", label: "Normal", px: 40 },
    { value: "comfortable", label: "Cómodo", px: 48 },
  ];

  // Handlers
  function handleThemeChange(value: AGGridTheme) {
    theme = value;
    agGridSettings.setTheme(gridId, value);
  }

  function handleFontChange(value: AGGridFont) {
    font = value;
    agGridSettings.setFont(gridId, value);
  }

  function handleDensityChange(value: RowHeight) {
    rowHeight = value;
    agGridSettings.setRowHeight(gridId, value);

    if (gridApi) {
      const px = densities.find((d) => d.value === value)?.px || 40;
      gridApi.setGridOption("rowHeight", px);
      gridApi.resetRowHeights();
    }
  }

  function handleAnimateChange() {
    animateRows = !animateRows;
    agGridSettings.setAnimateRows(gridId, animateRows);
    if (gridApi) {
      gridApi.setGridOption("animateRows", animateRows);
    }
  }

  function handleTextSelectionChange() {
    cellTextSelection = !cellTextSelection;
    agGridSettings.setCellTextSelection(gridId, cellTextSelection);
    if (gridApi) {
      gridApi.setGridOption("enableCellTextSelection", cellTextSelection);
    }
  }

  function handleToolbarPositionChange(value: "top" | "bottom") {
    agGridSettings.setToolbarPosition(gridId, value);
  }
</script>

<div class="space-y-6">
  <!-- Tema -->
  <section>
    <h3
      class="text-xs font-semibold uppercase tracking-wider text-[#8b949e] mb-3"
    >
      Tema
    </h3>
    <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
      {#each themes as t}
        <button
          onclick={() => handleThemeChange(t.value)}
          class="relative flex flex-col items-center gap-2 p-3 rounded-md border transition-all
            {theme === t.value
            ? 'border-[#238636] bg-[#238636]/10'
            : 'border-[#30363d] hover:border-[#8b949e] bg-[#161b22]'}"
        >
          <div
            class="w-full h-8 rounded flex items-center justify-center text-xs font-mono
              {t.dark
              ? 'bg-[#0d1117] text-[#8b949e] border border-[#30363d]'
              : 'bg-[#f6f8fa] text-[#24292f] border border-[#d0d7de]'}"
          >
            Abc
          </div>
          <span class="text-xs text-[#e6edf3]">{t.label}</span>

          {#if theme === t.value}
            <div class="absolute top-1.5 right-1.5">
              <Check size={14} class="text-[#238636]" />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Fuente -->
  <section>
    <h3
      class="text-xs font-semibold uppercase tracking-wider text-[#8b949e] mb-3"
    >
      Fuente
    </h3>
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
      {#each fonts as f}
        <button
          onclick={() => handleFontChange(f.value)}
          class="relative flex flex-col items-center gap-1.5 p-3 rounded-md border transition-all
            {font === f.value
            ? 'border-[#238636] bg-[#238636]/10'
            : 'border-[#30363d] hover:border-[#8b949e] bg-[#161b22]'}"
        >
          <span
            class="text-lg text-[#e6edf3]
              {f.value === 'inter' ? 'font-inter' : ''}
              {f.value === 'roboto' ? 'font-roboto' : ''}
              {f.value === 'source-sans' ? 'font-source-sans' : ''}"
          >
            {f.preview}
          </span>
          <span class="text-xs text-[#8b949e]">{f.label}</span>

          {#if font === f.value}
            <div class="absolute top-1 right-1">
              <Check size={12} class="text-[#238636]" />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Densidad -->
  <section>
    <h3
      class="text-xs font-semibold uppercase tracking-wider text-[#8b949e] mb-3"
    >
      Densidad de filas
    </h3>
    <div class="flex gap-2">
      {#each densities as d}
        <button
          onclick={() => handleDensityChange(d.value)}
          class="flex-1 flex flex-col items-center gap-1 p-3 rounded-md border transition-all
            {rowHeight === d.value
            ? 'border-[#238636] bg-[#238636]/10'
            : 'border-[#30363d] hover:border-[#8b949e] bg-[#161b22]'}"
        >
          <div class="w-full flex flex-col gap-0.5">
            {#each [1, 2, 3] as _}
              <div
                class="w-full rounded-sm {rowHeight === d.value
                  ? 'bg-[#238636]'
                  : 'bg-[#30363d]'}"
                style="height: {d.px / 6}px"
              ></div>
            {/each}
          </div>
          <span class="text-xs text-[#e6edf3] mt-1">{d.label}</span>
          <span class="text-[10px] text-[#8b949e]">{d.px}px</span>
        </button>
      {/each}
    </div>
  </section>

  <!-- Ubicación de Toolbar -->
  <section>
    <h3
      class="text-xs font-semibold uppercase tracking-wider text-[#8b949e] mb-3"
    >
      Ubicación de Toolbar
    </h3>
    <div class="flex gap-2">
      {#each [["top", "Superior"], ["bottom", "Inferior"]] as [value, label]}
        <button
          onclick={() => handleToolbarPositionChange(value as "top" | "bottom")}
          class="relative flex-1 flex flex-col items-center gap-2 p-3 rounded-md border transition-all
            {toolbarPosition === value
            ? 'border-[#238636] bg-[#238636]/10'
            : 'border-[#30363d] hover:border-[#8b949e] bg-[#161b22]'}"
        >
          <div class="w-full flex flex-col gap-1">
            {#if value === "top"}
              <div class="w-full h-2 bg-[#f78166] rounded-sm"></div>
              <div
                class="w-full h-8 bg-[#21262d] rounded-sm border border-[#30363d]"
              ></div>
            {:else}
              <div
                class="w-full h-8 bg-[#21262d] rounded-sm border border-[#30363d]"
              ></div>
              <div class="w-full h-2 bg-[#f78166] rounded-sm"></div>
            {/if}
          </div>
          <span class="text-xs text-[#e6edf3]">{label}</span>
          {#if toolbarPosition === value}
            <div class="absolute top-1 right-1">
              <Check size={12} class="text-[#238636]" />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Opciones adicionales -->
  <section>
    <h3
      class="text-xs font-semibold uppercase tracking-wider text-[#8b949e] mb-3"
    >
      Opciones
    </h3>
    <div class="space-y-2">
      <label
        class="flex items-center justify-between p-3 rounded-md bg-[#161b22] border border-[#30363d]
          hover:border-[#8b949e] cursor-pointer transition-colors"
      >
        <div>
          <p class="text-sm text-[#e6edf3]">Animar filas</p>
          <p class="text-xs text-[#8b949e]">Animaciones al ordenar y filtrar</p>
        </div>
        <input
          type="checkbox"
          checked={animateRows}
          onchange={handleAnimateChange}
          class="w-4 h-4 rounded bg-[#0d1117] border-[#30363d] text-[#238636] focus:ring-[#238636]"
        />
      </label>

      <label
        class="flex items-center justify-between p-3 rounded-md bg-[#161b22] border border-[#30363d]
          hover:border-[#8b949e] cursor-pointer transition-colors"
      >
        <div>
          <p class="text-sm text-[#e6edf3]">Seleccionar texto</p>
          <p class="text-xs text-[#8b949e]">
            Permitir copiar texto de las celdas
          </p>
        </div>
        <input
          type="checkbox"
          checked={cellTextSelection}
          onchange={handleTextSelectionChange}
          class="w-4 h-4 rounded bg-[#0d1117] border-[#30363d] text-[#238636] focus:ring-[#238636]"
        />
      </label>
    </div>
  </section>
</div>
