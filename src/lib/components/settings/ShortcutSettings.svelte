<script lang="ts">
  import {
    shortcutStore,
    SHORTCUT_DEFS,
    type ShortcutAction,
  } from "$lib/stores/shortcutStore";
  import { fade } from "svelte/transition";
  import { Keyboard, RotateCcw } from "lucide-svelte";
  import { createEventDispatcher } from "svelte";

  // Agrupar por categoría
  const groupedShortcuts = Object.values(SHORTCUT_DEFS).reduce(
    (acc, def) => {
      if (!acc[def.category]) acc[def.category] = [];
      acc[def.category].push(def);
      return acc;
    },
    {} as Record<string, (typeof SHORTCUT_DEFS)[ShortcutAction][]>,
  );

  let recordingId: string | null = null;

  function handleKeyRecord(e: KeyboardEvent, actionId: ShortcutAction) {
    e.preventDefault();
    e.stopPropagation();

    // Ignorar teclas modificadoras solas
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

    const parts = [];
    if (e.ctrlKey) parts.push("Control");
    if (e.metaKey) parts.push("Meta");
    if (e.altKey) parts.push("Alt");
    if (e.shiftKey) parts.push("Shift");

    // Normalizar teclas especiales
    let key = e.key;
    if (key === " ") key = "Space";
    if (key.length === 1) key = key.toLowerCase();

    parts.push(key);
    const newCombo = parts.join("+");

    shortcutStore.updateShortcut(actionId, newCombo);
    recordingId = null;
  }

  function startRecording(id: string) {
    recordingId = id;
  }

  function cancelRecording() {
    recordingId = null;
  }
  function focusOnMount(node: HTMLElement) {
    node.focus();
  }
</script>

<div class="space-y-6 p-6 bg-[#1e1e1e] rounded-lg text-gray-200">
  <div class="flex items-center justify-between border-b border-white/10 pb-4">
    <div>
      <h2 class="text-xl font-semibold flex items-center gap-2">
        <Keyboard size={24} class="text-blue-400" />
        Atajos de Teclado
      </h2>
      <p class="text-sm text-gray-400 mt-1">
        Personaliza las combinaciones de teclas para cada acción.
      </p>
    </div>
  </div>

  <div class="grid gap-8">
    {#each Object.entries(groupedShortcuts) as [category, defs]}
      <div>
        <h3
          class="text-sm font-medium text-blue-400 uppercase tracking-wider mb-3 px-2"
        >
          {category}
        </h3>
        <div class="space-y-2">
          {#each defs as def (def.id)}
            <div
              class="flex items-center justify-between p-3 rounded-lg bg-[#252526] hover:bg-[#2d2d2d] transition-colors border border-white/5"
            >
              <div class="flex flex-col">
                <span class="font-medium text-gray-200">{def.label}</span>
                <span class="text-xs text-gray-500">ID: {def.id}</span>
              </div>

              <div class="flex items-center gap-2">
                {#if recordingId === def.id}
                  <!-- Modo Grabación -->
                  <div class="relative">
                    <input
                      type="text"
                      class="bg-blue-900/30 border border-blue-500 text-blue-300 px-4 py-1.5 rounded-md text-sm font-mono text-center min-w-[120px] focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                      value="Presiona teclas..."
                      readonly
                      on:keydown={(e) => handleKeyRecord(e, def.id)}
                      on:blur={cancelRecording}
                      use:focusOnMount
                    />
                    <span
                      class="absolute -top-8 left-1/2 -translate-x-1/2 text-xs bg-black/80 text-white px-2 py-1 rounded whitespace-nowrap"
                    >
                      Esc para cancelar
                    </span>
                  </div>
                {:else}
                  <!-- Modo Visualización -->
                  <button
                    class="bg-[#333] hover:bg-[#404040] border border-white/10 text-gray-300 px-4 py-1.5 rounded-md text-sm font-mono min-w-[120px] transition-all hover:border-white/20 active:scale-95 shadow-sm"
                    on:click={() => startRecording(def.id)}
                    title="Clic para cambiar"
                  >
                    {$shortcutStore[def.id] || def.defaultKey}
                  </button>
                {/if}

                {#if $shortcutStore[def.id] !== def.defaultKey}
                  <button
                    class="p-1.5 text-gray-500 hover:text-yellow-400 hover:bg-white/5 rounded-full transition-colors"
                    title="Restaurar valor original"
                    on:click={() =>
                      shortcutStore.updateShortcut(def.id, def.defaultKey)}
                  >
                    <RotateCcw size={14} />
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>
