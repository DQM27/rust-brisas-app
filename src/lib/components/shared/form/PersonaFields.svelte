<script lang="ts">
  import type { SuperForm } from "sveltekit-superforms";
  import { invoke } from "@tauri-apps/api/core";
  import { Plus } from "lucide-svelte";

  interface Props {
    // Aceptamos el objeto 'form' (store), 'errors' (store), y 'constraints' (store)
    // Usamos 'any' genérico porque el esquema varía ligeramente entre Contratista/Proveedor
    // pero los campos base (nombre, apellido, cedula) son coincidentes.
    form: any;
    errors: any;
    constraints: any;
    loading?: boolean;
    isEditMode?: boolean;
    readonly?: boolean;
    empresas?: Array<{ id: string; nombre: string }>;
    showEmpresa?: boolean;
    tableName?: string; // Tabla para validar unicidad (ej: "proveedor", "contratista")
    currentId?: string; // ID para excluir en validación (edición)
    onCreateEmpresa?: () => void; // Callback para crear nueva empresa
  }

  let {
    form,
    errors,
    constraints,
    loading = false,
    isEditMode = false,
    readonly = false,
    empresas = [],
    showEmpresa = true,
    tableName,
    currentId,
    onCreateEmpresa,
  }: Props = $props();

  let checkTimeout: any;
  let cedulaDuplicateError = $state<string | null>(null); // Estado independiente para persistencia

  // Validación en tiempo real (debounced)
  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = input.value;

    // Normalizar entrada (formato regex básico visual)
    // Dejar que Zod maneje el formato estricto, aquí solo unicidad

    if (checkTimeout) clearTimeout(checkTimeout);

    // Solo validar si tiene longitud decente
    if (value.length < 4 || !tableName) return;

    checkTimeout = setTimeout(async () => {
      console.log(`🔍 Checking uniqueness for ${tableName}.${value}...`);
      try {
        const isUnique = await invoke<boolean>("check_unique", {
          table: tableName,
          field: "cedula",
          value,
          excludeId: currentId,
        });

        console.log(`✅ Uniqueness result: ${isUnique}`);

        if (!isUnique) {
          cedulaDuplicateError = "Esta cédula ya está registrada.";
          // También forzamos el error en el store para control (opcional)
          errors.update((errs: any) => ({
            ...errs,
            cedula: ["Esta cédula ya está registrada."],
          }));
        } else {
          cedulaDuplicateError = null;
          // Limpiamos error solo si era de duplicado
          errors.update((errs: any) => {
            if (errs.cedula && errs.cedula.includes("registrada")) {
              const { cedula, ...rest } = errs;
              return rest;
            }
            return errs;
          });
        }
      } catch (e) {
        console.error("Error validando unicidad:", e);
      }
    }, 400); // 400ms debounce
  }

  // Lógica reactiva para clases de input
  const baseInputClass =
    "w-full rounded-md border px-3 py-2 text-sm placeholder:text-gray-400 focus:outline-none focus:ring-2 disabled:opacity-60 transition-colors";

  function getInputClass(hasError: boolean, isReadonly: boolean) {
    const base =
      "w-full bg-black/20 border rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 transition-all outline-none disabled:opacity-50";

    const state = hasError
      ? "!border-red-500/50 !ring-1 !ring-red-500/20"
      : "border-white/10 focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20";

    const readonlyState = isReadonly ? "opacity-70 bg-gray-800/50" : "";

    return `${base} ${state} ${readonlyState}`;
  }

  const labelClass = "block text-xs font-medium text-secondary mb-1";
  const errorClass = "text-xs text-red-500 mt-1";
  const sectionClass =
    "text-base font-semibold text-primary border-b border-surface pb-2 mb-4 flex items-center gap-2";
</script>

