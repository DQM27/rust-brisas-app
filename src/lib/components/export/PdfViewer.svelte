<!-- src/lib/components/export/PdfViewer.svelte -->
<!-- Visor PDF profesional con zoom y pan usando pdf.js -->
<script lang="ts">
  // @ts-nocheck - Svelte 5 runes
  import { onMount, onDestroy, tick } from "svelte";
  import * as pdfjsLib from "pdfjs-dist";

  // Configurar worker de PDF.js
  pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
    "pdfjs-dist/build/pdf.worker.min.mjs",
    import.meta.url,
  ).toString();

  interface Props {
    pdfData: Uint8Array | null;
    onError?: (error: string) => void;
  }

  let { pdfData, onError }: Props = $props();

  let containerRef: HTMLDivElement;
  let canvasRef: HTMLCanvasElement;
  let pdfDoc: any = null;
  let currentPage = $state(1);
  let totalPages = $state(0);
  let scale = $state(1.0);
  let isLoading = $state(false);
  let isRendered = $state(false);
  let renderScale = $state(1.0); // Scala actual del renderizado (para comparar con scale visual)
  let zoomTimeout: any = null;

  // Pan state
  let isPanning = $state(false);
  let isSpacePressed = $state(false);
  let panStart = { x: 0, y: 0 };
  let scrollStart = { x: 0, y: 0 };

  const MIN_SCALE = 0.25;
  const MAX_SCALE = 4.0;
  const SCALE_STEP = 0.15;

  async function loadPdf(data: Uint8Array) {
    if (!data || data.length === 0) return;

    isLoading = true;
    isRendered = false;
    try {
      const loadingTask = pdfjsLib.getDocument({ data });
      pdfDoc = await loadingTask.promise;
      totalPages = pdfDoc.numPages;
      currentPage = 1;

      // Esperar a que el DOM se actualice para que canvasRef esté disponible
      await tick();

      // Reintentar si canvas no está listo
      let attempts = 0;
      while (!canvasRef && attempts < 10) {
        await new Promise((r) => setTimeout(r, 50));
        attempts++;
      }

      if (canvasRef) {
        // Reset scales
        renderScale = scale;
        await renderPage(currentPage);
        isRendered = true;
      }
    } catch (error: any) {
      console.error("Error loading PDF:", error);
      onError?.("Error al cargar el PDF: " + error.message);
    } finally {
      isLoading = false;
    }
  }

  async function renderPage(pageNum: number) {
    if (!pdfDoc || !canvasRef) return;

    try {
      const page = await pdfDoc.getPage(pageNum);
      // Usar renderScale para el renderizado real del canvas
      const viewport = page.getViewport({ scale: renderScale });

      const canvas = canvasRef;
      const context = canvas.getContext("2d");

      canvas.height = viewport.height;
      canvas.width = viewport.width;

      const renderContext = {
        canvasContext: context,
        viewport: viewport,
      };

      await page.render(renderContext).promise;
    } catch (error: any) {
      console.error("Error rendering page:", error);
    }
  }

  // Zoom con mouse wheel
  async function handleWheel(event: WheelEvent) {
    // Ctrl + Scroll = Zoom
    if (event.ctrlKey) {
      event.preventDefault();
      event.stopPropagation();

      const delta = event.deltaY > 0 ? -SCALE_STEP : SCALE_STEP;
      const oldScale = scale;
      const newScale = Math.max(MIN_SCALE, Math.min(MAX_SCALE, scale + delta));

      if (newScale !== oldScale) {
        // Calcular posición relativa del mouse en el contenido
        const rect = containerRef.getBoundingClientRect();
        // Coordenadas del mouse relativas al viewport del contenedor
        const mouseX = event.clientX - rect.left;
        const mouseY = event.clientY - rect.top;

        // Coordenadas del mouse relativas al contenido escalado actual (incluyendo scroll)
        const contentX = mouseX + containerRef.scrollLeft;
        const contentY = mouseY + containerRef.scrollTop;

        // Ratio del punto bajo el mouse (0-1 relativo al tamaño total) no es necesario,
        // simplemente escalamos la posición del contenido.
        // Nueva posición del punto bajo el mouse después del escalado
        const newContentX = contentX * (newScale / oldScale);
        const newContentY = contentY * (newScale / oldScale);

        // Actualizar escala visual
        scale = newScale;

        // Ajustar scroll para mantener el punto bajo el mouse estable
        if (containerRef) {
          containerRef.scrollLeft = newContentX - mouseX;
          containerRef.scrollTop = newContentY - mouseY;
        }

        // Debounce el renderizado costoso
        if (zoomTimeout) clearTimeout(zoomTimeout);
        zoomTimeout = setTimeout(async () => {
          renderScale = newScale;
          await renderPage(currentPage);
        }, 150);
      }
      return;
    }

    // Alt + Scroll = Horizontal Scroll
    if (event.altKey) {
      event.preventDefault();
      event.stopPropagation();
      if (containerRef) {
        containerRef.scrollLeft += event.deltaY;
      }
      return;
    }

    // Shift + Scroll = Horizontal Scroll (Nativo en navegadores, pero aseguramos)
    if (event.shiftKey) {
      // Dejar comportamiento nativo o forzarlo si es necesario.
      // Generalmente el navegador maneja shift+wheel como horizontal scroll.
      return;
    }

    // Default: Vertical scroll (no preventDefault)
    // El navegador manejará el scroll vertical normalmente
  }

  // Keyboard events para spacebar pan y prevenir focus loss con Alt
  function handleKeyDown(event: KeyboardEvent) {
    // Spacebar pan logic
    if (event.code === "Space") {
      event.preventDefault();
      event.stopPropagation();
      if (!isSpacePressed) {
        isSpacePressed = true;
        if (containerRef) {
          containerRef.style.cursor = "grab";
        }
      }
      return;
    }

    // Prevenir que Alt enfoque el menú del navegador (Windows)
    if (event.key === "Alt") {
      event.preventDefault();
    }
  }

  function handleKeyUp(event: KeyboardEvent) {
    if (event.code === "Space") {
      event.preventDefault();
      event.stopPropagation();
      isSpacePressed = false;
      isPanning = false;
      if (containerRef) {
        containerRef.style.cursor = "default";
      }
    }
  }

  // Mouse events para pan
  function handleMouseDown(event: MouseEvent) {
    if (isSpacePressed) {
      isPanning = true;
      panStart = { x: event.clientX, y: event.clientY };
      scrollStart = {
        x: containerRef.scrollLeft,
        y: containerRef.scrollTop,
      };
      if (containerRef) {
        containerRef.style.cursor = "grabbing";
      }
    }
  }

  function handleMouseMove(event: MouseEvent) {
    if (isPanning && isSpacePressed) {
      const dx = event.clientX - panStart.x;
      const dy = event.clientY - panStart.y;
      containerRef.scrollLeft = scrollStart.x - dx;
      containerRef.scrollTop = scrollStart.y - dy;
    }
  }

  function handleMouseUp() {
    if (isPanning) {
      isPanning = false;
      if (containerRef && isSpacePressed) {
        containerRef.style.cursor = "grab";
      }
    }
  }

  // Controles de zoom
  function zoomIn() {
    scale = Math.min(MAX_SCALE, scale + SCALE_STEP);
    renderPage(currentPage);
  }

  function zoomOut() {
    scale = Math.max(MIN_SCALE, scale - SCALE_STEP);
    renderPage(currentPage);
  }

  function resetZoom() {
    scale = 1.0;
    renderPage(currentPage);
  }

  function fitToWidth() {
    if (!containerRef || !pdfDoc) return;
    // Aproximar al ancho del contenedor
    scale = (containerRef.clientWidth - 40) / 600; // 600 es un ancho típico de PDF
    renderPage(currentPage);
  }

  // Navegación de páginas
  function prevPage() {
    if (currentPage > 1) {
      currentPage--;
      renderPage(currentPage);
    }
  }

  function nextPage() {
    if (currentPage < totalPages) {
      currentPage++;
      renderPage(currentPage);
    }
  }

  // Reactivo a cambios en pdfData
  $effect(() => {
    if (pdfData) {
      loadPdf(pdfData);
    }
  });

  onMount(() => {
    // Focus para capturar keyboard events
    containerRef?.focus();
  });

  onDestroy(() => {
    if (pdfDoc) {
      pdfDoc.destroy();
    }
  });
