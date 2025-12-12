<script>
  import { onMount } from "svelte";

  // Opcional: Escuchar eventos de progreso si decides implementarlos en el backend
</script>

<div
  class="h-screen w-screen bg-[#1e1e1e] flex flex-col items-center justify-center select-none overflow-hidden cursor-wait"
>
  <!-- Logo Container -->
  <div class="mb-8 relative">
    <!-- Logo Placeholder o Imagen -->
    <div
      class="w-24 h-24 bg-gradient-to-tr from-blue-500 to-purple-600 rounded-2xl shadow-2xl flex items-center justify-center"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="w-12 h-12 text-white"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
      </svg>
    </div>

    <!-- Glow Effect -->
    <div
      class="absolute top-0 left-0 w-full h-full bg-blue-500/30 blur-xl -z-10 rounded-full"
    ></div>
  </div>

  <!-- App Name -->
  <h1 class="text-3xl font-bold text-white mb-2 tracking-wide">Brisas App</h1>
  <p class="text-gray-400 text-sm mb-12">Cargando sistema...</p>

  <!-- Loading Spinner -->
  <div class="relative w-48 h-1 bg-gray-800 rounded-full overflow-hidden">
    <div
      class="absolute top-0 left-0 h-full w-1/3 bg-blue-500 rounded-full animate-loading-bar"
    ></div>
  </div>
</div>

<style>
  @keyframes loading-bar {
    0% {
      left: -35%;
    }
    100% {
      left: 100%;
    }
  }
  .animate-loading-bar {
    animation: loading-bar 1.5s infinite linear;
  }
</style>
