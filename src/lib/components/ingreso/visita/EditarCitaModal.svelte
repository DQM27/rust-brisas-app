<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { CitaPopulated } from "$lib/types/cita";
  import { X, Calendar, Clock, Save } from "lucide-svelte";
  import { citaService } from "$lib/services/citaService";
  import { toast } from "svelte-5-french-toast";

  export let cita: CitaPopulated;
  export let onClose: () => void;
  export let onSave: () => void;

  // Form state - initialized from cita
  let fechaCita = cita.fecha_cita.slice(0, 10); // YYYY-MM-DD
  let horaCita = cita.fecha_cita.slice(11, 16); // HH:MM
  let anfitrion = cita.anfitrion;
  let areaVisitada = cita.area_visitada;
  let motivo = cita.motivo || "";

  let loading = false;

  async function handleSubmit() {
    if (!fechaCita || !horaCita || !anfitrion || !areaVisitada) {
      toast.error("Completa los campos requeridos");
      return;
    }

    loading = true;
    try {
      const fechaCompleta = `${fechaCita}T${horaCita}:00`;
      
      await citaService.actualizarCita(cita.id, {
        fecha_cita: fechaCompleta,
        anfitrion,
        area_visitada: areaVisitada,
        motivo,
      });
      
      toast.success("Cita actualizada correctamente");
      onSave();
      onClose();
    } catch (error: any) {
      toast.error(error.message || "Error al actualizar cita");
    } finally {
      loading = false;
    }
  }
</script>

<!-- Modal backdrop -->
<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
  <div class="bg-[#161b22] border border-[#30363d] rounded-lg shadow-xl w-full max-w-md mx-4">
    <!-- Header -->
    <div class="flex items-center justify-between px-5 py-4 border-b border-[#30363d]">
      <h3 class="text-lg font-semibold text-[#f0f6fc]">Editar Cita</h3>
      <button
        onclick={onClose}
        class="p-1 text-[#8d96a0] hover:text-[#f0f6fc] hover:bg-[#21262d] rounded transition-colors"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Content -->
    <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="p-5 space-y-4">
      <!-- Info del visitante (solo lectura) -->
      <div class="bg-[#0d1117] rounded-lg p-3 space-y-1">
        <p class="text-[#8d96a0] text-xs">Visitante</p>
        <p class="text-[#f0f6fc] font-medium">{cita.visitante_nombre_completo}</p>
        <p class="text-[#8d96a0] text-xs font-mono">{cita.visitante_cedula}</p>
      </div>

      <!-- Fecha | Hora -->
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1">
          <label for="edit-fecha" class="text-xs font-medium text-[#c9d1d9] flex items-center gap-1">
            <Calendar size={12} /> Fecha
          </label>
          <input
            id="edit-fecha"
            type="date"
            bind:value={fechaCita}
            class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded text-sm text-[#f0f6fc] focus:border-[#388bfd] outline-none"
          />
        </div>
        <div class="space-y-1">
          <label for="edit-hora" class="text-xs font-medium text-[#c9d1d9] flex items-center gap-1">
            <Clock size={12} /> Hora
          </label>
          <input
            id="edit-hora"
            type="time"
            bind:value={horaCita}
            class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded text-sm text-[#f0f6fc] focus:border-[#388bfd] outline-none"
          />
        </div>
      </div>

      <!-- Anfitrión | Área -->
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1">
          <label for="edit-anfitrion" class="text-xs font-medium text-[#c9d1d9]">Anfitrión *</label>
          <input
            id="edit-anfitrion"
            type="text"
            bind:value={anfitrion}
            class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
            placeholder="Quien recibe"
          />
        </div>
        <div class="space-y-1">
          <label for="edit-area" class="text-xs font-medium text-[#c9d1d9]">Área *</label>
          <input
            id="edit-area"
            type="text"
            bind:value={areaVisitada}
            class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
            placeholder="Área destino"
          />
        </div>
      </div>

      <!-- Motivo -->
      <div class="space-y-1">
        <label for="edit-motivo" class="text-xs font-medium text-[#8d96a0]">Motivo</label>
        <input
          id="edit-motivo"
          type="text"
          bind:value={motivo}
          class="w-full px-3 py-2 bg-[#0d1117] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
          placeholder="Motivo de visita (opcional)"
        />
      </div>

      <!-- Footer -->
      <div class="flex gap-3 pt-2">
        <button
          type="button"
          onclick={onClose}
          class="flex-1 py-2 px-4 border border-[#30363d] rounded-md text-sm font-medium text-[#c9d1d9] bg-[#21262d] hover:bg-[#30363d] transition-colors"
        >
          Cancelar
        </button>
        <button
          type="submit"
          disabled={loading}
          class="flex-[2] flex justify-center items-center gap-2 py-2 px-4 rounded-md text-sm font-medium text-white bg-[#238636] hover:bg-[#2ea043] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <Save size={14} />
          {loading ? "Guardando..." : "Guardar Cambios"}
        </button>
      </div>
    </form>
  </div>
</div>
