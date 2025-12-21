<script lang="ts">
  import type { UserResponse } from "$lib/types/user";
  import {
    CreateUserSchema,
    type CreateUserForm,
  } from "$lib/schemas/userSchema";
  import { z } from "zod";
  import { X } from "lucide-svelte";
  import { onMount } from "svelte";

  interface Props {
    loading?: boolean;
    createdUser?: UserResponse | null;
    onSubmit: (data: CreateUserForm) => void;
    onReset: () => void;
  }

  let {
    loading = false,
    createdUser = null,
    onSubmit,
    onReset,
  }: Props = $props();

  // Estado del formulario
  let formData = $state<CreateUserForm>({
    cedula: "",
    nombre: "",
    apellido: "",
    segundoNombre: "",
    segundoApellido: "",
    email: "",
    password: "", // Opcional
    role: "guardia",
    telefono: "",
    direccion: "",
    fechaInicioLabores: "",
    numeroGafete: "",
    fechaNacimiento: "",
    contactoEmergenciaNombre: "",
    contactoEmergenciaTelefono: "",
  });

  let errors = $state<Record<string, string>>({});
  let copied = $state(false);

  // Validación reactiva
  $effect(() => {
    if (Object.values(formData).some((v) => v !== "")) {
      const result = CreateUserSchema.safeParse(formData);
      if (!result.success) {
        const newErrors: Record<string, string> = {};
        result.error.issues.forEach((issue) => {
          if (issue.path[0]) {
            newErrors[String(issue.path[0])] = issue.message;
          }
        });
        errors = newErrors;
      } else {
        errors = {};
      }
    }
  });

  function handleSubmit(event: Event) {
    event.preventDefault();
    const result = CreateUserSchema.safeParse(formData);

    if (result.success) {
      onSubmit(result.data);
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

  function copyPassword() {
    if (createdUser?.temporaryPassword) {
      navigator.clipboard.writeText(createdUser.temporaryPassword);
      copied = true;
      setTimeout(() => (copied = false), 2000);
    }
  }

  function handleGafeteInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value = input.value;

    // Normalizamos: dejamos solo números
    const numbers = value.replace(/[^0-9]/g, "");

    // Construimos el nuevo valor
    const newValue = `K-${numbers}`;

    // Actualizamos el estado
    formData.numeroGafete = newValue;

    // IMPORTANTE: Forzamos la actualización visual si Svelte no lo hace
    // (Esto evita que caracteres inválidos se queden en pantalla)
    if (input.value !== newValue) {
      input.value = newValue;
      // Ajustamos el cursor al final para mejor UX
      input.selectionStart = input.selectionEnd = newValue.length;
    }
  }

  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value.replace(/[^0-9-]/g, "");
    formData.cedula = newValue;
    if (input.value !== newValue) input.value = newValue;
  }

  function handleNameInput(event: Event, field: keyof CreateUserForm) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value.replace(/[^a-zA-ZáéíóúÁÉÍÓÚñÑ\s]/g, "");
    // @ts-ignore
    formData[field] = newValue;
    if (input.value !== newValue) input.value = newValue;
  }

  function handlePhoneInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/[^0-9]/g, "");

    // Si está vacío, permitimos borrar todo
    if (value === "") {
      // @ts-ignore - Generic handler logic or specific field logic needed
      // Since we can't easily map field name from event in this helper without passing it,
      // we will use a closure or check id.
      // Better: Make it generic or rely on input.id matching field name if possible?
      // Actually, UserRegisterPanel uses specific bindings.
      // Let's modify this to take the field name like handleNameInput.
    }
  }

  function handlePhoneKeydown(event: KeyboardEvent) {
    if (
      [
        "Backspace",
        "Delete",
        "Tab",
        "Escape",
        "Enter",
        "ArrowLeft",
        "ArrowRight",
        "Home",
        "End",
        "End",
      ].includes(event.key)
    ) {
      return;
    }
    // Permitir Ctrl+A, Ctrl+C, Ctrl+V, Ctrl+X
    if (event.ctrlKey || event.metaKey) {
      return;
    }
    if (!/^[0-9]$/.test(event.key)) {
      event.preventDefault();
    }
  }

  // Refined generic phone handler
  function handleGenericPhoneInput(event: Event, field: keyof CreateUserForm) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/[^0-9]/g, "");

    // Limitar a máximo 11 dígitos (3+4+4)
    if (value.length > 11) {
      value = value.substring(0, 11);
    }

    if (value === "") {
      // @ts-ignore
      formData[field] = "";
      return;
    }

    let formatted = "+";
    if (value.length > 0) formatted += value.substring(0, 3);
    if (value.length > 3) formatted += " " + value.substring(3, 7);
    if (value.length > 7) formatted += "-" + value.substring(7, 11);

    // @ts-ignore
    formData[field] = formatted;

    if (input.value !== formatted) {
      input.value = formatted;
      input.setSelectionRange(formatted.length, formatted.length);
    }
  }

  const inputFieldClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[#2da44e] disabled:opacity-60 transition-all";

  const errorClass = "text-xs text-red-500 mt-1";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300";
  const sectionTitleClass =
    "mb-3 text-lg font-medium text-gray-900 dark:text-gray-100 border-b border-gray-200 dark:border-gray-700 pb-2";
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div
    class="w-full max-w-2xl rounded-lg bg-white dark:bg-[#0d1117] p-8 shadow-xl border border-gray-200 dark:border-gray-700"
  >
    {#if createdUser && createdUser.temporaryPassword}
      <div
        class="mb-6 rounded-md border border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20 p-6 text-center"
      >
        <h3 class="mb-2 text-xl font-bold text-green-700 dark:text-green-400">
          ¡Usuario Creado Exitosamente!
        </h3>
        <p class="mb-4 text-sm text-gray-600 dark:text-gray-300">
          El usuario se generó con una contraseña temporal. Por favor, cópiala y
          entrégasela al usuario.
        </p>

        <div class="flex items-center justify-center gap-3">
          <code
            class="rounded bg-white dark:bg-[#161b22] px-4 py-2 text-lg font-mono font-bold tracking-wider text-gray-900 dark:text-gray-100 border border-gray-200 dark:border-gray-700"
          >
            {createdUser.temporaryPassword}
          </code>
          <button
            onclick={copyPassword}
            class="rounded-md bg-white dark:bg-[#161b22] p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
            title="Copiar contraseña"
          >
            {#if copied}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="text-green-500"><path d="M20 6 9 17l-5-5" /></svg
              >
            {:else}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                ><rect width="14" height="14" x="8" y="8" rx="2" ry="2" /><path
                  d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
                /></svg
              >
            {/if}
          </button>
        </div>

        <button
          onclick={onReset}
          class="mt-6 text-sm font-medium text-gray-500 dark:text-gray-400 hover:text-[#2da44e] underline transition-colors"
        >
          Registrar otro usuario
        </button>
      </div>
    {:else}
      <h2
        class="mb-6 border-b border-gray-200 dark:border-gray-700 pb-3 text-2xl font-semibold text-gray-900 dark:text-gray-100"
      >
        Registrar Nuevo Usuario
      </h2>

      <form onsubmit={handleSubmit} class="space-y-6">
        <!-- CÉDULA (NUEVO REQUERIDO) -->
        <div>
          <div class="space-y-2">
            <label for="cedula" class={labelClass}>Cédula *</label>
            <input
              id="cedula"
              type="text"
              value={formData.cedula}
              oninput={handleCedulaInput}
              placeholder="Ej: 1-1122-0333"
              disabled={loading}
              class={inputFieldClass}
            />
            {#if errors.cedula}<p class={errorClass}>{errors.cedula}</p>{/if}
          </div>
        </div>

        <!-- SECCIÓN 1: DATOS PERSONALES -->
        <div>
          <h3 class={sectionTitleClass}>Datos Personales</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Nombre -->
            <div class="space-y-2">
              <label for="nombre" class={labelClass}>Primer Nombre *</label>
              <input
                id="nombre"
                type="text"
                value={formData.nombre}
                oninput={(e) => handleNameInput(e, "nombre")}
                placeholder="Ej: Juan"
                disabled={loading}
                class={inputFieldClass}
              />
              {#if errors.nombre}<p class={errorClass}>{errors.nombre}</p>{/if}
            </div>

            <!-- Segundo Nombre -->
            <div class="space-y-2">
              <label for="segundoNombre" class={labelClass}
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                type="text"
                value={formData.segundoNombre}
                oninput={(e) => handleNameInput(e, "segundoNombre")}
                placeholder="Ej: Carlos"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>

            <!-- Apellido -->
            <div class="space-y-2">
              <label for="apellido" class={labelClass}>Primer Apellido *</label>
              <input
                id="apellido"
                type="text"
                value={formData.apellido}
                oninput={(e) => handleNameInput(e, "apellido")}
                placeholder="Ej: Pérez"
                disabled={loading}
                class={inputFieldClass}
              />
              {#if errors.apellido}<p class={errorClass}>
                  {errors.apellido}
                </p>{/if}
            </div>

            <!-- Segundo Apellido -->
            <div class="space-y-2">
              <label for="segundoApellido" class={labelClass}
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                type="text"
                value={formData.segundoApellido}
                oninput={(e) => handleNameInput(e, "segundoApellido")}
                placeholder="Ej: González"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>
          </div>
        </div>

        <!-- SECCIÓN: INFORMACIÓN LABORAL -->
        <div>
          <h3 class={sectionTitleClass}>Información Laboral</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Número de Gafete -->
            <div class="space-y-2">
              <label for="numeroGafete" class={labelClass}>Número Gafete</label>
              <input
                id="numeroGafete"
                type="text"
                value={formData.numeroGafete}
                oninput={handleGafeteInput}
                placeholder="K-017367"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>

            <!-- Fecha Inicio Labores -->
            <div class="space-y-2">
              <label for="fechaInicioLabores" class={labelClass}
                >Fecha Inicio Labores</label
              >
              <input
                id="fechaInicioLabores"
                type="date"
                bind:value={formData.fechaInicioLabores}
                disabled={loading}
                class={inputFieldClass}
              />
            </div>
          </div>
        </div>

        <!-- SECCIÓN 2: CONTACTO BÁSICO -->
        <div>
          <h3 class={sectionTitleClass}>Cuenta y Acceso</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Email -->
            <div class="space-y-2">
              <label for="email" class={labelClass}>Email *</label>
              <input
                id="email"
                type="email"
                bind:value={formData.email}
                placeholder="correo@ejemplo.com"
                disabled={loading}
                class={inputFieldClass}
              />
              {#if errors.email}<p class={errorClass}>{errors.email}</p>{/if}
            </div>

            <!-- Rol -->
            <div class="space-y-2">
              <label for="role" class={labelClass}>Rol *</label>
              <select
                id="role"
                bind:value={formData.role}
                disabled={loading}
                class={inputFieldClass}
              >
                <option value="guardia">Guardia</option>
                <option value="supervisor">Supervisor</option>
                <option value="admin">Administrador</option>
              </select>
            </div>
          </div>
        </div>

        <!-- SECCIÓN: CONTACTO -->
        <div>
          <h3 class={sectionTitleClass}>Contacto</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Teléfono -->
            <div class="space-y-2">
              <label for="telefono" class={labelClass}>Teléfono</label>
              <input
                id="telefono"
                type="tel"
                value={formData.telefono}
                oninput={(e) => handleGenericPhoneInput(e, "telefono")}
                onkeydown={handlePhoneKeydown}
                placeholder="+505 8888-8888"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>

            <!-- Dirección -->
            <div class="col-span-1 sm:col-span-2 space-y-2">
              <label for="direccion" class={labelClass}>Dirección</label>
              <textarea
                id="direccion"
                bind:value={formData.direccion}
                disabled={loading}
                class={inputFieldClass}
                rows="2"
                placeholder="Dirección completa"
              ></textarea>
            </div>
          </div>
        </div>

        <!-- SECCIÓN: EMERGENCIA -->
        <div>
          <h3 class={sectionTitleClass}>Contacto de Emergencia</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Nombre Contacto -->
            <div class="space-y-2">
              <label for="contactoEmergenciaNombre" class={labelClass}
                >Nombre Contacto</label
              >
              <input
                id="contactoEmergenciaNombre"
                type="text"
                value={formData.contactoEmergenciaNombre}
                oninput={(e) => handleNameInput(e, "contactoEmergenciaNombre")}
                disabled={loading}
                class={inputFieldClass}
                placeholder="Nombre del familiar"
              />
            </div>

            <!-- Teléfono Contacto -->
            <div class="space-y-2">
              <label for="contactoEmergenciaTelefono" class={labelClass}
                >Teléfono Contacto</label
              >
              <input
                id="contactoEmergenciaTelefono"
                type="tel"
                value={formData.contactoEmergenciaTelefono}
                oninput={(e) =>
                  handleGenericPhoneInput(e, "contactoEmergenciaTelefono")}
                onkeydown={handlePhoneKeydown}
                placeholder="+505 8888-8888"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>
          </div>
        </div>

        <!-- Botón -->
        <button
          type="submit"
          disabled={loading}
          class="mt-6 w-full rounded-md bg-[#2da44e] px-4 py-2.5 font-medium text-white
                transition-all hover:bg-[#2c974b] disabled:cursor-not-allowed disabled:opacity-60 shadow-sm"
        >
          {loading ? "Procesando..." : "Registrar Usuario"}
        </button>
      </form>
    {/if}
  </div>
</div>
