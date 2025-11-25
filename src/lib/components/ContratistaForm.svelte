<script lang="ts">
  import { onMount } from "svelte";
  import { fly, fade, scale } from "svelte/transition";
  import {
    submitCreateEmpresa,
    submitFetchActiveEmpresas,
  } from "$lib/logic/empresa/empresaService";
  import type { TipoVehiculo } from "$lib/types/vehiculo";

  export let loading = false;
  export let onSubmit: (data: any) => void;
  export let empresas: { id: string; nombre: string }[] = [];

  // --- ESTADO DEL FORMULARIO ---
  let cedula = "";
  let nombre = "";
  let segundoNombre = "";
  let apellido = "";
  let segundoApellido = "";
  let empresaId = "";
  let fechaVencimientoPraind = "";

  // Vehículo
  let tieneVehiculo = false;
  let tipoVehiculo: TipoVehiculo | "" = "";
  let placa = "";
  let marca = "";
  let modelo = "";
  let color = "";

  // --- ESTADOS UI ---
  let loadingEmpresas = false;
  let showEmpresaModal = false;
  let nuevaEmpresaNombre = "";
  let creatingEmpresa = false;
  let empresaError = "";

  // --- CARGA INICIAL ---
  onMount(async () => {
    if (empresas.length > 0) return;
    loadingEmpresas = true;
    const resultado = await submitFetchActiveEmpresas();
    if (resultado.ok) {
      empresas = resultado.empresas;
    }
    loadingEmpresas = false;
  });

  export function reset() {
    cedula = "";
    nombre = "";
    segundoNombre = "";
    apellido = "";
    segundoApellido = "";
    empresaId = "";
    fechaVencimientoPraind = "";
    tieneVehiculo = false;
    tipoVehiculo = "";
    placa = "";
    marca = "";
    modelo = "";
    color = "";
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    onSubmit({
      cedula,
      nombre,
      segundoNombre,
      apellido,
      segundoApellido,
      empresaId,
      fechaVencimientoPraind,
      tieneVehiculo,
      tipoVehiculo: tipoVehiculo || undefined,
      placa,
      marca,
      modelo,
      color,
    });
  }

  $: isFormValid =
    cedula.trim() &&
    nombre.trim() &&
    apellido.trim() &&
    empresaId.trim() &&
    fechaVencimientoPraind.trim() &&
    (!tieneVehiculo || (tipoVehiculo && placa.trim() && marca.trim()));

  async function handleCrearEmpresa() {
    if (!nuevaEmpresaNombre.trim()) return;
    creatingEmpresa = true;
    empresaError = "";
    const result = await submitCreateEmpresa(nuevaEmpresaNombre);
    if (result.ok) {
      empresas = [
        ...empresas,
        { id: result.empresa.id, nombre: result.empresa.nombre },
      ];
      empresaId = result.empresa.id;
      nuevaEmpresaNombre = "";
      showEmpresaModal = false;
    } else {
      empresaError = result.error;
    }
    creatingEmpresa = false;
  }
</script>

