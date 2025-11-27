<script lang="ts">
  import type { GridId, AGGridTheme, AGGridFont } from '$lib/types/agGrid';
  import type { GridApi } from '@ag-grid-community/core';
  import { agGridSettings } from '$lib/stores/agGridSettings.svelte';
  import { Palette, Type, Wand2 } from 'lucide-svelte';

  interface Props {
    gridId: GridId;
    gridApi: GridApi | null;
  }

  let { gridId, gridApi }: Props = $props();

  // Estado local para preview inmediato
  let selectedTheme = $state<AGGridTheme>(agGridSettings.getTheme(gridId));
  let selectedFont = $state<AGGridFont>(agGridSettings.getFont(gridId));

  const themes: { value: AGGridTheme; label: string }[] = [
    { value: 'ag-theme-quartz', label: 'Quartz (Claro)' },
    { value: 'ag-theme-quartz-dark', label: 'Quartz (Oscuro)' },
    { value: 'ag-theme-alpine', label: 'Alpine (Claro)' },
    { value: 'ag-theme-alpine-dark', label: 'Alpine (Oscuro)' },
    { value: 'ag-theme-balham', label: 'Balham' },
  ];

  const fonts: { value: AGGridFont; label: string; class: string }[] = [
    { value: 'system', label: 'Sistema', class: 'font-system' },
    { value: 'inter', label: 'Inter', class: 'font-inter' },
    { value: 'roboto', label: 'Roboto', class: 'font-roboto' },
    { value: 'source-sans', label: 'Source Sans Pro', class: 'font-source-sans' },
  ];

  function handleThemeChange(theme: AGGridTheme) {
    selectedTheme = theme;
    agGridSettings.setTheme(gridId, theme);
  }

  function handleFontChange(font: AGGridFont) {
    selectedFont = font;
    agGridSettings.setFont(gridId, font);
  }

  let showOrganizeMode = $state(false);

  function startOrganizeMode() {
    showOrganizeMode = true;
    // El botón "Organizar Botones" se manejará en el tab de Botones
    // Este es solo un placeholder
  }
</script>

<div class="space-y-6">
  <!-- Tema -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Palette size={18} class="text-blue-400" />
      <h3 class="text-sm font-semibold text-white">Tema de Grid</h3>
    </div>
    
    <div class="space-y-2">
      {#each themes as theme}
        <label class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all
          {selectedTheme === theme.value
            ? 'bg-blue-500/10 border-blue-500/30'
            : 'bg-[#252526] border-white/10 hover:border-white/20'}">
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
            <div class="w-2 h-2 rounded-full bg-blue-500"></div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <!-- Fuente -->
  <div>
    <div class="flex items-center gap-2 mb-3">
      <Type size={18} class="text-green-400" />
      <h3 class="text-sm font-semibold text-white">Fuente</h3>
    </div>
    
    <div class="space-y-2">
      {#each fonts as font}
        <label class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all
          {selectedFont === font.value
            ? 'bg-green-500/10 border-green-500/30'
            : 'bg-[#252526] border-white/10 hover:border-white/20'}">
          <input
            type="radio"
            name="font"
            value={font.value}
            checked={selectedFont === font.value}
            onchange={() => handleFontChange(font.value)}
            class="w-4 h-4 text-green-500 bg-[#1e1e1e] border-white/20 focus:ring-green-500 focus:ring-2"
          />
          <span class="text-sm text-white flex-1 {font.class}">{font.label}</span>
          {#if selectedFont === font.value}
            <div class="w-2 h-2 rounded-full bg-green-500"></div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <!-- Preview -->
  <div class="p-4 bg-[#252526] border border-white/10 rounded-lg">
    <div class="flex items-center gap-2 mb-2">
      <Wand2 size={16} class="text-purple-400" />
      <span class="text-xs font-medium text-purple-400">Vista Previa</span>
    </div>
    <div class="text-sm text-gray-400">
      Los cambios se aplican inmediatamente a la grid
    </div>
  </div>
</div>