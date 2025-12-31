<script lang="ts">
  import { Truck } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    // Stores de superforms
    form: any;
    errors: any;
    constraints: any;
    tainted?: any; // Para verificar si cambió
    loading?: boolean;
    readonly?: boolean;
    originalPlaca?: string; // Para evitar validar el valor original
  }

  let {
    form,
    errors,
    constraints,
    tainted,
    loading = false,
    readonly = false,
    originalPlaca = "",
  }: Props = $props();

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-1";

  let checkTimeout: any;

  function handlePlacaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = input.value.trim().toUpperCase();

    // Auto upper-case
    if (input.value !== value) {
      input.value = value;
      $form.placa = value;
    } else {
      $form.placa = value; // Ensure form value is updated even if no case change
    }

    if (checkTimeout) clearTimeout(checkTimeout);

    // Solo validar si cambió respecto al original Y tiene longitud
    if (value.length < 3 || value === originalPlaca) {
      // Clear existing placa unique error if it's the original value or too short
      errors.update((errs: any) => {
        if (errs.placa && errs.placa.includes("Esta placa ya existe.")) {
          const { placa, ...rest } = errs;
          return rest;
        }
        return errs;
      });
      return;
    }

    checkTimeout = setTimeout(async () => {
      try {
        // Validamos contra la tabla 'vehiculo'
        const isUnique = await invoke<boolean>("check_unique", {
          table: "vehiculo",
          field: "placa",
          value,
          excludeId: null, // No tenemos vehiculoId fácil, confiamos en originalPlaca check
        });

        if (!isUnique) {
          errors.update((errs: any) => ({
            ...errs,
            placa: ["Esta placa ya existe."],
          }));
        } else {
          errors.update((errs: any) => {
            if (errs.placa && errs.placa.includes("Esta placa ya existe.")) {
              const { placa, ...rest } = errs;
              return rest;
            }
            return errs;
          });
        }
      } catch (e) {
        console.error("Error validando placa:", e);
      }
    }, 400);
  }

  // Efecto para limpiar campos cuando se desmarca (asumiendo que el componente padre
  // o el super refine schema se encargan, pero aquí podemos ayudar visualmente si se requiere
  // aunque sveltekit-superforms maneja el estado globalmente).
  // La lógica de limpieza suele estar mejor en el componente padre o en un efecto global sobre el store.
</script>

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
        bind:checked={$form.tieneVehiculo}
        class="sr-only peer"
        disabled={loading || readonly}
      />
      <div
        class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
      ></div>
      <span class="ml-3 text-sm font-medium text-gray-700 dark:text-gray-300"
        >Tiene Vehículo</span
      >
    </label>
  </div>

  {#if $form.tieneVehiculo}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 animate-scale-in">
      <div>
        <label for="placa" class={labelClass}>Placa *</label>
        <input
          id="placa"
          name="placa"
          type="text"
          bind:value={$form.placa}
          oninput={handlePlacaInput}
          disabled={loading || readonly}
          class={inputClass}
          placeholder="M 123-456"
          {...$constraints.placa}
        />
        {#if $errors.placa}<p class={errorClass}>{$errors.placa}</p>{/if}
      </div>

      <div>
        <label for="tipoVehiculo" class={labelClass}>Tipo Vehículo *</label>
        <select
          id="tipoVehiculo"
          name="tipoVehiculo"
          bind:value={$form.tipoVehiculo}
          disabled={loading || readonly}
          class={inputClass}
          {...$constraints.tipoVehiculo}
        >
          <option value="">Seleccione tipo</option>
          <option value="AUTOMOVIL">Automóvil</option>
          <option value="MOTOCICLETA">Motocicleta</option>
          <option value="CAMIONETA">Camioneta</option>
          <option value="CAMION">Camión</option>
          <option value="OTRO">Otro</option>
        </select>
        {#if $errors.tipoVehiculo}<p class={errorClass}>
            {$errors.tipoVehiculo}
          </p>{/if}
      </div>

      <div>
        <label for="marca" class={labelClass}>Marca</label>
        <input
          id="marca"
          name="marca"
          type="text"
          bind:value={$form.marca}
          disabled={loading || readonly}
          class={inputClass}
          placeholder="Toyota"
          {...$constraints.marca}
        />
        {#if $errors.marca}<p class={errorClass}>{$errors.marca}</p>{/if}
      </div>

      <div>
        <label for="modelo" class={labelClass}>Modelo</label>
        <input
          id="modelo"
          name="modelo"
          type="text"
          bind:value={$form.modelo}
          disabled={loading || readonly}
          class={inputClass}
          placeholder="Hilux"
          {...$constraints.modelo}
        />
        {#if $errors.modelo}<p class={errorClass}>{$errors.modelo}</p>{/if}
      </div>

      <div>
        <label for="color" class={labelClass}>Color</label>
        <input
          id="color"
          name="color"
          type="text"
          bind:value={$form.color}
          disabled={loading || readonly}
          class={inputClass}
          placeholder="Blanco"
          {...$constraints.color}
        />
        {#if $errors.color}<p class={errorClass}>{$errors.color}</p>{/if}
      </div>
    </div>
  {/if}
</div>
