<script lang="ts">
  import {
    X,
    Calendar,
    Clock,
    User,
    Building,
    MapPin,
    FileText,
  } from "lucide-svelte";
  import type { CitaPopulated } from "$lib/types/cita";
  import type { IngresoVisita } from "$lib/types/ingreso-nuevos";

  export let data: CitaPopulated | IngresoVisita | any;
  export let onClose: () => void;

  // Detect type and normalize fields
  const isCita = "fecha_cita" in data;

  const nombre = isCita
    ? data.visitante_nombre_completo
    : `${data.visitanteNombre || ""} ${data.visitanteApellido || ""}`;

  const cedula = isCita ? data.visitante_cedula : data.visitanteCedula;
  const empresa = isCita ? data.visitante_empresa : data.visitanteEmpresa;
  const anfitrion = data.anfitrion;
  const area = isCita ? data.area_visitada : data.areaVisitada;
  const motivo = data.motivo || "-";

  const fecha = isCita
    ? new Date(data.fecha_cita).toLocaleDateString("es-CR", {
        weekday: "long",
        day: "numeric",
        month: "long",
        year: "numeric",
      })
    : new Date(data.fechaIngreso).toLocaleDateString("es-CR", {
        weekday: "long",
        day: "numeric",
        month: "long",
        year: "numeric",
      });

  const hora = isCita
    ? new Date(data.fecha_cita).toLocaleTimeString("es-CR", {
        hour: "2-digit",
        minute: "2-digit",
      })
    : new Date(data.fechaIngreso).toLocaleTimeString("es-CR", {
        hour: "2-digit",
        minute: "2-digit",
      });

  const gafete = isCita ? null : data.gafete;
  const horaSalida =
    !isCita && data.fechaSalida
      ? new Date(data.fechaSalida).toLocaleTimeString("es-CR", {
          hour: "2-digit",
          minute: "2-digit",
        })
      : null;
</script>

<!-- Modal backdrop -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
>
  <div
    class="bg-[#161b22] border border-[#30363d] rounded-lg shadow-xl w-full max-w-lg mx-4"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-5 py-4 border-b border-[#30363d]"
    >
      <h3 class="text-lg font-semibold text-[#f0f6fc]">Detalles de Visita</h3>
      <button
        onclick={onClose}
        class="p-1 text-[#8d96a0] hover:text-[#f0f6fc] hover:bg-[#21262d] rounded transition-colors"
      >
        <X size={18} />
      </button>
    </div>

    <!-- Content -->
    <div class="p-5 space-y-4">
      <!-- Visitante info -->
      <div class="bg-[#0d1117] rounded-lg p-4">
        <h4
          class="text-xs font-semibold text-[#8d96a0] uppercase tracking-wider mb-3"
        >
          Visitante
        </h4>
        <div class="space-y-2">
          <div class="flex items-center gap-3">
            <User size={16} class="text-[#8d96a0]" />
            <span class="text-[#f0f6fc] font-medium text-lg">{nombre}</span>
          </div>
          <div class="flex items-center gap-3">
            <span class="text-[#8d96a0] text-sm w-16">Cédula</span>
            <span class="text-[#f0f6fc] font-mono">{cedula}</span>
          </div>
          {#if empresa}
            <div class="flex items-center gap-3">
              <Building size={16} class="text-[#8d96a0]" />
              <span class="text-[#f0f6fc]">{empresa}</span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Detalles de visita -->
      <div class="bg-[#0d1117] rounded-lg p-4">
        <h4
          class="text-xs font-semibold text-[#8d96a0] uppercase tracking-wider mb-3"
        >
          Detalles
        </h4>
        <div class="grid grid-cols-2 gap-3">
          <div class="flex items-center gap-2">
            <Calendar size={14} class="text-[#8d96a0]" />
            <span class="text-[#f0f6fc] text-sm">{fecha}</span>
          </div>
          <div class="flex items-center gap-2">
            <Clock size={14} class="text-[#8d96a0]" />
            <span class="text-[#f0f6fc] text-sm">{hora}</span>
          </div>
          <div>
            <span class="text-[#8d96a0] text-xs">Anfitrión</span>
            <p class="text-[#f0f6fc] text-sm">{anfitrion}</p>
          </div>
          <div>
            <span class="text-[#8d96a0] text-xs">Área</span>
            <p class="text-[#f0f6fc] text-sm">{area}</p>
          </div>
        </div>
        {#if motivo !== "-"}
          <div class="mt-3 pt-3 border-t border-[#21262d]">
            <span class="text-[#8d96a0] text-xs">Motivo</span>
            <p class="text-[#f0f6fc] text-sm mt-1">{motivo}</p>
          </div>
        {/if}
      </div>

      <!-- Estado (si es ingreso) -->
      {#if !isCita}
        <div class="bg-[#0d1117] rounded-lg p-4">
          <h4
            class="text-xs font-semibold text-[#8d96a0] uppercase tracking-wider mb-3"
          >
            Ingreso
          </h4>
          <div class="grid grid-cols-2 gap-3">
            {#if gafete}
              <div>
                <span class="text-[#8d96a0] text-xs">Gafete</span>
                <p class="text-blue-400 font-mono font-bold">{gafete}</p>
              </div>
            {/if}
            {#if horaSalida}
              <div>
                <span class="text-[#8d96a0] text-xs">Hora Salida</span>
                <p class="text-[#f0f6fc] text-sm">{horaSalida}</p>
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex justify-end px-5 py-4 border-t border-[#30363d]">
      <button
        onclick={onClose}
        class="py-2 px-6 border border-[#30363d] rounded-md text-sm font-medium text-[#c9d1d9] bg-[#21262d] hover:bg-[#30363d] transition-colors"
      >
        Cerrar
      </button>
    </div>
  </div>
</div>
