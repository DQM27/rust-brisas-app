<script lang="ts">
  import type { UserResponse } from "$lib/types/user";
  import {
    UpdateUserSchema,
    type UpdateUserForm,
  } from "$lib/schemas/userSchema";
  import { auth } from "$lib/api/auth";
  import AdminConfirmModal from "$lib/components/AdminConfirmModal.svelte";
  import { currentUser } from "$lib/stores/auth";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";
  import { toast } from "svelte-5-french-toast";
  import type { UserPermissions } from "$lib/logic/permissions";

  interface Props {
    user: UserResponse;
    loading?: boolean;
    permissions: UserPermissions;
    isSelf?: boolean;
    onUpdate: (data: UpdateUserForm) => Promise<void>;
    onStatusChange?: (isActive: boolean) => Promise<void>;
  }

  let {
    user,
    loading = false,
    permissions,
    isSelf = false,
    onUpdate,
    onStatusChange,
  }: Props = $props();

  // Estado del formulario de edición
  let formData = $state<UpdateUserForm>({
    nombre: user.nombre,
    apellido: user.apellido,
    email: user.email,
    cedula: user.cedula,
    role: user.role, // Ahora relevante para admins
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
  let showAdminConfirm = $state(false);
  let generatedPassword = $state<string | null>(null);
  let showSuccessModal = $state(false);

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
    if (input.value !== newValue) {
      input.value = newValue;
      // Ajustamos el cursor al final para mejor UX
      input.selectionStart = input.selectionEnd = newValue.length;
    }
  }

  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    // Permitir solo números y guiones
    const newValue = input.value.replace(/[^0-9-]/g, "");
    // @ts-ignore
    formData.cedula = newValue;
    if (input.value !== newValue) input.value = newValue;
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

  function handlePhoneInput(event: Event, field: keyof UpdateUserForm) {
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
  }

  async function handleResetPasswordClick() {
    showAdminConfirm = true;
  }

  async function onAdminConfirm(adminPass: string) {
    showAdminConfirm = false;

    // 1. Verify Admin Password
    if (!$currentUser?.email) {
      toast.error("Error de sesión");
      return;
    }

    const toastId = toast.loading("Verificando permisos...");
    try {
      // Try to login to verify password (simple verification)
      await auth.login($currentUser.email, adminPass);

      // 2. Generate Random Password
      const newPass =
        Math.random().toString(36).slice(-8) +
        Math.random().toString(36).slice(-2).toUpperCase(); // Alphanumeric mix

      // 3. Update User
      await onUpdate({
        ...formData, // Send current form data to avoid partial updates if necessary, or just empty object if API supports partial
        password: newPass,
      });

      generatedPassword = newPass;
      showSuccessModal = true;

      toast.success("Contraseña restablecida", { id: toastId });
    } catch (err) {
      console.error(err);
      toast.error("Contraseña de administrador incorrecta", { id: toastId });
    }
  }

  function copyNewPassword() {
    if (generatedPassword) {
      navigator.clipboard.writeText(generatedPassword);
      toast.success("Copiado al portapapeles");
    }
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
      <div class="flex flex-col">
        <h2 class="text-2xl font-bold text-primary">
          {isSelf ? "Mi Perfil" : "Edición de Perfil"}
        </h2>
        {#if !isSelf}
          <p class="text-sm text-tertiary">
            Editando a: <span class="font-medium text-primary"
              >{user.nombre} {user.apellido}</span
            >
          </p>
        {/if}
      </div>
      {#if permissions.canEditSensitive}
        <div class="flex items-center gap-2">
          <!-- Role Selector -->
          <select
            bind:value={formData.role}
            class="px-2 py-1 rounded bg-surface-1 border border-emphasis text-sm text-primary focus:border-accent focus:outline-none"
          >
            <option value="admin">Administrador</option>
            <option value="supervisor">Supervisor</option>
            <option value="guardia">Guardia</option>
          </select>

          <!-- Status Toggle -->
          <button
            type="button"
            onclick={() => onStatusChange?.(!user.isActive)}
            class={`px-3 py-1 rounded-full text-sm font-medium border transition-colors ${user.isActive ? "bg-green-900/20 text-green-400 border-green-900/30 hover:bg-red-900/20 hover:text-red-400 hover:border-red-900/30" : "bg-red-900/20 text-red-400 border-red-900/30 hover:bg-green-900/20 hover:text-green-400 hover:border-green-900/30"}`}
          >
            {user.isActive ? "Activo" : "Inactivo"}
          </button>
        </div>
      {:else}
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
      {/if}
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
              <label for="cedula" class={labelClass}>Cédula</label>
              <input
                id="cedula"
                type="text"
                value={formData.cedula}
                oninput={handleCedulaInput}
                class={inputClass}
                disabled={loading || !permissions.canEditBasic}
                placeholder="Ej: 001-000000-0000A"
              />
            </div>
            <div>
              <label for="email" class={labelClass}>Email</label>
              <input
                id="email"
                type="email"
                bind:value={formData.email}
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                onkeydown={handlePhoneKeydown}
                class={inputClass}
                disabled={loading || !permissions.canEditBasic}
                placeholder="+505 8888-8888"
              />
            </div>
            <div class="col-span-1 md:col-span-2">
              <label for="direccion" class={labelClass}>Dirección</label>
              <textarea
                id="direccion"
                bind:value={formData.direccion}
                class={inputClass}
                disabled={loading || !permissions.canEditBasic}
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
                disabled={loading || !permissions.canEditBasic}
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
                onkeydown={handlePhoneKeydown}
                class={inputClass}
                disabled={loading || !permissions.canEditBasic}
                placeholder="+505 8888-8888"
              />
            </div>
          </div>
        </div>

        <!-- Sección: Datos Laborales (Admin/Supervisor Only) -->
        {#if permissions.canEditSensitive}
          <div>
            <h3 class={sectionTitleClass}>Datos Laborales</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label for="numeroGafete" class={labelClass}
                  >Número de Gafete</label
                >
                <input
                  id="numeroGafete"
                  type="text"
                  value={formData.numeroGafete}
                  oninput={handleGafeteInput}
                  class={inputClass}
                  disabled={loading}
                  placeholder="Ej: K-1234"
                />
              </div>
              <div>
                <label for="fechaInicioLabores" class={labelClass}
                  >Fecha Inicio Labores</label
                >
                <input
                  id="fechaInicioLabores"
                  type="date"
                  bind:value={formData.fechaInicioLabores}
                  class={inputClass}
                  disabled={loading}
                />
              </div>
            </div>
          </div>
        {/if}

        <!-- Botones de Acción -->
        <div
          class="flex flex-col-reverse sm:flex-row items-center justify-between gap-4 pt-6 mt-6 border-t border-surface-tertiary"
        >
          {#if permissions.canChangePassword}
            <!-- User changing their OWN password -->
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
          {:else if permissions.canResetPassword}
            <!-- Admin resetting user password -->
            <button
              type="button"
              onclick={handleResetPasswordClick}
              class="text-orange-400 hover:text-orange-300 font-medium text-sm flex items-center gap-2 transition-colors"
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
                ><path
                  d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"
                /><path d="M3 3v5h5" /><path
                  d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"
                /><path d="M16 21h5v-5" /></svg
              >
              Restablecer Contraseña
            </button>
          {:else}
            <div></div>
            <!-- Spacer -->
          {/if}

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

<!-- Modales -->
<AdminConfirmModal
  isOpen={showAdminConfirm}
  onConfirm={onAdminConfirm}
  onCancel={() => (showAdminConfirm = false)}
/>

{#if showSuccessModal && generatedPassword}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
  >
    <div
      class="w-full max-w-md bg-surface-2 rounded-lg shadow-xl border border-green-500/30 p-6 animate-scale-in"
    >
      <div class="text-center">
        <div
          class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-green-500/20 text-green-400 mb-4"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"><polyline points="20 6 9 17 4 12" /></svg
          >
        </div>
        <h3 class="text-xl font-bold text-white mb-2">
          Contraseña Restablecida
        </h3>
        <p class="text-gray-400 text-sm mb-6">
          La contraseña ha sido generada exitosamente. Por favor compártela con
          el usuario.
        </p>

        <div
          class="flex items-center justify-center gap-3 bg-surface-1 p-3 rounded-lg border border-surface-tertiary mb-6"
        >
          <code class="text-lg font-mono font-bold text-accent tracking-wider"
            >{generatedPassword}</code
          >
          <button
            onclick={copyNewPassword}
            class="p-2 text-gray-400 hover:text-white transition-colors"
            title="Copiar"
          >
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
              ><rect width="14" height="14" x="8" y="8" rx="2" ry="2" /><path
                d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"
              /></svg
            >
          </button>
        </div>

        <button
          onclick={() => (showSuccessModal = false)}
          class="w-full bg-surface-3 hover:bg-surface-4 text-white font-medium py-2 rounded transition-colors"
        >
          Cerrar
        </button>
      </div>
    </div>
  </div>
{/if}
