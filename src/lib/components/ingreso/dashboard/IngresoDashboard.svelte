<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";

  // Components
  import IngresoWizard from "../wizard/IngresoWizard.svelte";

  // State
  let stats = {
    contratistas: 0,
    visitas: 0,
    proveedores: 0,
    total: 0,
  };
  let loading = true;
  let showWizard = false;

  // Mock refresh needed only if we don't have real-time subscriptions yet
  async function loadStats() {
    loading = true;
    try {
      // We can use the 'get_ingresos_abiertos' unified command and count
      const activos: any[] = await invoke("get_ingresos_abiertos");

      stats.contratistas = activos.filter(
        (i) => i.tipo_ingreso === "Contratista",
      ).length;
      stats.proveedores = activos.filter(
        (i) => i.tipo_ingreso === "Proveedor",
      ).length;
      stats.visitas = activos.filter((i) => i.tipo_ingreso === "Visita").length;
      stats.total = activos.length;
    } catch (e) {
      console.error("Error loading stats", e);
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadStats();
  });

  function handleWizardComplete() {
    showWizard = false;
    loadStats();
    // Toast success?
  }
</script>

<div class="h-full flex flex-col gap-6">
  <!-- Top Action Bar -->
  <div
    class="flex justify-between items-center bg-base-100 p-4 rounded-xl shadow-sm border border-base-200"
  >
    <div>
      <h2 class="text-2xl font-bold">Panel de Control</h2>
      <p class="text-base-content/60 text-sm">
        Resumen de ingresos activos en planta
      </p>
    </div>
    <div>
      {#if !showWizard}
        <button
          class="btn btn-primary btn-lg shadow-lg gap-3"
          on:click={() => (showWizard = true)}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 4v16m8-8H4"
            />
          </svg>
          Nuevo Ingreso
        </button>
      {:else}
        <button class="btn btn-ghost" on:click={() => (showWizard = false)}>
          Cancelar Registro
        </button>
      {/if}
    </div>
  </div>

  {#if showWizard}
    <!-- WIZARD MODE -->
    <div in:fade class="flex-1 overflow-hidden">
      <IngresoWizard on:complete={handleWizardComplete} />
    </div>
  {:else}
    <!-- DASHBOARD MODE -->
    <div in:fade class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <!-- Stat Cards -->
      <div class="stats shadow bg-base-100 border border-base-200">
        <div class="stat">
          <div class="stat-figure text-primary">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-8 h-8 stroke-current"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
              ></path></svg
            >
          </div>
          <div class="stat-title">Contratistas</div>
          <div class="stat-value text-primary">{stats.contratistas}</div>
          <div class="stat-desc">Trabajando actualmente</div>
        </div>
      </div>

      <div class="stats shadow bg-base-100 border border-base-200">
        <div class="stat">
          <div class="stat-figure text-secondary">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-8 h-8 stroke-current"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
              ></path></svg
            >
          </div>
          <div class="stat-title">Visitas</div>
          <div class="stat-value text-secondary">{stats.visitas}</div>
          <div class="stat-desc">En reunión / recorrido</div>
        </div>
      </div>

      <div class="stats shadow bg-base-100 border border-base-200">
        <div class="stat">
          <div class="stat-figure text-accent">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              class="inline-block w-8 h-8 stroke-current"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
              ></path></svg
            >
          </div>
          <div class="stat-title">Proveedores</div>
          <div class="stat-value text-accent">{stats.proveedores}</div>
          <div class="stat-desc">Entregas en curso</div>
        </div>
      </div>
    </div>

    <!-- Activity Stream / Latest Entries (Optional for dashboard v2) -->
    <div class="flex-1 bg-base-100 rounded-xl border border-base-200 p-4">
      <h3 class="font-bold text-lg mb-4">Actividad Reciente</h3>
      <div
        class="flex flex-col items-center justify-center h-48 text-base-content/40"
      >
        <span>Listado de ingresos recientes (Próximamente)</span>
        <!-- Here we could reuse an AG Grid or a simple list -->
      </div>
    </div>
  {/if}
</div>
