<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { onMount } from "svelte";
  import { preventDefault } from "svelte/legacy";
  import { shortcutService } from "$lib/services/shortcutService";
  import { cubicOut } from "svelte/easing";
  import { activeTabId } from "$lib/stores/tabs";

  import IngresoContratistaForm from "./contratista/IngresoContratistaForm.svelte";
  import IngresoVisitaForm from "./visita/IngresoVisitaForm.svelte";
  import IngresoProveedorForm from "./proveedor/IngresoProveedorForm.svelte";

  import IngresosActivosTable from "./contratista/IngresosActivosTable.svelte";
  import IngresoVisitasTable from "./visita/IngresoVisitasTable.svelte";
  import IngresoProveedoresTable from "./proveedor/IngresoProveedoresTable.svelte";
  import CitasList from "./CitasList.svelte";

  let isFormOpen = false;
  let activeTab: "contratistas" | "visitas" | "proveedores" | "citas" =
    "contratistas";

  function setActiveTab(tab: typeof activeTab) {
    activeTab = tab;
  }

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

  // Registrar manejador para el comando "new" en el contexto "ingreso-list"
  // Esto escucha cuando se dispara el comando (ej. Ctrl+N)
  onMount(() => {
    return shortcutService.registerHandler("ingreso-list", "new", openForm);
  });
</script>

<!--
  Vista principal del módulo de Ingreso
  Usa use:shortcutService.useScope para activar el contexto "ingreso-list"
-->
<div
  class="h-full flex gap-6 overflow-hidden relative bg-[#1e1e1e]"
  use:shortcutService.useScope={"ingreso-list"}
>
  <!-- Panel Izquierdo: Formulario de Entrada (Colapsable) -->
  {#if isFormOpen}
    <div
      class="w-full lg:w-1/3 h-full shrink-0"
      transition:fly={{ x: -300, duration: 300, opacity: 0, easing: cubicOut }}
    >
      <div class="h-full overflow-y-auto pr-1">
        {#if activeTab === "contratistas"}
          <IngresoContratistaForm onSuccess={closeForm} onClose={closeForm} />
        {:else if activeTab === "visitas"}
          <IngresoVisitaForm onSuccess={closeForm} onClose={closeForm} />
        {:else if activeTab === "proveedores"}
          <IngresoProveedorForm onSuccess={closeForm} onClose={closeForm} />
        {:else}
          <!-- Fallback por si acaso -->
          <div class="bg-gray-800 p-4 rounded text-white">
            Seleccione una pestaña válida
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Panel Derecho: Contenido con Pestañas -->
  <div class="flex-1 h-full min-w-0 flex flex-col transition-all duration-300">
    <!-- Tabs Header -->
    <div
      class="bg-[#252526] border-b border-white/10 px-4 pt-2 flex gap-1 items-end"
    >
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
        'contratistas'
          ? 'border-blue-500 text-blue-500 bg-white/5'
          : 'border-transparent text-gray-400 hover:text-gray-200'}"
        on:click={() => setActiveTab("contratistas")}
      >
        Contratistas
      </button>
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
        'visitas'
          ? 'border-blue-500 text-blue-500 bg-white/5'
          : 'border-transparent text-gray-400 hover:text-gray-200'}"
        on:click={() => setActiveTab("visitas")}
      >
        Visitas
      </button>
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
        'proveedores'
          ? 'border-blue-500 text-blue-500 bg-white/5'
          : 'border-transparent text-gray-400 hover:text-gray-200'}"
        on:click={() => setActiveTab("proveedores")}
      >
        Proveedores
      </button>
      <button
        class="px-4 py-2 text-sm font-medium border-b-2 transition-colors {activeTab ===
        'citas'
          ? 'border-blue-500 text-blue-500 bg-white/5'
          : 'border-transparent text-gray-400 hover:text-gray-200'}"
        on:click={() => setActiveTab("citas")}
      >
        Citas (Hoy)
      </button>
    </div>

    <!-- Tab Content -->
    <div class="flex-1 relative overflow-hidden">
      {#if activeTab === "contratistas"}
        <IngresosActivosTable
          onRegisterClick={openForm}
          onCloseForm={closeForm}
          {isFormOpen}
        />
      {:else if activeTab === "visitas"}
        <IngresoVisitasTable
          onRegisterClick={openForm}
          onCloseForm={closeForm}
          {isFormOpen}
        />
      {:else if activeTab === "proveedores"}
        <IngresoProveedoresTable
          onRegisterClick={openForm}
          onCloseForm={closeForm}
          {isFormOpen}
        />
      {:else if activeTab === "citas"}
        <CitasList />
      {/if}
    </div>
  </div>
</div>