<div>
  <h3 class={sectionClass}>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      ><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" /><circle
        cx="9"
        cy="7"
        r="4"
      /><path d="M22 21v-2a4 4 0 0 0-3-3.87" /><path
        d="M16 3.13a4 4 0 0 1 0 7.75"
      /></svg
    >
    Datos Personales
  </h3>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
    <div class="md:col-span-2">
      <label for="cedula" class={labelClass}>Cédula de Identidad *</label>
      <input
        id="cedula"
        name="cedula"
        type="text"
        bind:value={$form.cedula}
        oninput={handleCedulaInput}
        aria-invalid={$errors.cedula || cedulaDuplicateError
          ? "true"
          : undefined}
        disabled={loading || isEditMode || readonly}
        class={getInputClass(
          !!($errors.cedula || cedulaDuplicateError),
          isEditMode || readonly,
        )}
        placeholder="Ej: 001-010203-0001A"
        {...$constraints.cedula}
      />
      {#if $errors.cedula || cedulaDuplicateError}
        <p class={errorClass}>{$errors.cedula || cedulaDuplicateError}</p>
      {/if}
    </div>
  </div>

  <!-- Fila 2: Nombres (Nombre + Segundo Nombre) -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
    <div>
      <label for="nombre" class={labelClass}>Primer Nombre *</label>
      <input
        id="nombre"
        name="nombre"
        type="text"
        bind:value={$form.nombre}
        disabled={loading || readonly}
        class={getInputClass(!!$errors.nombre, readonly)}
        placeholder="Juan"
        {...$constraints.nombre}
      />
      {#if $errors.nombre}<p class={errorClass}>{$errors.nombre}</p>{/if}
    </div>

    <div>
      <label for="segundoNombre" class={labelClass}>Segundo Nombre</label>
      <input
        id="segundoNombre"
        name="segundoNombre"
        type="text"
        bind:value={$form.segundoNombre}
        disabled={loading || readonly}
        class={getInputClass(!!$errors.segundoNombre, readonly)}
        placeholder="Carlos"
        {...$constraints.segundoNombre}
      />
      {#if $errors.segundoNombre}<p class={errorClass}>
          {$errors.segundoNombre}
        </p>{/if}
    </div>
  </div>

  <!-- Fila 3: Apellidos (Apellido + Segundo Apellido) -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
    <div>
      <label for="apellido" class={labelClass}>Primer Apellido *</label>
      <input
        id="apellido"
        name="apellido"
        type="text"
        bind:value={$form.apellido}
        disabled={loading || readonly}
        class={getInputClass(!!$errors.apellido, readonly)}
        placeholder="Pérez"
        {...$constraints.apellido}
      />
      {#if $errors.apellido}<p class={errorClass}>{$errors.apellido}</p>{/if}
    </div>

    <div>
      <label for="segundoApellido" class={labelClass}>Segundo Apellido</label>
      <input
        id="segundoApellido"
        name="segundoApellido"
        type="text"
        bind:value={$form.segundoApellido}
        disabled={loading || readonly}
        class={getInputClass(!!$errors.segundoApellido, readonly)}
        placeholder="González"
        {...$constraints.segundoApellido}
      />
      {#if $errors.segundoApellido}<p class={errorClass}>
          {$errors.segundoApellido}
        </p>{/if}
    </div>
  </div>

  <!-- Fila 4: Empresa (siempre visible si showEmpresa) -->
  {#if showEmpresa}
    <div>
      <label for="empresaId" class={labelClass}>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="inline mr-1"
          ><path d="M6 22V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v18Z" /><path
            d="M6 12H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2"
          /><path d="M18 9h2a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2h-2" /><path
            d="M10 6h4"
          /><path d="M10 10h4" /><path d="M10 14h4" /><path d="M10 18h4" /></svg
        >
        Empresa Proveedora *
      </label>
      <div class="flex gap-2">
        <select
          id="empresaId"
          name="empresaId"
          bind:value={$form.empresaId}
          disabled={loading || readonly || empresas.length === 0}
          class={`${getInputClass(!!$errors.empresaId, readonly)} flex-1`}
          {...$constraints.empresaId}
        >
          {#if empresas.length === 0}
            <option value="">Cargando empresas...</option>
          {:else}
            <option value="">Seleccione una empresa</option>
            {#each empresas as emp}
              <option value={emp.id}>{emp.nombre}</option>
            {/each}
          {/if}
        </select>
        {#if onCreateEmpresa && !readonly}
          <button
            type="button"
            onclick={onCreateEmpresa}
            disabled={loading}
            class="px-3 h-[34px] rounded-lg border-2 border-surface text-secondary hover:border-accent hover:text-accent transition-all flex items-center gap-1 text-xs disabled:opacity-50"
            title="Crear nueva empresa"
          >
            <Plus size={16} />
            <span class="hidden sm:inline">Nueva</span>
          </button>
        {/if}
      </div>
      {#if $errors.empresaId}<p class={errorClass}>{$errors.empresaId}</p>{/if}
    </div>
  {/if}
</div>
