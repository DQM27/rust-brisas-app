<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let modoIngreso: "caminando" | "vehiculo" = "caminando";
  export let tieneVehiculos: boolean = false;

  function handleModoChange(modo: "caminando" | "vehiculo") {
    dispatch("change", modo);
  }
</script>

<!-- 
  Selector de modo de ingreso (caminando/vehículo)
  Estilo: Segmented Control (GitHub style / iOS style)
-->

<div>
  <div class="flex items-center justify-between mb-2">
    <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
      Modo de Ingreso
    </span>
    {#if !tieneVehiculos}
      <span
        class="text-xs text-amber-600 dark:text-amber-500 flex items-center gap-1"
      >
        <svg
          class="w-3 h-3"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          />
        </svg>
        Sin vehículos
      </span>
    {/if}
  </div>

  <div class="bg-gray-100 dark:bg-gray-700/50 p-1 rounded-md flex">
    <!-- Caminando -->
    <button
      type="button"
      on:click={() => handleModoChange("caminando")}
      class="flex-1 flex items-center justify-center gap-2 px-3 py-1.5 text-sm font-medium rounded transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-blue-500 {modoIngreso ===
      'caminando'
        ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
    >
      <svg
        class="w-4 h-4"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
        />
      </svg>
      Caminando
    </button>

    <!-- Vehículo -->
    <button
      type="button"
      on:click={() => handleModoChange("vehiculo")}
      disabled={!tieneVehiculos}
      class="flex-1 flex items-center justify-center gap-2 px-3 py-1.5 text-sm font-medium rounded transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-blue-500 {modoIngreso ===
      'vehiculo'
        ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
        : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'} {!tieneVehiculos
        ? 'opacity-50 cursor-not-allowed'
        : ''}"
    >
      <svg
        class="w-4 h-4"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 17a2 2 0 11-4 0 2 2 0 014 0zM19 17a2 2 0 11-4 0 2 2 0 014 0z"
        />
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M13 16V6a1 1 0 00-1-1H4a1 1 0 00-1 1v10a1 1 0 001 1h1m8-1a1 1 0 01-1 1H9m4-1V8a1 1 0 011-1h2.586a1 1 0 01.707.293l3.414 3.414a1 1 0 01.293.707V16a1 1 0 01-1 1h-1m-6-1a1 1 0 001 1h1M5 17a2 2 0 104 0m-4 0a2 2 0 114 0m6 0a2 2 0 104 0m-4 0a2 2 0 114 0"
        />
      </svg>
      Vehículo
    </button>
  </div>
</div>
