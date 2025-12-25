<!-- src/lib/components/user/UserFormModal.svelte -->
<!-- Modal reutilizable para crear y editar usuarios -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Camera } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";
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
  import {
    ROLE_ADMIN_ID,
    ROLE_SUPERVISOR_ID,
    ROLE_GUARDIA_ID,
  } from "$lib/types/role";
  import AdminConfirmModal from "$lib/components/AdminConfirmModal.svelte";
  import { auth } from "$lib/api/auth";
  import { currentUser } from "$lib/stores/auth";
  import { toast } from "svelte-5-french-toast";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";

  import * as roleService from "$lib/logic/role/roleService";
  import * as userService from "$lib/logic/user/userService";
  import type { RoleResponse as RoleType } from "$lib/types/role";

  interface Props {
    show: boolean;
    user?: UserResponse | null; // Si viene, es edición; si no, creación
    loading?: boolean;
    onSave: (
      data: CreateUserInput | UpdateUserInput,
    ) => Promise<boolean | void>;
    onClose: () => void;
  }

  let { show, user = null, loading = false, onSave, onClose }: Props = $props();

  // Roles state
  let availableRoles = $state<RoleType[]>([]);
  let rolesLoading = $state(false);

  // Avatar State (Encrypted Vault)
  let activeAvatar = $state<string | null>(null);
  let avatarLoading = $state(false);

  async function loadAvatar(userId: string) {
    try {
      avatarLoading = true;
      const result = await userService.getUserAvatar(userId);
      if (result.ok) {
        activeAvatar = `data:image/webp;base64,${result.data}`;
      } else {
        activeAvatar = null;
      }
    } catch (e) {
      console.log("No avatar or error:", e);
      activeAvatar = null;
    } finally {
      avatarLoading = false;
    }
  }

  async function handleAvatarUpload() {
    if (!user) return;

    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "Imagen",
            extensions: ["png", "jpg", "jpeg", "webp"],
          },
        ],
      });

      if (selected && typeof selected === "string") {
        avatarLoading = true;
        const toastId = toast.loading("Encriptando y subiendo a Bóveda...");

        const result = await userService.uploadUserAvatar(user.id, selected);

        if (!result.ok) {
          throw new Error(result.error);
        }

        toast.success("Foto blindada exitosamente", { id: toastId });
        await loadAvatar(user.id);
      }
    } catch (e) {
      console.error(e);
      toast.error("Error al subir imagen");
    } finally {
      avatarLoading = false;
    }
  }

  // Modo derivado
  const isEditMode = $derived(!!user);
  const isSelf = $derived(user && $currentUser && user.id === $currentUser.id);

  const modalTitle = $derived(
    isEditMode
      ? `Editar: ${user?.nombre} ${user?.apellido}`.trim()
      : "Crear Nuevo Usuario",
  );

  // Estado del formulario
  let formData = $state<CreateUserForm>({
    cedula: "",
    nombre: "",
    apellido: "",
    segundoNombre: "",
    segundoApellido: "",
    email: "",
    password: "",
    roleId: ROLE_GUARDIA_ID,
    telefono: "",
    direccion: "",
    fechaInicioLabores: "",
    numeroGafete: "",
    fechaNacimiento: "",
    contactoEmergenciaNombre: "",
    contactoEmergenciaTelefono: "",
  });

  let errors = $state<Record<string, string>>({});

  // Cargar roles
  async function loadRoles() {
    try {
      rolesLoading = true;
      const result = await roleService.fetchAllRoles();
      if (result.ok) {
        availableRoles = result.data.roles;
      }
    } catch (e) {
      console.error("Error loading roles:", e);
    } finally {
      rolesLoading = false;
    }
  }

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
      loadRoles();
      activeAvatar = null;
    }

    if (show && user) {
      loadAvatar(user.id);
      formData = {
        cedula: user.cedula || "",
        nombre: user.nombre || "",
        apellido: user.apellido || "",
        segundoNombre: user.segundoNombre || "",
        segundoApellido: user.segundoApellido || "",
        email: user.email || "",
        password: "", // No mostramos password existente
        roleId: user.roleId || ROLE_GUARDIA_ID,
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
        roleId: ROLE_GUARDIA_ID,
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
      // Logic: If creation and password is empty, generate one automatically
      let finalPassword = formData.password;

      if (!isEditMode && !finalPassword) {
        finalPassword =
          Math.random().toString(36).slice(-8) +
          Math.random().toString(36).slice(-2).toUpperCase();
        // Update payload
        result.data.password = finalPassword;
        result.data.mustChangePassword = true;
      }

      // Capture password for modal (using the final one)
      const tempPassword = !isEditMode ? finalPassword : null;

      const success = await onSave(result.data);
      const isSuccess = typeof success === "boolean" ? success : true;

      if (isSuccess) {
        if (!isEditMode && tempPassword) {
          generatedPassword = tempPassword;
          showSuccessModal = true;
        } else {
          onClose();
        }
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
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-2.5 py-1.5 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-60";
  const labelClass =
    "block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1";
  const errorClass = "text-xs text-red-500 mt-0.5";
  const sectionClass =
    "text-sm font-semibold text-gray-800 dark:text-gray-200 border-b border-gray-200 dark:border-gray-700 pb-1 mb-2";
</script>

```
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
      class="relative z-10 w-full max-w-5xl max-h-[95vh] overflow-hidden rounded-lg bg-white dark:bg-[#0d1117] shadow-2xl border border-gray-200 dark:border-gray-700 flex flex-col"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="flex-none flex items-center justify-between px-6 py-4 bg-white dark:bg-[#0d1117] border-b border-gray-200 dark:border-gray-700"
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

      <!-- Form Content (Scrollable only if screen is very small, otherwise fits) -->
      <div class="flex-1 overflow-y-auto p-6">
        {#if isChangingPassword && user}
          <div class="flex justify-center h-full items-center">
            <ChangePasswordPanel
              userId={user.id}
              onSuccess={() => {
                isChangingPassword = false;
              }}
              onCancel={() => (isChangingPassword = false)}
            />
          </div>
        {:else}
          <form
            onsubmit={handleSubmit}
            class="grid grid-cols-1 lg:grid-cols-3 gap-6 h-full"
          >
            <!-- COL 1: Identidad -->
            <!-- COL 1: Identidad + Avatar -->
            <div class="space-y-4">
              <!-- Secure Avatar Component -->
              <div class="flex flex-col items-center justify-center mb-6">
                <div class="relative group">
                  <div
                    class="w-28 h-28 rounded-full overflow-hidden bg-gray-100 dark:bg-gray-800 border-4 border-white dark:border-gray-700 shadow-lg flex items-center justify-center relative"
                  >
                    {#if avatarLoading}
                      <div
                        class="absolute inset-0 flex items-center justify-center bg-black/10 backdrop-blur-sm z-10"
                      >
                        <div
                          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"
                        ></div>
                      </div>
                    {/if}

                    {#if activeAvatar}
                      <img
                        src={activeAvatar}
                        alt="Avatar"
                        class="w-full h-full object-cover"
                      />
                    {:else}
                      <div
                        class="flex flex-col items-center justify-center text-gray-400 dark:text-gray-600"
                      >
                        <span class="text-3xl font-bold">
                          {formData.nombre
                            ? formData.nombre[0].toUpperCase()
                            : "?"}{formData.apellido
                            ? formData.apellido[0].toUpperCase()
                            : ""}
                        </span>
                      </div>
                    {/if}
                  </div>

                  {#if isEditMode}
                    <button
                      type="button"
                      onclick={handleAvatarUpload}
                      disabled={avatarLoading}
                      class="absolute bottom-1 right-1 p-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-full shadow-lg transition-all hover:scale-110 focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 z-20"
                      title="Subir foto segura"
                    >
                      <Camera size={16} />
                    </button>
                  {/if}
                </div>
                {#if isEditMode}
                  <div
                    class="mt-3 flex items-center gap-1.5 px-3 py-1 bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400 rounded-full text-[10px] font-medium border border-green-100 dark:border-green-900/30"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      width="12"
                      height="12"
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
                    Bóveda Encriptada
                  </div>
                {/if}
              </div>

              <h3 class={sectionClass}>Identidad</h3>

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
                {#if errors.cedula}<p class={errorClass}>
                    {errors.cedula}
                  </p>{/if}
              </div>

              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label for="nombre" class={labelClass}>Nombre *</label>
                  <input
                    id="nombre"
                    type="text"
                    value={formData.nombre}
                    oninput={(e) => handleNameInput(e, "nombre")}
                    disabled={loading}
                    class={inputClass}
                  />
                  {#if errors.nombre}<p class={errorClass}>
                      {errors.nombre}
                    </p>{/if}
                </div>
                <div>
                  <label for="segundoNombre" class={labelClass}
                    >2do Nombre</label
                  >
                  <input
                    id="segundoNombre"
                    type="text"
                    value={formData.segundoNombre}
                    oninput={(e) => handleNameInput(e, "segundoNombre")}
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>

              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label for="apellido" class={labelClass}>Apellido *</label>
                  <input
                    id="apellido"
                    type="text"
                    value={formData.apellido}
                    oninput={(e) => handleNameInput(e, "apellido")}
                    disabled={loading}
                    class={inputClass}
                  />
                  {#if errors.apellido}<p class={errorClass}>
                      {errors.apellido}
                    </p>{/if}
                </div>
                <div>
                  <label for="segundoApellido" class={labelClass}
                    >2do Apellido</label
                  >
                  <input
                    id="segundoApellido"
                    type="text"
                    value={formData.segundoApellido}
                    oninput={(e) => handleNameInput(e, "segundoApellido")}
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>

              <div>
                <label for="fechaNacimiento" class={labelClass}
                  >Fecha Nacimiento</label
                >
                <input
                  id="fechaNacimiento"
                  type="date"
                  bind:value={formData.fechaNacimiento}
                  disabled={loading}
                  class={inputClass}
                />
              </div>
            </div>

            <!-- COL 2: Institucional -->
            <div class="space-y-3">
              <h3 class={sectionClass}>Institucional & Cuenta</h3>

              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label for="numeroGafete" class={labelClass}>Gafete</label>
                  <input
                    id="numeroGafete"
                    type="text"
                    value={formData.numeroGafete}
                    oninput={handleGafeteInput}
                    placeholder="K-XXXXXX"
                    disabled={loading}
                    class={inputClass}
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
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>

              <div>
                <label for="email" class={labelClass}>Email *</label>
                <input
                  id="email"
                  type="email"
                  bind:value={formData.email}
                  disabled={loading}
                  class={inputClass}
                />
                {#if errors.email}<p class={errorClass}>{errors.email}</p>{/if}
              </div>

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

              <!-- Roles -->
              {#if !isSelf}
                <div>
                  <label for="roleId" class={labelClass}>Rol *</label>
                  <select
                    id="roleId"
                    bind:value={formData.roleId}
                    disabled={loading || rolesLoading}
                    class={inputClass}
                  >
                    {#if rolesLoading}
                      <option disabled selected>Cargando roles...</option>
                    {:else}
                      <optgroup label="Roles del Sistema">
                        {#each availableRoles.filter((r) => r.isSystem) as role}
                          <option value={role.id}>{role.name}</option>
                        {/each}
                      </optgroup>

                      {#if availableRoles.some((r) => !r.isSystem)}
                        <optgroup label="Roles Personalizados">
                          {#each availableRoles.filter((r) => !r.isSystem) as role}
                            <option value={role.id}>{role.name}</option>
                          {/each}
                        </optgroup>
                      {/if}
                    {/if}
                  </select>
                </div>
              {:else}
                <div
                  class="p-3 bg-gray-50 dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 text-xs text-gray-500 text-center"
                >
                  Tu rol y permisos son gestionados por un administrador.
                </div>
              {/if}
            </div>

            <!-- COL 3: Contacto -->
            <div class="space-y-3">
              <h3 class={sectionClass}>Contacto</h3>

              <div class="grid grid-cols-2 gap-2">
                <div>
                  <label for="contactoEmergenciaNombre" class={labelClass}
                    >Emergencia</label
                  >
                  <input
                    id="contactoEmergenciaNombre"
                    type="text"
                    value={formData.contactoEmergenciaNombre}
                    oninput={(e) =>
                      handleNameInput(e, "contactoEmergenciaNombre")}
                    disabled={loading}
                    class={inputClass}
                    placeholder="Nombre"
                  />
                </div>
                <div>
                  <label for="contactoEmergenciaTelefono" class={labelClass}
                    >Tel. Emergencia</label
                  >
                  <input
                    id="contactoEmergenciaTelefono"
                    type="tel"
                    value={formData.contactoEmergenciaTelefono}
                    oninput={(e) =>
                      handleGenericPhoneInput(e, "contactoEmergenciaTelefono")}
                    onkeydown={handlePhoneKeydown}
                    placeholder="Teléfono"
                    disabled={loading}
                    class={inputClass}
                  />
                </div>
              </div>

              <div>
                <label for="direccion" class={labelClass}>Dirección</label>
                <textarea
                  id="direccion"
                  bind:value={formData.direccion}
                  disabled={loading}
                  class={inputClass}
                  rows="3"
                  placeholder="Dirección completa..."
                ></textarea>
              </div>
            </div>
          </form>
        {/if}
      </div>

      <!-- Footer Actions -->
      <div
        class="flex-none flex gap-2 p-4 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-[#0d1117]/50"
      >
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-gray-800 transition-colors text-sm font-medium"
        >
          Cancelar
        </button>

        <div class="flex-1"></div>

        {#if isSelf && !isChangingPassword}
          <button
            type="button"
            onclick={() => (isChangingPassword = true)}
            disabled={loading}
            class="px-4 py-2 rounded-md border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-white dark:hover:bg-gray-800 transition-colors flex items-center gap-2 text-sm font-medium"
          >
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
              ><rect width="18" height="11" x="3" y="11" rx="2" ry="2" /><path
                d="M7 11V7a5 5 0 0 1 10 0v4"
              /></svg
            >
            Cambiar Contraseña
          </button>
        {/if}

        {#if isEditMode && !isSelf && $currentUser?.roleId === ROLE_ADMIN_ID && !isChangingPassword}
          <button
            type="button"
            onclick={handleResetPasswordClick}
            disabled={loading}
            class="px-4 py-2 rounded-md border border-orange-200 dark:border-orange-900/50 text-orange-600 dark:text-orange-400 hover:bg-orange-50 dark:hover:bg-orange-900/20 transition-colors text-sm font-medium"
          >
            Reset Password
          </button>
        {/if}

        {#if !isChangingPassword}
          <button
            onclick={handleSubmit}
            disabled={loading}
            class="px-6 py-2 rounded-md bg-blue-600 text-white font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors text-sm shadow-sm"
          >
            {loading
              ? "Guardando..."
              : isEditMode
                ? "Guardar Cambios"
                : "Crear Usuario"}
          </button>
        {/if}
      </div>
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
          onclick={() => {
            showSuccessModal = false;
            onClose();
          }}
          class="w-full bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-900 dark:text-white font-medium py-2 rounded-md transition-colors border border-gray-200 dark:border-gray-700"
        >
          Cerrar
        </button>
      </div>
    </div>
  </div>
{/if}
