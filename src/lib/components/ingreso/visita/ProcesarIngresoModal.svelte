<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { CitaPopulated } from "$lib/types/cita";
  import { X, UserCheck } from "lucide-svelte";

  export let cita: CitaPopulated;
  export let onCancel: () => void;
  export let onConfirm: (event: CustomEvent<{ gafete: string }>) => void;

  let gafete = "";
  let loading = false;

  const dispatch = createEventDispatcher<{ confirm: { gafete: string } }>();

  function handleSubmit() {
    if (!gafete.trim()) return;
    loading = true;
    // Simular dispatch como CustomEvent
    onConfirm(
      new CustomEvent("confirm", { detail: { gafete: gafete.trim() } }),
    );
  }
  function focusOnMount(node: HTMLElement) {
    node.focus();
  }
</script>

<!-- Modal backdrop -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
>
  <div
    class="bg-[#161b22] border border-[#30363d] rounded-lg shadow-xl w-full max-w-md mx-4"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-5 py-4 border-b border-[#30363d]"
    >
      <div class="flex items-center gap-3">
        <div class="p-2 bg-green-500/10 rounded-full">
          <UserCheck size={20} class="text-green-500" />
        </div>
        <h3 class="text-lg font-semibold text-[#f0f6fc]">Procesar Ingreso</h3>
      </div>
      <button
        onclick={onCancel}
        class="p-1 text-[#8d96a0] hover:text-[#f0f6fc] hover:bg-[#21262d] rounded transition-colors"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Content -->
    <div class="p-5 space-y-4">
      <!-- Info del visitante -->
      <div class="bg-[#0d1117] rounded-lg p-4 space-y-2">
        <div class="flex justify-between">
          <span class="text-[#8d96a0] text-sm">Visitante</span>
          <span class="text-[#f0f6fc] font-medium"
            >{cita.visitante_nombre_completo}</span
          >
        </div>
        <div class="flex justify-between">
          <span class="text-[#8d96a0] text-sm">Cédula</span>
          <span class="text-[#f0f6fc] font-mono">{cita.visitante_cedula}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-[#8d96a0] text-sm">Hora cita</span>
          <span class="text-blue-400 font-medium">
            {new Date(cita.fecha_cita).toLocaleTimeString("es-CR", {
              hour: "2-digit",
              minute: "2-digit",
            })}
          </span>
        </div>
        <div class="flex justify-between">
          <span class="text-[#8d96a0] text-sm">Visita a</span>
          <span class="text-[#f0f6fc]">{cita.anfitrion}</span>
        </div>
      </div>

      <!-- Campo gafete -->
      <div class="space-y-2">
        <label
          for="gafete-input"
          class="block text-sm font-medium text-[#c9d1d9]"
        >
          Número de Gafete <span class="text-[#f85149]">*</span>
        </label>
        <input
          id="gafete-input"
          type="text"
          bind:value={gafete}
          class="w-full px-4 py-3 bg-[#0d1117] border border-[#30363d] rounded-lg text-2xl font-mono text-center text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
          placeholder="000"
          use:focusOnMount
        />
      </div>
    </div>

    <!-- Footer -->
    <div class="flex gap-3 px-5 py-4 border-t border-[#30363d]">
      <button
        onclick={onCancel}
        class="flex-1 py-2 px-4 border border-[#30363d] rounded-md text-sm font-medium text-[#c9d1d9] bg-[#21262d] hover:bg-[#30363d] transition-colors"
      >
        Cancelar
      </button>
      <button
        onclick={handleSubmit}
        disabled={!gafete.trim() || loading}
        class="flex-[2] py-2 px-4 rounded-md text-sm font-medium text-white bg-[#238636] hover:bg-[#2ea043] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {loading ? "Procesando..." : "Confirmar Ingreso"}
      </button>
    </div>
  </div>
</div>
