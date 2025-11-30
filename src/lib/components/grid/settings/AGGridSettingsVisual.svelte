<script lang="ts">
  import type { GridId, AGGridTheme, AGGridFont } from "$lib/types/agGrid";
  import type { GridApi } from "@ag-grid-community/core";
  import { agGridSettings } from "$lib/stores/agGridSettings.svelte";
  import { Palette, Type, Info } from "lucide-svelte";

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  let selectedTheme = $state<AGGridTheme>(agGridSettings.getTheme(gridId));
  let selectedFont = $state<AGGridFont>(agGridSettings.getFont(gridId));

  const themes: { value: AGGridTheme; label: string }[] = [
    { value: "ag-theme-quartz", label: "Quartz (Claro)" },
    { value: "ag-theme-quartz-dark", label: "Quartz (Oscuro)" },
    { value: "ag-theme-alpine", label: "Alpine (Claro)" },
    { value: "ag-theme-alpine-dark", label: "Alpine (Oscuro)" },
    { value: "ag-theme-balham", label: "Balham" },
  ];

  const fonts: { value: AGGridFont; label: string; class: string }[] = [
    { value: "system", label: "Sistema", class: "font-system" },
    { value: "inter", label: "Inter", class: "font-inter" },
    { value: "roboto", label: "Roboto", class: "font-roboto" },
    {
      value: "source-sans",
      label: "Source Sans Pro",
      class: "font-source-sans",
    },
  ];

  function handleThemeChange(theme: AGGridTheme) {
    selectedTheme = theme;
    agGridSettings.setTheme(gridId, theme);
  }

  function handleFontChange(font: AGGridFont) {
    selectedFont = font;
    agGridSettings.setFont(gridId, font);
  }
</script>

<div class="space-y-6">
  <!-- Tema -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Palette size={16} class="text-blue-400" />
      <h3 class="text-sm font-semibold text-white">Tema de Grid</h3>
    </div>

    <div class="space-y-2">
      {#each themes as theme}
        <label
          class="flex items-center gap-3 p-2.5 rounded-lg border cursor-pointer transition-all
          {selectedTheme === theme.value
            ? 'bg-blue-500/10 border-blue-500/50'
            : 'bg-[#252526] border-white/10 hover:border-white/20 hover:bg-[#2a2a2b]'}"
        >
          <input
            type="radio"
            name="theme"
            value={theme.value}
            checked={selectedTheme === theme.value}
            onchange={() => handleThemeChange(theme.value)}
            class="w-4 h-4 text-blue-500 bg-[#1e1e1e] border-white/20 focus:ring-blue-500 focus:ring-2"
          />
          <span class="text-sm text-white flex-1">{theme.label}</span>
          {#if selectedTheme === theme.value}
            <div class="w-1.5 h-1.5 rounded-full bg-blue-500"></div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <!-- Fuente -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Type size={16} class="text-green-400" />
      <h3 class="text-sm font-semibold text-white">Fuente</h3>
    </div>

    <div class="space-y-2">
      {#each fonts as font}
        <label
          class="flex items-center gap-3 p-2.5 rounded-lg border cursor-pointer transition-all
          {selectedFont === font.value
            ? 'bg-green-500/10 border-green-500/50'
            : 'bg-[#252526] border-white/10 hover:border-white/20 hover:bg-[#2a2a2b]'}"
        >
          <input
            type="radio"
            name="font"
            value={font.value}
            checked={selectedFont === font.value}
            onchange={() => handleFontChange(font.value)}
            class="w-4 h-4 text-green-500 bg-[#1e1e1e] border-white/20 focus:ring-green-500 focus:ring-2"
          />
          <span class="text-sm text-white flex-1 {font.class}"
            >{font.label}</span
          >
          {#if selectedFont === font.value}
            <div class="w-1.5 h-1.5 rounded-full bg-green-500"></div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <!-- Info -->
  <div class="p-3 bg-[#252526] border border-white/10 rounded-lg">
    <div class="flex items-start gap-2">
      <Info size={16} class="text-blue-400 mt-0.5 flex-shrink-0" />
      <div>
        <p class="text-xs text-gray-400 leading-relaxed">
          Los cambios se aplican inmediatamente a la tabla.
        </p>
      </div>
    </div>
  </div>
</div>
