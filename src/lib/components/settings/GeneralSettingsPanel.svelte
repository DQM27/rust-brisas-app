<script lang="ts">
  import { generalSettings, type Season } from "$lib/stores/settingsStore";
  import { particleSettings } from "$lib/stores/particleSettingsStore";
  import { scale, slide } from "svelte/transition";
  import {
    CloudRain,
    Check,
    X,
    Layout,
    Mountain,
    Sun,
    Moon,
    Cloud,
    Sparkles,
    Star,
    Cake,
    RotateCcw,
    Eye,
    EyeOff,
    Type,
  } from "lucide-svelte";

  // ==========================================================================
  // Toggle Component (reusable)
  // ==========================================================================

  // Inline toggle to reduce repetition
  function Toggle(props: {
    checked: boolean;
    onChange: () => void;
    srLabel: string;
    accentColor?: string;
  }) {
    return props;
  }
</script>

<!-- Reusable Toggle Switch -->
{#snippet toggleSwitch(checked: boolean, onChange: () => void, srLabel: string)}
  <button
    onclick={onChange}
    class="relative inline-flex h-7 w-12 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2
    {checked ? 'bg-green-500' : 'bg-gray-300 dark:bg-gray-700'}"
  >
    <span class="sr-only">{srLabel}</span>
    <span
      class="pointer-events-none inline-flex h-6 w-6 items-center justify-center transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out
      {checked ? 'translate-x-5' : 'translate-x-0'}"
    >
      {#if checked}
        <Check size={12} class="text-green-600" strokeWidth={3} />
      {:else}
        <X size={12} class="text-gray-400" strokeWidth={3} />
      {/if}
    </span>
  </button>
{/snippet}

<!-- Setting Row -->
{#snippet settingRow(
  icon: any,
  iconBg: string,
  iconColor: string,
  label: string,
  checked: boolean,
  onChange: () => void,
)}
  <div class="flex items-center justify-between py-3">
    <div class="flex items-center gap-3">
      <div class="p-2 rounded-md {iconBg}">
        <svelte:component this={icon} size={18} class={iconColor} />
      </div>
      <span class="text-secondary font-medium">{label}</span>
    </div>
    {@render toggleSwitch(checked, onChange, label)}
  </div>
{/snippet}

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2>
    <p class="text-secondary mt-1">
      Configura las preferencias visuales del sistema.
    </p>
  </div>

  <div class="grid gap-4 max-w-3xl pb-8">
    <!-- ================================================================== -->
    <!-- VISUAL ELEMENTS CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400"
        >
          <Eye size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Elementos Visuales</h3>
          <p class="text-sm text-secondary">
            Activa o desactiva cada capa del paisaje.
          </p>
        </div>
      </div>

      <div class="divide-y divide-emphasis">
        {@render settingRow(
          Mountain,
          "bg-emerald-50 dark:bg-emerald-900/20",
          "text-emerald-500",
          "Paisaje de Montañas",
          $generalSettings.showBackground,
          () => generalSettings.toggleBackground(),
        )}

        {@render settingRow(
          Cloud,
          "bg-sky-50 dark:bg-sky-900/20",
          "text-sky-500",
          "Nubes Animadas",
          $generalSettings.showClouds,
          () => generalSettings.toggleClouds(),
        )}

        {@render settingRow(
          Star,
          "bg-indigo-50 dark:bg-indigo-900/20",
          "text-indigo-400",
          "Estrellas (Noche)",
          $generalSettings.showStars,
          () => generalSettings.toggleStars(),
        )}

        {@render settingRow(
          Sun,
          "bg-amber-50 dark:bg-amber-900/20",
          "text-amber-500",
          "Sol / Luna",
          $generalSettings.showCelestial,
          () => generalSettings.toggleCelestial(),
        )}

        {@render settingRow(
          CloudRain,
          "bg-blue-50 dark:bg-blue-900/20",
          "text-blue-500",
          "Efectos Climáticos",
          $generalSettings.enableWeatherEffects,
          () => generalSettings.toggleWeather(),
        )}

        {@render settingRow(
          Sparkles,
          "bg-pink-50 dark:bg-pink-900/20",
          "text-pink-500",
          "Efecto Bokeh",
          $generalSettings.showBokeh,
          () => generalSettings.toggleBokeh(),
        )}

        {#if $generalSettings.showBokeh}
          <div
            class="pl-12 pr-4 pt-1 pb-4 space-y-4"
            transition:slide={{ duration: 200 }}
          >
            <!-- Count Slider -->
            <div class="space-y-1">
              <div class="flex justify-between text-xs text-secondary">
                <span>Cantidad</span>
                <span>{$particleSettings.bokehCount}</span>
              </div>
              <input
                type="range"
                min="5"
                max="100"
                value={$particleSettings.bokehCount}
                oninput={(e) =>
                  particleSettings.updateBokehCount(
                    parseInt(e.currentTarget.value),
                  )}
                class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
              />
            </div>

            <!-- Opacity Slider -->
            <div class="space-y-1">
              <div class="flex justify-between text-xs text-secondary">
                <span>Opacidad Máxima</span>
                <span
                  >{Math.round($particleSettings.bokehMaxOpacity * 100)}%</span
                >
              </div>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={$particleSettings.bokehMaxOpacity}
                oninput={(e) =>
                  particleSettings.updateBokehOpacity(
                    parseFloat(e.currentTarget.value),
                  )}
                class="w-full h-1.5 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-pink-500"
              />
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- UI ELEMENTS CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center gap-4 mb-4">
        <div
          class="p-3 rounded-lg bg-violet-100 text-violet-600 dark:bg-violet-900/30 dark:text-violet-400"
        >
          <Layout size={22} />
        </div>
        <div>
          <h3 class="text-lg font-semibold text-primary">Interfaz</h3>
          <p class="text-sm text-secondary">
            Configura los elementos de la UI.
          </p>
        </div>
      </div>

      <div class="divide-y divide-emphasis">
        {@render settingRow(
          Type,
          "bg-cyan-50 dark:bg-cyan-900/20",
          "text-cyan-500",
          "Texto de Bienvenida",
          $generalSettings.showWelcomeText,
          () => generalSettings.toggleWelcomeText(),
        )}

        {@render settingRow(
          Layout,
          "bg-violet-50 dark:bg-violet-900/20",
          "text-violet-500",
          "Tarjetas de Módulos",
          $generalSettings.showWelcomeCards,
          () => generalSettings.toggleCards(),
        )}
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- TIME CONTROL CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-4">
          <div
            class="p-3 rounded-lg bg-orange-100 text-orange-600 dark:bg-orange-900/30 dark:text-orange-400"
          >
            {#if $generalSettings.overrideHour !== null && $generalSettings.overrideHour >= 6 && $generalSettings.overrideHour < 18}
              <Sun size={22} />
            {:else}
              <Moon size={22} />
            {/if}
          </div>
          <div>
            <h3 class="text-lg font-semibold text-primary">Ciclo Día/Noche</h3>
            <p class="text-sm text-secondary">Controla la hora manualmente.</p>
          </div>
        </div>

        {#if $generalSettings.overrideHour !== null}
          <button
            class="flex items-center gap-1 text-xs text-accent hover:underline"
            onclick={() => ($generalSettings.overrideHour = null)}
          >
            <RotateCcw size={12} />
            Auto
          </button>
        {/if}
      </div>

      <div class="space-y-3">
        <!-- Time Display -->
        <div class="flex items-center justify-between">
          <span class="text-sm text-secondary">Hora simulada:</span>
          <span class="font-mono text-lg font-semibold text-primary">
            {#if $generalSettings.overrideHour !== null}
              {Math.floor($generalSettings.overrideHour)
                .toString()
                .padStart(2, "0")}:{Math.round(
                ($generalSettings.overrideHour % 1) * 60,
              )
                .toString()
                .padStart(2, "0")}
            {:else}
              <span class="text-accent">Tiempo Real</span>
            {/if}
          </span>
        </div>

        <!-- Time Slider -->
        <div class="flex items-center gap-3">
          <Moon size={16} class="text-indigo-400" />
          <input
            type="range"
            min="0"
            max="24"
            step="0.25"
            class="flex-1 h-2 bg-surface-3 rounded-lg appearance-none cursor-pointer accent-accent"
            value={$generalSettings.overrideHour ??
              new Date().getHours() + new Date().getMinutes() / 60}
            oninput={(e) =>
              ($generalSettings.overrideHour =
                parseFloat(e.currentTarget.value) % 24)}
          />
          <Sun size={16} class="text-amber-400" />
        </div>

        <!-- Quick Time Buttons -->
        <div class="flex gap-2 pt-2">
          {#each [{ label: "🌅 6:00", hour: 6 }, { label: "☀️ 12:00", hour: 12 }, { label: "🌆 18:00", hour: 18 }, { label: "🌙 0:00", hour: 0 }] as preset}
            <button
              class="flex-1 py-1.5 px-2 text-xs rounded-md transition-colors
                {$generalSettings.overrideHour === preset.hour
                ? 'bg-accent text-white'
                : 'bg-surface-2 hover:bg-surface-hover text-secondary'}"
              onclick={() => ($generalSettings.overrideHour = preset.hour)}
            >
              {preset.label}
            </button>
          {/each}
        </div>
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- SEASON PREVIEW CARD -->
    <!-- ================================================================== -->
    <!-- ================================================================== -->
    <!-- SEASON PREVIEW CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-4">
          <div
            class="p-3 rounded-lg bg-pink-100 text-pink-600 dark:bg-pink-900/30 dark:text-pink-400"
          >
            <Sparkles size={22} />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-primary">Estación</h3>
            <p class="text-sm text-secondary">
              Previsualiza efectos estacionales.
            </p>
          </div>
        </div>

        {#if $generalSettings.overrideSeason !== null}
          <button
            class="flex items-center gap-1 text-xs text-accent hover:underline"
            onclick={() => ($generalSettings.overrideSeason = null)}
          >
            <RotateCcw size={12} />
            Auto
          </button>
        {/if}
      </div>

      <div class="grid grid-cols-5 gap-2">
        {#each [{ label: "Auto", value: null, icon: "🔄", bg: "bg-gray-500" }, { label: "Invierno", value: "winter" as Season, icon: "❄️", bg: "bg-blue-500" }, { label: "Primavera", value: "spring" as Season, icon: "🌸", bg: "bg-pink-400" }, { label: "Verano", value: "summer" as Season, icon: "✨", bg: "bg-yellow-500" }, { label: "Otoño", value: "autumn" as Season, icon: "🍂", bg: "bg-orange-500" }] as season}
          <button
            class="flex flex-col items-center gap-1 py-2 px-1 rounded-lg transition-all
                {$generalSettings.overrideSeason === season.value
              ? `${season.bg} text-white scale-105 shadow-md`
              : 'bg-surface-2 hover:bg-surface-hover text-secondary hover:scale-102'}"
            onclick={() => ($generalSettings.overrideSeason = season.value)}
          >
            <span class="text-lg">{season.icon}</span>
            <span class="text-xs font-medium">{season.label}</span>
          </button>
        {/each}
      </div>
    </div>

    <!-- ================================================================== -->
    <!-- BIRTHDAY TEST CARD -->
    <!-- ================================================================== -->
    <div class="card-base p-5">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="p-3 rounded-lg bg-rose-100 text-rose-600 dark:bg-rose-900/30 dark:text-rose-400"
          >
            <Cake size={22} />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-primary">Modo Cumpleaños</h3>
            <p class="text-sm text-secondary">
              Prueba la celebración de cumpleaños.
            </p>
          </div>
        </div>

        {@render toggleSwitch(
          $generalSettings.overrideBirthday,
          () => generalSettings.toggleBirthdayTest(),
          "Activar modo cumpleaños",
        )}
      </div>

      {#if $generalSettings.overrideBirthday}
        <div
          class="mt-3 p-3 rounded-md bg-rose-50 dark:bg-rose-900/20 text-rose-700 dark:text-rose-300 text-sm"
          transition:slide={{ duration: 150 }}
        >
          🎉 ¡Modo cumpleaños activado! Revisa la pantalla de bienvenida.
        </div>
      {/if}
    </div>

    <!-- ================================================================== -->
    <!-- RESET BUTTON -->
    <!-- ================================================================== -->
    <div class="flex justify-end gap-2 pt-2">
      <button
        class="btn-base bg-surface-2 hover:bg-surface-hover text-secondary text-sm"
        onclick={() => generalSettings.resetOverrides()}
      >
        <RotateCcw size={14} />
        Resetear Previews
      </button>

      <button
        class="btn-base bg-red-100 hover:bg-red-200 dark:bg-red-900/30 dark:hover:bg-red-900/50 text-red-600 dark:text-red-400 text-sm"
        onclick={() => {
          if (
            confirm(
              "¿Restaurar todas las configuraciones a sus valores por defecto?",
            )
          ) {
            generalSettings.reset();
          }
        }}
      >
        Restaurar Todo
      </button>
    </div>
  </div>
</div>
