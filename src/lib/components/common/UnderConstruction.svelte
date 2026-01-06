<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { onMount } from "svelte";

  export let data: {
    type?: "development" | "maintenance";
    moduleName?: string;
  } = {};

  // Support direct props as fallback
  export let type: "development" | "maintenance" = "development";
  export let moduleName: string = "Módulo";

  $: effectiveType = data?.type || type || "development";
  $: effectiveModuleName = data?.moduleName || moduleName || "Módulo";

  $: config = {
    development: {
      title: "En Construcción",
      message: `El módulo <strong class="text-white">${effectiveModuleName}</strong> está siendo desarrollado actualmente.`,
      subMessage:
        "Estamos trabajando duro para traerte esta funcionalidad pronto.",
      icon: "🚧",
      color: "text-yellow-500",
      bg: "bg-yellow-500/10",
      border: "border-yellow-500/20",
    },
    maintenance: {
      title: "En Mantenimiento",
      message: `El módulo <strong class="text-white">${effectiveModuleName}</strong> está temporalmente deshabilitado.`,
      subMessage:
        "Estamos realizando mejoras de seguridad y rendimiento. Vuelve a intentarlo más tarde.",
      icon: "🔧",
      color: "text-orange-500",
      bg: "bg-orange-500/10",
      border: "border-orange-500/20",
    },
  };

  $: current = config[effectiveType] || config.development;
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

    <!-- Botón eliminado por solicitud del usuario -->
  </div>
</div>
