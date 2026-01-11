<script lang="ts">
  import { superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
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
    currentId?: string;
    onCreateEmpresa?: () => void;
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

  // Combinar tipos para tener acceso a todos los campos posibles en el template
  type CombinedForm = CreateProveedorForm & UpdateProveedorForm;

  // Static default values for initial form setup
  const emptyFormData: CombinedForm = {
    cedula: "",
    nombre: "",
    segundoNombre: "",
    apellido: "",
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

  // Schema selection based on mode - use $derived for reactivity
  const validationSchema = $derived(
    isEditMode ? UpdateProveedorSchema : CreateProveedorSchema,
  );

  // Inicializar el formulario con Superforms en modo SPA
  // Start with empty data, sync via $effect when props change
  const { form, errors, constraints, enhance, validate, tainted, reset } =
    superForm<CombinedForm>(emptyFormData, {
      SPA: true,
      validators: zod4(CreateProveedorSchema), // Start with create schema
      dataType: "json",
      async onUpdate({ form: f }) {
        if (f.valid) {
          const success = await onSave(f.data as any);
          if (success !== false) {
            onClose();
          }
        }
      },
    });

  // 🎯 Svelte 5 Idiomatic Pattern: Use $effect to sync form when props change
  $effect(() => {
    // This runs whenever `data` or `isEditMode` changes
    const newData: CombinedForm = {
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

    // Reset form with new data when props change
    reset({ data: newData });
  });

  // Clases CSS reutilizables
  const labelClass = "text-sm font-medium text-gray-700 dark:text-gray-300";
  const inputClass =
    "w-full px-3 py-2 text-sm border rounded-lg transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-indigo-500/50 dark:bg-gray-700 dark:border-gray-600 dark:text-white disabled:bg-gray-100 dark:disabled:bg-gray-800 disabled:cursor-not-allowed";
  const errorInputClass = "border-red-500 focus:ring-red-500/50";
  const buttonClass =
    "px-4 py-2 text-sm font-medium rounded-lg transition-all duration-150 focus:outline-none focus:ring-2";
</script>

<!-- Same template as before -->
<form use:enhance class="space-y-6">
  <PersonaFields
    {form}
    {errors}
    {constraints}
    {empresas}
    {loading}
    {isEditMode}
    {onCreateEmpresa}
    tableName="proveedor"
    {currentId}
  />

  <VehiculoFields {form} {errors} {constraints} {loading} />

  <!-- Botones -->
  <div class="flex justify-end gap-3 pt-4 border-t border-surface">
    <button
      type="button"
      onclick={onClose}
      disabled={loading}
      class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
    >
      Cancelar
    </button>
    <button
      type="submit"
      disabled={loading}
      class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50"
    >
      {#if loading}
        <span class="flex items-center gap-2">
          <svg class="animate-spin h-4 w-4" viewBox="0 0 24 24">
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
              fill="none"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            />
          </svg>
          Guardando...
        </span>
      {:else}
        {isEditMode ? "Guardar Cambios" : "Crear Proveedor"}
      {/if}
    </button>
  </div>
</form>
