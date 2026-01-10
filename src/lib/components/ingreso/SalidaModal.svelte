<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fade, scale, slide } from "svelte/transition";
  import {
    X,
    LogOut,
    CheckCircle,
    XCircle,
    ChevronDown,
    ChevronRight,
  } from "lucide-svelte";

  // Props
  interface Props {
    show: boolean;
    ingreso: any;
    loading?: boolean;
  }

  let {
    show = $bindable(false),
    ingreso = null,
    loading = false,
  }: Props = $props();

  // State
  let devolvioGafete = $state<boolean | null>(null);
  let observaciones = $state("");
  let showObservaciones = $state(false);
  let confirmButtonRef = $state<HTMLButtonElement>();

  const dispatch = createEventDispatcher();

  // ==========================================
  // HANDLERS
  // ==========================================
  function handleConfirm() {
    if (devolvioGafete === null) {
      return; // No permitir confirmar sin seleccionar
    }
    dispatch("confirm", {
      devolvioGafete,
      observaciones: observaciones.trim() || null,
    });
  }

  function handleClose() {
    if (loading) return;
    show = false;
    reset();
    dispatch("close");
  }

  function reset() {
    devolvioGafete = null;
    observaciones = "";
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      handleClose();
    }
    // Ctrl+S o Cmd+S para confirmar
    if ((e.ctrlKey || e.metaKey) && e.key === "s") {
      e.preventDefault();
      handleConfirm();
    }
  }

  // Reset cuando cambia el ingreso y auto-seleccionar devolvioGafete si no tiene gafete
  $effect(() => {
    if (ingreso && show) {
      reset();
      // Si no tiene gafete, auto-seleccionar true
      const tieneGafete =
        ingreso.gafeteNumero && ingreso.gafeteNumero !== "S/G";
      if (!tieneGafete) {
        devolvioGafete = true;
        // Enfocar el botón de confirmar tras un pequeño delay para que el DOM esté listo
        setTimeout(() => {
          if (confirmButtonRef) confirmButtonRef.focus();
        }, 100);
      }
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show && ingreso}
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
    onclick={(e) => {
      if (e.target === e.currentTarget) handleClose();
    }}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") {
        if (e.target === e.currentTarget) handleClose();
      }
    }}
    role="button"
    tabindex="-1"
  >
    <!-- Modal -->
    <div
      class="bg-surface-2 rounded-lg shadow-surface-xl border border-surface max-w-md w-full flex flex-col"
      transition:scale={{ duration: 200, start: 0.95 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between px-6 py-4 border-b border-surface"
      >
        <div class="flex items-center gap-3">
          <div class="p-2 bg-error bg-opacity-10 rounded-full">
            <LogOut size={20} class="text-error" />
          </div>
          <div>
            <h2 class="text-lg font-semibold text-primary">Registrar Salida</h2>
            <p class="text-sm text-secondary">
              Confirme la salida del contratista
            </p>
          </div>
        </div>
        <button
          onclick={handleClose}
          class="p-2 hover:bg-surface-hover rounded-md transition-colors"
          disabled={loading}
        >
          <X size={20} class="text-secondary" />
        </button>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-5">
        <!-- Info de la persona -->
        <div class="p-4 bg-surface-1 rounded-lg border border-surface">
          <div class="space-y-2">
            <div class="flex items-center">
              <span
                class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                >Nombre</span
              >
              <span class="text-primary font-semibold text-sm">
                {ingreso.nombreCompleto || "Sin nombre"}
              </span>
            </div>
            <div class="flex items-center">
              <span
                class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                >Cédula</span
              >
              <span class="text-primary font-mono text-sm">
                {ingreso.cedula || "N/A"}
              </span>
            </div>
            <div class="flex items-center">
              <span
                class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                >Empresa</span
              >
              <span class="text-primary text-sm">
                {ingreso.empresaNombre || "Sin empresa"}
              </span>
            </div>
            {#if ingreso.gafeteNumero && ingreso.gafeteNumero !== "S/G"}
              <div class="flex items-center">
                <span
                  class="text-[12px] font-bold uppercase tracking-wider text-gray-500 w-20 shrink-0"
                  >Gafete</span
                >
                <span class="text-accent font-mono text-sm">
                  {ingreso.gafeteNumero}
                </span>
              </div>
            {/if}
          </div>
        </div>

        <!-- Pregunta del gafete -->
        {#if ingreso.gafeteNumero && ingreso.gafeteNumero !== "S/G"}
          <div class="space-y-3">
            <span class="block text-sm font-medium text-primary">
              ¿El contratista devolvió el gafete?
            </span>
            <div class="grid grid-cols-2 gap-3">
              <button
                type="button"
                onclick={() => (devolvioGafete = true)}
                class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all {devolvioGafete ===
                true
                  ? 'border-success bg-success bg-opacity-10 text-success'
                  : 'border-surface hover:border-success/50 text-secondary hover:text-success'}"
              >
                <CheckCircle size={20} />
                <span class="font-medium">Sí, lo devolvió</span>
              </button>
              <button
                type="button"
                onclick={() => (devolvioGafete = false)}
                class="flex items-center justify-center gap-2 p-3 rounded-lg border-2 transition-all {devolvioGafete ===
                false
                  ? 'border-error bg-error bg-opacity-10 text-error'
                  : 'border-surface hover:border-error/50 text-secondary hover:text-error'}"
              >
                <XCircle size={20} />
                <span class="font-medium">No lo devolvió</span>
              </button>
            </div>
            {#if devolvioGafete === false}
              <div
                class="p-3 bg-warning bg-opacity-10 border border-warning rounded-lg text-warning text-sm"
                transition:fade
              >
                ⚠️ Se registrará una alerta por gafete no devuelto
              </div>
            {/if}
          </div>
        {:else}
          <!-- Si no tiene gafete, ya se auto-seleccionó true en el $effect -->
          <div
            class="p-3 bg-surface-1 rounded-lg border border-surface text-sm text-secondary"
          >
            Este contratista no tiene gafete asignado
          </div>
        {/if}

        <!-- Observaciones Collapsible -->
        <div class="space-y-2">
          <button
            type="button"
            onclick={() => (showObservaciones = !showObservaciones)}
            class="flex items-center gap-1.5 text-secondary hover:text-primary transition-colors text-sm"
          >
            {#if showObservaciones}
              <ChevronDown size={14} />
            {:else}
              <ChevronRight size={14} />
            {/if}
            <span>Observaciones</span>
            {#if !showObservaciones && observaciones.trim()}
              <span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
            {/if}
          </button>

          {#if showObservaciones}
            <div transition:slide={{ duration: 250 }}>
              <div
                class="obs-container w-full bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all outline-none"
              >
                <textarea
                  id="observaciones"
                  bind:value={observaciones}
                  rows={2}
                  class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none outline-none border-none appearance-none ring-0"
                  placeholder="Notas adicionales sobre la salida..."
                ></textarea>
              </div>
            </div>
          {/if}
        </div>
      </div>

      <!-- Footer -->
      <div
        class="flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
      >
        <button
          onclick={handleClose}
          disabled={loading}
          class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-white/60 hover:text-white/80 focus:outline-none disabled:opacity-50"
        >
          Cancelar
        </button>
        <button
          bind:this={confirmButtonRef}
          onclick={handleConfirm}
          disabled={loading || devolvioGafete === null}
          class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border-2 transition-all duration-200 border-surface text-secondary hover:border-success hover:text-success focus:outline-none disabled:opacity-50"
          title="Atajo: Ctrl + S"
        >
          {#if loading}
            <span class="inline-block animate-spin">⏳</span>
          {/if}
          <LogOut size={18} />
          Confirmar Salida
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Observaciones container - mismo estilo que GafeteInput */
  .obs-container,
  .obs-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  .obs-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
