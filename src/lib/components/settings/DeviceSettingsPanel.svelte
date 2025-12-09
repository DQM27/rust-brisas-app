<script lang="ts">
  import { scale } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Monitor, MapPin, Cpu, Save, Fingerprint, Copy } from "lucide-svelte";
  import { message } from "@tauri-apps/plugin-dialog";

  let terminalName = "";
  let terminalId = "";
  let terminalLocation = "";
  let loading = false;

  onMount(async () => {
    await loadConfig();
  });

  async function loadConfig() {
    try {
      const config: any = await invoke("get_app_config");
      terminalName = config.terminal.nombre;
      terminalId = config.terminal.id;
      terminalLocation = config.terminal.ubicacion || "";
    } catch (err) {
      console.error("Error loading config:", err);
    }
  }

  async function saveConfig() {
    if (!terminalName || !terminalLocation) {
      await message("El nombre y la ubicación son obligatorios", {
        title: "Validación",
        kind: "warning",
      });
      return;
    }

    loading = true;
    try {
      await invoke("update_terminal_config", {
        nombre: terminalName,
        ubicacion: terminalLocation,
      });
      await message("Configuración guardada correctamente.", {
        title: "Éxito",
        kind: "info",
      });
      await loadConfig(); // Recargar para confirmar
    } catch (err) {
      console.error("Error saving config:", err);
      await message(`Error al guardar: ${err}`, {
        title: "Error",
        kind: "error",
      });
    } finally {
      loading = false;
    }
  }

  function copyId() {
    navigator.clipboard.writeText(terminalId);
    // Opcional: Toast simple
  }
</script>

<div
  class="flex h-full flex-col bg-surface-1 p-6 overflow-y-auto"
  in:scale={{ duration: 300, start: 0.95 }}
>
  <div class="mb-6">
    <h2 class="text-2xl font-bold text-primary">Ajustes Generales</h2>
    <p class="text-secondary mt-1">
      Configura la identidad de este dispositivo.
    </p>
  </div>

  <div class="grid gap-6 max-w-2xl">
    <!-- ID CARD -->
    <div class="card-base p-5 bg-surface-2/50 border border-white/5">
      <div class="flex items-start gap-4">
        <div class="p-3 rounded-lg bg-purple-500/10 text-purple-400">
          <Fingerprint size={24} />
        </div>
        <div class="flex-1">
          <h3 class="text-lg font-semibold text-primary mb-1">
            ID del Dispositivo
          </h3>
          <p class="text-xs text-secondary mb-3">
            Identificador único de hardware. Necesario para licencias y soporte.
          </p>

          <div class="flex items-center gap-2">
            <code
              class="bg-black/30 px-3 py-2 rounded text-sm font-mono text-purple-300 w-full border border-white/10 select-all"
            >
              {terminalId || "Cargando..."}
            </code>
            <button
              class="p-2 hover:bg-white/10 rounded transition-colors text-secondary hover:text-white"
              onclick={copyId}
              title="Copiar ID"
            >
              <Copy size={16} />
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- CONFIG FORM -->
    <div class="card-base p-6 space-y-6">
      <h3 class="text-lg font-semibold text-primary flex items-center gap-2">
        <Monitor size={20} class="text-blue-400" />
        Identidad de la Terminal
      </h3>

      <!-- Nombre -->
      <div class="space-y-2">
        <label class="text-sm font-medium text-secondary"
          >Nombre de la Terminal</label
        >
        <div class="relative">
          <input
            type="text"
            bind:value={terminalName}
            placeholder="Ej: Portería Principal"
            class="w-full bg-surface-1 border border-white/10 rounded-lg px-4 py-2.5 pl-10 text-primary focus:outline-none focus:ring-2 focus:ring-blue-500/50 transition-all placeholder:text-white/20"
          />
          <Monitor
            size={18}
            class="absolute left-3 top-1/2 -translate-y-1/2 text-secondary"
          />
        </div>
        <p class="text-xs text-secondary/60">
          Nombre visible en reportes y logs.
        </p>
      </div>

      <!-- Ubicación -->
      <div class="space-y-2">
        <label class="text-sm font-medium text-secondary"
          >Ubicación Física</label
        >
        <div class="relative">
          <input
            type="text"
            bind:value={terminalLocation}
            placeholder="Ej: Edificio A - Planta Baja"
            class="w-full bg-surface-1 border border-white/10 rounded-lg px-4 py-2.5 pl-10 text-primary focus:outline-none focus:ring-2 focus:ring-blue-500/50 transition-all placeholder:text-white/20"
          />
          <MapPin
            size={18}
            class="absolute left-3 top-1/2 -translate-y-1/2 text-secondary"
          />
        </div>
        <p class="text-xs text-secondary/60">
          Ayuda a identificar dónde está instalado el equipo.
        </p>
      </div>

      <div class="pt-4 flex justify-end">
        <button
          class="btn-primary flex items-center gap-2 px-6"
          onclick={saveConfig}
          disabled={loading}
        >
          {#if loading}
            <div
              class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
            ></div>
          {:else}
            <Save size={18} />
          {/if}
          <span>Guardar Cambios</span>
        </button>
      </div>
    </div>
  </div>
</div>
