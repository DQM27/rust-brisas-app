<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { ShieldCheck, Ban, X } from "lucide-svelte";
  import type { ListaNegraResponse } from "$lib/types/listaNegra";

  interface Props {
    bloqueado: ListaNegraResponse;
    onUnblock: (data: {
      id: string;
      motivoDesbloqueo: string;
      observaciones?: string;
    }) => Promise<void>;
    onClose: () => void;
  }

  let { bloqueado, onUnblock, onClose }: Props = $props();

  let motivoDesbloqueo = $state("");
  let observaciones = $state("");
  let loading = $state(false);

  // Determinar si es desbloqueo o re-bloqueo
  const isUnblocking = $derived(bloqueado.isActive);
  const title = $derived(
    isUnblocking ? "Desbloquear Persona" : "Re-bloquear Persona",
  );
  const buttonText = $derived(
    isUnblocking ? "Confirmar Desbloqueo" : "Confirmar Re-bloqueo",
  );
  const Icon = $derived(isUnblocking ? ShieldCheck : Ban);
  const colorClass = $derived(isUnblocking ? "green" : "red");

  async function handleSubmit() {
    if (!motivoDesbloqueo.trim()) return;

    loading = true;

    try {
      await onUnblock({
        id: bloqueado.id,
        motivoDesbloqueo: motivoDesbloqueo.trim(),
        observaciones: observaciones.trim() || undefined,
      });
      onClose();
    } catch (error) {
      console.error("Error al procesar:", error);
    } finally {
      loading = false;
    }
  }

  function handleClose() {
    if (!loading) {
      onClose();
    }
  }

  const isFormValid = $derived(motivoDesbloqueo.trim().length > 0);
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4"
  transition:fade={{ duration: 200 }}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="absolute inset-0 bg-black/60 backdrop-blur-sm"
    role="button"
    tabindex="0"
    onclick={handleClose}
    onkeydown={(e) => e.key === "Escape" && handleClose()}
  ></div>
  <div
    class="relative z-10 w-full max-w-md rounded-xl bg-[#1e1e1e] shadow-2xl ring-1 ring-white/10"
    transition:fly={{ y: 20, duration: 300 }}
  >
    <!-- Header -->
    <div class="border-b border-white/10 px-6 py-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 items-center justify-center rounded-lg {isUnblocking
              ? 'bg-green-500/10'
              : 'bg-red-500/10'}"
          >
            <Icon
              size={24}
              class={isUnblocking ? "text-green-500" : "text-red-500"}
            />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-white">
              {title}
            </h3>
            <p class="text-sm text-gray-400">
              {bloqueado.nombreCompleto}
            </p>
          </div>
        </div>
        <button
          onclick={handleClose}
          disabled={loading}
          class="text-gray-400 hover:text-white transition-colors disabled:opacity-50"
          title="Cerrar"
        >
          <X size={20} />
        </button>
      </div>
    </div>

    <!-- Content -->
    <form
      onsubmit={(e) => {
        e.preventDefault();
        handleSubmit();
      }}
      class="p-6 space-y-4"
    >
      <!-- Info del bloqueado -->
      <div
        class="rounded-lg border p-3 {isUnblocking
          ? 'border-green-500/20 bg-green-500/5'
          : 'border-red-500/20 bg-red-500/5'}"
      >
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
            class="flex items-start gap-2 pt-2 mt-2 border-t {isUnblocking
              ? 'border-green-500/20'
              : 'border-red-500/20'}"
          >
            <span class="text-gray-400">Motivo del bloqueo:</span>
            <span class="text-white flex-1">{bloqueado.motivoBloqueo}</span>
          </div>
          {#if bloqueado.observaciones}
            <div class="flex items-start gap-2 pt-1">
              <span class="text-gray-400">Observaciones:</span>
              <span class="text-gray-300 flex-1 italic">
                {bloqueado.observaciones}
              </span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Motivo -->
      <div class="space-y-1.5">
        <label for="motivo" class="text-sm font-medium text-gray-300">
          {isUnblocking ? "Motivo del Desbloqueo" : "Motivo del Re-bloqueo"}
          <span class="text-red-500">*</span>
        </label>
        <textarea
          id="motivo"
          bind:value={motivoDesbloqueo}
          rows="3"
          disabled={loading}
          placeholder={isUnblocking
            ? "Ej: Cumplió sanción, revisión de caso, error administrativo..."
            : "Ej: Reincidencia, nueva infracción, decisión administrativa..."}
          class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-1 resize-y disabled:opacity-50 {isUnblocking
            ? 'focus:border-green-500 focus:ring-green-500'
            : 'focus:border-red-500 focus:ring-red-500'}"
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
          class="w-full rounded-lg border border-white/10 bg-[#2d2d2d] px-3 py-2.5 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-1 resize-y disabled:opacity-50 {isUnblocking
            ? 'focus:border-green-500 focus:ring-green-500'
            : 'focus:border-red-500 focus:ring-red-500'}"
        ></textarea>
      </div>

      <!-- Buttons -->
      <div class="flex justify-end gap-3 pt-2">
        <button
          type="button"
          onclick={handleClose}
          disabled={loading}
          class="rounded-lg px-4 py-2 text-sm font-medium text-gray-400 hover:text-white transition-colors disabled:opacity-50"
        >
          Cancelar
        </button>
        <button
          type="submit"
          disabled={loading || !isFormValid}
          class="rounded-lg px-4 py-2 text-sm font-medium text-white transition-all disabled:opacity-50 disabled:cursor-not-allowed {isUnblocking
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
          {:else}
            {buttonText}
          {/if}
        </button>
      </div>
    </form>
  </div>
</div>
