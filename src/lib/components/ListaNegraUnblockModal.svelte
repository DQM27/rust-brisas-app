<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { ShieldCheck } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";

  export let bloqueado: ListaNegraResponse;
  export let onUnblock: (data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) => Promise<void>;

  let showModal = false;
  let motivoDesbloqueo = "";
  let observaciones = "";
  let loading = false;

  function openModal() {
    console.log(
      "Opening unblock modal for:",
      bloqueado.nombreCompleto,
      "isActive:",
      bloqueado.isActive,
    );
    showModal = true;
    motivoDesbloqueo = "";
    observaciones = "";
  }

  function closeModal() {
    showModal = false;
    motivoDesbloqueo = "";
    observaciones = "";
  }

  async function handleSubmit() {
    if (!motivoDesbloqueo.trim()) return;

    loading = true;

    await onUnblock({
      id: bloqueado.id,
      motivoDesbloqueo: motivoDesbloqueo,
      observaciones: observaciones.trim() || undefined,
    });

    loading = false;
    closeModal();
  }

  $: isFormValid = motivoDesbloqueo.trim().length > 0;
</script>

{#if bloqueado.isActive}
  <!-- Botón de acción -->
  <button
    type="button"
    on:click={openModal}
    class="inline-flex items-center gap-1.5 rounded-lg border border-green-500/20 bg-green-500/10 px-3 py-1.5 text-xs font-medium text-green-400 transition-all hover:bg-green-500/20"
    title="Desbloquear"
  >
    <ShieldCheck size={14} />
    Desbloquear
  </button>
{:else}
  <span class="text-xs text-gray-500">Desbloqueado</span>
{/if}

<!-- Modal -->
{#if showModal}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 200 }}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="absolute inset-0 bg-black/60 backdrop-blur-sm"
      on:click={closeModal}
    ></div>
    <div
      class="relative z-10 w-full max-w-md rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10"
      transition:fly={{ y: 20, duration: 300 }}
    >
      <!-- Header -->
      <div class="border-b border-white/10 px-6 py-4">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-500/10"
          >
            <ShieldCheck class="h-6 w-6 text-green-500" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white">
              Desbloquear Persona
            </h3>
            <p class="text-sm text-gray-400">
              {bloqueado.nombreCompleto}
            </p>
          </div>
        </div>
      </div>

      <!-- Content -->
      <form on:submit|preventDefault={handleSubmit} class="p-6 space-y-4">
        <!-- Info del bloqueado -->
        <div class="rounded-lg border border-green-500/20 bg-green-500/5 p-3">
          <div class="space-y-1.5 text-sm">
            <div class="flex items-center gap-2">
              <span class="text-gray-400">Cédula:</span>
              <span class="font-mono text-white">{bloqueado.cedula}</span>
            </div>
            {#if bloqueado.empresaNombre}
              <div class="flex items-center gap-2">
                <span class="text-gray-400">Empresa:</span>
                <span class="text-white">{bloqueado.empresaNombre}</span>
              </div>
            {/if}
            <div
              class="flex items-start gap-2 pt-2 mt-2 border-t border-green-500/20"
            >
              <span class="text-gray-400">Motivo del bloqueo:</span>
              <span class="text-white flex-1">{bloqueado.motivoBloqueo}</span>
            </div>
          </div>
        </div>

        <!-- Motivo -->
        <div class="space-y-1.5">
          <label for="motivo" class="text-sm font-medium text-gray-300">
            Motivo del Desbloqueo
            <span class="text-red-500">*</span>
          </label>
          <textarea
            id="motivo"
            bind:value={motivoDesbloqueo}
            rows="3"
            disabled={loading}
            placeholder="Ej: Cumplió sanción, revisión de caso, error administrativo..."
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:border-green-500 focus:ring-1 focus:ring-green-500 focus:outline-none resize-y disabled:opacity-50"
          ></textarea>
        </div>

        <!-- Observaciones -->
        <div class="space-y-1.5">
          <label for="observaciones" class="text-sm font-medium text-gray-300">
            Observaciones (Opcional)
          </label>
          <textarea
            id="observaciones"
            bind:value={observaciones}
            rows="2"
            disabled={loading}
            placeholder="Notas internas adicionales..."
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:border-green-500 focus:ring-1 focus:ring-green-500 focus:outline-none resize-y disabled:opacity-50"
          ></textarea>
        </div>

        <!-- Buttons -->
        <div class="flex justify-end gap-3 pt-2">
          <button
            type="button"
            on:click={closeModal}
            disabled={loading}
            class="rounded-lg px-4 py-2 text-sm font-medium text-gray-400 hover:text-white transition-colors disabled:opacity-50"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading || !isFormValid}
            class="rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white transition-all hover:bg-green-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {#if loading}
              <svg
                class="animate-spin -ml-1 mr-2 h-4 w-4 inline-block"
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
              Procesando...
            {:else}
              Confirmar Desbloqueo
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
