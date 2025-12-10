<script lang="ts">
  import { X, Download, Maximize2 } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";

  interface Props {
    pdfUrl: string;
    fileName?: string;
    onClose: () => void;
  }

  let { pdfUrl, fileName = "documento.pdf", onClose }: Props = $props();

  function handleDownload() {
    const link = document.createElement("a");
    link.href = pdfUrl;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }
</script>

<div
  class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm focus:outline-none"
  transition:fade={{ duration: 200 }}
  role="presentation"
  onclick={handleBackdropClick}
  onkeydown={(e) => e.key === "Escape" && onClose()}
  tabindex="-1"
>
  <div
    class="relative w-full max-w-5xl h-[85vh] bg-gray-900 rounded-lg shadow-2xl flex flex-col overflow-hidden focus:outline-none"
    transition:scale={{ duration: 200, start: 0.95 }}
    role="dialog"
    aria-modal="true"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    tabindex="-1"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-4 py-3 border-b border-gray-700 bg-gray-800"
    >
      <div class="flex items-center gap-3">
        <div class="p-2 bg-red-500/10 rounded-lg">
          <span class="text-red-500 font-bold text-xs uppercase tracking-wider"
            >PDF</span
          >
        </div>
        <div>
          <h3
            class="text-white font-medium text-sm md:text-base truncate max-w-[200px] md:max-w-md"
          >
            {fileName}
          </h3>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <button
          onclick={handleDownload}
          class="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
          title="Descargar"
        >
          <Download size={20} />
        </button>
        <button
          onclick={onClose}
          class="p-2 text-gray-400 hover:text-white hover:bg-red-500/20 hover:text-red-400 rounded-lg transition-colors"
          title="Cerrar"
        >
          <X size={20} />
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 bg-gray-900 relative">
      {#if pdfUrl}
        <iframe
          src={pdfUrl}
          title="Vista previa del PDF"
          class="w-full h-full border-none"
        ></iframe>
      {:else}
        <div
          class="absolute inset-0 flex items-center justify-center text-gray-500"
        >
          <p>No se pudo cargar el PDF</p>
        </div>
      {/if}
    </div>
  </div>
</div>
