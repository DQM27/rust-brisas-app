<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { IngresoResponse } from "$lib/types/ingreso";

  export let ingreso: IngresoResponse;
  export let loading = false;

  const dispatch = createEventDispatcher();

  let devolvioGafete = true;
  let observaciones = "";

  function handleSubmit() {
    dispatch("confirm", {
      ingresoId: ingreso.id,
      devolvioGafete,
      observaciones: observaciones.trim() || undefined
    });
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-md mx-auto">
  <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">
    Registrar Salida
  </h2>

  <div class="mb-6">
    <p class="text-sm text-gray-500 dark:text-gray-400">Confirmar salida para:</p>
    <p class="text-lg font-medium text-gray-900 dark:text-white">{ingreso.nombreCompleto}</p>
    {#if ingreso.gafeteNumero}
      <p class="text-sm text-blue-600 dark:text-blue-400 mt-1">
        Gafete asignado: <span class="font-bold">{ingreso.gafeteNumero}</span>
      </p>
    {/if}
  </div>

  <form on:submit|preventDefault={handleSubmit} class="space-y-4">
    {#if ingreso.gafeteNumero}
      <div class="flex items-center space-x-3 p-3 bg-yellow-50 dark:bg-yellow-900/20 rounded-md border border-yellow-200 dark:border-yellow-800">
        <input
          type="checkbox"
          id="devolvioGafete"
          bind:checked={devolvioGafete}
          class="h-5 w-5 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
        />
        <label for="devolvioGafete" class="text-sm font-medium text-gray-900 dark:text-gray-200">
          ¿Devolvió el gafete físico?
        </label>
      </div>
    {/if}

    <div>
      <label for="observaciones" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Observaciones (Opcional)
      </label>
      <textarea
        id="observaciones"
        bind:value={observaciones}
        rows="3"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white sm:text-sm"
        placeholder="Comentarios sobre la salida..."
      ></textarea>
    </div>

    <div class="flex justify-end space-x-3 mt-6">
      <button
        type="button"
        on:click={() => dispatch("cancel")}
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 dark:bg-gray-700 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-600"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-white bg-red-600 border border-transparent rounded-md shadow-sm hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
      >
        {#if loading}
          Procesando...
        {:else}
          Registrar Salida
        {/if}
      </button>
    </div>
  </form>
</div>
