<script lang="ts">
  import { onMount } from "svelte";
  import { shortcutService } from "$lib/services/shortcutService";
  import { invoke } from "@tauri-apps/api/core";
  import type { ShortcutConfig } from "$lib/types/shortcuts";
  import { Loader2, Save, Undo2 } from "lucide-svelte";
  import { toast } from "svelte-5-french-toast";

  let config = $state<ShortcutConfig | null>(null);
  let loading = $state(true);
  let recordingKeyFor: { context: string; command: string } | null =
    $state(null);

  onMount(async () => {
    await loadConfig();
  });

  async function loadConfig() {
    loading = true;
    try {
      config = await invoke("get_shortcuts");
    } catch (e) {
      console.error(e);
      toast.error("Error al cargar atajos");
    } finally {
      loading = false;
    }
  }

  async function saveConfig() {
    if (!config) return;
    try {
      await invoke("update_shortcuts", { config });
      toast.success("Atajos guardados correctamente");
      // Recargar servicio para aplicar cambios
      window.location.reload(); // O idealmente método en service para recargar
    } catch (e) {
      console.error(e);
      toast.error("Error al guardar configuración");
    }
  }

  async function resetConfig() {
    if (!confirm("¿Restaurar atajos por defecto?")) return;
    try {
      config = await invoke("reset_shortcuts");
      toast.success("Valores por defecto restaurados");
      window.location.reload();
    } catch (e) {
      toast.error("Error al restaurar");
    }
  }

  function handleRecordKey(e: KeyboardEvent) {
    if (!recordingKeyFor || !config) return;

    e.preventDefault();
    e.stopPropagation();

    if (e.key === "Escape") {
      recordingKeyFor = null;
      return;
    }

    // Ignorar modificadores solos
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;

    // Construir string
    const parts = [];
    if (e.ctrlKey) parts.push("Ctrl");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");

    let k = e.key;
    if (k === " ") k = "Space";
    if (k.length === 1) k = k.toUpperCase();

    parts.push(k);
    const keyString = parts.join("+");

    // Guardar
    const { context, command } = recordingKeyFor;

    if (context === "global") {
      config.global[command] = keyString;
    } else {
      if (config.contexts[context]) {
        config.contexts[context][command] = keyString;
      }
    }

    recordingKeyFor = null;
  }
</script>

<svelte:window on:keydown={recordingKeyFor ? handleRecordKey : undefined} />

<div class="space-y-6">
  {#if loading}
    <div class="flex items-center justify-center py-10">
      <Loader2 class="animate-spin text-blue-500" size={32} />
    </div>
  {:else if config}
    <!-- HEADER -->
    <div class="flex items-center justify-between mb-6">
      <div class="text-sm text-gray-400">
        Haz clic en un atajo para editarlo. Pulsa ESC para cancelar la
        grabación.
      </div>
      <div class="flex gap-2">
        <button
          class="btn btn-secondary text-sm flex items-center gap-2"
          on:click={resetConfig}
        >
          <Undo2 size={16} />
          Restaurar Defaults
        </button>
        <button
          class="btn btn-primary text-sm flex items-center gap-2"
          on:click={saveConfig}
        >
          <Save size={16} />
          Guardar Cambios
        </button>
      </div>
    </div>

    <!-- GLOBAL COMMANDS -->
    <div class="card-base p-4">
      <h3
        class="text-lg font-bold text-primary mb-4 border-b border-white/10 pb-2"
      >
        Globales
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        {#each Object.entries(config.global) as [cmd, key]}
          <div
            class="flex items-center justify-between bg-black/20 p-3 rounded-lg border border-white/5"
          >
            <span class="text-gray-300 font-medium">{cmd}</span>
            <button
              class="px-3 py-1.5 rounded bg-[#333] border border-white/10 text-white font-mono text-sm min-w-[80px] text-center hover:bg-blue-600/50 transition-colors"
              class:ring-2={recordingKeyFor?.context === "global" &&
                recordingKeyFor?.command === cmd}
              class:ring-blue-500={recordingKeyFor?.context === "global" &&
                recordingKeyFor?.command === cmd}
              on:click={() =>
                (recordingKeyFor = { context: "global", command: cmd })}
            >
              {recordingKeyFor?.context === "global" &&
              recordingKeyFor?.command === cmd
                ? "Grabando..."
                : key}
            </button>
          </div>
        {/each}
      </div>
    </div>

    <!-- CONTEXTUAL COMMANDS -->
    <div class="card-base p-4">
      <h3
        class="text-lg font-bold text-primary mb-4 border-b border-white/10 pb-2"
      >
        Contextuales
      </h3>

      {#each Object.entries(config.contexts) as [ctxName, commands]}
        <div class="mb-6 last:mb-0">
          <h4
            class="text-sm font-semibold text-blue-400 uppercase tracking-wider mb-2"
          >
            {ctxName}
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each Object.entries(commands) as [cmd, key]}
              <div
                class="flex items-center justify-between bg-black/20 p-3 rounded-lg border border-white/5"
              >
                <span class="text-gray-300 font-medium">{cmd}</span>
                <button
                  class="px-3 py-1.5 rounded bg-[#333] border border-white/10 text-white font-mono text-sm min-w-[80px] text-center hover:bg-blue-600/50 transition-colors"
                  class:ring-2={recordingKeyFor?.context === ctxName &&
                    recordingKeyFor?.command === cmd}
                  class:ring-blue-500={recordingKeyFor?.context === ctxName &&
                    recordingKeyFor?.command === cmd}
                  on:click={() =>
                    (recordingKeyFor = { context: ctxName, command: cmd })}
                >
                  {recordingKeyFor?.context === ctxName &&
                  recordingKeyFor?.command === cmd
                    ? "Grabando..."
                    : key}
                </button>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
