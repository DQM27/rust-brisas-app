<script lang="ts">
  import { slide } from "svelte/transition";
  import { particleSettings } from "$lib/stores/particleSettingsStore";
  import { generalSettings, type Season } from "$lib/stores/settingsStore";
  import { currentTime } from "$lib/stores/timeStore";
  import { currentSeason } from "$lib/utils/season";
  import { TIME } from "$lib/components/visual/constants";
  import { X, RotateCcw, Sliders } from "lucide-svelte";

  export let onClose: () => void;
  export let mode: "weather" | "bokeh" = "weather";

  const MIN_OPACITY = 0.05;
  const MAX_OPACITY = 1.0;

  // Determine active context
  $: effectiveHour =
    $generalSettings.overrideHour ??
    $currentTime.getHours() + $currentTime.getMinutes() / 60;
  $: activeSeason = ($generalSettings.overrideSeason ??
    $currentSeason) as Season;
  $: isNight =
    effectiveHour >= TIME.DUSK_END || effectiveHour < TIME.DAWN_START;

  $: activeWeatherKey =
    activeSeason === "summer" && isNight ? "summerNight" : activeSeason;

  $: activeWeatherConfig =
    activeWeatherKey === "summerNight"
      ? $particleSettings.weather.summer.nightVariant
      : $particleSettings.weather[activeSeason];

  function updateWeatherConfig(changes: any) {
    // @ts-ignore - Valid keys constrained by store logic
    particleSettings.updateWeather(activeWeatherKey, changes);
  }

  function resetSettings() {
    if (confirm("¿Restaurar valores por defecto?")) {
      particleSettings.reset();
    }
  }

  $: title = mode === "weather" ? "Personalizar Clima" : "Personalizar Bokeh";
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-[2px]"
  on:click={onClose}
  role="button"
  tabindex="0"
  on:keydown={(e) => e.key === "Escape" && onClose()}
