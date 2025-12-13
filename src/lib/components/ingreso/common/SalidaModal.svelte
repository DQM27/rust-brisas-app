<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { IngresoResponse } from '$lib/types/ingreso';

  /**
   * Modal para registrar salida de contratista
   * Componente de presentación puro
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  export let ingreso: IngresoResponse;
  export let loading = false;

  // ==========================================
  // ESTADO LOCAL
  // ==========================================

  // Inicializar checkbox: si tiene gafete, por defecto debe devolverlo
  let devolvioGafete = !!ingreso.gafeteNumero;
  let observaciones = '';

  // ==========================================
  // HANDLERS
  // ==========================================

  function handleSubmit() {
    dispatch('confirm', {
      ingresoId: ingreso.id,
      devolvioGafete,
      observaciones: observaciones.trim() || undefined
    });
  }

  function handleCancel() {
    dispatch('cancel');
  }
</script>

<!-- 
  Modal de registro de salida
  Componente de presentación puro
-->

<div
  class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-md mx-auto"
>
  <!-- HEADER -->
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">
    Registrar Salida
  </h2>

  <!-- INFORMACIÓN DEL CONTRATISTA -->
  <div class="mb-6">
    <p class="text-sm text-gray-500 dark:text-gray-400">
      Confirmar salida para:
    </p>
    <p class="text-lg font-medium text-gray-900 dark:text-white mt-1">
      {ingreso.nombreCompleto}
    </p>
    {#if ingreso.cedula}
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
        Cédula: {ingreso.cedula}
      </p>
    {/if}
    {#if ingreso.gafeteNumero}
      <p class="text-sm text-blue-600 dark:text-blue-400 mt-2">
        Gafete asignado: <span class="font-bold font-mono">{ingreso.gafeteNumero}</span>
      </p>
    {/if}
  </div>

  <!-- FORMULARIO -->
  <form on:submit|preventDefault={handleSubmit} class="space-y-4">
    <!-- CHECKBOX DEVOLUCIÓN DE GAFETE -->
    {#if ingreso.gafeteNumero}
      <div
        class="flex items-center space-x-3 p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded-md border border-yellow-200 dark:border-yellow-800"
      >
        <input
          type="checkbox"
          id="devolvioGafete"
          bind:checked={devolvioGafete}
          disabled={loading}
          class="h-5 w-5 text-blue-600 focus:ring-blue-500 border-gray-300 rounded disabled:opacity-50"
        />
        <label
          for="devolvioGafete"
          class="text-sm font-medium text-gray-900 dark:text-gray-200"
        >
          ¿Devolvió el gafete físico?
        </label>
      </div>

      {#if !devolvioGafete}
        <div
          class="flex items-start gap-2 p-2 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md"
        >
          <svg
            class="w-5 h-5 text-red-600 dark:text-red-400 mt-0.5 shrink-0"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z"
              clip-rule="evenodd"
            />
          </svg>
          <p class="text-xs text-red-800 dark:text-red-200">
            Se generará una alerta de gafete pendiente
          </p>
        </div>
      {/if}
    {/if}

    <!-- OBSERVACIONES -->
    <div>
      <label
        for="observaciones"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
      >
        Observaciones (Opcional)
      </label>
      <textarea
        id="observaciones"
        bind:value={observaciones}
        disabled={loading}
        rows="3"
        maxlength="500"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm disabled:opacity-50"
        placeholder="Comentarios sobre la salida..."
      ></textarea>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
        {observaciones.length}/500 caracteres
      </p>
    </div>

    <!-- BOTONES DE ACCIÓN -->
    <div class="flex justify-end space-x-3 mt-6">
      <button
        type="button"
        on:click={handleCancel}
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-600"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-white bg-red-600 border border-transparent rounded-md shadow-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if loading}
          <span class="flex items-center">
            <svg
              class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
            Procesando...
          </span>
        {:else}
          Registrar Salida
        {/if}
      </button>
    </div>
  </form>
</div>