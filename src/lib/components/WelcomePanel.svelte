<script lang="ts">
  import {
    Users,
    ShieldAlert,
    LogIn,
    HardHat,
    FileText,
    Settings,
  } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import { currentUser } from "$lib/stores/auth";
  import { totalPersonasAdentro } from "$lib/stores/ingresoStore";
  import { onMount } from "svelte";
  import { ingresoStore } from "$lib/stores/ingresoStore";
  import { generalSettings } from "$lib/stores/settingsStore";
  import WeatherEffect from "./visual/WeatherEffect.svelte";
  import CelestialCycle from "./visual/CelestialCycle.svelte";
  import MountainLandscape from "./visual/MountainLandscape.svelte";
  import BirthdayCelebration from "./visual/BirthdayCelebration.svelte"; // Import the party!
  import { currentSeason } from "$lib/utils/season";

  // Cargar datos de ingresos al montar para que el contador sea real
  onMount(() => {
    ingresoStore.load();
  });

  // Birthday Logic 🎂
  $: isBirthday = checkBirthday($currentUser?.fechaNacimiento);

  function checkBirthday(dateString?: string | null): boolean {
    if (!dateString) return false;
    try {
      // Assuming format YYYY-MM-DD
      // We want to match MM-DD with local time
      const birth = new Date(dateString);
      const today = new Date();
      // Handle timezone offset issues by using getMonth/getDate directly from the ISO string parts if possible,
      // but standard Date object usually works fine if we just compare Month and Date.

      // Note: dateString "2000-01-01" might be parsed as UTC.
      // Let's be safe and split the string if it matches YYYY-MM-DD
      if (dateString.match(/^\d{4}-\d{2}-\d{2}$/)) {
        const [_, month, day] = dateString.split("-").map(Number);
        return today.getMonth() + 1 === month && today.getDate() === day;
      }

      return (
        birth.getMonth() === today.getMonth() &&
        birth.getDate() === today.getDate()
      );
    } catch (e) {
      console.error("Error checking birthday", e);
      return false;
    }
  }

  const modules = [
    {
      icon: LogIn,
      title: "Control de Acceso",
      description: "Registro de ingresos y salidas de contratistas",
      stat: totalPersonasAdentro,
      statLabel: "Adentro",
      color: "text-green-500",
      delay: 0,
    },
    {
      icon: HardHat,
      title: "Contratistas",
      description: "Gestión de personal, empresas y vehículos",
      delay: 50,
    },
    {
      icon: ShieldAlert,
      title: "Seguridad",
      description: "Listas negras, alertas de gafetes y bloqueos",
      delay: 100,
    },
    {
      icon: Users,
      title: "Usuarios",
      description: "Administración de usuarios y permisos del sistema",
      delay: 150,
    },
    {
      icon: FileText,
      title: "Reportes",
      description: "Exportación de datos y generación de informes PDF",
      delay: 200,
    },
    {
      icon: Settings,
      title: "Configuración",
      description: "Ajustes generales del sistema y preferencias gloables",
      delay: 250,
    },
  ];

  // Logic: Use dark text ONLY if it's Day AND NOT Winter AND Background is visible matches the bright day background.
  // Winter day background is dark mountains, so needs white text.
  // Night always needs white text.
  $: effectiveHour = $generalSettings.overrideHour ?? new Date().getHours();
  $: isDay = effectiveHour >= 7 && effectiveHour < 18; // 7am to 6pm matching MountainLandscape logic
  $: useDarkText =
    $generalSettings.showBackground && isDay && $currentSeason !== "winter";

  $: textColorClass = isBirthday
    ? "text-white drop-shadow-xl tracking-wide" // White text with shadow for birthday
    : useDarkText
      ? "text-slate-800 drop-shadow-sm"
      : "text-white drop-shadow-md";

  // Lógica de saludo dinámico
  $: currentHour = new Date().getHours();
  $: greeting = isBirthday
    ? "¡Feliz Cumpleaños!"
    : currentHour < 6
      ? "Feliz madrugada"
      : currentHour < 12
        ? "Buenos días"
        : currentHour < 18
          ? "Buenas tardes"
          : "Buenas noches";
</script>

<div
  class="relative flex h-full items-center justify-center bg-surface-1 px-6 overflow-hidden"
