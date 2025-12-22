<!-- src/lib/components/user/UserFormModal.svelte -->
<!-- Modal reutilizable para crear y editar usuarios -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X } from "lucide-svelte";
  import type {
    UserResponse,
    CreateUserInput,
    UpdateUserInput,
  } from "$lib/types/user";
  import {
    CreateUserSchema,
    UpdateUserSchema,
    type CreateUserForm,
  } from "$lib/schemas/userSchema";
  import AdminConfirmModal from "$lib/components/AdminConfirmModal.svelte";
  import { auth } from "$lib/api/auth";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";

  interface Props {
    show: boolean;
    user?: UserResponse | null; // Si viene, es edición; si no, creación
    loading?: boolean;
    onSave: (data: CreateUserInput | UpdateUserInput) => Promise<void>;
    onClose: () => void;
  }

  let { show, user = null, loading = false, onSave, onClose }: Props = $props();

  // Modo derivado
  const isEditMode = $derived(!!user);
  const isSelf = $derived(user && $currentUser && user.id === $currentUser.id);

  const modalTitle = $derived.by(() => {
    if (isSelf) return "Mi Perfil";
    return isEditMode ? `Editar: ${user?.nombre}` : "Nuevo Usuario";
  });

  // Estado del formulario
  let formData = $state<CreateUserForm>({
    cedula: "",
    nombre: "",
    apellido: "",
    segundoNombre: "",
    segundoApellido: "",
    email: "",
    password: "",
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

  // Estado para reset de contraseña
  let showAdminConfirm = $state(false);
  let showSuccessModal = $state(false);
  let generatedPassword = $state<string | null>(null);

  // Estado para "Cambiar Contraseña" (Self)
  let isChangingPassword = $state(false);

  // Cargar datos del usuario cuando se abre en modo edición
  $effect(() => {
    if (show) {
      // Reset view modes
      isChangingPassword = false;
    }

    if (show && user) {
      formData = {
        cedula: user.cedula || "",
        nombre: user.nombre || "",
        apellido: user.apellido || "",
        segundoNombre: user.segundoNombre || "",
        segundoApellido: user.segundoApellido || "",
        email: user.email || "",
        password: "", // No mostramos password existente
        role: user.role || "guardia",
        telefono: user.telefono || "",
        direccion: user.direccion || "",
        fechaInicioLabores: user.fechaInicioLabores || "",
        numeroGafete: user.numeroGafete || "",
        fechaNacimiento: user.fechaNacimiento || "",
        contactoEmergenciaNombre: user.contactoEmergenciaNombre || "",
        contactoEmergenciaTelefono: user.contactoEmergenciaTelefono || "",
      };
      errors = {};
    } else if (show && !user) {
      // Reset para creación
      formData = {
        cedula: "",
        nombre: "",
        apellido: "",
        segundoNombre: "",
        segundoApellido: "",
        email: "",
        password: "",
        role: "guardia",
        telefono: "",
        direccion: "",
        fechaInicioLabores: "",
        numeroGafete: "",
        fechaNacimiento: "",
        contactoEmergenciaNombre: "",
        contactoEmergenciaTelefono: "",
      };
      errors = {};
    }
  });

  // Validación reactiva
  $effect(() => {
    if (Object.values(formData).some((v) => v !== "")) {
      const schema = isEditMode ? UpdateUserSchema : CreateUserSchema;
      const result = schema.safeParse(formData);
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

  async function handleSubmit(event: Event) {
    event.preventDefault();
    const schema = isEditMode ? UpdateUserSchema : CreateUserSchema;
    const result = schema.safeParse(formData);

    if (result.success) {
      await onSave(result.data);
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

  // Input handlers
  function handleGafeteInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const numbers = input.value.replace(/[^0-9]/g, "");
    const newValue = `K-${numbers}`;
    formData.numeroGafete = newValue;
    if (input.value !== newValue) {
      input.value = newValue;
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
    )
      return;
    if (event.ctrlKey || event.metaKey) return;
    if (!/^[0-9]$/.test(event.key)) event.preventDefault();
  }

  function handleGenericPhoneInput(event: Event, field: keyof CreateUserForm) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/[^0-9]/g, "");
    if (value.length > 11) value = value.substring(0, 11);
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

  // Password Reset Logic
  async function handleResetPasswordClick() {
    showAdminConfirm = true;
  }

  async function onAdminConfirm(adminPass: string) {
    showAdminConfirm = false;

    if (!$currentUser?.email) {
      toast.error("Error de sesión");
      return;
    }

    const toastId = toast.loading("Verificando permisos...");
    try {
      // 1. Verify Admin Password
      await auth.login($currentUser.email, adminPass);

      // 2. Generate Random Password
      const newPass =
        Math.random().toString(36).slice(-8) +
        Math.random().toString(36).slice(-2).toUpperCase();

      // 3. Update User (usando onSave con los datos actuales + password)
      // Nota: onSave espera CreateUserInput o UpdateUserInput.
      // Modificamos para enviar la password
      const updateData = {
        ...formData,
        password: newPass,
        mustChangePassword: true,
      } as unknown as UpdateUserInput; // Cast necesario porque formData no tiene password en UpdateUserForm

      await onSave(updateData);

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

  // Styles
  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";
  const labelClass =
    "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-1";
  const sectionClass =
    "text-base font-semibold text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-2 mb-3";
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
      aria-label="Cerrar modal"
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
          class="p-1 rounded-full text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
      </div>

      <!-- Form or Password Change -->
      {#if isChangingPassword && user}
        <div class="p-6 flex justify-center">
          <ChangePasswordPanel
            userId={user.id}
            onSuccess={() => {
              isChangingPassword = false;
              // Optionally close modal or toast
            }}
            onCancel={() => (isChangingPassword = false)}
          />
        </div>
      {:else}
        <form onsubmit={handleSubmit} class="p-6 space-y-5">
          <!-- Cédula -->
          <div>
            <label for="cedula" class={labelClass}>Cédula *</label>
            <input
              id="cedula"
              type="text"
              value={formData.cedula}
              oninput={handleCedulaInput}
              placeholder="Ej: 1-1122-0333"
              disabled={loading}
              class={inputClass}
            />
            {#if errors.cedula}<p class={errorClass}>{errors.cedula}</p>{/if}
          </div>

          <!-- Datos Personales -->
          <div>
            <h3 class={sectionClass}>Datos Personales</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="nombre" class={labelClass}>Nombre *</label>
                <input
                  id="nombre"
                  type="text"
                  value={formData.nombre}
                  oninput={(e) => handleNameInput(e, "nombre")}
                  placeholder="Juan"
                  disabled={loading}
                  class={inputClass}
                />
                {#if errors.nombre}<p class={errorClass}>
                    {errors.nombre}
                  </p>{/if}
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
                  placeholder="Carlos"
                  disabled={loading}
                  class={inputClass}
                />
              </div>
              <div>
                <label for="apellido" class={labelClass}>Apellido *</label>
                <input
                  id="apellido"
                  type="text"
                  value={formData.apellido}
                  oninput={(e) => handleNameInput(e, "apellido")}
                  placeholder="Pérez"
                  disabled={loading}
                  class={inputClass}
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
                  placeholder="González"
                  disabled={loading}
                  class={inputClass}
                />
              </div>
            </div>
          </div>

          <!-- Información Laboral -->
          <div>
            <h3 class={sectionClass}>Información Laboral</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="numeroGafete" class={labelClass}
                  >Número Gafete</label
                >
                <input
                  id="numeroGafete"
                  type="text"
                  value={formData.numeroGafete}
                  oninput={handleGafeteInput}
                  placeholder="K-017367"
                  disabled={loading}
                  class={inputClass}
                />
              </div>
              <div>
                <label for="fechaInicioLabores" class={labelClass}
                  >Fecha Inicio</label
                >
                <input
                  id="fechaInicioLabores"
                  type="date"
                  bind:value={formData.fechaInicioLabores}
                  disabled={loading}
                  class={inputClass}
                />
              </div>
            </div>
          </div>

          <!-- Cuenta -->
          <div>
            <h3 class={sectionClass}>Cuenta y Acceso</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="email" class={labelClass}>Email *</label>
                <input
                  id="email"
                  type="email"
                  bind:value={formData.email}
                  placeholder="correo@ejemplo.com"
                  disabled={loading}
                  class={inputClass}
                />
                {#if errors.email}<p class={errorClass}>{errors.email}</p>{/if}
              </div>
              <!-- Roles (Solo si no es self y es admin/supervisor) -->
              {#if !isSelf}
                <div>
                  <label for="role" class={labelClass}>Rol *</label>
                  <select
                    id="role"
                    bind:value={formData.role}
                    disabled={loading}
                    class={inputClass}
                  >
                    <option value="guardia">Guardia</option>
                    <option value="supervisor">Supervisor</option>
                    <option value="admin">Administrador</option>
                  </select>
                </div>
              {/if}

              <!-- Contraseña Temporal (Solo Creación) -->
              {#if !isEditMode}
                <div class="col-span-2">
                  <label for="password" class={labelClass}
                    >Contraseña Temporal *</label
                  >
                  <input
                    id="password"
                    type="text"
                    bind:value={formData.password}
                    placeholder="Contraseña inicial para el usuario"
                    disabled={loading}
                    class={inputClass}
                  />
                  {#if errors.password}<p class={errorClass}>
                      {errors.password}
                    </p>{/if}
                </div>
              {/if}
            </div>
          </div>

          <!-- Contacto -->
          <div>
            <h3 class={sectionClass}>Contacto</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="telefono" class={labelClass}>Teléfono</label>
                <input
                  id="telefono"
                  type="tel"
                  value={formData.telefono}
                  oninput={(e) => handleGenericPhoneInput(e, "telefono")}
                  onkeydown={handlePhoneKeydown}
                  placeholder="+505 8888-8888"
                  disabled={loading}
                  class={inputClass}
                />
              </div>
              <div class="col-span-2">
                <label for="direccion" class={labelClass}>Dirección</label>
                <textarea
                  id="direccion"
                  bind:value={formData.direccion}
                  disabled={loading}
                  class={inputClass}
                  rows="2"
                  placeholder="Dirección completa"
                ></textarea>
              </div>
            </div>
          </div>

          <!-- Emergencia -->
          <div>
            <h3 class={sectionClass}>Contacto de Emergencia</h3>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label for="contactoEmergenciaNombre" class={labelClass}
                  >Nombre</label
                >
                <input
                  id="contactoEmergenciaNombre"
                  type="text"
                  value={formData.contactoEmergenciaNombre}
                  oninput={(e) =>
                    handleNameInput(e, "contactoEmergenciaNombre")}
                  disabled={loading}
                  class={inputClass}
                  placeholder="Nombre familiar"
                />
              </div>
              <div>
                <label for="contactoEmergenciaTelefono" class={labelClass}
                  >Teléfono</label
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
                  class={inputClass}
                />
              </div>
            </div>
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

            {#if isSelf}
              <button
                type="button"
                onclick={() => (isChangingPassword = true)}
                disabled={loading}
                class="flex-1 py-2.5 px-4 rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors flex items-center justify-center gap-2"
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
                  ><rect
                    width="18"
                    height="11"
                    x="3"
                    y="11"
                    rx="2"
                    ry="2"
                  /><path d="M7 11V7a5 5 0 0 1 10 0v4" /></svg
                >
                Cambiar Contraseña
              </button>
            {/if}

            {#if isEditMode && !isSelf && $currentUser?.role === "admin"}
              <button
                type="button"
                onclick={handleResetPasswordClick}
                disabled={loading}
                class="flex-1 py-2.5 px-4 rounded-md border border-orange-200 dark:border-orange-900/50 text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-900/20 transition-colors"
              >
                Reset Password
              </button>
            {/if}
            <button
              type="submit"
              disabled={loading}
              class="flex-1 py-2.5 px-4 rounded-md bg-blue-600 text-white font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
            >
              {loading
                ? "Guardando..."
                : isEditMode
                  ? "Guardar Cambios"
                  : "Crear Usuario"}
            </button>
          </div>
        </form>
      {/if}
    </div>
  </div>
{/if}

<!-- Modales -->
<AdminConfirmModal
  isOpen={showAdminConfirm}
  onConfirm={onAdminConfirm}
  onCancel={() => (showAdminConfirm = false)}
/>

{#if showSuccessModal && generatedPassword}
  <div
    class="fixed inset-0 z-[60] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
  >
    <div
      class="w-full max-w-md bg-white dark:bg-[#0d1117] rounded-lg shadow-xl border border-green-200 dark:border-green-900/50 p-6 animate-scale-in"
    >
      <div class="text-center">
        <div
          class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 mb-4"
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
        <h3 class="text-xl font-bold text-gray-900 dark:text-white mb-2">
          Contraseña Restablecida
        </h3>
        <p class="text-gray-500 dark:text-gray-400 text-sm mb-6">
          La contraseña ha sido generada exitosamente. Por favor compártela con
          el usuario.
        </p>

        <div
          class="flex items-center justify-center gap-3 bg-gray-50 dark:bg-[#161b22] p-3 rounded-md border border-gray-200 dark:border-gray-700 mb-6"
        >
          <code
            class="text-lg font-mono font-bold text-gray-900 dark:text-white tracking-wider"
            >{generatedPassword}</code
          >
          <button
            onclick={copyNewPassword}
            class="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
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
          class="w-full bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-900 dark:text-white font-medium py-2 rounded-md transition-colors border border-gray-200 dark:border-gray-700"
        >
          Cerrar
        </button>
      </div>
    </div>
  </div>
{/if}
