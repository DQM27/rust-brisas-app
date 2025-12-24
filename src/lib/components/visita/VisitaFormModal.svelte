<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import {
    X,
    Save,
    Calendar,
    User,
    Clock,
    Building,
    MapPin,
    Car,
  } from "lucide-svelte";
  import type { CreateCitaInput, CreateVisitanteInput } from "$lib/types/cita";
  import { VisitaSchema, type VisitaForm } from "$lib/schemas/visitaSchema";
  import { toast } from "svelte-5-french-toast";
  import { fade, fly } from "svelte/transition";
  import { currentUser } from "$lib/stores/auth";

  // Props
  export let show = false;
  export let loading = false;
  // TODO: Soporte para editar citas existentes (por ahora solo CREATE)
  // export let visita: CitaPopulated | null = null;
  export let onClose: () => void;
  export let onSave: (data: {
    cita: CreateCitaInput;
    visitante: CreateVisitanteInput;
  }) => Promise<boolean>;

  // Estado del Formulario
  let formData: VisitaForm = initForm();
  let errors: Record<string, string> = {};

  function initForm(): VisitaForm {
    const now = new Date();
    // Default: fecha hoy, hora siguiente hora en punto
    now.setHours(now.getHours() + 1, 0, 0, 0);

    return {
      fecha: new Date().toISOString().split("T")[0],
      hora: now.toTimeString().substring(0, 5),
      anfitrion: "",
      areaVisitada: "",
      motivo: "",

      cedula: "",
      nombre: "",
      segundoNombre: "",
      apellido: "",
      segundoApellido: "",
      empresa: "",

      tieneVehiculo: false,
      placa: "",
    };
  }

  // Reset form when modal opens/closes
  $: if (show) {
    formData = initForm();
    errors = {};
  }

  // Validación
  function validate() {
    const result = VisitaSchema.safeParse(formData);
    if (!result.success) {
      const fieldErrors: Record<string, string> = {};
      result.error.issues.forEach((issue) => {
        fieldErrors[issue.path[0] as string] = issue.message;
      });
      errors = fieldErrors;
      return false;
    }
    errors = {};
    return true;
  }

  async function handleSubmit() {
    if (!validate()) {
      toast.error("Por favor corrija los errores en el formulario");
      return;
    }

    if (!$currentUser) return;

    // Transformar a Input Backend
    const fechaCita = `${formData.fecha}T${formData.hora}:00`;

    const visitanteInput: CreateVisitanteInput = {
      cedula: formData.cedula,
      nombre: formData.nombre,
      apellido: formData.apellido,
      segundo_nombre: formData.segundoNombre || undefined,
      segundo_apellido: formData.segundoApellido || undefined,
      empresa: formData.empresa || undefined,
      has_vehicle: formData.tieneVehiculo,
      // Si tiene vehiculo, enviar datos extra si el backend lo soporta en CreateVisitanteInput (actualmente solo bool)
      // La lógica de backend actual para 'create_cita' podría necesitar expansión si queremos guardar placa en ese momento
      // o el servicio 'create_cita' llama internamente a guardar vehiculo.
      // REVISIÓN: El endpoint create_cita recibe CreateVisitanteInput, que solo tiene has_vehicle.
      // Para guardar la placa, el backend necesitaría actualización o una llamada separada.
      // POR AHORA: Guardamos has_vehicle, la placa se pedirá en garita al ingreso.
    };

    const citaInput: CreateCitaInput = {
      visitante_id: "", // Se resuelve en backend
      fecha_cita: fechaCita,
      anfitrion: formData.anfitrion,
      area_visitada: formData.areaVisitada,
      motivo: formData.motivo || "",
      registrado_por: $currentUser.id,
    };

    const success = await onSave({
      cita: citaInput,
      visitante: visitanteInput,
    });
    if (success) {
      onClose();
    }
  }
  const inputClass =
    "w-full px-3 py-2 bg-white dark:bg-[#111] border border-gray-300 dark:border-white/10 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-1 block";
  const inputErrorClass = "border-red-500 focus:ring-red-500/20";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4"
    transition:fade
  >
    <div
      class="bg-white dark:bg-[#1e1e1e] rounded-xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden flex flex-col"
      transition:fly={{ y: 20 }}
    >
      <!-- Header -->
      <div
        class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-white/10"
      >
        <h2
          class="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2"
        >
          <Calendar class="text-blue-500" />
          Agendar Nueva Visita
        </h2>
        <button
          on:click={onClose}
          class="p-2 hover:bg-gray-100 dark:hover:bg-white/10 rounded-full transition-colors"
        >
          <X size={20} class="text-gray-500" />
        </button>
      </div>

      <!-- Body -->
      <div class="flex-1 overflow-y-auto p-6 space-y-8">
        <!-- Sección 1: Datos Cita -->
        <section class="space-y-4">
          <h3
            class="text-sm font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center gap-2 border-b border-gray-200 dark:border-white/5 pb-2"
          >
            <Clock size={16} /> Detalles de Cita
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div class="form-group">
              <label for="fecha" class={labelClass}>Fecha *</label>
              <input
                id="fecha"
                type="date"
                bind:value={formData.fecha}
                class="{inputClass} {errors.fecha ? inputErrorClass : ''}"
              />
              {#if errors.fecha}<span class={errorClass}>{errors.fecha}</span
                >{/if}
            </div>
            <div class="form-group">
              <label for="hora" class={labelClass}>Hora *</label>
              <input
                id="hora"
                type="time"
                bind:value={formData.hora}
                class="{inputClass} {errors.hora ? inputErrorClass : ''}"
              />
              {#if errors.hora}<span class={errorClass}>{errors.hora}</span
                >{/if}
            </div>
            <div class="form-group md:col-span-2 lg:col-span-2">
              <label for="anfitrion" class={labelClass}
                >Anfitrión (Empleado) *</label
              >
              <div class="relative">
                <User class="absolute left-3 top-2.5 text-gray-400" size={16} />
                <input
                  id="anfitrion"
                  type="text"
                  bind:value={formData.anfitrion}
                  class="{inputClass} pl-9 {errors.anfitrion
                    ? inputErrorClass
                    : ''}"
                  placeholder="A quien visita"
                />
              </div>
              {#if errors.anfitrion}<span class={errorClass}
                  >{errors.anfitrion}</span
                >{/if}
            </div>

            <div class="form-group md:col-span-2">
              <label for="areaVisitada" class={labelClass}
                >Área / Departamento *</label
              >
              <div class="relative">
                <MapPin
                  class="absolute left-3 top-2.5 text-gray-400"
                  size={16}
                />
                <input
                  id="areaVisitada"
                  type="text"
                  bind:value={formData.areaVisitada}
                  class="{inputClass} pl-9 {errors.areaVisitada
                    ? inputErrorClass
                    : ''}"
                  placeholder="Ej. Ventas"
                />
              </div>
              {#if errors.areaVisitada}<span class={errorClass}
                  >{errors.areaVisitada}</span
                >{/if}
            </div>
            <div class="form-group md:col-span-2">
              <label for="motivo" class={labelClass}>Motivo</label>
              <input
                id="motivo"
                type="text"
                bind:value={formData.motivo}
                class={inputClass}
                placeholder="Reunión..."
              />
            </div>
          </div>
        </section>

        <!-- Sección 2: Datos Visitante -->
        <section class="space-y-4">
          <h3
            class="text-sm font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider flex items-center gap-2 border-b border-gray-200 dark:border-white/5 pb-2"
          >
            <User size={16} /> Datos del Visitante
          </h3>

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div class="form-group">
              <label for="cedula" class={labelClass}>Cédula / Documento *</label
              >
              <input
                id="cedula"
                type="text"
                bind:value={formData.cedula}
                class="{inputClass} {errors.cedula ? inputErrorClass : ''}"
                placeholder="Identificación"
              />
              {#if errors.cedula}<span class={errorClass}>{errors.cedula}</span
                >{/if}
            </div>
            <div class="form-group">
              <label for="nombre" class={labelClass}>Nombre *</label>
              <input
                id="nombre"
                type="text"
                bind:value={formData.nombre}
                class="{inputClass} {errors.nombre ? inputErrorClass : ''}"
              />
              {#if errors.nombre}<span class={errorClass}>{errors.nombre}</span
                >{/if}
            </div>
            <div class="form-group">
              <label for="apellido" class={labelClass}>Apellido *</label>
              <input
                id="apellido"
                type="text"
                bind:value={formData.apellido}
                class="{inputClass} {errors.apellido ? inputErrorClass : ''}"
              />
              {#if errors.apellido}<span class={errorClass}
                  >{errors.apellido}</span
                >{/if}
            </div>

            <!-- Opcionales -->
            <div class="form-group">
              <label for="segundoNombre" class="{labelClass} text-gray-500"
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                type="text"
                bind:value={formData.segundoNombre}
                class="{inputClass} bg-gray-50 dark:bg-white/5"
              />
              {#if errors.segundoNombre}<span class={errorClass}
                  >{errors.segundoNombre}</span
                >{/if}
            </div>
            <div class="form-group">
              <label for="segundoApellido" class="{labelClass} text-gray-500"
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                type="text"
                bind:value={formData.segundoApellido}
                class="{inputClass} bg-gray-50 dark:bg-white/5"
              />
              {#if errors.segundoApellido}<span class={errorClass}
                  >{errors.segundoApellido}</span
                >{/if}
            </div>

            <div class="form-group">
              <label for="empresa" class={labelClass}
                >Empresa / Procedencia</label
              >
              <div class="relative">
                <Building
                  class="absolute left-3 top-2.5 text-gray-400"
                  size={16}
                />
                <input
                  id="empresa"
                  type="text"
                  bind:value={formData.empresa}
                  class="{inputClass} pl-9"
                  placeholder="Opcional"
                />
              </div>
            </div>
          </div>

          <!-- Vehículo Toggle -->
          <div
            class="p-4 bg-gray-50 dark:bg-white/5 rounded-lg border border-gray-100 dark:border-white/5 mt-4"
          >
            <label class="flex items-center gap-3 cursor-pointer mb-4">
              <input
                type="checkbox"
                bind:checked={formData.tieneVehiculo}
                class="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="font-medium flex items-center gap-2"
                ><Car size={18} /> Ingresa con Vehículo</span
              >
            </label>

            {#if formData.tieneVehiculo}
              <div
                class="grid grid-cols-1 md:grid-cols-2 gap-4 animate-in fade-in slide-in-from-top-2 duration-200"
              >
                <div class="form-group">
                  <label for="placa" class={labelClass}>Placa *</label>
                  <input
                    id="placa"
                    type="text"
                    bind:value={formData.placa}
                    class="{inputClass} uppercase font-mono {errors.placa
                      ? inputErrorClass
                      : ''}"
                    placeholder="ABC-123"
                  />
                  {#if errors.placa}<span class={errorClass}
                      >{errors.placa}</span
                    >{/if}
                  <p class="text-xs text-amber-600 mt-1 dark:text-amber-500">
                    * Se validará ingreso en garita
                  </p>
                </div>
              </div>
            {/if}
          </div>
        </section>
      </div>

      <!-- Footer -->
      <div
        class="p-6 border-t border-gray-200 dark:border-white/10 flex justify-end gap-3 bg-gray-50 dark:bg-black/20"
      >
        <button
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-white/5 border border-gray-300 dark:border-gray-600 rounded-lg transition-colors"
          on:click={onClose}
          disabled={loading}
        >
          Cancelar
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg shadow-sm transition-all flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
          on:click={handleSubmit}
          disabled={loading}
        >
          {#if loading}
            <span class="animate-spin">⌛</span> Guardando...
          {:else}
            <Save size={16} /> Agendar Visita
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
