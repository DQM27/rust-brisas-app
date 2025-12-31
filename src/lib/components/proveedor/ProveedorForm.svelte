<script lang="ts">
  import { superForm } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import {
    CreateProveedorSchema,
    UpdateProveedorSchema,
    type CreateProveedorForm,
    type UpdateProveedorForm,
  } from "$lib/schemas/proveedorSchema";
  import type {
    CreateProveedorInput,
    UpdateProveedorInput,
  } from "$lib/types/proveedor";
  import PersonaFields from "$lib/components/shared/form/PersonaFields.svelte";
  import VehiculoFields from "$lib/components/shared/form/VehiculoFields.svelte";

  // Definir props usando la sintaxis de Svelte 5
  interface Props {
    data?: Partial<CreateProveedorForm & UpdateProveedorForm> | null;
    isEditMode?: boolean;
    loading?: boolean;
    empresas: Array<{ id: string; nombre: string }>;
    onSave: (
      data: CreateProveedorInput | UpdateProveedorInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
    // Props para validación
    currentId?: string;
    onCreateEmpresa?: () => void; // Callback para crear nueva empresa
  }

  let {
    data = null,
    isEditMode = false,
    loading = false,
    empresas = [],
    onSave,
    onClose,
    currentId = "",
    onCreateEmpresa,
  }: Props = $props();

  // 1. Esquema Completo para Validación (Refinements)
  // Este es el que contiene las reglas complejas (ej: placa requerida si tiene vehiculo)
  const validationSchema = isEditMode
    ? UpdateProveedorSchema
    : CreateProveedorSchema;

  // Combinar tipos para tener acceso a todos los campos posibles en el template
  type CombinedForm = CreateProveedorForm & UpdateProveedorForm;

  // 2. Construir datos iniciales manualmente para evitar el error de inferencia de Superforms
  // Esto es más robusto que usar `defaults()` con esquemas complejos
  const initialFormData: CombinedForm = {
    cedula: data?.cedula ?? "",
    nombre: data?.nombre ?? "",
    segundoNombre: data?.segundoNombre ?? "",
    apellido: data?.apellido ?? "",
    segundoApellido: data?.segundoApellido ?? "",
    empresaId: data?.empresaId ?? "",
    estado: data?.estado ?? "ACTIVO",
    tieneVehiculo: data?.tieneVehiculo ?? false,
    tipoVehiculo: data?.tipoVehiculo ?? "",
    placa: data?.placa ?? "",
    marca: data?.marca ?? "",
    modelo: data?.modelo ?? "",
    color: data?.color ?? "",
  };

  // Inicializar el formulario con Superforms en modo SPA
  // Usamos los datos iniciales manuales en lugar de defaults()
  const { form, errors, constraints, enhance, validate, tainted } =
    superForm<CombinedForm>(initialFormData as any, {
      SPA: true,
      validators: zodClient(validationSchema as any),
      dataType: "json",
      async onUpdate({ form }) {
        if (form.valid) {
          // Si es válido, llamar a onSave con los datos
          const success = await onSave(form.data as any);
          if (success !== false) {
            // Asumir éxito si devuelve true o void
            onClose();
          }
        }
      },
    });

  // Clases CSS reutilizables
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-1";
  const sectionClass =
    "text-base font-semibold text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-2 mb-3 flex items-center gap-2";
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";

  // Efecto reactivo para limpiar campos si se desmarca "Tiene Vehículo"
  // IMPORTANTE: Usamos guardia para evitar ciclos infinitos al escribir en form
  let previousTieneVehiculo = $form.tieneVehiculo;
  $effect(() => {
    const current = $form.tieneVehiculo;
    // Solo limpiar si cambió de true a false
    if (previousTieneVehiculo === true && current === false) {
      $form.tipoVehiculo = "";
      $form.placa = "";
      $form.marca = "";
      $form.modelo = "";
      $form.color = "";
    }
    previousTieneVehiculo = current;
  });
</script>

<form use:enhance class="p-6 space-y-6">
  <!-- Datos Personales (Componente Compartido) -->
  <PersonaFields
    {form}
    {errors}
    {constraints}
    {loading}
    {isEditMode}
    {empresas}
    showEmpresa={true}
    tableName="proveedor"
    {currentId}
    {onCreateEmpresa}
  />

  <!-- Estado (Solo Editar) -->
  {#if isEditMode}
    <div>
      <h3 class={sectionClass}>Estado</h3>
      <div>
        <label for="estado" class={labelClass}>Estado Actual</label>
        <select
          id="estado"
          name="estado"
          bind:value={$form.estado}
          disabled={loading}
          class={inputClass}
          {...$constraints.estado}
        >
          <option value="ACTIVO">Activo</option>
          <option value="INACTIVO">Inactivo</option>
          <option value="SUSPENDIDO">Suspendido</option>
        </select>
        {#if $errors.estado}<p class={errorClass}>{$errors.estado}</p>{/if}
      </div>
    </div>
  {/if}

  <!-- Datos Vehículo (Componente Compartido) -->
  <VehiculoFields
    {form}
    {errors}
    {constraints}
    {loading}
    tainted={$tainted}
    originalPlaca={data?.placa || ""}
  />

  <!-- Buttons -->
  <div class="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
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
