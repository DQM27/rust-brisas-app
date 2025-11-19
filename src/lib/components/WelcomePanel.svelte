<script lang="ts">
  import { User, Shield, BarChart3 } from "lucide-svelte";
  import { fade, fly } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  const features = [
    {
      icon: User,
      title: "Gestión de Usuarios",
      description: "Crea, edita y administra usuarios del sistema",
      delay: 0
    },
    {
      icon: Shield,
      title: "Autenticación Segura",
      description: "Login con hashing Argon2 + sesiones persistentes",
      delay: 100
    },
    {
      icon: BarChart3,
      title: "Panel de Control",
      description: "Métricas y estadísticas en tiempo real",
      delay: 200
    }
  ];
</script>

<div class="flex h-full items-center justify-center bg-[#1e1e1e] px-6">
  <div class="w-full max-w-4xl text-center">
    <!-- Header con animación de fade y fly -->
    <div 
      in:fly={{ y: -30, duration: 800, easing: quintOut }}
      class="mb-12"
    >
      <h1 class="mb-3 bg-linear-to-r from-white via-blue-100 to-white bg-clip-text text-5xl font-bold tracking-tight text-transparent">
        Bienvenido a Brisas App
      </h1>
      <p class="text-lg text-gray-400">
        Sistema de gestión de usuarios
      </p>
    </div>

    <!-- Feature Cards con stagger animation -->
    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
      {#each features as feature, i (i)}
        <div
          in:fly={{ y: 50, duration: 600, delay: feature.delay, easing: quintOut }}
          class="group relative overflow-hidden rounded-lg border border-[#333] bg-[#252526] p-6 transition-all duration-300 hover:-translate-y-2 hover:border-[#007acc] hover:shadow-lg hover:shadow-[#007acc]/20"
        >
          <!-- Efecto de brillo en hover -->
          <div class="absolute inset-0 -translate-x-full bg-linear-to-r from-transparent via-white/5 to-transparent transition-transform duration-700 group-hover:translate-x-full"></div>
          
          <!-- Contenido -->
          <div class="relative z-10">
            <!-- Icon con animación de escala -->
            <div class="mb-4 inline-flex rounded-lg bg-[#007acc]/10 p-3 text-[#007acc] transition-all duration-300 group-hover:scale-110 group-hover:bg-[#007acc]/20">
              <svelte:component this={feature.icon} size={28} strokeWidth={2} />
            </div>

            <h3 class="mb-2 text-base font-semibold text-white transition-colors duration-300 group-hover:text-[#007acc]">
              {feature.title}
            </h3>
            
            <p class="text-sm leading-relaxed text-gray-400">
              {feature.description}
            </p>
          </div>

          <!-- Borde animado inferior -->
          <div class="absolute bottom-0 left-0 h-1 w-0 bg-linear-to-r from-[#007acc] to-[#0098ff] transition-all duration-500 group-hover:w-full"></div>
        </div>
      {/each}
    </div>

    <!-- Indicador sutil animado -->
    <div 
      in:fade={{ duration: 1000, delay: 400 }}
      class="mt-16"
    >
      <div class="inline-flex items-center gap-2 rounded-full border border-[#333] bg-[#252526]/50 px-4 py-2 text-xs text-gray-400 backdrop-blur-sm">
        <div class="h-2 w-2 animate-pulse rounded-full bg-green-500"></div>
        Sistema operativo
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes shimmer {
    from {
      transform: translateX(-100%);
    }
    to {
      transform: translateX(100%);
    }
  }
</style>