<script lang="ts">
  import { generalSettings } from "$lib/stores/settingsStore";
  import { scale } from "svelte/transition";
  import { CloudRain, Check, X, Layout, Mountain } from "lucide-svelte";

  function toggleWeather() {
    $generalSettings.enableWeatherEffects =
      !$generalSettings.enableWeatherEffects;
  }

  function toggleCards() {
    $generalSettings.showWelcomeCards = !$generalSettings.showWelcomeCards;
  }

  function toggleBackground() {
    $generalSettings.showBackground = !$generalSettings.showBackground;
  }
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-8">
    <h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2>
    <p class="text-secondary mt-1">
      Configura las preferencias globales del sistema.
    </p>
  </div>

  <div class="grid gap-4 max-w-3xl pb-8">
    <!-- Visual Customization Card (All Toggles) -->
    <div class="card-base p-6">
      <div class="flex items-center gap-4 mb-6">
        <div
          class="p-3 rounded-lg bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400"
        >
          <Layout size={24} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Personalización</h3>
          <p class="text-sm text-secondary mt-1">
            Configura todos los elementos visuales de la pantalla.
          </p>
        </div>
      </div>

      <div class="space-y-4">
        <!-- Weather Effect Toggle -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div
              class="p-2 rounded-md bg-blue-50 text-blue-500/80 dark:bg-blue-900/20"
            >
              <CloudRain size={18} />
            </div>
            <span class="text-secondary font-medium">Efectos Climáticos</span>
          </div>
          <button
            on:click={toggleWeather}
            class="relative inline-flex h-8 w-14 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2
            {$generalSettings.enableWeatherEffects
              ? 'bg-green-500'
              : 'bg-gray-300 dark:bg-gray-700'}"
          >
            <span class="sr-only">Activar efectos climáticos</span>
            <span
              class="pointer-events-none inline-block h-7 w-7 transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
              {$generalSettings.enableWeatherEffects
                ? 'translate-x-6'
                : 'translate-x-0'}"
            >
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.enableWeatherEffects
                  ? 'opacity-100 duration-200 ease-in'
                  : 'opacity-0 duration-100 ease-out'}"
              >
                <Check size={14} class="text-green-600" strokeWidth={3} />
              </span>
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.enableWeatherEffects
                  ? 'opacity-0 duration-100 ease-out'
                  : 'opacity-100 duration-200 ease-in'}"
              >
                <X size={14} class="text-gray-400" strokeWidth={3} />
              </span>
            </span>
          </button>
        </div>

        <!-- Cards Toggle -->
        <div
          class="flex items-center justify-between pt-4 border-t border-emphasis"
        >
          <div class="flex items-center gap-3">
            <div
              class="p-2 rounded-md bg-purple-50 text-purple-500/80 dark:bg-purple-900/20"
            >
              <Layout size={18} />
            </div>
            <span class="text-secondary font-medium">Mostrar Tarjetas</span>
          </div>
          <button
            on:click={toggleCards}
            class="relative inline-flex h-8 w-14 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-purple-500 focus-visible:ring-offset-2
            {$generalSettings.showWelcomeCards
              ? 'bg-green-500'
              : 'bg-gray-300 dark:bg-gray-700'}"
          >
            <span class="sr-only">Mostrar tarjetas</span>
            <span
              class="pointer-events-none inline-block h-7 w-7 transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
              {$generalSettings.showWelcomeCards
                ? 'translate-x-6'
                : 'translate-x-0'}"
            >
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.showWelcomeCards
                  ? 'opacity-100 duration-200 ease-in'
                  : 'opacity-0 duration-100 ease-out'}"
              >
                <Check size={14} class="text-green-600" strokeWidth={3} />
              </span>
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.showWelcomeCards
                  ? 'opacity-0 duration-100 ease-out'
                  : 'opacity-100 duration-200 ease-in'}"
              >
                <X size={14} class="text-gray-400" strokeWidth={3} />
              </span>
            </span>
          </button>
        </div>

        <!-- Background Toggle -->
        <div
          class="flex items-center justify-between pt-4 border-t border-emphasis"
        >
          <div class="flex items-center gap-3">
            <div
              class="p-2 rounded-md bg-emerald-50 text-emerald-500/80 dark:bg-emerald-900/20"
            >
              <Mountain size={18} />
            </div>
            <span class="text-secondary font-medium">Mostrar Paisaje</span>
          </div>
          <button
            on:click={toggleBackground}
            class="relative inline-flex h-8 w-14 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-purple-500 focus-visible:ring-offset-2
            {$generalSettings.showBackground
              ? 'bg-green-500'
              : 'bg-gray-300 dark:bg-gray-700'}"
          >
            <span class="sr-only">Mostrar paisaje</span>
            <span
              class="pointer-events-none inline-block h-7 w-7 transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
              {$generalSettings.showBackground
                ? 'translate-x-6'
                : 'translate-x-0'}"
            >
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.showBackground
                  ? 'opacity-100 duration-200 ease-in'
                  : 'opacity-0 duration-100 ease-out'}"
              >
                <Check size={14} class="text-green-600" strokeWidth={3} />
              </span>
              <span
                class="absolute inset-0 flex h-full w-full items-center justify-center transition-opacity
                {$generalSettings.showBackground
                  ? 'opacity-0 duration-100 ease-out'
                  : 'opacity-100 duration-200 ease-in'}"
              >
                <X size={14} class="text-gray-400" strokeWidth={3} />
              </span>
            </span>
          </button>
        </div>
      </div>
    </div>

    <!-- Time Control (God Mode) - Moved UP -->
    <div class="card-base p-6" in:scale={{ duration: 300, delay: 150 }}>
      <div class="flex items-center justify-between mb-4">
        <div>
          <h3 class="text-lg font-semibold text-primary">Ciclo Solar/Lunar</h3>
          <p class="text-sm text-secondary">
            Controla el tiempo manualmente (⚡).
          </p>
        </div>
        {#if $generalSettings.overrideHour !== null}
          <button
            class="text-xs text-accent hover:underline"
            on:click={() => ($generalSettings.overrideHour = null)}
          >
            Restaurar a Tiempo Real
          </button>
        {/if}
      </div>

      <div class="flex items-center gap-4">
        <span class="text-sm font-mono text-secondary w-12">
          {$generalSettings.overrideHour !== null
            ? Math.floor($generalSettings.overrideHour ?? 0)
                .toString()
                .padStart(2, "0") + ":00"
            : "Auto"}
        </span>
        <input
          type="range"
          min="0"
          max="23"
          step="0.5"
          class="w-full h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-primary"
          value={$generalSettings.overrideHour ?? new Date().getHours()}
          on:input={(e) =>
            ($generalSettings.overrideHour = parseFloat(e.currentTarget.value))}
        />
        <span class="text-xl">
          {$generalSettings.overrideHour != null &&
          ($generalSettings.overrideHour ?? 0) >= 6 &&
          ($generalSettings.overrideHour ?? 0) < 18
            ? "☀️"
            : "🌙"}
        </span>
      </div>
    </div>

    <!-- Preview Controls - Moved DOWN -->
    {#if $generalSettings.enableWeatherEffects}
      <div class="card-base p-6 mt-2" in:scale={{ duration: 300, delay: 100 }}>
        <h3 class="text-lg font-semibold text-primary mb-4">Vista Previa</h3>
        <p class="text-sm text-secondary mb-4">
          Selecciona un efecto para previsualizarlo instantáneamente.
        </p>

        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-2">
          <button
            class="btn-base w-full justify-center {$generalSettings.overrideSeason ===
            null
              ? 'bg-primary text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = null)}
          >
            Auto
          </button>

          <button
            class="btn-base w-full justify-center {$generalSettings.overrideSeason ===
            'winter'
              ? 'bg-blue-500 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "winter")}
          >
            Invierno ❄️
          </button>

          <button
            class="btn-base w-full justify-center {$generalSettings.overrideSeason ===
            'spring'
              ? 'bg-pink-400 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "spring")}
          >
            Primavera 🌸
          </button>

          <button
            class="btn-base w-full justify-center {$generalSettings.overrideSeason ===
            'summer'
              ? 'bg-yellow-500 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "summer")}
          >
            Verano ✨
          </button>

          <button
            class="btn-base w-full justify-center {$generalSettings.overrideSeason ===
            'autumn'
              ? 'bg-orange-500 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "autumn")}
          >
            Otoño 🍂
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
