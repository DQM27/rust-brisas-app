<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import { AlertTriangle, X } from "lucide-svelte";
  import { createEventDispatcher } from "svelte";

  export let show = false;
  export let title = "Confirmar Acción";
  export let message = "¿Estás seguro de realizar esta acción?";
  export let confirmText = "Confirmar";
  export let cancelText = "Cancelar";
  export let type: "danger" | "warning" | "info" = "warning";
  export let loading = false;

  const dispatch = createEventDispatcher();

  function handleConfirm() {
    dispatch("confirm");
  }

  function handleClose() {
    if (!loading) {
      dispatch("close");
    }
  }

  // Colores según tipo
  $: headerColor =
    type === "danger"
      ? "text-red-400"
      : type === "warning"
        ? "text-orange-400"
        : "text-blue-400";
  $: iconColor =
    type === "danger"
      ? "text-red-500"
      : type === "warning"
        ? "text-orange-500"
        : "text-blue-500";
  $: btnColor =
    type === "danger"
      ? "bg-red-600 hover:bg-red-700"
      : type === "warning"
        ? "bg-orange-600 hover:bg-orange-700"
        : "bg-blue-600 hover:bg-blue-700";
</script>

{#if show}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/80 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
    on:click={handleClose}
    role="button"
    tabindex="0"
    on:keydown={(e) => e.key === "Escape" && handleClose()}
  >
    <div
      class="w-full max-w-md overflow-hidden rounded-xl border border-white/10 bg-[#1e1e1e] shadow-2xl text-left cursor-auto"
      transition:scale={{ duration: 200, start: 0.95 }}
      on:click|stopPropagation
      role="alertdialog"
      aria-modal="true"
      aria-labelledby="modal-title"
      aria-describedby="modal-desc"
      tabindex="-1"
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between border-b border-white/5 bg-white/5 px-6 py-4"
      >
        <h3
          id="modal-title"
          class="text-lg font-semibold {headerColor} flex items-center gap-2"
        >
          <AlertTriangle size={20} class={iconColor} />
          {title}
        </h3>
        <button
          on:click={handleClose}
          type="button"
          class="text-gray-400 hover:text-white transition-colors"
          disabled={loading}
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Body -->
      <div class="px-6 py-6">
        <p id="modal-desc" class="text-gray-300 leading-relaxed text-sm">
          {message}
        </p>
      </div>

      <!-- Footer -->
      <div
        class="flex justify-end gap-3 border-t border-white/5 bg-[#252526] px-6 py-4"
      >
        <button
          on:click={handleClose}
          class="rounded-md border border-white/10 bg-transparent px-4 py-2 text-sm font-medium text-gray-300 transition-colors hover:bg-white/5 hover:text-white"
          disabled={loading}
        >
          {cancelText}
        </button>
        <button
          on:click={handleConfirm}
          class="flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium text-white shadow-lg transition-all hover:scale-105 active:scale-95 {btnColor} disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={loading}
        >
          {#if loading}
            <div
              class="h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"
            ></div>
          {/if}
          {confirmText}
        </button>
      </div>
    </div>
  </div>
{/if}
