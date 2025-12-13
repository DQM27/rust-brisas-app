<script lang="ts">
  // ==========================================
  // TipoIngresoSelector.svelte
  // ==========================================
  // Selector de tipo de ingreso (Contratista/Visita/Proveedor)

  import { UserCircle, Users, Building2 } from "lucide-svelte";

  export let tipoSeleccionado: "contratista" | "visita" | "proveedor" =
    "contratista";
  export let onChange: (
    tipo: "contratista" | "visita" | "proveedor",
  ) => void = () => {};

  const tipos = [
    {
      id: "contratista" as const,
      label: "Contratista",
      icon: UserCircle,
      descripcion: "Trabajador con PRAIND",
    },
    {
      id: "visita" as const,
      label: "Visita",
      icon: Users,
      descripcion: "Visitante con anfitrión",
    },
    {
      id: "proveedor" as const,
      label: "Proveedor",
      icon: Building2,
      descripcion: "Proveedor de empresa",
    },
  ];

  function handleSelect(tipo: typeof tipoSeleccionado) {
    tipoSeleccionado = tipo;
    onChange(tipo);
  }
</script>

<div class="grid grid-cols-3 gap-2 mb-4 w-full">
  {#each tipos as tipo}
    <div class="relative group w-full">
      <button
        type="button"
        class="w-full flex items-center justify-center py-2.5 rounded-md border transition-all duration-200 focus:outline-none cursor-pointer {tipoSeleccionado ===
        tipo.id
          ? 'bg-blue-500/10 border-2 border-blue-500/50 text-blue-500 dark:text-blue-400'
          : 'bg-transparent border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400 hover:border-gray-300 dark:hover:border-white/20 hover:bg-gray-50 dark:hover:bg-[#161b22]'}"
        on:click={() => handleSelect(tipo.id)}
        aria-label={tipo.label}
      >
        <svelte:component this={tipo.icon} size={20} strokeWidth={2} />
      </button>

      <!-- Tooltip GitHub Style -->
      <div
        class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 bg-gray-900 text-white text-xs rounded shadow-lg whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none z-10 dark:bg-gray-700"
      >
        <div class="font-medium">{tipo.label}</div>
        <div class="text-[10px] text-gray-300">{tipo.descripcion}</div>
        <!-- Arrow -->
        <div
          class="absolute top-full left-1/2 -translate-x-1/2 -mt-1 border-4 border-transparent border-t-gray-900 dark:border-t-gray-700"
        ></div>
      </div>
    </div>
  {/each}
</div>

<style>
  /* Styles handled by Tailwind mostly, keeping minimal custom css if needed */
</style>
