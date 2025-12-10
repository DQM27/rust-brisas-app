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
    Sun,
    Moon,
    Cloud,
    Wind,
    Waves,
    Trees,
    Building2,
    Umbrella,
    Mountain,
  } from "lucide-svelte";
  import { generalSettings } from "$lib/stores/settingsStore";
  import { particleSettings } from "$lib/stores/particleSettingsStore";
  import { LANDSCAPE_TYPES } from "$lib/components/visual/systems/landscapeData";

  export let onClose: () => void = () => {};
  export let embedded = false;
  export let mode:
    | "bokeh"
    | "weather"
    | "celestial"
    | "stars"
    | "clouds"
    | "landscape" = "bokeh"; // New prop

  // Helper to format percentage
  function pct(value: number): string {
    return Math.round(value * 100) + "%";
  }

  const MOON_PHASES = [
    { id: "new", label: "🌑", name: "Nueva" },
    { id: "waxing-crescent", label: "🌒", name: "Creciente" },
    { id: "first-quarter", label: "🌓", name: "Cuarto Creciente" },
    { id: "waxing-gibbous", label: "🌔", name: "Gibosa Creciente" },
    { id: "full", label: "🌕", name: "Llena" },
    { id: "waning-gibbous", label: "🌖", name: "Gibosa Menguante" },
    { id: "last-quarter", label: "🌗", name: "Cuarto Menguante" },
    { id: "waning-crescent", label: "🌘", name: "Menguante" },
  ];
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
          onclick={onClose}
          aria-label="Cerrar"
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
          onclick={particleSettings.reset}
        >
          Restaurar Originales
        </button>
        <button
          class="px-4 py-2 text-sm font-medium bg-primary text-primary-inverse rounded-lg hover:brightness-110 shadow-lg shadow-primary/20 transition-all"
          onclick={onClose}
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
        onclick={particleSettings.reset}
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
          oninput={() =>
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
          oninput={() =>
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
          oninput={() =>
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
          oninput={() =>
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
          oninput={() =>
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
          oninput={() =>
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
          oninput={() =>
            particleSettings.updateWeatherTurbulence(
              $particleSettings.weatherTurbulence,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
        />
      </div>
    </div>
  {:else if mode === "celestial"}
    <!-- Section: Celestial -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <Sun size={16} class="text-amber-400" />
        <h3 class="text-sm font-semibold text-primary uppercase tracking-wider">
          Cuerpos Celestes
        </h3>
      </div>

      <!-- Moon Phase -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Fase Lunar</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5 capitalize"
            >{MOON_PHASES.find((p) => p.id === $particleSettings.moonPhase)
              ?.name ?? $particleSettings.moonPhase}</span
          >
        </div>
        <div class="grid grid-cols-4 gap-2">
          {#each MOON_PHASES as phase}
            <button
              class="flex flex-col items-center p-2 rounded-lg border transition-all text-center
                {$particleSettings.moonPhase === phase.id
                ? 'bg-primary/20 border-primary text-primary'
                : 'bg-surface-3 border-transparent text-secondary hover:bg-surface-hover'}"
              onclick={() => particleSettings.updateMoonPhase(phase.id)}
              title={phase.name}
            >
              <span class="text-lg leading-none mb-1">{phase.label}</span>
            </button>
          {/each}
        </div>
      </div>

      <!-- Sun Style -->
      <div class="control-group mt-6">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Estilo del Sol</span>
        </div>
        <div class="flex gap-3">
          <button
            class="flex-1 flex items-center justify-center gap-2 p-3 rounded-lg border transition-all
                {$particleSettings.sunStyle === 'normal'
              ? 'bg-amber-500/20 border-amber-500 text-amber-500'
              : 'bg-surface-3 border-transparent text-secondary hover:bg-surface-hover'}"
            onclick={() => particleSettings.updateSunStyle("normal")}
          >
            <Sun size={18} />
            <span class="text-sm font-medium">Normal</span>
          </button>
          <button
            class="flex-1 flex items-center justify-center gap-2 p-3 rounded-lg border transition-all
                {$particleSettings.sunStyle === 'cloudy'
              ? 'bg-blue-400/20 border-blue-400 text-blue-400'
              : 'bg-surface-3 border-transparent text-secondary hover:bg-surface-hover'}"
            onclick={() => particleSettings.updateSunStyle("cloudy")}
          >
            <div class="relative">
              <Sun
                size={18}
                class="text-amber-400 absolute -top-1 -left-1 opacity-50"
              />
              <CloudRain size={18} />
            </div>
            <span class="text-sm font-medium">Con Nubes</span>
          </button>
        </div>
      </div>
    </div>
  {:else if mode === "stars"}
    <!-- Section: Stars & Shooting Stars -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <Sparkles size={16} class="text-yellow-200" />
        <h3 class="text-sm font-semibold text-primary uppercase tracking-wider">
          Estrellas & Fugaces
        </h3>
      </div>

      <!-- Meteor Shower Toggle -->
      <div
        class="control-group bg-yellow-400/10 border border-yellow-400/20 p-3 rounded-lg mb-4"
      >
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <Zap size={16} class="text-yellow-400" />
            <span class="text-sm font-medium text-yellow-100"
              >Lluvia de Meteoros</span
            >
          </div>
          <button
            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-yellow-400 focus:ring-offset-2 focus:ring-offset-gray-900
              {$particleSettings.meteorShowerEnabled
              ? 'bg-yellow-400'
              : 'bg-surface-3'}"
            onclick={particleSettings.toggleMeteorShower}
            aria-label="Toggle Meteor Shower"
          >
            <span
              class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform
              {$particleSettings.meteorShowerEnabled
                ? 'translate-x-6'
                : 'translate-x-1'}"
            ></span>
          </button>
        </div>
        {#if $particleSettings.meteorShowerEnabled}
          <p class="text-xs text-yellow-200/70 mt-2 ml-1" transition:slide>
            ¡Prepárate para el espectáculo! (Frecuencia x100)
          </p>
        {/if}
      </div>

      <!-- Star Count -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Cantidad Estrellas</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.starCountMultiplier}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="2"
          step="0.1"
          bind:value={$particleSettings.starCountMultiplier}
          oninput={() =>
            particleSettings.updateStarCount(
              $particleSettings.starCountMultiplier,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-amber-300"
        />
      </div>

      <!-- Star Twinkle -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Vel. Parpadeo</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.starTwinkleSpeed}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="3"
          step="0.1"
          bind:value={$particleSettings.starTwinkleSpeed}
          oninput={() =>
            particleSettings.updateStarTwinkle(
              $particleSettings.starTwinkleSpeed,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-amber-300"
        />
      </div>

      <!-- Shooting Star Freq -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Frecuencia Fugaces</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.shootingStarFrequency}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="5"
          step="0.5"
          bind:value={$particleSettings.shootingStarFrequency}
          oninput={() =>
            particleSettings.updateShootingStarFreq(
              $particleSettings.shootingStarFrequency,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-yellow-200"
        />
      </div>

      <!-- Shooting Star Speed -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Velocidad Fugaces</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.shootingStarSpeed}</span
          >
        </div>
        <input
          type="range"
          min="0.5"
          max="3"
          step="0.1"
          bind:value={$particleSettings.shootingStarSpeed}
          oninput={() =>
            particleSettings.updateShootingStarSpeed(
              $particleSettings.shootingStarSpeed,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-yellow-200"
        />
      </div>
    </div>
  {:else if mode === "landscape"}
    <div class="space-y-4">
      <div class="grid grid-cols-2 gap-3">
        {#each LANDSCAPE_TYPES as biome}
          <button
            class="flex flex-col items-center justify-center gap-2 p-3 rounded-xl border transition-all duration-200
            {$generalSettings.landscapeType === biome.id
              ? 'bg-blue-500/10 border-blue-500 text-blue-500'
              : 'bg-surface-2 border-white/5 text-secondary hover:bg-surface-3 hover:border-white/10'}"
            onclick={() => generalSettings.setLandscapeType(biome.id)}
          >
            <!-- Dynamic Icon Rendering -->
            {#if biome.icon === "Mountain"}
              <Mountain size={24} />
            {:else if biome.icon === "Trees"}
              <Trees size={24} />
            {:else if biome.icon === "Building2"}
              <Building2 size={24} />
            {:else if biome.icon === "Sun"}
              <Sun size={24} />
            {:else if biome.icon === "Umbrella"}
              <Umbrella size={24} />
            {:else if biome.icon === "Moon"}
              <Moon size={24} />
            {/if}

            <span class="text-xs font-medium">{biome.name}</span>
          </button>
        {/each}
      </div>

      <p class="text-xs text-secondary/60 text-center mt-2 px-2">
        El paisaje cambia dinámicamente según la hora del día y la estación del
        año.
      </p>
    </div>
  {:else if mode === "clouds"}
    <!-- Section: Cloud Customization -->
    <div class="space-y-4">
      <div class="flex items-center gap-2 mb-2">
        <Cloud size={16} class="text-blue-200" />
        <h3 class="text-sm font-semibold text-primary uppercase tracking-wider">
          Nubes
        </h3>
      </div>

      <!-- Cloud Style (Cartoon vs Soft) -->
      <div class="control-group">
        <span class="text-sm text-secondary block mb-2">Estilo de Nube</span>
        <div class="grid grid-cols-2 gap-2">
          <button
            class="px-3 py-2 rounded-lg text-xs font-medium border transition-all text-center
              {$particleSettings.cloudStyle === 'cartoon'
              ? 'bg-blue-500/20 border-blue-500 text-blue-200'
              : 'bg-surface-3 border-white/5 text-secondary hover:bg-surface-2'}"
            onclick={() => particleSettings.updateCloudStyle("cartoon")}
          >
            Cartoon
          </button>
          <button
            class="px-3 py-2 rounded-lg text-xs font-medium border transition-all text-center
              {$particleSettings.cloudStyle === 'soft'
              ? 'bg-blue-500/20 border-blue-500 text-blue-200'
              : 'bg-surface-3 border-white/5 text-secondary hover:bg-surface-2'}"
            onclick={() => particleSettings.updateCloudStyle("soft")}
          >
            Suave
          </button>
        </div>
      </div>

      <!-- Cloud Opacity -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Opacidad</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{Math.round($particleSettings.cloudOpacity * 100)}%</span
          >
        </div>
        <input
          type="range"
          min="0.1"
          max="1.0"
          step="0.05"
          bind:value={$particleSettings.cloudOpacity}
          oninput={() =>
            particleSettings.updateCloudOpacity($particleSettings.cloudOpacity)}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-400"
        />
      </div>

      <!-- Cloud Count -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <span class="text-sm text-secondary">Cantidad</span>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{$particleSettings.cloudCount}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="10"
          step="1"
          bind:value={$particleSettings.cloudCount}
          oninput={() =>
            particleSettings.updateCloudCount($particleSettings.cloudCount)}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-400"
        />
      </div>

      <!-- Cloud Wind Speed -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <div class="flex items-center gap-2">
            <Wind size={14} class="text-secondary" />
            <span class="text-sm text-secondary">Viento</span>
          </div>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >x{$particleSettings.cloudWindSpeed}</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="3"
          step="0.1"
          bind:value={$particleSettings.cloudWindSpeed}
          oninput={() =>
            particleSettings.updateCloudWindSpeed(
              $particleSettings.cloudWindSpeed,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-400"
        />
      </div>

      <!-- Cloud Turbulence -->
      <div class="control-group">
        <div class="flex justify-between items-center mb-2">
          <div class="flex items-center gap-2">
            <Waves size={14} class="text-secondary" />
            <span class="text-sm text-secondary">Turbulencia</span>
          </div>
          <span
            class="px-2 py-0.5 rounded text-xs font-mono bg-surface-3 text-secondary border border-white/5"
            >{Math.round($particleSettings.cloudTurbulence * 100)}%</span
          >
        </div>
        <input
          type="range"
          min="0"
          max="1.0"
          step="0.05"
          bind:value={$particleSettings.cloudTurbulence}
          oninput={() =>
            particleSettings.updateCloudTurbulence(
              $particleSettings.cloudTurbulence,
            )}
          class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-400"
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
