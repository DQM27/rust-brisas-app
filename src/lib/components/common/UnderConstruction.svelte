<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { onMount } from "svelte";

  export let data: {
    type?: "development" | "maintenance";
    moduleName?: string;
  } = {};
  export let onBack: () => void = () => window.history.back();

  $: type = data.type || "development";
  $: moduleName = data.moduleName || "Módulo";

  const config = {
    development: {
      title: "En Construcción",
      message: `El módulo <strong>${moduleName}</strong> está siendo desarrollado actualmente.`,
      subMessage:
        "Estamos trabajando duro para traerte esta funcionalidad pronto.",
      icon: "🚧",
      color: "text-yellow-500",
      bg: "bg-yellow-500/10",
      border: "border-yellow-500/20",
    },
    maintenance: {
      title: "En Mantenimiento",
      message: `El módulo <strong>${moduleName}</strong> está temporalmente deshabilitado.`,
      subMessage:
        "Estamos realizando mejoras de seguridad y rendimiento. Vuelve a intentarlo más tarde.",
      icon: "🔧",
      color: "text-orange-500",
      bg: "bg-orange-500/10",
      border: "border-orange-500/20",
    },
  };

  $: current = config[type];
</script>

<div
  class="w-full h-full flex flex-col items-center justify-center p-8 bg-surface-1 relative overflow-hidden"
  in:fade
>
  <!-- Decoración de fondo -->
  <div class="absolute inset-0 overflow-hidden pointer-events-none opacity-5">
    <div
      class="absolute top-1/4 left-1/4 w-96 h-96 bg-primary rounded-full blur-[128px]"
    ></div>
    <div
      class="absolute bottom-1/4 right-1/4 w-64 h-64 bg-secondary rounded-full blur-[96px]"
    ></div>
  </div>

  <div
    class="relative z-10 max-w-lg w-full text-center"
    in:fly={{ y: 20, duration: 800, delay: 200 }}
  >
    <!-- Icono Container -->
    <div
      class="mx-auto mb-8 w-24 h-24 rounded-3xl flex items-center justify-center text-5xl shadow-2xl backdrop-blur-xl border {current.border} {current.bg}"
    >
      <span class="animate-pulse">{current.icon}</span>
    </div>

    <!-- Textos -->
    <h1
      class="text-4xl font-bold mb-4 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-white to-gray-400"
    >
      {current.title}
    </h1>

    <p class="text-xl text-gray-300 mb-2">
      {@html current.message}
    </p>

    <p class="text-base text-gray-500 mb-10">
      {current.subMessage}
    </p>

    <!-- Botón de acción -->
    <button
      on:click={onBack}
      class="px-8 py-3 rounded-xl bg-surface-2 hover:bg-surface-3 border border-white/10 text-white font-medium transition-all hover:scale-105 active:scale-95 shadow-lg flex items-center gap-2 mx-auto"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"><path d="m15 18-6-6 6-6" /></svg
      >
      Volver al Inicio
    </button>
  </div>
</div>
