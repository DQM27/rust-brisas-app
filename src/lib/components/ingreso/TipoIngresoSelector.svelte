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

<div class="flex items-center gap-2 mb-4">
  {#each tipos as tipo}
    <div class="relative group">
      <button
        type="button"
        class="flex items-center justify-center w-10 h-10 rounded-md border transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-[#1e1e1e] cursor-pointer {tipoSeleccionado ===
        tipo.id
          ? 'bg-blue-50 border-blue-200 text-blue-600 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-400'
          : 'bg-white border-gray-200 text-gray-500 hover:border-gray-300 hover:bg-gray-50 dark:bg-[#252526] dark:border-white/10 dark:text-gray-400 dark:hover:border-white/20 dark:hover:bg-[#2a2a2b]'}"
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
