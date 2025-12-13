<script lang="ts">
  import { onMount, createEventDispatcher } from "svelte";
  import { toast } from "svelte-5-french-toast";
  import { citaService } from "$lib/services/citaService";
  import { currentUser } from "$lib/stores/auth";
  import type { CreateCitaInput, CreateVisitanteInput } from "$lib/types/cita";
  import {
    Calendar,
    Clock,
    User,
    Building,
    MapPin,
    FileText,
    Save,
  } from "lucide-svelte";

  const dispatch = createEventDispatcher();

  // Estado del formulario
  let fecha = "";
  let hora = "";
  let anfitrion = "";
  let areaVisitada = "";
  let motivo = "";

  // Datos del visitante
  let cedula = "";
  let nombre = "";
  let apellido = "";
  let segundoNombre = "";
  let segundoApellido = "";
  let empresa = "";
  let hasVehicle = false;

  let loading = false;

  onMount(() => {
    // Default fecha hoy
    const now = new Date();
    fecha = now.toISOString().split("T")[0];
    // Default hora siguiente
    now.setHours(now.getHours() + 1);
    now.setMinutes(0);
    hora = now.toTimeString().split(" ")[0].substring(0, 5);
  });

  async function handleSubmit() {
    if (!$currentUser) return;
    if (
      !cedula ||
      !nombre ||
      !apellido ||
      !anfitrion ||
      !areaVisitada ||
      !fecha ||
      !hora
    ) {
      toast.error("Por favor complete los campos obligatorios");
      return;
    }

    loading = true;
    try {
      // Combinar fecha y hora
      const fechaCita = `${fecha}T${hora}:00`;

      const visitante: CreateVisitanteInput = {
        cedula,
        nombre,
        apellido,
        segundo_nombre: segundoNombre || undefined,
        segundo_apellido: segundoApellido || undefined,
        empresa: empresa || undefined,
        has_vehicle: hasVehicle,
      };

      const cita: CreateCitaInput = {
        visitante_id: "", // Se resolverá en backend o si ya existiera podríamos buscarlo antes
        fecha_cita: fechaCita,
        anfitrion,
        area_visitada: areaVisitada,
        motivo,
        registrado_por: $currentUser.id,
      };

      // El servicio maneja la creación/búsqueda del visitante
      await citaService.createCita(cita, visitante);

      toast.success("Cita agendada correctamente");
      dispatch("success");
      resetForm();
    } catch (e: any) {
      console.error(e);
      toast.error(e.message || "Error al agendar cita");
    } finally {
      loading = false;
    }
  }

  function resetForm() {
    cedula = "";
    nombre = "";
    apellido = "";
    segundoNombre = "";
    segundoApellido = "";
    empresa = "";
    anfitrion = "";
    areaVisitada = "";
    motivo = "";
  }
</script>

<div class="h-full flex flex-col gap-6 p-6">
  <div
    class="flex items-center gap-3 border-b border-gray-200 dark:border-white/10 pb-4"
  >
    <div class="p-2 bg-blue-500/10 rounded-lg">
      <Calendar class="w-6 h-6 text-blue-600 dark:text-blue-400" />
    </div>
    <div>
      <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">
        Agendar Nueva Visita
      </h2>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Pre-registro de visitantes para acceso rápido
      </p>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 overflow-y-auto">
    <!-- Columna Izquierda: Datos de la Cita -->
    <div class="space-y-6">
      <h3
        class="text-sm font-semibold text-gray-400 uppercase tracking-wider flex items-center gap-2"
      >
        <Clock size={16} /> Detalles de la Cita
      </h3>

      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Fecha *</label
          >
          <input type="date" bind:value={fecha} class="input-base" />
        </div>
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Hora Aprox. *</label
          >
          <input type="time" bind:value={hora} class="input-base" />
        </div>
      </div>

      <div class="space-y-2">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Anfitrión (A quien visita) *</label
        >
        <div class="relative">
          <User class="absolute left-3 top-2.5 text-gray-400" size={18} />
          <input
            type="text"
            bind:value={anfitrion}
            placeholder="Nombre del empleado"
            class="input-base pl-10"
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Área *</label
          >
          <div class="relative">
            <MapPin class="absolute left-3 top-2.5 text-gray-400" size={18} />
            <input
              type="text"
              bind:value={areaVisitada}
              placeholder="Ej. Ventas, Gerencia"
              class="input-base pl-10"
            />
          </div>
        </div>
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Motivo</label
          >
          <input
            type="text"
            bind:value={motivo}
            placeholder="Reunión, Entrevista..."
            class="input-base"
          />
        </div>
      </div>
    </div>

    <!-- Columna Derecha: Datos del Visitante -->
    <div class="space-y-6">
      <h3
        class="text-sm font-semibold text-gray-400 uppercase tracking-wider flex items-center gap-2"
      >
        <User size={16} /> Datos del Visitante
      </h3>

      <div class="space-y-2">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Cédula / ID *</label
        >
        <input
          type="text"
          bind:value={cedula}
          placeholder="Identificación única"
          class="input-base"
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Nombre *</label
          >
          <input type="text" bind:value={nombre} class="input-base" />
        </div>
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
            >Apellido *</label
          >
          <input type="text" bind:value={apellido} class="input-base" />
        </div>
      </div>

      <!-- Opcionales colapsados visualmente o en grid -->
      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-500 dark:text-gray-400"
            >Segundo Nombre</label
          >
          <input type="text" bind:value={segundoNombre} class="input-base" />
        </div>
        <div class="space-y-2">
          <label class="text-sm font-medium text-gray-500 dark:text-gray-400"
            >Segundo Apellido</label
          >
          <input type="text" bind:value={segundoApellido} class="input-base" />
        </div>
      </div>

      <div class="space-y-2">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300"
          >Empresa / Procedencia</label
        >
        <div class="relative">
          <Building class="absolute left-3 top-2.5 text-gray-400" size={18} />
          <input
            type="text"
            bind:value={empresa}
            placeholder="Empresa del visitante"
            class="input-base pl-10"
          />
        </div>
      </div>
    </div>
  </div>

  <div
    class="pt-4 border-t border-gray-200 dark:border-white/10 flex justify-end"
  >
    <button
      on:click={handleSubmit}
      disabled={loading}
      class="btn-primary flex items-center gap-2 px-6 py-2.5"
    >
      {#if loading}
        <span class="animate-spin">⌛</span> Guardando...
      {:else}
        <Save size={18} /> Agendar Visita
      {/if}
    </button>
  </div>
</div>

<style>
  .input-base {
    @apply w-full px-3 py-2 bg-white dark:bg-[#252526] border border-gray-300 dark:border-white/10 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 dark:text-gray-100 transition-all duration-200;
  }
  .btn-primary {
    @apply bg-blue-600 hover:bg-blue-700 text-white rounded-md font-medium shadow-sm transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed;
  }
</style>
