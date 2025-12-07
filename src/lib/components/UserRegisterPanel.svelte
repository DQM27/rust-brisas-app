<script lang="ts">
  import type { UserResponse } from "$lib/types/user";
  import {
    CreateUserSchema,
    type CreateUserForm,
  } from "$lib/schemas/userSchema";
  import { z } from "zod";

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

  const inputFieldClass =
    "w-full rounded border border-emphasis bg-surface-1 px-3 py-2 text-sm text-primary placeholder:text-tertiary focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent disabled:opacity-60";

  const errorClass = "text-xs text-red-500 mt-1";
</script>

<div class="flex min-h-full items-center justify-center p-6">
  <div class="w-full max-w-2xl rounded-lg bg-surface-2 p-8 shadow-xl">
    {#if createdUser && createdUser.temporaryPassword}
      <div
        class="mb-6 rounded-lg border border-green-500/50 bg-green-500/10 p-6 text-center"
      >
        <h3 class="mb-2 text-xl font-bold text-green-500">
          ¡Usuario Creado Exitosamente!
        </h3>
        <p class="mb-4 text-sm text-primary">
          El usuario se generó con una contraseña temporal. Por favor, cópiala y
          entrégasela al usuario.
        </p>

        <div class="flex items-center justify-center gap-3">
          <code
            class="rounded bg-surface-1 px-4 py-2 text-lg font-mono font-bold tracking-wider text-accent"
          >
            {createdUser.temporaryPassword}
          </code>
          <button
            onclick={copyPassword}
            class="rounded bg-surface-1 p-2 text-primary hover:bg-surface-3 transition-colors"
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
          class="mt-6 text-sm font-medium text-tertiary hover:text-primary underline"
        >
          Registrar otro usuario
        </button>
      </div>
    {:else}
      <h2
        class="mb-6 border-b border-accent pb-3 text-2xl font-semibold text-primary"
      >
        Registrar Nuevo Usuario
      </h2>

      <form onsubmit={handleSubmit} class="space-y-6">
        <!-- CÉDULA (NUEVO REQUERIDO) -->
        <div>
          <div class="space-y-2">
            <label for="cedula" class="block text-sm font-medium text-primary"
              >Cédula *</label
            >
            <input
              id="cedula"
              type="text"
              bind:value={formData.cedula}
              placeholder="Ej: 1-1122-0333"
              disabled={loading}
              class={inputFieldClass}
            />
            {#if errors.cedula}<p class={errorClass}>{errors.cedula}</p>{/if}
          </div>
        </div>

        <!-- SECCIÓN 1: DATOS PERSONALES -->
        <div>
          <h3 class="mb-3 text-lg font-medium text-accent">Datos Personales</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Nombre -->
            <div class="space-y-2">
              <label for="nombre" class="block text-sm font-medium text-primary"
                >Primer Nombre *</label
              >
              <input
                id="nombre"
                type="text"
                bind:value={formData.nombre}
                placeholder="Ej: Juan"
                disabled={loading}
                class={inputFieldClass}
              />
              {#if errors.nombre}<p class={errorClass}>{errors.nombre}</p>{/if}
            </div>

            <!-- Segundo Nombre -->
            <div class="space-y-2">
              <label
                for="segundoNombre"
                class="block text-sm font-medium text-primary"
                >Segundo Nombre</label
              >
              <input
                id="segundoNombre"
                type="text"
                bind:value={formData.segundoNombre}
                placeholder="Ej: Carlos"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>

            <!-- Apellido -->
            <div class="space-y-2">
              <label
                for="apellido"
                class="block text-sm font-medium text-primary"
                >Primer Apellido *</label
              >
              <input
                id="apellido"
                type="text"
                bind:value={formData.apellido}
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
              <label
                for="segundoApellido"
                class="block text-sm font-medium text-primary"
                >Segundo Apellido</label
              >
              <input
                id="segundoApellido"
                type="text"
                bind:value={formData.segundoApellido}
                placeholder="Ej: González"
                disabled={loading}
                class={inputFieldClass}
              />
            </div>
          </div>
        </div>

        <!-- SECCIÓN 2: CONTACTO BÁSICO -->
        <div>
          <h3 class="mb-3 text-lg font-medium text-accent">Cuenta y Acceso</h3>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <!-- Email -->
            <div class="space-y-2">
              <label for="email" class="block text-sm font-medium text-primary"
                >Email *</label
              >
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
              <label for="role" class="block text-sm font-medium text-primary"
                >Rol *</label
              >
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

        <!-- Botón -->
        <button
          type="submit"
          disabled={loading}
          class="mt-6 w-full rounded bg-accent px-4 py-2.5 font-medium text-white
                transition-colors hover:bg-accent-hover disabled:cursor-not-allowed disabled:opacity-60"
        >
          {loading ? "Procesando..." : "Registrar Usuario"}
        </button>
      </form>
    {/if}
  </div>
</div>
