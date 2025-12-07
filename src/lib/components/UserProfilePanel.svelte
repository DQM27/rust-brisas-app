<script lang="ts">
  import type { UserResponse } from "$lib/types/user";
  import {
    UpdateUserSchema,
    type UpdateUserForm,
  } from "$lib/schemas/userSchema";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";
  import { toast } from "svelte-5-french-toast";
  import { auth } from "$lib/api/auth";

  interface Props {
    user: UserResponse;
    loading?: boolean;
    onUpdate: (data: UpdateUserForm) => Promise<void>;
  }

  let { user, loading = false, onUpdate }: Props = $props();

  // Estado del formulario de edición
  let formData = $state<UpdateUserForm>({
    nombre: user.nombre,
    apellido: user.apellido,
    email: user.email,
    cedula: user.cedula,
    segundoNombre: user.segundoNombre || "",
    segundoApellido: user.segundoApellido || "",
    telefono: user.telefono || "",
    direccion: user.direccion || "",
    contactoEmergenciaNombre: user.contactoEmergenciaNombre || "",
    contactoEmergenciaTelefono: user.contactoEmergenciaTelefono || "",
    // Campos no editables se omiten
    fechaInicioLabores: user.fechaInicioLabores ?? undefined,
    numeroGafete: user.numeroGafete ?? undefined,
    fechaNacimiento: user.fechaNacimiento ?? undefined,
  });

  let errors = $state<Record<string, string>>({});
  let isEditingPassword = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();

    // Validar con Zod
    const result = UpdateUserSchema.safeParse(formData);

    if (!result.success) {
      const newErrors: Record<string, string> = {};
      result.error.issues.forEach((issue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
      return;
    }

    errors = {};

    try {
      await onUpdate(result.data);
      toast.success("Perfil actualizado correctamente");
    } catch (err) {
      console.error(err);
    }
  }

  function handleNameInput(event: Event, field: keyof UpdateUserForm) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value.replace(/[^a-zA-ZáéíóúÁÉÍÓÚñÑ\s]/g, "");

    // @ts-ignore
    formData[field] = newValue;

    if (input.value !== newValue) {
      input.value = newValue;
    }
  }

  function handlePhoneInput(event: Event, field: keyof UpdateUserForm) {
    console.log("🔥 PHONE HANDLER EJECUTADO", field, event);
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/[^0-9]/g, ""); // Solo números

    // Limitar a máximo 11 dígitos (3+4+4)
    if (value.length > 11) {
      value = value.substring(0, 11);
    }

    // Si está vacío, permitimos borrar todo
    if (value === "") {
      // @ts-ignore
      formData[field] = "";
      return;
    }

    // Aplicar máscara +XXX XXXX-XXXX
    let formatted = "+";

    // Primer bloque (3 dígitos)
    if (value.length > 0) {
      formatted += value.substring(0, 3);
    }

    // Segundo bloque (4 dígitos)
    if (value.length > 3) {
      formatted += " " + value.substring(3, 7);
    }

    // Tercer bloque (4 dígitos)
    if (value.length > 7) {
      formatted += "-" + value.substring(7, 11);
    }

    // @ts-ignore
    formData[field] = formatted;

    // Actualización visual forzada
    if (input.value !== formatted) {
      input.value = formatted;
      input.setSelectionRange(formatted.length, formatted.length);
    }
    console.log(
      "✅ FIN HANDLER - input.value:",
      input.value,
      "formData:",
      formData[field],
    );
  }

  const inputClass =
    "w-full rounded border border-emphasis bg-surface-1 px-3 py-2 text-sm text-primary placeholder:text-tertiary focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent disabled:opacity-60";
  const labelClass = "block text-sm font-medium text-primary mb-1";
  const errorClass = "text-xs text-red-500 mt-1";
  const sectionTitleClass =
    "text-lg font-medium text-accent mb-4 border-b border-surface-tertiary pb-2";
</script>

