<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { activeTabId } from "$lib/stores/tabs";
  import IngresoFormContainer from "./IngresoFormContainer.svelte";
  import IngresosActivosTable from "./IngresosActivosTable.svelte";

  let isFormOpen = false;

  // Cerrar formulario automáticamente cuando se cambia de pestaña
  $: $activeTabId, (isFormOpen = false);

  function toggleForm() {
    isFormOpen = !isFormOpen;
  }

  function closeForm() {
    isFormOpen = false;
  }

  function openForm() {
    isFormOpen = true;
  }
</script>

<!-- 
  Vista principal del módulo de Ingreso
  Refactorizada para soportar formulario colapsable
-->

<div class="h-full p-4 flex gap-6 overflow-hidden relative">
  <!-- Panel Izquierdo: Formulario de Entrada (Colapsable) -->
  {#if isFormOpen}
    <div
      class="w-full lg:w-1/3 h-full shrink-0"
      transition:fly={{ x: -300, duration: 300, opacity: 0, easing: cubicOut }}
    >
      <div class="h-full overflow-y-auto pr-1">
        <IngresoFormContainer onSuccess={closeForm} onClose={closeForm} />
      </div>
    </div>
  {/if}

  <!-- Panel Derecho: Lista de Activos -->
  <div class="flex-1 h-full min-w-0 transition-all duration-300">
    <IngresosActivosTable
      onRegisterClick={openForm}
      onCloseForm={closeForm}
      {isFormOpen}
    />
  </div>
</div>
