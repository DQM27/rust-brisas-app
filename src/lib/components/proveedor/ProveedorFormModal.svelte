<!-- src/lib/components/proveedor/ProveedorFormModal.svelte -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Truck } from "lucide-svelte"; // Icon for vehicle section
  import type {
    ProveedorResponse,
    CreateProveedorInput,
    UpdateProveedorInput,
  } from "$lib/types/proveedor";
  import {
    CreateProveedorSchema,
    UpdateProveedorSchema,
    type CreateProveedorForm,
    type UpdateProveedorForm,
  } from "$lib/schemas/proveedorSchema";
  import { submitFetchActiveEmpresas } from "$lib/logic/empresa/empresaService";
  import { toast } from "svelte-5-french-toast";

  interface Props {
    show: boolean;
    proveedor?: ProveedorResponse | null;
    loading?: boolean;
    onSave: (
      data: CreateProveedorInput | UpdateProveedorInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
  }

  let {
    show,
    proveedor = null,
    loading = false,
    onSave,
    onClose,
  }: Props = $props();

  const isEditMode = $derived(!!proveedor);
  const modalTitle = $derived(
    isEditMode ? `Editar Proveedor: ${proveedor?.nombre}` : "Nuevo Proveedor",
  );

  // Empresas state
  let empresas = $state<Array<{ id: string; nombre: string }>>([]);
  let loadingEmpresas = $state(false);

  // Form state
  let formData = $state<CreateProveedorForm & UpdateProveedorForm>({
    cedula: "",
    nombre: "",
    apellido: "",
    segundoNombre: "",
    segundoApellido: "",
    empresaId: "",

    // Vehículo
    tieneVehiculo: false,
    tipoVehiculo: "",
    placa: "",
    marca: "",
    modelo: "",
    color: "",

    // Status (only update)
    estado: "ACTIVO",
  });

  let errors = $state<Record<string, string>>({});

  // Cargar empresas
  $effect(() => {
    if (show) {
      loadEmpresas();
    }
  });

  async function loadEmpresas() {
    loadingEmpresas = true;
    const res = await submitFetchActiveEmpresas();
    if (res.ok) {
      // Mapear respuesta { ok: true, empresas: [...] }
      empresas = res.empresas.map((e: any) => ({ id: e.id, nombre: e.nombre }));
    } else {
      toast.error(res.error || "Error cargando empresas");
    }
    loadingEmpresas = false;
  }

  // Cargar datos al abrir
  $effect(() => {
    if (show && proveedor) {
      // Edit Mode
      formData = {
        cedula: proveedor.cedula,
        nombre: proveedor.nombre,
        apellido: proveedor.apellido,
        segundoNombre: proveedor.segundoNombre || "",
        segundoApellido: proveedor.segundoApellido || "",
        empresaId: proveedor.empresaId,
        estado: (proveedor.estado as any) || "ACTIVO", // Cast 'ACTIVO' | 'INACTIVO' etc

        tieneVehiculo: !!proveedor.vehiculoPlaca,
        tipoVehiculo: proveedor.vehiculoTipo || "",
        placa: proveedor.vehiculoPlaca || "",
        marca: proveedor.vehiculoMarca || "",
        modelo: proveedor.vehiculoModelo || "",
        color: proveedor.vehiculoColor || "",
      };
      errors = {};
    } else if (show && !proveedor) {
      // Create Mode
      formData = {
        cedula: "",
        nombre: "",
        apellido: "",
        segundoNombre: "",
        segundoApellido: "",
        empresaId: "",
        estado: "ACTIVO",

        tieneVehiculo: false,
        tipoVehiculo: "",
        placa: "",
        marca: "",
        modelo: "",
        color: "",
      };
      errors = {};
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const schema = isEditMode ? UpdateProveedorSchema : CreateProveedorSchema;

    // Limpieza de datos antes de validar: Si no tiene vehículo, limpiar campos
    // Aunque el schema maneja esto, es mejor prevenir
    if (!formData.tieneVehiculo) {
      formData.tipoVehiculo = "";
      formData.placa = "";
      formData.marca = "";
      formData.modelo = "";
      formData.color = "";
    }

    const result = schema.safeParse(formData);

    if (result.success) {
      const success = await onSave(result.data);
      if (typeof success === "boolean" && success) {
        onClose();
      } else if (success === undefined) {
        // Void return means success usually if no throw
        onClose();
      }
    } else {
      const newErrors: Record<string, string> = {};
      result.error.issues.forEach((issue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
    }
  }

  // Inputs Handlers
  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const val = input.value.replace(/[^0-9-]/g, "");
    formData.cedula = val;
    if (input.value !== val) input.value = val;
  }

  function handlePlacaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const val = input.value.replace(/[^a-zA-Z0-9-]/g, "").toUpperCase();
    formData.placa = val;
    if (input.value !== val) input.value = val;
  }

  function handleTextInput(event: Event, field: keyof typeof formData) {
    // Generic handler if needed
  }

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-1";
  const sectionClass =
    "text-base font-semibold text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-2 mb-3 flex items-center gap-2";
</script>

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    transition:fade={{ duration: 150 }}
  >
    <!-- Backdrop -->
    <button
      class="absolute inset-0 bg-black/60 backdrop-blur-sm border-0 cursor-default"
      onclick={onClose}
      aria-label="Cerrar"
    ></button>

    <!-- Modal Content -->
    <div
      class="relative z-10 w-full max-w-2xl max-h-[90vh] overflow-auto rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="sticky top-0 z-20 flex items-center justify-between px-6 py-4 bg-white dark:bg-[#0d1117] border-b border-gray-200 dark:border-gray-700"
      >
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          {modalTitle}
        </h2>
        <button
          onclick={onClose}
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
        >
          <X size={20} />
        </button>
      </div>

      <form onsubmit={handleSubmit} class="p-6 space-y-6">
        <!-- ================= DATOS PERSONALES ================= -->
        <div>
          <h3 class={sectionClass}>Datos Personales</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <!-- Cédula -->
            <div>
              <label for="cedula" class={labelClass}>Cédula *</label>
              <input
                id="cedula"
                type="text"
                value={formData.cedula}
                oninput={handleCedulaInput}
                disabled={loading || isEditMode}
                class="{inputClass} {isEditMode ? 'opacity-70 bg-gray-50' : ''}"
                placeholder="000-000000-0000A"
              />
              {#if errors.cedula}<p class={errorClass}>{errors.cedula}</p>{/if}
            </div>

            <!-- Empresa -->
            <div>
              <label for="empresaId" class={labelClass}>Empresa *</label>
              <select
                id="empresaId"
                bind:value={formData.empresaId}
                disabled={loading}
                class={inputClass}
              >
                <option value="">Seleccione una empresa</option>
                {#each empresas as emp}
                  <option value={emp.id}>{emp.nombre}</option>
                {/each}
              </select>
              {#if errors.empresaId}<p class={errorClass}>
                  {errors.empresaId}
                </p>{/if}
            </div>

            <!-- Nombres -->
            <div>
              <label for="nombre" class={labelClass}>Nombre *</label>
              <input
                id="nombre"
                type="text"
                bind:value={formData.nombre}
                disabled={loading}
                class={inputClass}
                placeholder="Juan"
              />
              {#if errors.nombre}<p class={errorClass}>{errors.nombre}</p>{/if}
            </div>

            <div>
              <label for="segundoNombre" class={labelClass}
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                type="text"
                bind:value={formData.segundoNombre}
                disabled={loading}
                class={inputClass}
              />
            </div>

            <!-- Apellidos -->
            <div>
              <label for="apellido" class={labelClass}>Apellido *</label>
              <input
                id="apellido"
                type="text"
                bind:value={formData.apellido}
                disabled={loading}
                class={inputClass}
                placeholder="Pérez"
              />
              {#if errors.apellido}<p class={errorClass}>
                  {errors.apellido}
                </p>{/if}
            </div>

            <div>
              <label for="segundoApellido" class={labelClass}
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                type="text"
                bind:value={formData.segundoApellido}
                disabled={loading}
                class={inputClass}
              />
            </div>
          </div>
        </div>

        <!--Estado (Solo Editar) -->
        {#if isEditMode}
          <div>
            <h3 class={sectionClass}>Estado</h3>
            <div>
              <label for="estado" class={labelClass}>Estado Actual</label>
              <select
                id="estado"
                bind:value={formData.estado}
                disabled={loading}
                class={inputClass}
              >
                <option value="ACTIVO">Activo</option>
                <option value="INACTIVO">Inactivo</option>
                <option value="SUSPENDIDO">Suspendido</option>
              </select>
            </div>
          </div>
        {/if}

        <!-- ================= VEHÍCULO ================= -->
        <div
          class="bg-gray-50 dark:bg-gray-800/50 p-4 rounded-lg border border-gray-200 dark:border-gray-700"
        >
          <div class="flex items-center justify-between mb-4">
            <h3
              class="text-base font-semibold text-gray-800 dark:text-gray-200 flex items-center gap-2 m-0 border-0 p-0"
            >
              <Truck size={18} />
              Datos del Vehículo
            </h3>

            <label class="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                bind:checked={formData.tieneVehiculo}
                class="sr-only peer"
                disabled={loading}
              />
              <div
                class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
              ></div>
              <span
                class="ml-3 text-sm font-medium text-gray-700 dark:text-gray-300"
                >Tiene Vehículo</span
              >
            </label>
          </div>

          {#if formData.tieneVehiculo}
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 animate-scale-in">
              <div>
                <label for="placa" class={labelClass}>Placa *</label>
                <input
                  id="placa"
                  type="text"
                  value={formData.placa}
                  oninput={handlePlacaInput}
                  disabled={loading}
                  class={inputClass}
                  placeholder="M 123-456"
                />
                {#if errors.placa}<p class={errorClass}>{errors.placa}</p>{/if}
              </div>

              <div>
                <label for="tipoVehiculo" class={labelClass}
                  >Tipo Vehículo *</label
                >
                <select
                  id="tipoVehiculo"
                  bind:value={formData.tipoVehiculo}
                  disabled={loading}
                  class={inputClass}
                >
                  <option value="">Seleccione tipo</option>
                  <option value="AUTOMOVIL">Automóvil</option>
                  <option value="MOTOCICLETA">Motocicleta</option>
                  <option value="CAMIONETA">Camioneta</option>
                  <option value="CAMION">Camión</option>
                  <option value="OTRO">Otro</option>
                </select>
                {#if errors.tipoVehiculo}<p class={errorClass}>
                    {errors.tipoVehiculo}
                  </p>{/if}
              </div>

              <div>
                <label for="marca" class={labelClass}>Marca</label>
                <input
                  id="marca"
                  type="text"
                  bind:value={formData.marca}
                  disabled={loading}
                  class={inputClass}
                  placeholder="Toyota"
                />
              </div>

              <div>
                <label for="modelo" class={labelClass}>Modelo</label>
                <input
                  id="modelo"
                  type="text"
                  bind:value={formData.modelo}
                  disabled={loading}
                  class={inputClass}
                  placeholder="Hilux"
                />
              </div>

              <div>
                <label for="color" class={labelClass}>Color</label>
                <input
                  id="color"
                  type="text"
                  bind:value={formData.color}
                  disabled={loading}
                  class={inputClass}
                  placeholder="Blanco"
                />
              </div>
            </div>
          {/if}
        </div>

        <!-- Buttons -->
        <div
          class="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700"
        >
          <button
            type="button"
            onclick={onClose}
            class="flex-1 py-2.5 px-4 rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading}
            class="flex-1 py-2.5 px-4 rounded-md bg-blue-600 text-white font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
          >
            {loading
              ? "Guardando..."
              : isEditMode
                ? "Guardar Cambios"
                : "Crear Proveedor"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