<div class="flex min-h-full flex-col items-center p-6">
  <div class="w-full max-w-4xl rounded-lg bg-surface-2 p-8 shadow-xl">
    <div class="flex items-center justify-between mb-8">
      <h2 class="text-2xl font-bold text-primary">Mi Perfil</h2>
      <div class="flex items-center gap-2">
        <span
          class="px-3 py-1 rounded-full bg-surface-3 text-sm font-medium text-tertiary border border-surface-tertiary capitalize"
        >
          {user.roleDisplay}
        </span>
        <span
          class={`px-3 py-1 rounded-full text-sm font-medium border ${user.isActive ? "bg-green-900/20 text-green-400 border-green-900/30" : "bg-red-900/20 text-red-400 border-red-900/30"}`}
        >
          {user.isActive ? "Activo" : "Inactivo"}
        </span>
      </div>
    </div>

    {#if isEditingPassword}
      <div class="flex justify-center py-8">
        <ChangePasswordPanel
          userId={user.id}
          onSuccess={() => (isEditingPassword = false)}
          onCancel={() => (isEditingPassword = false)}
        />
      </div>
    {:else}
      <form onsubmit={handleSubmit} class="space-y-8">
        <!-- Sección: Identificación -->
        <div>
          <h3 class={sectionTitleClass}>Información Personal</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="email" class={labelClass}>Email</label>
              <input
                id="email"
                type="email"
                bind:value={formData.email}
                disabled={loading}
                class={inputClass}
                title="Puedes actualizar tu email si es necesario"
              />
            </div>
            <div>
              <label for="fechaNacimiento" class={labelClass}
                >Fecha de Nacimiento</label
              >
              <input
                id="fechaNacimiento"
                type="date"
                bind:value={formData.fechaNacimiento}
                class={inputClass}
                disabled={loading}
              />
            </div>

            <div>
              <label for="nombre" class={labelClass}>Primer Nombre</label>
              <input
                id="nombre"
                type="text"
                value={formData.nombre}
                oninput={(e) => handleNameInput(e, "nombre")}
                class={inputClass}
                disabled={loading}
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
                value={formData.segundoNombre}
                oninput={(e) => handleNameInput(e, "segundoNombre")}
                class={inputClass}
                disabled={loading}
              />
            </div>

            <div>
              <label for="apellido" class={labelClass}>Primer Apellido</label>
              <input
                id="apellido"
                type="text"
                value={formData.apellido}
                oninput={(e) => handleNameInput(e, "apellido")}
                class={inputClass}
                disabled={loading}
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
                value={formData.segundoApellido}
                oninput={(e) => handleNameInput(e, "segundoApellido")}
                class={inputClass}
                disabled={loading}
              />
            </div>
          </div>
        </div>

        <!-- Sección: Contacto -->
        <div>
          <h3 class={sectionTitleClass}>Contacto</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="telefono" class={labelClass}>Teléfono</label>
              <input
                id="telefono"
                type="tel"
                value={formData.telefono}
                oninput={(e) => handlePhoneInput(e, "telefono")}
                class={inputClass}
                disabled={loading}
                placeholder="+505 8888-8888"
              />
            </div>
            <div class="col-span-1 md:col-span-2">
              <label for="direccion" class={labelClass}>Dirección</label>
              <textarea
                id="direccion"
                bind:value={formData.direccion}
                class={inputClass}
                disabled={loading}
                rows="2"
              ></textarea>
            </div>
          </div>
        </div>

        <!-- Sección: Emergencia -->
        <div>
          <h3 class={sectionTitleClass}>Contacto de Emergencia</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="contactoEmergenciaNombre" class={labelClass}
                >Nombre Contacto</label
              >
              <input
                id="contactoEmergenciaNombre"
                type="text"
                value={formData.contactoEmergenciaNombre}
                oninput={(e) => handleNameInput(e, "contactoEmergenciaNombre")}
                class={inputClass}
                disabled={loading}
              />
            </div>
            <div>
              <label for="contactoEmergenciaTelefono" class={labelClass}
                >Teléfono Contacto</label
              >
              <input
                id="contactoEmergenciaTelefono"
                type="tel"
                value={formData.contactoEmergenciaTelefono}
                oninput={(e) =>
                  handlePhoneInput(e, "contactoEmergenciaTelefono")}
                class={inputClass}
                disabled={loading}
                placeholder="+505 8888-8888"
              />
            </div>
          </div>
        </div>

        <!-- Botones de Acción -->
        <div
          class="flex flex-col-reverse sm:flex-row items-center justify-between gap-4 pt-6 mt-6 border-t border-surface-tertiary"
        >
          <button
            type="button"
            onclick={() => (isEditingPassword = true)}
            class="text-accent hover:text-accent-hover font-medium text-sm flex items-center gap-2 transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
              ><rect width="18" height="11" x="3" y="11" rx="2" ry="2" /><path
                d="M7 11V7a5 5 0 0 1 10 0v4"
              /></svg
            >
            Cambiar Contraseña
          </button>

          <button
            type="submit"
            disabled={loading}
            class="w-full sm:w-auto bg-accent hover:bg-accent-hover text-white font-medium py-2 px-6 rounded transition-colors shadow-lg shadow-accent/20 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? "Guardando..." : "Guardar Cambios"}
          </button>
        </div>
      </form>
    {/if}
  </div>
</div>
