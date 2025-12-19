<script lang="ts">
  import { onMount } from "svelte";
  import { citaService } from "$lib/services/citaService";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import { X, Calendar, Clock } from "lucide-svelte";
  import type {
    CreateCitaInput,
    CreateVisitanteInput,
    CitaPopulated,
  } from "$lib/types/cita";

  export let onClose: () => void;
  export let onSuccess: () => void;
  export let editingCita: CitaPopulated | null = null;

  // Modo edición
  $: isEditing = !!editingCita;

  // Datos del visitante
  let cedula = "";
  let nombre = "";
  let apellido = "";
  let empresa = "";

  // Datos de la cita
  let fecha = "";
  let hora = "";
  let anfitrion = "";
  let areaVisitada = "";
  let motivo = "";

  let loading = false;

  // Validación simple
  $: isValid =
    cedula && nombre && apellido && anfitrion && areaVisitada && fecha && hora;

  onMount(() => {
    if (editingCita) {
      // Pre-poblar con datos existentes
      cedula = editingCita.visitante_cedula || "";
      nombre = editingCita.visitante_nombre || "";
      apellido = editingCita.visitante_apellido || "";
      empresa = editingCita.visitante_empresa || "";
      fecha = editingCita.fecha_cita.slice(0, 10);
      hora = editingCita.fecha_cita.slice(11, 16);
      anfitrion = editingCita.anfitrion;
      areaVisitada = editingCita.area_visitada;
      motivo = editingCita.motivo || "";
    } else {
      // Default: mañana a las 9:00
      const tomorrow = new Date();
      tomorrow.setDate(tomorrow.getDate() + 1);
      fecha = tomorrow.toISOString().split("T")[0];
      hora = "09:00";
    }
  });

  async function handleSubmit() {
    if (!isValid || !$currentUser) return;
    loading = true;

    try {
      const fechaCita = `${fecha}T${hora}:00`;

      if (isEditing && editingCita) {
        // Actualizar cita existente
        await citaService.actualizarCita(editingCita.id, {
          fecha_cita: fechaCita,
          anfitrion,
          area_visitada: areaVisitada,
          motivo,
        });
        toast.success("Cita actualizada correctamente");
      } else {
        // Crear nueva cita
        const visitante: CreateVisitanteInput = {
          cedula,
          nombre,
          apellido,
          empresa: empresa || undefined,
          has_vehicle: false,
        };

        const cita: CreateCitaInput = {
          visitante_id: "",
          fecha_cita: fechaCita,
          anfitrion,
          area_visitada: areaVisitada,
          motivo,
          registrado_por: $currentUser.id,
        };

        await citaService.createCita(cita, visitante);
        toast.success("Visita pre-registrada correctamente");
      }

      resetForm();
      onSuccess();
    } catch (error: any) {
      toast.error(
        error.message ||
          (isEditing ? "Error al actualizar" : "Error al pre-registrar"),
      );
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    cedula = "";
    nombre = "";
    apellido = "";
    empresa = "";
    anfitrion = "";
    areaVisitada = "";
    motivo = "";
  }

  function handleCloseForm() {
    resetForm();
    onClose();
  }
</script>

<div
  class="bg-[#0d1117] px-4 py-3 h-full flex flex-col border-r border-[#30363d] w-[340px] shrink-0"
>
  <!-- Header -->
  <div class="flex justify-between items-center mb-3 shrink-0">
    <h2 class="text-sm font-semibold text-[#f0f6fc]">
      {isEditing ? "Editar Visita" : "Pre-registro de Visita"}
    </h2>
    <button
      on:click={handleCloseForm}
      class="text-[#8d96a0] hover:text-[#f0f6fc] p-0.5 rounded hover:bg-[#21262d] transition-colors"
      type="button"
      aria-label="Cerrar"
    >
      <X size={16} />
    </button>
  </div>

  <!-- Form -->
  <form
    on:submit|preventDefault={handleSubmit}
    class="flex-1 flex flex-col min-h-0 overflow-y-auto"
  >
    <div class="space-y-2 flex-1">
      <!-- Cédula (full width) - deshabilitado en edición -->
      <div class="flex flex-col gap-0.5">
        <label for="v-cedula" class="text-[11px] font-medium text-[#c9d1d9]"
          >Cédula <span class="text-[#f85149]">*</span></label
        >
        <input
          id="v-cedula"
          type="text"
          bind:value={cedula}
          disabled={isEditing}
          class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none disabled:opacity-60 disabled:cursor-not-allowed"
          placeholder="1-1234-5678"
        />
      </div>

      <!-- Nombre | Apellido - deshabilitados en edición -->
      <div class="grid grid-cols-2 gap-2">
        <div class="flex flex-col gap-0.5">
          <label for="v-nombre" class="text-[11px] font-medium text-[#c9d1d9]"
            >Nombre <span class="text-[#f85149]">*</span></label
          >
          <input
            id="v-nombre"
            type="text"
            bind:value={nombre}
            disabled={isEditing}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none disabled:opacity-60 disabled:cursor-not-allowed"
            placeholder="Nombre"
          />
        </div>
        <div class="flex flex-col gap-0.5">
          <label for="v-apellido" class="text-[11px] font-medium text-[#c9d1d9]"
            >Apellido <span class="text-[#f85149]">*</span></label
          >
          <input
            id="v-apellido"
            type="text"
            bind:value={apellido}
            disabled={isEditing}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none disabled:opacity-60 disabled:cursor-not-allowed"
            placeholder="Apellido"
          />
        </div>
      </div>

      <!-- Empresa (opcional) - deshabilitado en edición -->
      <div class="flex flex-col gap-0.5">
        <label for="v-empresa" class="text-[11px] font-medium text-[#8d96a0]"
          >Empresa</label
        >
        <input
          id="v-empresa"
          type="text"
          bind:value={empresa}
          disabled={isEditing}
          class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none disabled:opacity-60 disabled:cursor-not-allowed"
          placeholder="Particular"
        />
      </div>

      <!-- Separador -->
      <div class="border-t border-[#21262d] pt-2 mt-1">
        <p
          class="text-[10px] font-medium text-[#8d96a0] uppercase tracking-wider mb-2"
        >
          Detalles de la Visita
        </p>
      </div>

      <!-- Fecha | Hora -->
      <div class="grid grid-cols-2 gap-2">
        <div class="flex flex-col gap-0.5">
          <label
            for="v-fecha"
            class="text-[11px] font-medium text-[#c9d1d9] flex items-center gap-1"
          >
            <Calendar size={12} /> Fecha <span class="text-[#f85149]">*</span>
          </label>
          <input
            id="v-fecha"
            type="date"
            bind:value={fecha}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] focus:border-[#388bfd] outline-none calendar-dark"
          />
        </div>
        <div class="flex flex-col gap-0.5">
          <label
            for="v-hora"
            class="text-[11px] font-medium text-[#c9d1d9] flex items-center gap-1"
          >
            <Clock size={12} /> Hora <span class="text-[#f85149]">*</span>
          </label>
          <input
            id="v-hora"
            type="time"
            bind:value={hora}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] focus:border-[#388bfd] outline-none calendar-dark"
          />
        </div>
      </div>

      <!-- Anfitrión | Área -->
      <div class="grid grid-cols-2 gap-2">
        <div class="flex flex-col gap-0.5">
          <label
            for="v-anfitrion"
            class="text-[11px] font-medium text-[#c9d1d9]"
            >Anfitrión <span class="text-[#f85149]">*</span></label
          >
          <input
            id="v-anfitrion"
            type="text"
            bind:value={anfitrion}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
            placeholder="Recibe"
          />
        </div>
        <div class="flex flex-col gap-0.5">
          <label for="v-area" class="text-[11px] font-medium text-[#c9d1d9]"
            >Área <span class="text-[#f85149]">*</span></label
          >
          <input
            id="v-area"
            type="text"
            bind:value={areaVisitada}
            class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
            placeholder="Área"
          />
        </div>
      </div>

      <!-- Motivo -->
      <div class="flex flex-col gap-0.5">
        <label for="v-motivo" class="text-[11px] font-medium text-[#8d96a0]"
          >Motivo</label
        >
        <input
          id="v-motivo"
          type="text"
          bind:value={motivo}
          class="w-full px-2 py-1.5 bg-[#161b22] border border-[#30363d] rounded text-sm text-[#f0f6fc] placeholder-[#484f58] focus:border-[#388bfd] outline-none"
          placeholder="Motivo de visita (opcional)"
        />
      </div>
    </div>

    <!-- Footer -->
    <div class="pt-3 shrink-0 flex gap-2 mt-auto border-t border-[#21262d]">
      <button
        type="button"
        on:click={handleCloseForm}
        class="flex-1 py-1.5 px-3 border border-[#30363d] rounded text-xs font-medium text-[#c9d1d9] bg-[#21262d] hover:bg-[#30363d] transition-colors"
      >
        Cancelar
      </button>

      <button
        type="submit"
        disabled={loading || !isValid}
        class="flex-[2] flex justify-center items-center py-1.5 px-3 rounded text-xs font-medium text-white bg-[#238636] hover:bg-[#2ea043] disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {#if loading}
          <svg
            class="animate-spin mr-1.5 h-3 w-3 text-white"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          Guardando...
        {:else if isEditing}
          Guardar Cambios
        {:else}
          Agendar Visita
        {/if}
      </button>
    </div>
  </form>
</div>

<style>
  .calendar-dark::-webkit-calendar-picker-indicator {
    filter: invert(0.7);
    cursor: pointer;
  }
</style>