>
  {#if isBirthday}
    <BirthdayCelebration name={$currentUser?.nombre || "Usuario"} />
    <div class="absolute inset-0 bg-black/10 pointer-events-none"></div>
    <!-- Slight dim for text readability -->
  {:else if $generalSettings.showBackground}
    <MountainLandscape>
      <CelestialCycle />
    </MountainLandscape>
  {:else}
    <CelestialCycle />
  {/if}
  <WeatherEffect />
  <div class="relative z-10 w-full max-w-5xl text-center">
    <!-- Header con animación de fade y fly -->
    <div
      in:fly={{ y: -30, duration: 800, easing: quintOut }}
      class="flex flex-col items-center {$generalSettings.showWelcomeCards
        ? 'mb-4 md:mb-6'
        : 'mb-12 md:mb-16'}"
    >
      {#if isBirthday}
        <!-- Diseño Especial de Cumpleaños 🎂 -->
        <div class="flex flex-col items-center gap-2">
          <h2
            class="{$generalSettings.showWelcomeCards
              ? 'text-3xl md:text-4xl mb-2'
              : 'text-5xl md:text-6xl mb-4'} font-black text-white drop-shadow-lg tracking-widest uppercase"
          >
            {greeting}
          </h2>
          <h1
            class="{$generalSettings.showWelcomeCards
              ? 'text-5xl md:text-7xl'
              : 'text-7xl md:text-9xl'} font-black text-transparent bg-clip-text bg-gradient-to-r from-yellow-200 via-amber-200 to-yellow-400 drop-shadow-xl animate-pulse"
            style="filter: drop-shadow(0 4px 6px rgba(0,0,0,0.3));"
          >
            {$currentUser?.nombre || "Usuario"}
          </h1>
        </div>
      {:else}
        <!-- Saludo Normal -->
        <h2
          class="tracking-wide transition-colors duration-1000 {$generalSettings.showWelcomeCards
            ? 'text-xl md:text-2xl mb-0'
            : 'text-3xl md:text-4xl mb-1'} font-medium {useDarkText
            ? 'text-slate-700/80'
            : 'text-white/90 drop-shadow-md'}"
        >
          {greeting}
        </h2>

        <!-- Nombre Elegante y Adaptable -->
        <h1
          class="tracking-tight transition-all duration-1000 font-bold
          {$generalSettings.showWelcomeCards
            ? 'text-4xl md:text-5xl'
            : 'text-6xl md:text-7xl'}
          {useDarkText
            ? 'text-transparent bg-clip-text bg-gradient-to-br from-slate-800 via-slate-600 to-slate-800 drop-shadow-sm'
            : 'text-transparent bg-clip-text bg-gradient-to-b from-white via-white to-blue-50 drop-shadow-lg'}"
          style={useDarkText
            ? ""
            : "filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2));"}
        >
          {$currentUser?.nombre || "Usuario"}
        </h1>
      {/if}

      <p
        class="transition-colors duration-1000 {textColorClass} {isBirthday
          ? 'text-xl font-bold mt-2'
          : $generalSettings.showWelcomeCards
            ? 'text-sm mt-2 opacity-80'
            : 'text-xl font-medium tracking-wide opacity-90 mt-6'}"
      >
        {isBirthday
          ? "Te desea el equipo de Brisas App 🎉"
          : "Brisas App - Sistema Integral de Control de Acceso"}
      </p>
    </div>

    <!-- Modules Grid -->
    {#if $generalSettings.showWelcomeCards}
      <div
        class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3"
        transition:fade={{ duration: 300 }}
      >
        {#each modules as module, i (i)}
          <div
            in:fly={{
              y: 50,
              duration: 600,
              delay: module.delay,
              easing: quintOut,
            }}
            class="group relative overflow-hidden rounded-lg border border-emphasis bg-surface-2 p-6 text-left transition-all duration-300 hover:-translate-y-2 hover:border-accent hover:shadow-lg hover:shadow-accent/20"
          >
            <!-- Efecto de brillo en hover -->
            <div
              class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-surface-1/5 to-transparent transition-transform duration-700 group-hover:translate-x-full"
            ></div>

            <!-- Contenido -->
            <div class="relative z-10 flex flex-col h-full justify-between">
              <div>
                <!-- Icon con animación de escala -->
                <div
                  class="mb-4 inline-flex rounded-lg bg-surface-1 p-3 {module.color ||
                    'text-accent'} transition-all duration-300 group-hover:scale-110 group-hover:bg-accent/10"
                >
                  <svelte:component
                    this={module.icon}
                    size={28}
                    strokeWidth={2}
                  />
                </div>

                <h3
                  class="mb-2 text-lg font-semibold text-primary transition-colors duration-300 group-hover:text-accent"
                >
                  {module.title}
                </h3>

                <p class="text-sm leading-relaxed text-secondary">
                  {module.description}
                </p>
              </div>

              <!-- Stat opcional (solo si existe) -->
              {#if module.stat}
                <div
                  class="mt-4 flex items-center gap-2 border-t border-emphasis pt-3"
                >
                  <span class="text-2xl font-bold text-primary"
                    >{$totalPersonasAdentro}</span
                  >
                  <span
                    class="text-xs font-medium text-secondary uppercase tracking-wider"
                    >{module.statLabel}</span
                  >
                  <div
                    class="ml-auto h-2 w-2 animate-pulse rounded-full bg-green-500"
                  ></div>
                </div>
              {/if}
            </div>

            <!-- Borde animado inferior -->
            <div
              class="absolute bottom-0 left-0 h-1 w-0 bg-linear-to-r from-[#007acc] to-[#0098ff] transition-all duration-500 group-hover:w-full"
            ></div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  /* Animaciones adicionales personalizadas si fueran necesarias */
</style>
