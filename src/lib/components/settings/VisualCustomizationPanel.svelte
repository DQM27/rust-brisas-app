<script lang="ts">
  import { fade, scale, slide } from "svelte/transition"; // Added slide
  import {
    X,
    Sparkles,
    Sliders,
    Zap,
    Maximize,
    Circle,
    CloudRain,
  } from "lucide-svelte";
  import { particleSettings } from "$lib/stores/particleSettingsStore";

  export let onClose: () => void = () => {};
  export let embedded = false;
  export let mode: "bokeh" | "weather" = "bokeh"; // New prop

  // Helper to format percentage
  function pct(value: number): string {
    return Math.round(value * 100) + "%";
  }
</script>

{#if !embedded}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
  >
    <div
      class="w-full max-w-md bg-surface-1 rounded-2xl shadow-2xl border border-white/10 overflow-hidden"
      transition:scale={{ duration: 300, start: 0.95 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-4 border-b border-emphasis bg-surface-2"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-pink-500/10 text-pink-500">
            <Sliders size={20} />
          </div>
          <div>
            <h2 class="text-lg font-bold text-primary">
              Personalización Avanzada
            </h2>
            <p class="text-xs text-secondary">
              Ajuste fino de efectos visuales
            </p>
          </div>
        </div>
        <button
          class="p-2 hover:bg-white/10 rounded-full transition-colors text-secondary hover:text-primary"
          on:click={onClose}
        >
          <X size={20} />
        </button>
      </div>

      <!-- Content Wrapper for Modal -->
      <div class="p-6 space-y-8 max-h-[70vh] overflow-y-auto custom-scrollbar">
        {@render content()}
      </div>

      <!-- Footer for Modal -->
      <div
        class="p-4 bg-surface-2 border-t border-emphasis flex justify-end gap-3"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-secondary hover:text-primary hover:bg-white/5 rounded-lg transition-colors"
          on:click={particleSettings.reset}
        >
          Restaurar Originales
        </button>
        <button
          class="px-4 py-2 text-sm font-medium bg-primary text-primary-inverse rounded-lg hover:brightness-110 shadow-lg shadow-primary/20 transition-all"
          on:click={onClose}
        >
          Listo
        </button>
      </div>
    </div>
  </div>
{:else}
  <!-- Inline Version -->
  <div
    class="w-full bg-surface-2/50 rounded-xl border border-white/5 p-4 mt-2"
    transition:slide
  >
    {@render content()}

    <div class="flex justify-end pt-4 mt-4 border-t border-white/5">
      <button
        class="text-xs text-secondary hover:text-primary hover:underline transition-colors"
        on:click={particleSettings.reset}
      >
        Restaurar Valores por Defecto
      </button>
    </div>
  </div>
{/if}

{#snippet content()}
  {#if mode === "bokeh"}
    <!-- Section: Bokeh Effect -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <Sparkles size={16} class="text-pink-400" />
        <h3 class="text-sm font-semibold text-primary uppercase tracking-wider">
          Efecto Bokeh (Flent)
        </h3>
      </div>

      <!-- Count -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Cantidad de Partículas</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{$particleSettings.bokehCount}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="150"
          step="1"
          bind:value={$particleSettings.bokehCount}
          on:input={() =>
            particleSettings.updateBokehCount($particleSettings.bokehCount)}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer"
        />
      </div>

      <!-- Opacity -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Opacidad Máxima</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{pct($particleSettings.bokehMaxOpacity)}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="1"
          step="0.05"
          bind:value={$particleSettings.bokehMaxOpacity}
          on:input={() =>
            particleSettings.updateBokehOpacity(
              $particleSettings.bokehMaxOpacity,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer"
        />
      </div>

      <!-- Size Range -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Tamaño (Mín - Máx)</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{$particleSettings.bokehMinSize}px - {$particleSettings.bokehMaxSize}px</span
          >
        </div>
        <div class="flex gap-4">
          <input
            type="range"
            min="5"
            max="100"
            step="5"
            bind:value={$particleSettings.bokehMinSize}
            class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer flex-1"
            title="Tamaño Mínimo"
          />
          <input
            type="range"
            min="5"
            max="200"
            step="5"
            bind:value={$particleSettings.bokehMaxSize}
            class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer flex-1"
            title="Tamaño Máximo"
          />
        </div>
      </div>

      <!-- Speed Multiplier -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Velocidad del Bokeh</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{Math.round($particleSettings.bokehSpeedMultiplier * 100)}%</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="5"
          step="0.1"
          bind:value={$particleSettings.bokehSpeedMultiplier}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
        />
      </div>
    </div>
  {:else if mode === "weather"}
    <!-- Section: Weather Effect -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <CloudRain size={16} class="text-blue-400" />
        <h3 class="text-sm font-semibold text-primary uppercase tracking-wider">
          Clima (Lluvia/Nieve/Polen)
        </h3>
      </div>

      <!-- Density -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Densidad de Partículas</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.weatherDensityMultiplier}</span
          >
        </div>
        <input
          type="range"
          min="0.1"
          max="3"
          step="0.1"
          bind:value={$particleSettings.weatherDensityMultiplier}
          on:input={() =>
            particleSettings.updateWeatherDensity(
              $particleSettings.weatherDensityMultiplier,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>

      <!-- Speed -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Velocidad de Caída/Vuelo</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.weatherSpeedMultiplier}</span
          >
        </div>
        <input
          type="range"
          min="0.1"
          max="3"
          step="0.1"
          bind:value={$particleSettings.weatherSpeedMultiplier}
          on:input={() =>
            particleSettings.updateWeatherSpeed(
              $particleSettings.weatherSpeedMultiplier,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>

      <!-- Size -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Tamaño</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.weatherSizeMultiplier}</span
          >
        </div>
        <input
          type="range"
          min="0.5"
          max="3"
          step="0.1"
          bind:value={$particleSettings.weatherSizeMultiplier}
          on:input={() =>
            particleSettings.updateWeatherSize(
              $particleSettings.weatherSizeMultiplier,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>

      <!-- Wind Influence -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Influencia Viento</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.weatherWindInfluence}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="3"
          step="0.1"
          bind:value={$particleSettings.weatherWindInfluence}
          on:input={() =>
            particleSettings.updateWeatherWind(
              $particleSettings.weatherWindInfluence,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>

      <!-- Turbulence -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Turbulencia / Caos</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.weatherTurbulence}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="3"
          step="0.1"
          bind:value={$particleSettings.weatherTurbulence}
          on:input={() =>
            particleSettings.updateWeatherTurbulence(
              $particleSettings.weatherTurbulence,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>
    </div>
  {/if}
{/snippet}

<style lang="postcss">
  /* Badge styling moved to inline classes */

  .custom-scrollbar::-webkit-scrollbar {
    width: 6px;
  }

  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 9999px;
  }

  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background-color: rgba(255, 255, 255, 0.2);
  }

  /* Slider Styling */
  /* .range-slider moved to inline classes */

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1rem;
    height: 1rem;
    border-radius: 9999px;
    background: white;
    box-shadow:
      0 4px 6px -1px rgb(0 0 0 / 0.1),
      0 2px 4px -2px rgb(0 0 0 / 0.1);
    transition: transform 0.15s ease;
  }

  input[type="range"]::-webkit-slider-thumb:hover {
    transform: scale(1.1);
  }
</style>
