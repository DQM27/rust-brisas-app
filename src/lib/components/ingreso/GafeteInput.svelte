<script lang="ts">
  import type { GafeteResponse } from "$lib/types/gafete";

  export let gafeteNumero: string = "";
  export let gafetesDisponibles: GafeteResponse[] = [];

  let gafeteValido = false;
  let gafeteSugerencias: string[] = [];

  // Validación de gafete en tiempo real
  $: {
    if (gafeteNumero.trim()) {
      const normalizado = gafeteNumero.trim().toUpperCase();
      gafeteValido = gafetesDisponibles.some(
        (g) => g.numero === normalizado && g.estaDisponible,
      );

      // Generar sugerencias
      if (!gafeteValido) {
        gafeteSugerencias = gafetesDisponibles
          .filter((g) => g.numero.includes(normalizado))
          .map((g) => g.numero)
          .slice(0, 5);
      } else {
        gafeteSugerencias = [];
      }
    } else {
      gafeteValido = true; // Sin gafete es válido
      gafeteSugerencias = [];
    }
  }

  export function isValid(): boolean {
    return gafeteNumero.trim() === "" || gafeteValido;
  }
</script>

<div>
  <label
    for="gafete"
    class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
  >
    Número de Gafete (Opcional)
  </label>
  <div class="relative">
    <input
      type="text"
      id="gafete"
      bind:value={gafeteNumero}
      placeholder="Ej: 027"
      class="w-full px-3 py-2 border {gafeteNumero.trim() && !gafeteValido
        ? 'border-red-500'
        : 'border-gray-300 dark:border-gray-600'} rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white uppercase font-mono"
      maxlength="20"
    />
    {#if gafeteNumero.trim()}
      <div class="absolute right-3 top-2.5">
        {#if gafeteValido}
          <svg
            class="w-5 h-5 text-green-500"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
              clip-rule="evenodd"
            />
          </svg>
        {:else}
          <svg
            class="w-5 h-5 text-red-500"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clip-rule="evenodd"
            />
          </svg>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Sugerencias -->
  {#if gafeteSugerencias.length > 0}
    <div
      class="mt-2 p-2 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md"
    >
      <p class="text-xs text-yellow-800 dark:text-yellow-200 mb-1">
        Sugerencias disponibles:
      </p>
      <div class="flex flex-wrap gap-1">
        {#each gafeteSugerencias as sugerencia}
          <button
            type="button"
            on:click={() => (gafeteNumero = sugerencia)}
            class="px-2 py-1 text-xs bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded hover:bg-gray-50 dark:hover:bg-gray-600"
          >
            {sugerencia}
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
    Disponibles: {gafetesDisponibles.length} gafetes tipo contratista
  </p>
</div>
