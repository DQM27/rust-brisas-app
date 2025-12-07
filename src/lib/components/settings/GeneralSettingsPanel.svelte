<script lang="ts">
  import { generalSettings } from "$lib/stores/settingsStore";
  import { scale } from "svelte/transition";
  import { CloudRain, Check, X } from "lucide-svelte";

  // Toggle function
  function toggleWeather() {
    $generalSettings.enableWeatherEffects =
      !$generalSettings.enableWeatherEffects;
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

  <div class="grid gap-6 max-w-3xl">
    <!-- Weather Effects Card -->
    <div class="card-base p-6 flex items-center justify-between">
      <div class="flex items-start gap-4">
        <div
          class="p-3 rounded-lg bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"
        >
          <CloudRain size={24} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Efectos Climáticos</h3>
          <p class="text-sm text-secondary mt-1 max-w-md">
            Habilita animaciones estacionales (nieve, lluvia, hojas) en la
            pantalla de bienvenida. Desactivar esto puede mejorar el rendimiento
            en equipos antiguos.
          </p>
        </div>
      </div>

      <!-- Toggle Switch -->
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

    <!-- Preview Controls -->
    {#if $generalSettings.enableWeatherEffects}
      <div class="card-base p-6" in:scale={{ duration: 300, delay: 100 }}>
        <h3 class="text-lg font-semibold text-primary mb-4">Vista Previa</h3>
        <p class="text-sm text-secondary mb-4">
          Selecciona un efecto para previsualizarlo instantáneamente.
        </p>

        <div class="flex flex-wrap gap-3">
          <button
            class="btn-base {$generalSettings.overrideSeason === null
              ? 'bg-primary text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = null)}
          >
            Automático
          </button>

          <button
            class="btn-base {$generalSettings.overrideSeason === 'winter'
              ? 'bg-blue-500 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "winter")}
          >
            Invierno ❄️
          </button>

          <button
            class="btn-base {$generalSettings.overrideSeason === 'spring'
              ? 'bg-pink-400 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "spring")}
          >
            Primavera 🌸
          </button>

          <button
            class="btn-base {$generalSettings.overrideSeason === 'summer'
              ? 'bg-yellow-500 text-white'
              : 'bg-surface-2 hover:bg-surface-hover'}"
            on:click={() => ($generalSettings.overrideSeason = "summer")}
          >
            Verano ✨
          </button>

          <button
            class="btn-base {$generalSettings.overrideSeason === 'autumn'
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
