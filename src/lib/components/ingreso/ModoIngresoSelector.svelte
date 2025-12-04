<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  /**
   * Selector de modo de ingreso
   * Componente de presentación puro
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  export let modoIngreso: 'caminando' | 'vehiculo' = 'caminando';
  export let tieneVehiculos: boolean = false;

  // ==========================================
  // HANDLERS
  // ==========================================

  function handleModoChange(modo: 'caminando' | 'vehiculo') {
    dispatch('change', modo);
  }
</script>

<!-- 
  Selector de modo de ingreso (caminando/vehículo)
  Componente de presentación puro
-->

<div>
  <label
    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
  >
    Modo de Ingreso
  </label>
  <div class="grid grid-cols-2 gap-3">
    <!-- Caminando -->
    <button
      type="button"
      on:click={() => handleModoChange('caminando')}
      class="relative flex items-center justify-center p-4 border-2 rounded-lg transition-all {modoIngreso ===
      'caminando'
        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 dark:border-blue-400'
        : 'border-gray-300 hover:border-gray-400 dark:border-gray-600 dark:hover:border-gray-500'}"
    >
      <div class="text-center">
        <svg
          class="w-8 h-8 mx-auto mb-2 {modoIngreso === 'caminando'
            ? 'text-blue-600 dark:text-blue-400'
            : 'text-gray-500'}"
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
        <span
          class="text-sm font-medium {modoIngreso === 'caminando'
            ? 'text-blue-700 dark:text-blue-300'
            : 'text-gray-700 dark:text-gray-300'}"
        >
          Caminando
        </span>
      </div>
      {#if modoIngreso === 'caminando'}
        <div
          class="absolute top-2 right-2 w-5 h-5 bg-blue-600 rounded-full flex items-center justify-center"
        >
          <svg
            class="w-3 h-3 text-white"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
      {/if}
    </button>

    <!-- Vehículo -->
    <button
      type="button"
      on:click={() => handleModoChange('vehiculo')}
      disabled={!tieneVehiculos}
      class="relative flex items-center justify-center p-4 border-2 rounded-lg transition-all {modoIngreso ===
      'vehiculo'
        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 dark:border-blue-400'
        : 'border-gray-300 hover:border-gray-400 dark:border-gray-600 dark:hover:border-gray-500'} {!tieneVehiculos
        ? 'opacity-50 cursor-not-allowed'
        : ''}"
    >
      <div class="text-center">
        <svg
          class="w-8 h-8 mx-auto mb-2 {modoIngreso === 'vehiculo'
            ? 'text-blue-600 dark:text-blue-400'
            : 'text-gray-500'}"
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
        <span
          class="text-sm font-medium {modoIngreso === 'vehiculo'
            ? 'text-blue-700 dark:text-blue-300'
            : 'text-gray-700 dark:text-gray-300'}"
        >
          Vehículo
        </span>
      </div>
      {#if modoIngreso === 'vehiculo'}
        <div
          class="absolute top-2 right-2 w-5 h-5 bg-blue-600 rounded-full flex items-center justify-center"
        >
          <svg
            class="w-3 h-3 text-white"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
      {/if}
    </button>
  </div>

  {#if !tieneVehiculos}
    <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
      Este contratista no tiene vehículos registrados
    </p>
  {/if}
</div>