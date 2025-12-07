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
  import WeatherEffect from "./visual/WeatherEffect.svelte";

  // Cargar datos de ingresos al montar para que el contador sea real
  onMount(() => {
    ingresoStore.load();
  });

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
  ];
  // Lógica de saludo dinámico
  $: currentHour = new Date().getHours();
  $: greeting =
    currentHour >= 6 && currentHour < 12
      ? "Buenos días"
      : currentHour >= 12 && currentHour < 20
        ? "Buenas tardes"
        : "Buenas noches";
</script>

<div
  class="relative flex h-full items-center justify-center bg-surface-1 px-6 overflow-hidden"
>
  <WeatherEffect />
  <div class="relative z-10 w-full max-w-5xl text-center">
    <!-- Header con animación de fade y fly -->
    <div in:fly={{ y: -30, duration: 800, easing: quintOut }} class="mb-12">
      <h1
        class="mb-3 bg-linear-to-r from-white via-blue-100 to-white bg-clip-text text-5xl font-bold tracking-tight text-transparent"
      >
        {greeting}, {$currentUser?.nombre || "Usuario"}
      </h1>
      <p class="text-lg text-secondary">
        Brisas App - Sistema Integral de Control de Acceso y Seguridad
      </p>
    </div>

    <!-- Modules Grid -->
    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
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

    <!-- Indicador de estado del sistema -->
    <div in:fade={{ duration: 1000, delay: 500 }} class="mt-16 text-center">
      <div
        class="inline-flex items-center gap-2 rounded-full border border-emphasis bg-surface-2/50 px-4 py-2 text-xs text-secondary backdrop-blur-sm"
      >
        <div class="h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
        Sistema Operativo y Sincronizado
      </div>
    </div>
  </div>
</div>

<style>
  /* Animaciones adicionales personalizadas si fueran necesarias */
</style>
