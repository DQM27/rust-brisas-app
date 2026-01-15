<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import { AlertTriangle, X } from "lucide-svelte";

  interface Props {
    show?: boolean;
    title?: string;
    message?: string;
    confirmText?: string;
    cancelText?: string;
    type?: "danger" | "warning" | "info";
    loading?: boolean;
    onConfirm: () => void;
    onClose: () => void;
  }

  let {
    show = false,
    title = "Confirmar Acción",
    message = "¿Estás seguro de realizar esta acción?",
    confirmText = "Confirmar",
    cancelText = "Cancelar",
    type = "warning",
    loading = false,
    onConfirm,
    onClose,
  }: Props = $props();

  // Colores según tipo (Svelte 5 derived)
  const headerColor = $derived(
    type === "danger"
      ? "text-red-400"
      : type === "warning"
        ? "text-orange-400"
        : "text-blue-400",
  );

  const iconColor = $derived(
    type === "danger"
      ? "text-red-500"
      : type === "warning"
        ? "text-orange-500"
        : "text-blue-500",
  );

  const btnColor = $derived(
    type === "danger"
      ? "bg-red-600 hover:bg-red-700"
      : type === "warning"
        ? "bg-orange-600 hover:bg-orange-700"
        : "bg-blue-600 hover:bg-blue-700",
  );

  function handleClose() {
    if (!loading) {
      onClose();
    }
  }
</script>

{#if show}
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/80 backdrop-blur-sm"
    transition:fade={{ duration: 200 }}
    onclick={handleClose}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === "Escape" && handleClose()}
    aria-label="Cerrar modal"
  >
    <div
      class="w-full max-w-md overflow-hidden rounded-xl border border-white/10 bg-[#1e1e1e] shadow-2xl text-left cursor-auto"
      transition:scale={{ duration: 200, start: 0.95 }}
      onclick={(e) => e.stopPropagation()}
      role="alertdialog"
      aria-modal="true"
      aria-labelledby="modal-title"
      aria-describedby="modal-desc"
      tabindex="-1"
      onkeydown={(e) => e.stopPropagation()}
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
          onclick={handleClose}
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
          onclick={handleClose}
          type="button"
          class="rounded-md border border-white/10 bg-transparent px-4 py-2 text-sm font-medium text-gray-300 transition-colors hover:bg-white/5 hover:text-white"
          disabled={loading}
        >
          {cancelText}
        </button>
        <button
          onclick={onConfirm}
          type="button"
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
