<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { ShieldAlert, ShieldCheck } from "lucide-svelte";
  import type { ContratistaResponse } from "$lib/types/contratista";

  export let contratista: ContratistaResponse;
  export let isBlocked: boolean;
  export let onBlock: (data: {
    contratistaId: string;
    motivoBloqueo: string;
    observaciones?: string;
  }) => Promise<void>;
  export let onUnblock: (data: {
    id: string;
    motivoDesbloqueo: string;
    observaciones?: string;
  }) => Promise<void>;

  let showModal = false;
  let motivoBloqueo = "";
  let observaciones = "";
  let loading = false;

  function openModal() {
    showModal = true;
    motivoBloqueo = "";
    observaciones = "";
  }

  function closeModal() {
    showModal = false;
    motivoBloqueo = "";
    observaciones = "";
  }

  async function handleSubmit() {
    if (!motivoBloqueo.trim()) return;

    loading = true;

    if (isBlocked) {
      // Desbloquear
      await onUnblock({
        id: contratista.id,
        motivoDesbloqueo: motivoBloqueo,
        observaciones: observaciones.trim() || undefined,
      });
    } else {
      // Bloquear
      await onBlock({
        contratistaId: contratista.id,
        motivoBloqueo: motivoBloqueo,
        observaciones: observaciones.trim() || undefined,
      });
    }

    loading = false;
    closeModal();
  }

  $: isFormValid = motivoBloqueo.trim().length > 0;
</script>

<!-- Botón de acción -->
<button
  on:click={openModal}
  class="inline-flex items-center gap-1.5 rounded-lg border px-3 py-1.5 text-xs font-medium transition-all {isBlocked
    ? 'border-green-500/20 bg-green-500/10 text-green-400 hover:bg-green-500/20'
    : 'border-red-500/20 bg-red-500/10 text-red-400 hover:bg-red-500/20'}"
  title={isBlocked ? "Desbloquear" : "Bloquear"}
>
  {#if isBlocked}
    <ShieldCheck size={14} />
    Desbloquear
  {:else}
    <ShieldAlert size={14} />
    Bloquear
  {/if}
</button>

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
            class="flex h-10 w-10 items-center justify-center rounded-lg {isBlocked
              ? 'bg-green-500/10'
              : 'bg-red-500/10'}"
          >
            {#if isBlocked}
              <ShieldCheck class="h-6 w-6 text-green-500" />
            {:else}
              <ShieldAlert class="h-6 w-6 text-red-500" />
            {/if}
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white">
              {isBlocked ? "Desbloquear Contratista" : "Bloquear Contratista"}
            </h3>
            <p class="text-sm text-gray-400">
              {contratista.nombreCompleto}
            </p>
          </div>
        </div>
      </div>

      <!-- Content -->
      <form on:submit|preventDefault={handleSubmit} class="p-6 space-y-4">
        <!-- Info del contratista -->
        <div
          class="rounded-lg border p-3 {isBlocked
            ? 'border-green-500/20 bg-green-500/5'
            : 'border-red-500/20 bg-red-500/5'}"
        >
          <div class="space-y-1.5 text-sm">
            <div class="flex items-center gap-2">
              <span class="text-gray-400">Cédula:</span>
              <span class="font-mono text-white">{contratista.cedula}</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="text-gray-400">Empresa:</span>
              <span class="text-white">{contratista.empresaNombre}</span>
            </div>
          </div>
        </div>

        <!-- Motivo -->
        <div class="space-y-1.5">
          <label for="motivo" class="text-sm font-medium text-gray-300">
            {isBlocked ? "Motivo del Desbloqueo" : "Motivo del Bloqueo"}
            <span class="text-red-500">*</span>
          </label>
          <textarea
            id="motivo"
            bind:value={motivoBloqueo}
            rows="3"
            disabled={loading}
            placeholder={isBlocked
              ? "Ej: Cumplió sanción, revisión de caso, error administrativo..."
              : "Ej: Agresión verbal, incumplimiento de normas, conducta inapropiada..."}
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:border-{isBlocked
              ? 'green'
              : 'red'}-500 focus:ring-1 focus:ring-{isBlocked
              ? 'green'
              : 'red'}-500 focus:outline-none resize-y disabled:opacity-50"
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
            class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:border-{isBlocked
              ? 'green'
              : 'red'}-500 focus:ring-1 focus:ring-{isBlocked
              ? 'green'
              : 'red'}-500 focus:outline-none resize-y disabled:opacity-50"
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
            class="rounded-lg px-4 py-2 text-sm font-medium text-white transition-all disabled:opacity-50 disabled:cursor-not-allowed {isBlocked
              ? 'bg-green-600 hover:bg-green-500'
              : 'bg-red-600 hover:bg-red-500'}"
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
            {:else if isBlocked}
              Confirmar Desbloqueo
            {:else}
              Confirmar Bloqueo
            {/if}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