<div class="flex min-h-full items-center justify-center p-4 sm:p-6">
  <div
    class="relative z-10 w-full max-w-[90vw] rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10 transition-all duration-500 ease-in-out {tieneVehiculo
      ? 'max-w-[590px]'
      : 'max-w-sm'}"
  >
    <div class="border-b border-white/10 px-6 py-4">
      <h2 class="text-lg font-semibold text-gray-100">Registrar Contratista</h2>
      <p class="mt-1 text-xs text-gray-400">
        Ingresa los datos requeridos para el acceso.
      </p>
    </div>

    <form on:submit={handleSubmit} class="p-6">
      <div class="flex flex-col gap-6 lg:flex-row">
        <!-- Columna Principal -->
        <div class="flex-1 space-y-4">
          <div class="space-y-1">
            <label for="cedula" class="text-xs font-medium text-gray-300"
              >Cédula</label
            >
            <input
              id="cedula"
              type="text"
              bind:value={cedula}
              placeholder="1-2345-6789"
              disabled={loading}
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none transition-all"
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="space-y-1">
              <label for="nombre" class="text-xs font-medium text-gray-300"
                >Nombre <span class="text-red-400">*</span></label
              >
              <input
                id="nombre"
                type="text"
                bind:value={nombre}
                placeholder="Juan"
                disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
              />
            </div>
            <div class="space-y-1">
              <label for="segundoNombre" class="text-xs font-medium text-gray-300"
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                type="text"
                bind:value={segundoNombre}
                placeholder="Carlos"
                disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <div class="space-y-1">
              <label for="apellido" class="text-xs font-medium text-gray-300"
                >Apellido <span class="text-red-400">*</span></label
              >
              <input
                id="apellido"
                type="text"
                bind:value={apellido}
                placeholder="Pérez"
                disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
              />
            </div>
            <div class="space-y-1">
              <label for="segundoApellido" class="text-xs font-medium text-gray-300"
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                type="text"
                bind:value={segundoApellido}
                placeholder="González"
                disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
              />
            </div>
          </div>

          <div class="space-y-1">
            <label for="empresaId" class="text-xs font-medium text-gray-300"
              >Empresa</label
            >
            <div class="flex gap-2">
              <div class="relative flex-1">
                <select
                  id="empresaId"
                  bind:value={empresaId}
                  disabled={loading || loadingEmpresas}
                  class="w-full appearance-none rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none disabled:opacity-50"
                >
                  <option value="" disabled selected>
                    {loadingEmpresas
                      ? "Cargando empresas..."
                      : "Seleccione una empresa"}
                  </option>
                  {#each empresas as empresa}
                    <option value={empresa.id}>{empresa.nombre}</option>
                  {/each}
                </select>
                <div
                  class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-400"
                >
                  {#if loadingEmpresas}
                    <svg
                      class="animate-spin h-3 w-3 text-blue-500"
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
                  {:else}
                    <svg
                      class="h-3 w-3"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      ><path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 9l-7 7-7-7"
                      ></path></svg
                    >
                  {/if}
                </div>
              </div>
              <button
                type="button"
                on:click={() => (showEmpresaModal = true)}
                disabled={loading}
                class="flex items-center justify-center rounded-lg bg-[#2d2d2d] px-2 text-gray-400 hover:bg-[#3d3d3d] hover:text-white border border-white/10 transition-colors flex-shrink-0"
                title="Agregar Nueva Empresa"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-4 w-4"
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
              </button>
            </div>
          </div>

          <div class="space-y-1">
            <label
              for="fechaVencimientoPraind"
              class="text-xs font-medium text-gray-300">Fecha PRAIND</label
            >
            <input
              id="fechaVencimientoPraind"
              type="date"
              bind:value={fechaVencimientoPraind}
              disabled={loading}
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none calendar-icon-white"
            />
          </div>

          <div
            class="pt-1 flex items-center justify-between rounded-lg border border-white/5 bg-white/5 p-3"
          >
            <div class="flex flex-col">
              <span class="text-xs font-medium text-gray-200"
                >¿Agregar Vehículo?</span
              >
            </div>
            <button
              type="button"
              role="switch"
              aria-checked={tieneVehiculo}
              aria-label="Agregar Vehículo"
              on:click={() => (tieneVehiculo = !tieneVehiculo)}
              class="group relative inline-flex h-5 w-9 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-[#1e1e1e] {tieneVehiculo
                ? 'bg-blue-600'
                : 'bg-[#3e3e3e]'}"
            >
              <span
                class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {tieneVehiculo
                  ? 'translate-x-4'
                  : 'translate-x-0'}"
              ></span>
            </button>
          </div>
        </div>

        {#if tieneVehiculo}
          <div
            class="hidden lg:block w-px bg-gradient-to-b from-transparent via-white/10 to-transparent self-stretch my-2"
            transition:fade
          ></div>

          <div
            class="flex-1 lg:min-w-[200px]"
            in:fly={{ x: -20, duration: 400, delay: 100 }}
            out:fade={{ duration: 200 }}
          >
            <div class="h-full space-y-4">
              <div class="mb-3">
                <h3 class="text-base font-medium text-gray-200">
                  Datos del Vehículo
                </h3>
                <p class="text-xs text-gray-500">
                  Información necesaria para el pase vehicular.
                </p>
              </div>

              <div class="space-y-1">
                <span class="block text-xs font-medium text-gray-300"
                  >Tipo de Vehículo</span
                >
                <div class="grid grid-cols-2 gap-2">
                  <button
                    type="button"
                    on:click={() => (tipoVehiculo = "motocicleta")}
                    class="flex items-center justify-center gap-1 rounded-lg border px-3 py-2 text-xs font-medium transition-all {tipoVehiculo ===
                    'motocicleta'
                      ? 'border-blue-500 bg-blue-500/10 text-blue-400'
                      : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
                  >
                    <svg
                      class="h-4 w-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                      />
                    </svg>
                    <span>Moto</span>
                  </button>
                  <button
                    type="button"
                    on:click={() => (tipoVehiculo = "automóvil")}
                    class="flex items-center justify-center gap-1 rounded-lg border px-3 py-2 text-xs font-medium transition-all {tipoVehiculo ===
                    'automóvil'
                      ? 'border-blue-500 bg-blue-500/10 text-blue-400'
                      : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
                  >
                    <svg
                      class="h-4 w-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"
                      />
                    </svg>
                    <span>Auto</span>
                  </button>
                </div>
              </div>

              <div class="space-y-1">
                <label for="placa" class="text-xs font-medium text-gray-300"
                  >Número de Placa</label
                >
                <input
                  id="placa"
                  type="text"
                  bind:value={placa}
                  placeholder="ABC-123"
                  class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2 text-sm text-white uppercase focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
                />
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                <div class="space-y-1">
                  <label for="marca" class="text-xs font-medium text-gray-300"
                    >Marca</label
                  >
                  <input
                    id="marca"
                    type="text"
                    bind:value={marca}
                    placeholder="Toyota"
                    class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
                  />
                </div>
                <div class="space-y-1">
                  <label for="modelo" class="text-xs font-medium text-gray-300"
                    >Modelo</label
                  >
                  <input
                    id="modelo"
                    type="text"
                    bind:value={modelo}
                    placeholder="Corolla"
                    class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
                  />
                </div>
              </div>

              <div class="space-y-1">
                <label for="color" class="text-xs font-medium text-gray-300"
                  >Color</label
                >
                <input
                  id="color"
                  type="text"
                  bind:value={color}
                  placeholder="Blanco"
                  class="w-full rounded-lg border border-white/10 bg-[#252526] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
                />
              </div>

              <div
                class="mt-3 rounded bg-blue-500/10 p-2 text-xs text-blue-200 border border-blue-500/20"
              >
                Recuerde verificar que la placa coincida con la tarjeta de
                circulación física.
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Botón centrado y más pequeño -->
      <div class="pt-6 border-t border-white/5 mt-6 flex justify-center">
        <button
          type="submit"
          disabled={loading || !isFormValid}
          class="w-auto min-w-[200px] rounded-lg bg-blue-600 px-6 py-2.5 text-sm font-semibold text-white shadow-lg shadow-blue-900/20 transition-all hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
        >
          {loading ? "Procesando..." : "Registrar Contratista"}
        </button>
      </div>
    </form>
  </div>
</div>

{#if showEmpresaModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={() => !creatingEmpresa && (showEmpresaModal = false)}
    ></div>
    <div
      class="relative w-full max-w-sm overflow-hidden rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10"
      transition:scale={{ start: 0.95, duration: 200 }}
    >
      <div class="px-5 py-4 border-b border-white/10">
        <h3 class="text-base font-semibold text-white">Nueva Empresa</h3>
        <p class="text-xs text-gray-400 mt-1">
          Agrega una nueva empresa al catálogo.
        </p>
      </div>
      <div class="p-5 space-y-3">
        {#if empresaError}
          <div
            class="rounded bg-red-500/10 border border-red-500/20 p-2 text-xs text-red-400"
          >
            {empresaError}
          </div>
        {/if}
        <div class="space-y-1">
          <label for="newEmpresa" class="text-xs font-medium text-gray-300"
            >Nombre de la Empresa</label
          >
          <input
            id="newEmpresa"
            type="text"
            bind:value={nuevaEmpresaNombre}
            placeholder="Ej. Servicios Generales S.A."
            disabled={creatingEmpresa}
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none"
            on:keydown={(e) => e.key === "Enter" && handleCrearEmpresa()}
          />
        </div>
      </div>
      <div
        class="flex justify-end gap-2 px-5 py-3 bg-[#252526] border-t border-white/10"
      >
        <button
          type="button"
          disabled={creatingEmpresa}
          on:click={() => (showEmpresaModal = false)}
          class="rounded-lg px-3 py-1.5 text-xs font-medium text-gray-300 hover:bg-white/5 transition-colors"
          >Cancelar</button
        >
        <button
          type="button"
          disabled={creatingEmpresa || !nuevaEmpresaNombre.trim()}
          on:click={handleCrearEmpresa}
          class="rounded-lg bg-blue-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-blue-500 disabled:opacity-50 flex items-center gap-1"
        >
          {creatingEmpresa ? "Guardando..." : "Guardar Empresa"}
        </button>
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