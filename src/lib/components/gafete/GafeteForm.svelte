<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade } from "svelte/transition";
  import type {
    CreateGafeteInput,
    UpdateGafeteInput,
    GafeteResponse,
  } from "$lib/types/gafete";

  export let loading = false;
  export let initialData: GafeteResponse | null = null;

  const dispatch = createEventDispatcher<{
    submit: CreateGafeteInput | UpdateGafeteInput;
    cancel: void;
  }>();

  // Estado del formulario
  let numero = "";
  let tipo: "contratista" | "proveedor" | "visita" | "otro" = "contratista";
  let isEditing = false;

  // Cargar datos iniciales si es edición
  $: if (initialData) {
    numero = initialData.numero;
    tipo = initialData.tipo; // initialData.tipo ya tiene el tipo correcto
    isEditing = true;
  } else {
    numero = "";
    tipo = "contratista";
    isEditing = false;
  }

  function handleSubmit() {
    if (!numero.trim()) return;

    const data = {
      numero,
      tipo,
    };

    dispatch("submit", data);
  }

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[#2da44e] disabled:opacity-60 transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
</script>

<div
  class="bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 p-6 w-full max-w-md mx-auto"
>
  <h2
    class="text-xl font-bold mb-4 text-gray-900 dark:text-gray-100 border-b border-gray-200 dark:border-gray-700 pb-2"
  >
    {isEditing ? "Editar Gafete" : "Nuevo Gafete"}
  </h2>

  <form on:submit|preventDefault={handleSubmit} class="space-y-4">
    <!-- Número de Gafete -->
    <div>
      <label for="numero" class={labelClass}> Número de Gafete </label>
      <input
        type="text"
        id="numero"
        bind:value={numero}
        disabled={isEditing || loading}
        class={inputClass}
        placeholder="Ej: G-101"
        required
      />
      {#if isEditing}
        <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
          El número no se puede cambiar una vez creado.
        </p>
      {/if}
    </div>

    <!-- Tipo de Gafete -->
    <div>
      <label for="tipo" class={labelClass}> Tipo </label>
      <select id="tipo" bind:value={tipo} disabled={loading} class={inputClass}>
        <option value="contratista">Contratista</option>
        <option value="proveedor">Proveedor</option>
        <option value="visita">Visita</option>
        <option value="otro">Otro</option>
      </select>
    </div>

    <!-- Botones de Acción -->
    <div
      class="flex justify-end space-x-3 mt-6 pt-4 border-t border-gray-200 dark:border-gray-700"
    >
      <button
        type="button"
        on:click={() => dispatch("cancel")}
        disabled={loading}
        class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 border border-transparent rounded-md transition-colors"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading || !numero.trim()}
        class="inline-flex justify-center px-4 py-2 text-sm font-medium text-white bg-[#2da44e] border border-transparent rounded-md shadow-sm hover:bg-[#2c974b] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#2da44e] disabled:opacity-50 disabled:cursor-not-allowed transition-all"
      >
        {#if loading}
          <svg
            class="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
            xmlns="http://www.w3.org/2000/svg"
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
          Guardando...
        {:else}
          {isEditing ? "Actualizar" : "Crear Gafete"}
        {/if}
      </button>
    </div>
  </form>
</div>