</script>

<div class="pdf-viewer-container flex flex-col h-full overflow-hidden">
  <!-- Toolbar -->
  <div
    class="pdf-toolbar flex items-center justify-between px-3 py-2 bg-[#21262d] border-b border-[#30363d]"
  >
    <!-- Zoom controls -->
    <div class="flex items-center gap-2">
      <button
        onclick={zoomOut}
        class="p-1.5 rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#30363d] transition-colors"
        title="Alejar"
      >
        <svg
          class="w-4 h-4"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35M8 11h6" />
        </svg>
      </button>
      <span class="text-xs text-[#8b949e] min-w-[50px] text-center">
        {Math.round(scale * 100)}%
      </span>
      <button
        onclick={zoomIn}
        class="p-1.5 rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#30363d] transition-colors"
        title="Acercar"
      >
        <svg
          class="w-4 h-4"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="11" cy="11" r="8" />
          <path d="m21 21-4.35-4.35M11 8v6M8 11h6" />
        </svg>
      </button>
      <button
        onclick={resetZoom}
        class="px-2 py-1 text-xs rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#30363d] transition-colors"
        title="100%"
      >
        100%
      </button>
    </div>

    <!-- Page navigation -->
    {#if totalPages > 1}
      <div class="flex items-center gap-2">
        <button
          onclick={prevPage}
          disabled={currentPage <= 1}
          class="p-1.5 rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#30363d] transition-colors disabled:opacity-30"
        >
          ←
        </button>
        <span class="text-xs text-[#8b949e]">
          {currentPage} / {totalPages}
        </span>
        <button
          onclick={nextPage}
          disabled={currentPage >= totalPages}
          class="p-1.5 rounded text-[#8b949e] hover:text-[#e6edf3] hover:bg-[#30363d] transition-colors disabled:opacity-30"
        >
          →
        </button>
      </div>
    {/if}

    <!-- Hint -->
    <div class="text-xs text-[#8b949e] hidden md:block">
      Scroll: zoom · Espacio+drag: mover
    </div>
  </div>

  <!-- Canvas container wrapper para clipping -->
  <div class="flex-1 overflow-hidden relative">
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      bind:this={containerRef}
      class="pdf-canvas-container absolute inset-0 overflow-auto bg-[#1c2128] p-4"
      tabindex="-1"
      role="application"
      aria-label="Visor PDF"
      onwheel={handleWheel}
      onkeydown={handleKeyDown}
      onkeyup={handleKeyUp}
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      onmouseup={handleMouseUp}
      onmouseleave={handleMouseUp}
    >
      <!-- Wrapper para centrar con margin:auto (permite scroll en todas direcciones) -->
      <div class="min-w-fit min-h-fit w-fit h-fit mx-auto">
        <canvas
          bind:this={canvasRef}
          class="shadow-lg rounded block origin-top-left"
          class:invisible={!isRendered}
          style:transform={`scale(${scale / renderScale})`}
          style="max-width: none;"
        ></canvas>
      </div>

      <!-- Loading overlay -->
      {#if isLoading || (!isRendered && pdfData)}
        <div
          class="absolute inset-0 flex items-center justify-center bg-[#1c2128]"
        >
          <div
            class="w-8 h-8 border-2 border-[#30363d] border-t-[#2563eb] rounded-full animate-spin"
          ></div>
        </div>
      {/if}

      <!-- Sin documento -->
      {#if !pdfData && !isLoading}
        <div
          class="absolute inset-0 flex items-center justify-center text-center text-[#8b949e]"
        >
          <p class="text-sm">Sin documento</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .pdf-canvas-container:focus {
    outline: none;
  }
</style>
