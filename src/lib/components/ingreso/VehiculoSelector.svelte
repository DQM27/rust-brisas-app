<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  /**
   * Selector de vehículo
   * Componente de presentación puro
   */

  const dispatch = createEventDispatcher();

  // ==========================================
  // PROPS
  // ==========================================

  export let vehiculos: any[] = [];
  export let vehiculoId: string | null = null;

  // ==========================================
  // HANDLERS
  // ==========================================

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const value = target.value === 'null' ? null : target.value;
    dispatch('change', value);
  }
</script>

<!-- 
  Selector de vehículo
  Componente de presentación puro
-->

{#if vehiculos && vehiculos.length > 0}
  <div class="pl-4 border-l-4 border-blue-500">
    <label
      for="vehiculo"
      class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
    >
      Seleccionar Vehículo
    </label>
    <select
      id="vehiculo"
      value={vehiculoId || 'null'}
      on:change={handleChange}
      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
    >
      <option value="null">-- Seleccione vehículo --</option>
      {#each vehiculos as vehiculo}
        <option value={vehiculo.id}>
          {vehiculo.placa} - {vehiculo.marca}
          {vehiculo.modelo}
          {vehiculo.tipoVehiculo ? `(${vehiculo.tipoVehiculo})` : ''}
        </option>
      {/each}
    </select>
  </div>
{/if}