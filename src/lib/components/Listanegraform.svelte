<script lang="ts">
  import { fly, fade } from "svelte/transition";

  // --- PROPS (solo datos y callbacks) ---
  export let loading = false;
  export let contratistas: { id: string; nombreCompleto: string; cedula: string }[] = [];
  export let loadingContratistas = false;
  
  // Form state (controlado desde afuera via bind)
  export let modoRegistro: "existente" | "manual" = "existente";
  export let contratistaId = "";
  export let cedula = "";
  export let nombre = "";
  export let apellido = "";
  export let motivoBloqueo = "";
  export let tipoBloqueo: "permanente" | "temporal" = "permanente";
  export let fechaFinBloqueo = "";
  export let bloqueadoPor = "";
  export let observaciones = "";
  
  // Callbacks
  export let onSubmit: () => void;
  export let onModoChange: (modo: "existente" | "manual") => void;
  export let onTipoBloqueoChange: (tipo: "permanente" | "temporal") => void;
  
  // Computed (solo UI)
  $: isFormValid = 
    motivoBloqueo.trim() && 
    bloqueadoPor.trim() &&
    (tipoBloqueo === "permanente" || fechaFinBloqueo.trim()) &&
    (
      (modoRegistro === "existente" && contratistaId.trim()) ||
      (modoRegistro === "manual" && cedula.trim() && nombre.trim() && apellido.trim())
    );
</script>

