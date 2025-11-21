<script lang="ts">
  import { onMount } from "svelte";
  import { fade, fly, scale } from "svelte/transition";
  import { submitCreateEmpresa, submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";

  export let loading = false;
  export let onSubmit: (data: any) => void;

  // Lista de empresas
  export let empresas: { id: string; nombre: string }[] = [];

  // --- Campos del formulario ---
  let cedula = "";
  let nombre = "";
  let apellido = "";
  let empresaId = "";
  let fechaVencimientoPraind = "";

  // Vehículo
  let tieneVehiculo = false;
  let placa = "";
  let marca = "";
  let modelo = "";
  let color = "";

  // --- UI States ---
  let loadingEmpresas = false;
  let showEmpresaModal = false;
  let nuevaEmpresaNombre = "";
  let creatingEmpresa = false;
  let empresaError = "";

  // Carga inicial de empresas
  onMount(async () => {
    if (empresas.length > 0) return;

    loadingEmpresas = true;
    const res = await submitFetchActiveEmpresas();
    if (res.ok) empresas = res.empresas;
    loadingEmpresas = false;
  });

  export function reset() {
    cedula = ""; nombre = ""; apellido = ""; empresaId = "";
    fechaVencimientoPraind = ""; tieneVehiculo = false;
    placa = ""; marca = ""; modelo = ""; color = "";
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    if (typeof onSubmit === "function") {
      onSubmit({ cedula, nombre, apellido, empresaId, fechaVencimientoPraind, tieneVehiculo, placa, marca, modelo, color });
    }
  }

  $: isFormValid = cedula.trim() && nombre.trim() && apellido.trim() && empresaId.trim() && fechaVencimientoPraind.trim() && (!tieneVehiculo || (placa.trim() && marca.trim()));

  async function handleCrearEmpresa() {
    if (!nuevaEmpresaNombre.trim()) return;
    creatingEmpresa = true; empresaError = "";
    const result = await submitCreateEmpresa(nuevaEmpresaNombre);
    if (result.ok) {
      empresas = [...empresas, { id: result.empresa.id, nombre: result.empresa.nombre }];
      empresaId = result.empresa.id;
      nuevaEmpresaNombre = ""; showEmpresaModal = false;
    } else empresaError = result.error;
    creatingEmpresa = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div class="relative z-10 w-full rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10 transition-[max-width] duration-500 ease-in-out {tieneVehiculo ? 'max-w-5xl' : 'max-w-xl'}">
    <div class="border-b border-white/10 px-8 py-5">
      <h2 class="text-xl font-semibold text-gray-100">Registrar Contratista</h2>
      <p class="mt-1 text-sm text-gray-400">Ingresa los datos requeridos para el acceso.</p>
    </div>

    <form on:submit={handleSubmit} class="p-8 space-y-8">
      <div class="flex flex-col gap-8 lg:flex-row">
        <!-- Columna 1: Datos Personales y Empresa -->
        <div class="flex-1 space-y-5">
          <div class="space-y-1.5">
            <label class="text-sm font-medium text-gray-300">Cédula</label>
            <input type="text" bind:value={cedula} placeholder="1-2345-6789" disabled={loading}
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none transition-all" />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-1.5">
              <label class="text-sm font-medium text-gray-300">Nombre</label>
              <input type="text" bind:value={nombre} placeholder="Juan" disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
            </div>
            <div class="space-y-1.5">
              <label class="text-sm font-medium text-gray-300">Apellido</label>
              <input type="text" bind:value={apellido} placeholder="Pérez" disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
            </div>
          </div>

          <div class="space-y-1.5">
            <label class="text-sm font-medium text-gray-300">Empresa</label>
            <select bind:value={empresaId} disabled={loading || loadingEmpresas}
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none disabled:opacity-50">
              <option value="" disabled selected>{loadingEmpresas ? 'Cargando empresas...' : 'Seleccione una empresa'}</option>
              {#each empresas as empresa}
                <option value={empresa.id}>{empresa.nombre}</option>
              {/each}
            </select>
          </div>

          <div class="space-y-1.5">
            <label class="text-sm font-medium text-gray-300">Fecha PRAIND</label>
            <input type="date" bind:value={fechaVencimientoPraind} disabled={loading}
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none calendar-icon-white" />
          </div>

          <!-- Toggle Vehículo -->
          <div class="pt-2 flex items-center justify-between rounded-lg border border-white/5 bg-white/5 p-3">
            <span class="text-sm font-medium text-gray-200">¿Agregar Vehículo?</span>
            <button type="button" role="switch" aria-checked={tieneVehiculo} on:click={() => tieneVehiculo = !tieneVehiculo}
              class="group relative inline-flex h-6 w-11 cursor-pointer rounded-full transition-colors duration-200 ease-in-out {tieneVehiculo ? 'bg-blue-600' : 'bg-[#3e3e3e]'}">
              <span class="pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white transition-transform {tieneVehiculo ? 'translate-x-5' : 'translate-x-0'}"></span>
            </button>
          </div>
        </div>

        <!-- Columna Vehículo -->
        {#if tieneVehiculo}
          <div class="hidden lg:block w-px bg-gradient-to-b from-transparent via-white/10 to-transparent" transition:fade></div>
          <div class="flex-1 lg:min-w-[300px]" in:fly={{ x: -20, duration: 400 }} out:fade={{ duration: 200 }}>
            <div class="space-y-5">
              <div class="space-y-1.5">
                <label class="text-sm font-medium text-gray-300">Placa</label>
                <input type="text" bind:value={placa} placeholder="ABC-123"
                  class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1.5">
                  <label class="text-sm font-medium text-gray-300">Marca</label>
                  <input type="text" bind:value={marca} placeholder="Toyota"
                    class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
                </div>
                <div class="space-y-1.5">
                  <label class="text-sm font-medium text-gray-300">Modelo</label>
                  <input type="text" bind:value={modelo} placeholder="Corolla"
                    class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
                </div>
              </div>
              <div class="space-y-1.5">
                <label class="text-sm font-medium text-gray-300">Color</label>
                <input type="text" bind:value={color} placeholder="Blanco"
                  class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none" />
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Submit -->
      <div class="pt-8 border-t border-white/5 mt-8">
        <button type="submit" disabled={loading || !isFormValid}
          class="w-full rounded-lg bg-blue-600 px-4 py-3 text-sm font-semibold text-white shadow-lg transition-all hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-50">
          {loading ? 'Procesando...' : 'Registrar Contratista'}
        </button>
      </div>
    </form>
  </div>
</div>

<!-- Modal Nueva Empresa -->
{#if showEmpresaModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6" transition:fade={{ duration: 200 }}>
    <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" on:click={() => !creatingEmpresa && (showEmpresaModal = false)}></div>
    <div class="relative w-full max-w-md overflow-hidden rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10" transition:scale={{ start: 0.95, duration: 200 }}>
      <div class="px-6 py-5 border-b border-white/10">
        <h3 class="text-lg font-semibold text-white">Nueva Empresa</h3>
        <p class="text-xs text-gray-400 mt-1">Agrega una nueva empresa al catálogo.</p>
      </div>
      <div class="p-6 space-y-4">
        {#if empresaError}
          <div class="rounded bg-red-500/10 border border-red-500/20 p-3 text-sm text-red-400">{empresaError}</div>
        {/if}
        <div class="space-y-1.5">
          <label class="text-sm font-medium text-gray-300">Nombre de la Empresa</label>
          <input type="text" bind:value={nuevaEmpresaNombre} placeholder="Ej. Servicios Generales S.A." disabled={creatingEmpresa} autofocus
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
            on:keydown={(e) => e.key === 'Enter' && handleCrearEmpresa()} />
        </div>
      </div>
      <div class="flex justify-end gap-3 px-6 py-4 bg-[#252526] border-t border-white/10">
        <button type="button" disabled={creatingEmpresa} on:click={() => showEmpresaModal = false} class="rounded-lg px-4 py-2 text-sm font-medium text-gray-300 hover:bg-white/5 transition-colors">Cancelar</button>
        <button type="button" disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()} on:click={handleCrearEmpresa} class="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-500 disabled:opacity-50">{creatingEmpresa ? 'Guardando...' : 'Guardar Empresa'}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .calendar-icon-white::-webkit-calendar-picker-indicator {
    filter: invert(1);
    opacity: 0.6;
    cursor: pointer;
  }
  .calendar-icon-white::-webkit-calendar-picker-indicator:hover {
    opacity: 1;
  }
</style>
