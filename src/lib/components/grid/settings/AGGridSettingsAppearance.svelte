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
</script>

<div class="space-y-6">
  <!-- Tema -->
  <section>
    <h3 class="text-sm font-medium text-white mb-3">Tema</h3>
    <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
      {#each themes as t}
        <button
          onclick={() => handleThemeChange(t.value)}
          class="relative flex flex-col items-center gap-2 p-3 rounded-lg border transition-all
            {theme === t.value
            ? 'border-blue-500 bg-blue-500/10'
            : 'border-white/10 hover:border-white/20 bg-[#252526]'}"
        >
          <!-- Preview -->
          <div
            class="w-full h-8 rounded flex items-center justify-center text-xs
              {t.dark
              ? 'bg-gray-800 text-gray-300'
              : 'bg-gray-100 text-gray-700'}"
          >
            Abc
          </div>
          <span class="text-xs text-gray-300">{t.label}</span>

          {#if theme === t.value}
            <div class="absolute top-1.5 right-1.5">
              <Check size={14} class="text-blue-400" />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Fuente -->
  <section>
    <h3 class="text-sm font-medium text-white mb-3">Fuente</h3>
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
      {#each fonts as f}
        <button
          onclick={() => handleFontChange(f.value)}
          class="relative flex flex-col items-center gap-1.5 p-3 rounded-lg border transition-all
            {font === f.value
            ? 'border-blue-500 bg-blue-500/10'
            : 'border-white/10 hover:border-white/20 bg-[#252526]'}"
        >
          <span
            class="text-lg text-white
              {f.value === 'inter' ? 'font-inter' : ''}
              {f.value === 'roboto' ? 'font-roboto' : ''}
              {f.value === 'source-sans' ? 'font-source-sans' : ''}"
          >
            {f.preview}
          </span>
          <span class="text-xs text-gray-400">{f.label}</span>

          {#if font === f.value}
            <div class="absolute top-1.5 right-1.5">
              <Check size={12} class="text-blue-400" />
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Densidad -->
  <section>
    <h3 class="text-sm font-medium text-white mb-3">Densidad de filas</h3>
    <div class="flex gap-2">
      {#each densities as d}
        <button
          onclick={() => handleDensityChange(d.value)}
          class="flex-1 flex flex-col items-center gap-1 p-3 rounded-lg border transition-all
            {rowHeight === d.value
            ? 'border-blue-500 bg-blue-500/10'
            : 'border-white/10 hover:border-white/20 bg-[#252526]'}"
        >
          <!-- Visual indicator -->
          <div class="w-full flex flex-col gap-0.5">
            {#each [1, 2, 3] as _}
              <div
                class="w-full bg-gray-600 rounded-sm"
                style="height: {d.px / 6}px"
              ></div>
            {/each}
          </div>
          <span class="text-xs text-gray-300 mt-1">{d.label}</span>
          <span class="text-[10px] text-gray-500">{d.px}px</span>
        </button>
      {/each}
    </div>
  </section>

  <!-- Opciones adicionales -->
  <section>
    <h3 class="text-sm font-medium text-white mb-3">Opciones</h3>
    <div class="space-y-2">
      <label
        class="flex items-center justify-between p-3 rounded-lg bg-[#252526] border border-white/10
          hover:border-white/20 cursor-pointer transition-colors"
      >
        <div>
          <p class="text-sm text-white">Animar filas</p>
          <p class="text-xs text-gray-500">Animaciones al ordenar y filtrar</p>
        </div>
        <input
          type="checkbox"
          checked={animateRows}
          onchange={handleAnimateChange}
          class="w-4 h-4 rounded bg-[#1e1e1e] border-white/20 text-blue-500 focus:ring-blue-500"
        />
      </label>

      <label
        class="flex items-center justify-between p-3 rounded-lg bg-[#252526] border border-white/10
          hover:border-white/20 cursor-pointer transition-colors"
      >
        <div>
          <p class="text-sm text-white">Seleccionar texto</p>
          <p class="text-xs text-gray-500">
            Permitir copiar texto de las celdas
          </p>
        </div>
        <input
          type="checkbox"
          checked={cellTextSelection}
          onchange={handleTextSelectionChange}
          class="w-4 h-4 rounded bg-[#1e1e1e] border-white/20 text-blue-500 focus:ring-blue-500"
        />
      </label>
    </div>
  </section>
</div>
