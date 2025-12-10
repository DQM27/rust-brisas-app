<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  /**
   * Campos adicionales del formulario
   * Componente de presentación puro
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  export let tipoAutorizacion: string = 'praind';
  export let observaciones: string = '';

  // ==========================================
  // HANDLERS
  // ==========================================

  function handleTipoChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    dispatch('tipoChange', target.value);
  }

  function handleObservacionesInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    dispatch('observacionesChange', target.value);
  }
</script>

<!-- 
  Campos adicionales: tipo de autorización y observaciones
  Componente de presentación puro
-->

<div class="space-y-4">
  <!-- TIPO DE AUTORIZACIÓN -->
  <div>
    <label
      for="auth"
      class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
    >
      Tipo de Autorización
    </label>
    <select
      id="auth"
      value={tipoAutorizacion}
      on:change={handleTipoChange}
      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
    >
      <option value="praind">PRAIND</option>
      <option value="correo">Correo/Autorización especial</option>
    </select>
  </div>
</div>
