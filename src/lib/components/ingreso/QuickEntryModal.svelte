<script lang="ts">
  import { onMount } from "svelte";
  import { fade, scale } from "svelte/transition";
  import {
    Search,
    X,
    User,
    Hash,
    CreditCard,
    XCircle,
    Building2,
  } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    show: boolean;
    onSelect: (person: any) => void;
    onClose: () => void;
    allowedTypes?: string[]; // New prop
  }

  let {
    show = $bindable(false),
    onSelect,
    onClose,
    allowedTypes = ["contratista"],
  }: Props = $props();

  let query = $state("");
  let inputRef = $state<HTMLInputElement>();
  let highlightedIndex = $state(0);
  let results = $state<any[]>([]);
  let loading = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout>;

  async function handleSearch() {
    const q = query.trim();
    if (q.length < 2) {
      results = [];
      return;
    }

    loading = true;
    try {
      const rawResults: any[] = await invoke("search_global", {
        query: q,
        limit: 8,
      });
      // Filtrar según tipos permitidos
      results = rawResults.filter((r) =>
        allowedTypes.includes(r.tipo.toLowerCase()),
      );
      highlightedIndex = 0;
    } catch (e) {
      console.error("Error searching globally:", e);
    } finally {
      loading = false;
    }
  }

  function handleInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(handleSearch, 300);
  }

  $effect(() => {
    if (show) {
      query = "";
      results = [];
      highlightedIndex = 0;
      setTimeout(() => inputRef?.focus(), 50);
    }
  });

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      highlightedIndex = Math.min(highlightedIndex + 1, results.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      highlightedIndex = Math.max(highlightedIndex - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (results[highlightedIndex]) {
        onSelect(results[highlightedIndex]);
      }
    }
  }

  function handleSelect(person: any) {
    onSelect(person);
  }
</script>

{#if show}
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100] flex items-center justify-center p-4 outline-none"
    transition:fade={{ duration: 150 }}
    onclick={(e) => e.target === e.currentTarget && onClose()}
    role="button"
    tabindex="-1"
    onkeydown={handleKeyDown}
  >
    <!-- Modal -->
    <div
      class="bg-[#1e1e1e] border border-white/10 w-full max-w-md rounded-lg shadow-2xl overflow-hidden flex flex-col outline-none"
      transition:scale={{ duration: 200, start: 0.95 }}
    >
      <!-- Search Input Section -->
      <div class="px-4 py-3 border-b border-white/5">
        <div
          class="search-container relative flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 transition-all outline-none"
        >
          <Search class="absolute left-3 text-gray-500" size={18} />
          <input
            bind:this={inputRef}
            bind:value={query}
            oninput={handleInput}
            type="text"
            placeholder="¿Quién va a ingresar? (Nombre o Cédula)"
            class="w-full bg-transparent pl-10 pr-4 py-2.5 text-[15px] text-white focus:outline-none outline-none border-none placeholder:text-gray-600 appearance-none ring-0"
            autocomplete="off"
            onkeydown={handleKeyDown}
          />
          {#if loading}
            <div class="absolute right-3 animate-spin text-gray-500 text-xs">
              ⏳
            </div>
          {/if}
        </div>
      </div>

      <!-- Results Section -->
      <div class="max-h-[60vh] overflow-y-auto p-2">
        {#if query.length < 2}
          <!-- Empty by design -->
        {:else if results.length === 0 && !loading}
          <div class="p-8 text-center text-gray-500">
            <XCircle size={32} class="mx-auto mb-3 opacity-20 text-error" />
            <p class="text-sm">No se encontró al contratista</p>
          </div>
        {:else}
          <div class="space-y-1">
            {#each results as person, i}
              <button
                onclick={() => handleSelect(person)}
                onmouseenter={() => (highlightedIndex = i)}
                class="w-full text-left p-2 rounded-lg flex items-center gap-3 transition-all
                  {i === highlightedIndex
                  ? 'bg-blue-600 text-white'
                  : 'hover:bg-white/5 text-gray-300'}"
              >
                <div class="flex-shrink-0 text-gray-500">
                  <User
                    size={14}
                    class={i === highlightedIndex
                      ? "text-white"
                      : "text-gray-500"}
                  />
                </div>

                <!-- Info -->
                <div class="flex-1 min-w-0">
                  <div class="font-semibold text-[14px] truncate">
                    {person.nombreCompleto}
                  </div>
                  <div
                    class="flex items-center gap-3 text-[11px] opacity-70 mt-0.5"
                  >
                    <span class="flex items-center gap-1 font-mono">
                      <CreditCard size={10} />
                      {person.cedula || "N/A"}
                    </span>
                    <span class="truncate"
                      >| {person.empresaNombre || "S/E"}</span
                    >
                  </div>
                </div>

                {#if i === highlightedIndex}
                  <div
                    class="text-[9px] font-bold opacity-50 px-1.5 py-0.5 border border-white/20 rounded"
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
          class="text-[9px] text-gray-500 font-medium uppercase tracking-wider"
        >
          {results.length} resultados
        </div>
        <div class="flex items-center gap-3">
          <span class="text-[9px] text-gray-600 flex items-center gap-1">
            <kbd
              class="px-1 py-0.5 bg-black/40 rounded border border-white/10 text-[8px]"
              >ESC</kbd
            >
          </span>
          <span class="text-[9px] text-gray-600 flex items-center gap-1">
            <kbd
              class="px-1 py-0.5 bg-black/40 rounded border border-white/10 text-[8px]"
              >↑↓</kbd
            >
          </span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  :global(body:has(.z-\[100\])) {
    overflow: hidden;
  }
  .search-container,
  .search-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }
  .search-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