>
  <div
    class="w-full max-w-md bg-surface-1 rounded-xl shadow-2xl overflow-hidden border border-white/10"
    role="dialog"
    aria-modal="true"
    transition:slide={{ duration: 200 }}
    on:click={(e) => e.stopPropagation()}
    tabindex="0"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between p-4 border-b border-surface-2 bg-surface-2"
    >
      <div class="flex items-center gap-2">
        <div
          class="p-2 rounded-lg bg-pink-100 text-pink-600 dark:bg-pink-900/30 dark:text-pink-400"
        >
          <Sliders size={20} />
        </div>
        <h2 class="text-lg font-bold text-primary">{title}</h2>
      </div>
      <button
        class="p-2 rounded-full hover:bg-surface-3 transition-colors text-secondary"
        on:click={onClose}
      >
        <X size={20} />
      </button>
    </div>

    <!-- Content -->
    <div class="p-6 max-h-[70vh] overflow-y-auto space-y-6">
      <!-- Section: Weather (Dynamic based on current season) -->
      {#if mode === "weather"}
        <div>
          <div class="flex items-center justify-between mb-4">
            <h3
              class="text-sm font-semibold text-secondary uppercase tracking-wider"
            >
              {#if activeWeatherKey === "summerNight"}
                Luciérnagas (Verano Noche)
              {:else if activeWeatherKey === "summer"}
                Polen (Verano Día)
              {:else if activeWeatherKey === "winter"}
                Nieve (Invierno)
              {:else if activeWeatherKey === "spring"}
                Pétalos (Primavera)
              {:else}
                Hojas (Otoño)
              {/if}
            </h3>
            <div class="h-px flex-1 bg-surface-3 ml-4"></div>
          </div>

          {#if activeWeatherConfig}
            <div class="space-y-5">
              <!-- Count -->
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-secondary">Cantidad de Partículas</span>
                  <span class="font-mono font-medium text-primary"
                    >{activeWeatherConfig.count}</span
                  >
                </div>
                <input
                  type="range"
                  min="0"
                  max="200"
                  step="5"
                  class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
                  value={activeWeatherConfig.count}
                  on:input={(e) =>
                    updateWeatherConfig({ count: +e.currentTarget.value })}
                />
              </div>

              <!-- Size Range -->
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-secondary">Tamaño (Min - Max)</span>
                  <span class="font-mono font-medium text-primary"
                    >{activeWeatherConfig.sizeRange[0]} - {activeWeatherConfig
                      .sizeRange[1]}</span
                  >
                </div>
                <div class="flex gap-2">
                  <input
                    type="range"
                    min="1"
                    max="20"
                    class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
                    value={activeWeatherConfig.sizeRange[0]}
                    on:input={(e) =>
                      updateWeatherConfig({
                        sizeRange: [
                          +e.currentTarget.value,
                          activeWeatherConfig.sizeRange[1],
                        ],
                      })}
                  />
                  <input
                    type="range"
                    min="1"
                    max="30"
                    class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
                    value={activeWeatherConfig.sizeRange[1]}
                    on:input={(e) =>
                      updateWeatherConfig({
                        sizeRange: [
                          activeWeatherConfig.sizeRange[0],
                          +e.currentTarget.value,
                        ],
                      })}
                  />
                </div>
              </div>

              <!-- Speed Range -->
              <div class="space-y-2">
                <div class="flex justify-between text-sm">
                  <span class="text-secondary">Velocidad (Min - Max)</span>
                  <span class="font-mono font-medium text-primary"
                    >{activeWeatherConfig.speedRange[0]} - {activeWeatherConfig
                      .speedRange[1]}</span
                  >
                </div>
                <div class="flex gap-2">
                  <input
                    type="range"
                    min="0.01"
                    max="2"
                    step="0.01"
                    class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
                    value={activeWeatherConfig.speedRange[0]}
                    on:input={(e) =>
                      updateWeatherConfig({
                        speedRange: [
                          +e.currentTarget.value,
                          activeWeatherConfig.speedRange[1],
                        ],
                      })}
                  />
                  <input
                    type="range"
                    min="0.01"
                    max="3"
                    step="0.01"
                    class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-blue-500"
                    value={activeWeatherConfig.speedRange[1]}
                    on:input={(e) =>
                      updateWeatherConfig({
                        speedRange: [
                          activeWeatherConfig.speedRange[0],
                          +e.currentTarget.value,
                        ],
                      })}
                  />
                </div>
              </div>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Section: Bokeh -->
      {#if mode === "bokeh"}
        <div>
          <div class="flex items-center justify-between mb-4">
            <h3
              class="text-sm font-semibold text-secondary uppercase tracking-wider"
            >
              Efecto Bokeh
            </h3>
            <div class="h-px flex-1 bg-surface-3 ml-4"></div>
          </div>

          <div class="space-y-5">
            <!-- Count -->
            <div class="space-y-2">
              <div class="flex justify-between text-sm">
                <span class="text-secondary">Cantidad de Partículas</span>
                <span class="font-mono font-medium text-primary"
                  >{$particleSettings.bokeh.count}</span
                >
              </div>
              <input
                type="range"
                min="0"
                max="100"
                step="1"
                class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
                bind:value={$particleSettings.bokeh.count}
              />
            </div>

            <!-- Size Range -->
            <div class="space-y-2">
              <div class="flex justify-between text-sm">
                <span class="text-secondary">Tamaño (Min - Max)</span>
                <span class="font-mono font-medium text-primary"
                  >{$particleSettings.bokeh.minSize}px - {$particleSettings
                    .bokeh.maxSize}px</span
                >
              </div>
              <div class="flex gap-2">
                <input
                  type="range"
                  min="5"
                  max="50"
                  class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
                  bind:value={$particleSettings.bokeh.minSize}
                />
                <input
                  type="range"
                  min="50"
                  max="150"
                  class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
                  bind:value={$particleSettings.bokeh.maxSize}
                />
              </div>
            </div>

            <!-- Opacity Range -->
            <div class="space-y-2">
              <div class="flex justify-between text-sm">
                <span class="text-secondary">Opacidad (Max)</span>
                <span class="font-mono font-medium text-primary"
                  >{Math.round($particleSettings.bokeh.maxOpacity * 100)}%</span
                >
              </div>
              <input
                type="range"
                min="0.1"
                max="1.0"
                step="0.05"
                class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
                bind:value={$particleSettings.bokeh.maxOpacity}
                style:background-size="{(($particleSettings.bokeh.maxOpacity -
                  0.1) *
                  100) /
                  0.9}% 100%"
              />
            </div>

            <!-- Speed Range -->
            <div class="space-y-2">
              <div class="flex justify-between text-sm">
                <span class="text-secondary">Velocidad</span>
                <span class="font-mono font-medium text-primary"
                  >{Math.round($particleSettings.bokeh.maxSpeed * 1000)}</span
                >
              </div>
              <input
                type="range"
                min="0.01"
                max="0.3"
                step="0.01"
                class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
                bind:value={$particleSettings.bokeh.maxSpeed}
              />
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div
      class="p-4 border-t border-surface-2 bg-surface-2 flex justify-between"
    >
      <button
        class="flex items-center gap-2 px-4 py-2 text-sm font-medium text-secondary hover:text-primary hover:bg-surface-3 rounded-lg transition-colors"
        on:click={resetSettings}
      >
        <RotateCcw size={16} />
        Resetear
      </button>

      <button
        class="px-6 py-2 bg-primary text-primary-content text-sm font-medium rounded-lg shadow-lg shadow-pink-500/20 hover:shadow-pink-500/30 transition-shadow bg-gradient-to-r from-pink-500 to-rose-500"
        on:click={onClose}
      >
        Listo
      </button>
    </div>
  </div>
</div>
