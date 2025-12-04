<script lang="ts">
  import IngresoFormContainer from './IngresoFormContainer.svelte';
  import IngresosActivosTable from './IngresosActivosTable.svelte';
</script>

<!-- 
  Vista principal del módulo de Ingreso
  
  Responsabilidades:
  - Layout y estructura visual
  - Composición de áreas principales
  - NO contiene lógica de negocio
-->

<div class="h-full p-4">
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 h-full">
    <!-- Panel Izquierdo: Formulario de Entrada -->
    <div class="lg:col-span-1">
      <IngresoFormContainer />
    </div>

    <!-- Panel Derecho: Lista de Activos -->
    <div class="lg:col-span-2 h-full min-h-0">
      <IngresosActivosTable />
    </div>
  </div>
</div>