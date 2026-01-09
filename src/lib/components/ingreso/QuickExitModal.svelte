<script lang="ts">
  import { onMount } from "svelte";
  import { fade, scale } from "svelte/transition";
  import { Search, X, User, Hash, CreditCard, XCircle } from "lucide-svelte";

  interface Props {
    show: boolean;
    activeEntries: any[];
    onSelect: (entry: any) => void;
    onClose: () => void;
  }

  let {
    show = $bindable(false),
    activeEntries = [],
    onSelect,
    onClose,
  }: Props = $props();

  let query = $state("");
  let inputRef = $state<HTMLInputElement>();
  let highlightedIndex = $state(0);

  // Filtrado local rápido
  let filteredEntries = $derived.by(() => {
    const q = query.toLowerCase().trim();
    if (!q) return [];

    return activeEntries
      .filter((e) => {
        const nombre = String(e.nombreCompleto || "").toLowerCase();
        const cedula = String(e.cedula || "").toLowerCase();
        const gafete = String(e.gafeteNumero || "").toLowerCase();
        return nombre.includes(q) || cedula.includes(q) || gafete.includes(q);
      })
      .slice(0, 8); // Limitar resultados para agilidad
  });

  $effect(() => {
    if (show) {
      query = "";
      highlightedIndex = 0;
      setTimeout(() => inputRef?.focus(), 50);
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      highlightedIndex = Math.min(
        highlightedIndex + 1,
        filteredEntries.length - 1,
      );
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      highlightedIndex = Math.max(highlightedIndex - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (filteredEntries[highlightedIndex]) {
        onSelect(filteredEntries[highlightedIndex]);
      }
    }
  }

  function handleSelect(entry: any) {
    onSelect(entry);
  }
</script>

{#if show}
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100] flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
    onclick={(e) => e.target === e.currentTarget && onClose()}
    role="button"
    tabindex="-1"
    onkeydown={handleKeyDown}
  >
    <div
      class="bg-[#1e1e1e] border border-white/10 w-full max-w-md rounded-lg shadow-2xl overflow-hidden flex flex-col outline-none shadow-blue-500/5"
      transition:scale={{ duration: 200, start: 0.95 }}
    >
      <!-- Search Input Section -->
      <div class="px-4 py-3 border-b border-white/5">
        <div
          class="search-container relative flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all outline-none"
        >
          <Search class="absolute left-3 text-gray-500" size={18} />
          <input
            bind:this={inputRef}
            bind:value={query}
            type="text"
            placeholder="Buscar por Gafete, Cédula o Nombre..."
            class="w-full bg-transparent pl-10 pr-4 py-2.5 text-[15px] text-white focus:outline-none outline-none border-none placeholder:text-gray-600 appearance-none ring-0"
            autocomplete="off"
            onkeydown={handleKeyDown}
          />
        </div>
      </div>

      <!-- Results Section -->
      <div class="max-h-[60vh] overflow-y-auto p-2">
        {#if query.length < 2}
          <!-- Empty by design until user types at least 2 chars -->
        {:else if filteredEntries.length === 0}
          <div class="p-8 text-center text-gray-500">
            <XCircle size={40} class="mx-auto mb-3 opacity-20 text-error" />
            <p>
              No se encontraron ingresos activos que coincidan con "{query}"
            </p>
          </div>
        {:else}
          <div class="space-y-1">
            {#each filteredEntries as entry, i}
              <button
                onclick={() => handleSelect(entry)}
                onmouseenter={() => (highlightedIndex = i)}
                class="w-full text-left p-2 rounded-lg flex items-center gap-3 transition-all
                  {i === highlightedIndex
                  ? 'bg-blue-600 text-white'
                  : 'hover:bg-white/5 text-gray-300'}"
              >
                <!-- Pequeño indicador/icono minimalista -->
                <div class="flex-shrink-0 text-gray-500">
                  <Search
                    size={14}
                    class={i === highlightedIndex
                      ? "text-white"
                      : "text-gray-500"}
                  />
                </div>

                <!-- Info -->
                <div class="flex-1 min-w-0">
                  <div class="font-semibold text-[15px] truncate">
                    {entry.nombreCompleto}
                  </div>
                  <div
                    class="flex items-center gap-3 text-xs opacity-70 mt-0.5"
                  >
                    <span class="flex items-center gap-1 font-mono">
                      <CreditCard size={12} />
                      {entry.cedula || "N/A"}
                    </span>
                    <span class="truncate">| {entry.empresaNombre}</span>
                  </div>
                </div>

                <!-- Gafete Badge -->
                {#if entry.gafeteNumero && entry.gafeteNumero !== "S/G"}
                  <div
                    class="px-3 py-1 rounded bg-black/30 font-mono text-sm border border-white/10"
                  >
                    G: {entry.gafeteNumero}
                  </div>
                {/if}

                <!-- Key Indicator -->
                {#if i === highlightedIndex}
                  <div
                    class="text-[10px] font-bold opacity-50 px-2 py-0.5 border border-white/20 rounded"
                  >
                    ENTER
                  </div>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Footer Info -->
      <div
        class="px-4 py-2 bg-black/20 border-t border-white/5 flex justify-between items-center"
      >
        <div
          class="text-[10px] text-gray-500 font-medium uppercase tracking-wider"
        >
          {filteredEntries.length} resultados encontrados
        </div>
        <div class="flex items-center gap-3">
          <span class="text-[10px] text-gray-600 flex items-center gap-1">
            <kbd
              class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[9px]"
              >ESC</kbd
            > Cancelar
          </span>
          <span class="text-[10px] text-gray-600 flex items-center gap-1">
            <kbd
              class="px-1.5 py-0.5 bg-black/40 rounded border border-white/10 text-[9px]"
              >↑↓</kbd
            > Navegar
          </span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  /* Prevenir scroll del body cuando el modal está abierto */
  :global(body:has(.z-\[100\])) {
    overflow: hidden;
  }

  /* Asegurar que nada tenga outline cuadrado del navegador */
  .search-container,
  .search-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  /* Re-aplicar el ring redondeado solo vía focus-within */
  .search-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
