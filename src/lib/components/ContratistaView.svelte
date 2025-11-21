<script lang="ts">
  // 1. Importamos onMount
  import { onMount } from "svelte";
  import { slide, fade, scale } from "svelte/transition"; 
  // 2. Importamos la función para traer las empresas
  import { submitCreateEmpresa, submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";

  export let loading = false;
  export let onSubmit: (data: any) => void;
  
  // Ya no es obligatorio que venga desde fuera, la inicializamos vacía
  export let empresas: { id: string; nombre: string }[] = [];

  // --- ESTADO DEL FORMULARIO ---
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

  // --- ESTADO DE CARGA DE DATOS ---
  let loadingEmpresas = false; // Variable para saber si estamos buscando empresas

  // --- ESTADO DEL MODAL ---
  let showEmpresaModal = false;
  let nuevaEmpresaNombre = "";
  let creatingEmpresa = false;
  let empresaError = "";

  // ============================================================
  //  AQUÍ ESTÁ LA MAGIA: Cargar empresas al iniciar
  // ============================================================
  onMount(async () => {
    // Si ya nos pasaron empresas desde fuera (el padre), no cargamos nada.
    if (empresas.length > 0) return;

    loadingEmpresas = true;
    const resultado = await submitFetchActiveEmpresas();
    
    if (resultado.ok) {
      empresas = resultado.empresas;
    } else {
      console.error("Error al cargar empresas:", resultado.error);
      // Aquí podrías mostrar una notificación de error si tienes un sistema de toasts
    }
    loadingEmpresas = false;
  });

  export function reset() {
    cedula = ""; nombre = ""; apellido = ""; empresaId = "";
    fechaVencimientoPraind = ""; tieneVehiculo = false;
    placa = ""; marca = ""; modelo = ""; color = "";
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    onSubmit({ cedula, nombre, apellido, empresaId, fechaVencimientoPraind, tieneVehiculo, placa, marca, modelo, color });
  }

  $: isFormValid = cedula.trim() && nombre.trim() && apellido.trim() && 
                   empresaId.trim() && fechaVencimientoPraind.trim() && 
                   (!tieneVehiculo || (placa.trim() && marca.trim()));

  // ... (La lógica del modal sigue igual) ...
  async function handleCrearEmpresa() {
    if (!nuevaEmpresaNombre.trim()) return;
    creatingEmpresa = true;
    empresaError = "";

    const result = await submitCreateEmpresa(nuevaEmpresaNombre);

    if (result.ok) {
      empresas = [...empresas, { id: result.empresa.id, nombre: result.empresa.nombre }];
      empresaId = result.empresa.id;
      nuevaEmpresaNombre = "";
      showEmpresaModal = false;
    } else {
      empresaError = result.error;
    }
    creatingEmpresa = false;
  }
</script>

<div class="space-y-1.5">
          <label for="empresaId" class="text-sm font-medium text-gray-300">Empresa</label>
          <div class="flex gap-2">
            <div class="relative w-full">
              
              <select id="empresaId" bind:value={empresaId} disabled={loading || loadingEmpresas}
                class="w-full appearance-none rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 focus:outline-none disabled:opacity-50">
                
                <option value="" disabled selected>
                  {#if loadingEmpresas}
                    Cargando empresas...
                  {:else}
                    Seleccione una empresa
                  {/if}
                </option>

                {#each empresas as empresa}
                  <option value={empresa.id}>{empresa.nombre}</option>
                {/each}
              </select>

              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-gray-400">
                {#if loadingEmpresas}
                   <svg class="animate-spin h-4 w-4 text-blue-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                {:else}
                  <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                {/if}
              </div>
            </div>

            <button 
              type="button"
              on:click={() => showEmpresaModal = true}
              disabled={loading || loadingEmpresas}
              class="flex items-center justify-center rounded-lg bg-[#2d2d2d] px-3 text-gray-400 hover:bg-[#3d3d3d] hover:text-white border border-white/10 transition-colors disabled:opacity-50"
              title="Agregar Nueva Empresa"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </button>
          </div>
        </div>