<div class="flex min-h-full items-center justify-center p-6">
  
  <div class="relative z-10 w-full max-w-2xl rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10">
    
    <div class="border-b border-white/10 px-8 py-5">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-red-500/10">
          <svg class="h-6 w-6 text-red-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
          </svg>
        </div>
        <div>
          <h2 class="text-xl font-semibold text-gray-100">Agregar a Lista Negra</h2>
          <p class="mt-1 text-sm text-gray-400">Bloquear acceso a una persona del sistema.</p>
        </div>
      </div>
    </div>

    <form on:submit|preventDefault={onSubmit} class="p-8">
      
      <!-- Selector de Modo de Registro -->
      <div class="mb-6 space-y-3">
        <label class="text-sm font-medium text-gray-300">Modo de Registro</label>
        <div class="grid grid-cols-2 gap-3">
          <button
            type="button"
            on:click={() => onModoChange("existente")}
            disabled={loading}
            class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {modoRegistro === 'existente' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
            </svg>
            <span>Contratista Existente</span>
          </button>
          <button
            type="button"
            on:click={() => onModoChange("manual")}
            disabled={loading}
            class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {modoRegistro === 'manual' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
            </svg>
            <span>Registro Manual</span>
          </button>
        </div>
      </div>

      <div class="space-y-5">
        
        <!-- MODO: CONTRATISTA EXISTENTE -->
        {#if modoRegistro === "existente"}
          <div class="space-y-1.5" in:fly={{ x: -20, duration: 300 }} out:fade={{ duration: 200 }}>
            <label for="contratistaId" class="text-sm font-medium text-gray-300">Contratista</label>
            <div class="relative">
              <select id="contratistaId" bind:value={contratistaId} disabled={loading || loadingContratistas}
                class="w-full appearance-none rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 pr-10 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none disabled:opacity-50">
                <option value="" disabled selected>
                  {loadingContratistas ? 'Cargando contratistas...' : 'Seleccione un contratista'}
                </option>
                {#each contratistas as contratista}
                  <option value={contratista.id}>{contratista.nombreCompleto} - {contratista.cedula}</option>
                {/each}
              </select>
              <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-gray-400">
                {#if loadingContratistas}
                  <svg class="animate-spin h-4 w-4 text-red-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                {:else}
                  <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                  </svg>
                {/if}
              </div>
            </div>
          </div>
        {/if}

        <!-- MODO: REGISTRO MANUAL -->
        {#if modoRegistro === "manual"}
          <div class="space-y-5" in:fly={{ x: -20, duration: 300 }} out:fade={{ duration: 200 }}>
            <div class="space-y-1.5">
              <label for="cedula" class="text-sm font-medium text-gray-300">Cédula</label>
              <input id="cedula" type="text" bind:value={cedula} placeholder="1-2345-6789" disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-1.5">
                <label for="nombre" class="text-sm font-medium text-gray-300">Nombre</label>
                <input id="nombre" type="text" bind:value={nombre} placeholder="Juan" disabled={loading}
                  class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
              </div>
              <div class="space-y-1.5">
                <label for="apellido" class="text-sm font-medium text-gray-300">Apellido</label>
                <input id="apellido" type="text" bind:value={apellido} placeholder="Pérez" disabled={loading}
                  class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
              </div>
            </div>
          </div>
        {/if}

        <!-- DATOS DEL BLOQUEO -->
        <div class="pt-4 border-t border-white/10 space-y-5">
          
          <div class="space-y-1.5">
            <label for="motivoBloqueo" class="text-sm font-medium text-gray-300">
              Motivo del Bloqueo <span class="text-red-500">*</span>
            </label>
            <textarea id="motivoBloqueo" bind:value={motivoBloqueo} rows="3" disabled={loading}
              placeholder="Describa el motivo por el cual se bloquea el acceso..."
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none resize-none"></textarea>
            <p class="text-xs text-gray-500">{motivoBloqueo.length}/500 caracteres</p>
          </div>

          <div class="space-y-3">
            <label class="text-sm font-medium text-gray-300">Tipo de Bloqueo</label>
            <div class="grid grid-cols-2 gap-3">
              <button
                type="button"
                on:click={() => onTipoBloqueoChange("permanente")}
                disabled={loading}
                class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {tipoBloqueo === 'permanente' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
              >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                </svg>
                <span>Permanente</span>
              </button>
              <button
                type="button"
                on:click={() => onTipoBloqueoChange("temporal")}
                disabled={loading}
                class="flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-sm font-medium transition-all {tipoBloqueo === 'temporal' ? 'border-red-500 bg-red-500/10 text-red-400' : 'border-white/10 bg-[#252526] text-gray-400 hover:border-white/20 hover:text-gray-300'}"
              >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
                <span>Temporal</span>
              </button>
            </div>
          </div>

          {#if tipoBloqueo === "temporal"}
            <div class="space-y-1.5" in:fly={{ y: -10, duration: 300 }} out:fade={{ duration: 200 }}>
              <label for="fechaFinBloqueo" class="text-sm font-medium text-gray-300">
                Fecha de Finalización del Bloqueo
              </label>
              <input id="fechaFinBloqueo" type="datetime-local" bind:value={fechaFinBloqueo} disabled={loading}
                class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none calendar-icon-white" />
              <p class="text-xs text-gray-500">El bloqueo se levantará automáticamente en esta fecha</p>
            </div>
          {/if}

          <div class="space-y-1.5">
            <label for="bloqueadoPor" class="text-sm font-medium text-gray-300">
              Bloqueado Por <span class="text-red-500">*</span>
            </label>
            <input id="bloqueadoPor" type="text" bind:value={bloqueadoPor} disabled={loading}
              placeholder="Nombre del responsable del bloqueo"
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none" />
          </div>

          <div class="space-y-1.5">
            <label for="observaciones" class="text-sm font-medium text-gray-300">
              Observaciones <span class="text-xs text-gray-500">(Opcional)</span>
            </label>
            <textarea id="observaciones" bind:value={observaciones} rows="2" disabled={loading}
              placeholder="Información adicional relevante..."
              class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white focus:border-red-500 focus:ring-1 focus:ring-red-500 focus:outline-none resize-none"></textarea>
          </div>
        </div>

        <!-- Advertencia -->
        <div class="rounded-lg bg-red-500/10 border border-red-500/20 p-4">
          <div class="flex gap-3">
            <svg class="h-5 w-5 text-red-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
            </svg>
            <div class="text-sm text-red-200">
              <p class="font-medium">Esta acción bloqueará el acceso al sistema</p>
              <p class="mt-1 text-xs text-red-300/80">La persona no podrá ingresar mientras el bloqueo esté activo. Asegúrese de que la información sea correcta.</p>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-6 border-t border-white/10 mt-6">
        <button type="submit" disabled={loading || !isFormValid}
          class="w-full rounded-lg bg-red-600 px-4 py-3 text-sm font-semibold text-white shadow-lg shadow-red-900/20 transition-all hover:bg-red-500 disabled:cursor-not-allowed disabled:opacity-50 flex items-center justify-center gap-2">
          <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
          </svg>
          {loading ? 'Bloqueando Acceso...' : 'Bloquear Acceso'}
        </button>
      </div>
    </form>
  </div>
</div>

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