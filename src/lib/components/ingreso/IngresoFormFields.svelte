<script lang="ts">
  export let tipoAutorizacion: string = "praind";
  export let observaciones: string = "";
</script>

<div class="space-y-4">
  <!-- AUTORIZACIÓN -->
  <div>
    <label
      for="auth"
      class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
    >
      Tipo de Autorización
    </label>
    <select
      id="auth"
      bind:value={tipoAutorizacion}
      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
    >
      <option value="praind">PRAIND</option>
      <option value="correo">Correo/Autorización especial</option>
    </select>
  </div>

  <!-- OBSERVACIONES -->
  <div>
    <label
      for="obs"
      class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
    >
      Observaciones
    </label>
    <textarea
      id="obs"
      bind:value={observaciones}
      rows="2"
      maxlength="500"
      placeholder="Comentarios adicionales..."
      class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white text-sm"
    ></textarea>
  </div>
</div>
