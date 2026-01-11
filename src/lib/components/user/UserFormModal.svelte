<!-- src/lib/components/user/UserFormModal.svelte -->
<!-- Modal reutilizable para crear y editar usuarios -->
<script lang="ts">
  import { fade, fly } from "svelte/transition";
  import { X, Camera, ChevronDown } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import type {
    UserResponse,
    CreateUserInput,
    UpdateUserInput,
  } from "$lib/types/user";
  import { Operacion } from "$lib/types/user";
  import {
    CreateUserSchema,
    UpdateUserSchema,
    type CreateUserForm,
  } from "$lib/schemas/userSchema";
  import { superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
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
    readonly?: boolean;
    isSelfEdit?: boolean; // True cuando el usuario edita su propio perfil
  }

  let {
    show,
    user = null,
    loading = false,
    onSave,
    onClose,
    readonly = false,
    isSelfEdit = false,
  }: Props = $props();

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

  // El usuario está editando su propio perfil (usando prop explícita o comparación de IDs)
  const isSelf = $derived(isSelfEdit);

  // Nombre completo para mostrar en el header
  const getFullName = (u: UserResponse | null | undefined) => {
    if (!u) return "";
    return [u.nombre, u.segundoNombre, u.apellido, u.segundoApellido]
      .filter(Boolean)
      .join(" ");
  };

  const modalTitle = $derived(
    isChangingPassword
      ? "Actualización de Contraseña"
      : readonly
        ? `Ver Detalle: ${getFullName(user)}`
        : isEditMode
          ? `Editar: ${getFullName(user)}`
          : "Crear Nuevo Usuario",
  );

  // --- SUPERFORMS SETUP ---
  // State for Role Dropdown
  let showRoleDropdown = $state(false);
  let showOperacionDropdown = $state(false); // New state for Operacion dropdown

  const initialValues: CreateUserForm = {
    cedula: "",
    nombre: "",
    apellido: "",
    segundoNombre: "",
    segundoApellido: "",
    email: "",
    operacion: "" as Operacion, // Empty to show placeholder
    password: "",
    roleId: "", // Empy to show placeholder and required validation
    telefono: "",
    direccion: "",
    fechaInicioLabores: "",
    numeroGafete: "",
    fechaNacimiento: "",
    contactoEmergenciaNombre: "",
    contactoEmergenciaTelefono: "",
    vencimientoPortacion: "",
    mustChangePassword: false,
  };

  const {
    form,
    errors,
    constraints,
    enhance,
    validateForm,
    tainted,
    reset: resetForm,
  } = superForm<CreateUserForm>(initialValues, {
    SPA: true,
    validators: zod4(CreateUserSchema),
    onUpdate: async ({ form: f }) => {
      if (!f.valid) return;

      let payloadData = f.data as CreateUserForm;
      let tempPassword: string | null = null;

      // Solo para creación: generar password si está vacía
      if (!isEditMode) {
        if (!payloadData.password) {
          tempPassword =
            Math.random().toString(36).slice(-8) +
            Math.random().toString(36).slice(-2).toUpperCase();
          payloadData.password = tempPassword;
          payloadData.mustChangePassword = true;
        } else {
          tempPassword = payloadData.password as string;
        }
      } else {
        // En modo edición: limpiar campos opcionales vacíos para evitar
        // que el backend intente guardar strings vacíos que violan el schema.
        const optionalFields = [
          "password",
          "segundoNombre",
          "segundoApellido",
          "telefono",
          "direccion",
          "fechaInicioLabores",
          "numeroGafete",
          "fechaNacimiento",
          "contactoEmergenciaNombre",
          "contactoEmergenciaTelefono",
        ] as const;

        for (const field of optionalFields) {
          const value = (payloadData as any)[field];
          if (
            value === "" ||
            (typeof value === "string" && value.trim() === "")
          ) {
            delete (payloadData as any)[field];
          }
        }

        // SEGURIDAD: El usuario NO puede cambiar su propio rol
        // Solo un administrador/God puede cambiar roles de otros usuarios
        if (isSelf) {
          delete (payloadData as any).roleId;
        }
      }

      const success = await onSave(payloadData as CreateUserInput);
      const isSuccess = typeof success === "boolean" ? success : true;

      if (isSuccess) {
        if (!isEditMode && tempPassword) {
          generatedPassword = tempPassword;
          showSuccessModal = true;
        } else {
          onClose();
        }
      }
    },
    resetForm: false, // Controlamos el reset manualmente
    taintedMessage: null,
  });

  // Derived state to find the name of the current role
  const currentRoleName = $derived(
    $form.roleId
      ? (availableRoles.find((r) => r.id === $form.roleId)?.name ??
          "Cargando...")
      : "Sin Rol",
  );

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

  // Validación de duplicados en tiempo real
  let checkTimeout: any;
  let cedulaDuplicateError = $state<string | null>(null);
  let emailDuplicateError = $state<string | null>(null);

  async function checkUniqueness(field: string, value: string) {
    if (value.length < 3) return null;
    try {
      const isUnique = await invoke<boolean>("check_unique", {
        table: "user",
        field,
        value,
        excludeId: user?.id,
      });
      return isUnique
        ? null
        : `Este ${field === "cedula" ? "número de cédula" : "correo electrónico"} ya está registrado.`;
    } catch (e) {
      console.error(`Error checking unique ${field}:`, e);
      return null;
    }
  }

  // Cargar datos del usuario cuando se abre en modo edición
  $effect(() => {
    if (show) {
      // Reset view modes
      isChangingPassword = false;
      loadRoles();
      activeAvatar = null;
      cedulaDuplicateError = null;
      emailDuplicateError = null;
    }

    if (show && user) {
      loadAvatar(user.id);
      $form = {
        cedula: user.cedula || "",
        nombre: user.nombre || "",
        apellido: user.apellido || "",
        segundoNombre: user.segundoNombre || "",
        segundoApellido: user.segundoApellido || "",
        email: user.email || "",
        operacion: user.operacion || Operacion.CalleBlancos,
        password: "",
        roleId: user.roleId || ROLE_GUARDIA_ID,
        telefono: user.telefono || "",
        direccion: user.direccion || "",
        fechaInicioLabores: user.fechaInicioLabores || "",
        numeroGafete: user.numeroGafete || "",
        fechaNacimiento: user.fechaNacimiento || "",
        contactoEmergenciaNombre: user.contactoEmergenciaNombre || "",
        contactoEmergenciaTelefono: user.contactoEmergenciaTelefono || "",
        vencimientoPortacion: user.vencimientoPortacion || "",
        mustChangePassword: user.mustChangePassword || false,
      };
    } else if (show && !user) {
      // Reset para creación
      resetForm({ data: initialValues });
    }
  });

  // Input handlers
  function handleGafeteInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const numbers = input.value.replace(/[^0-9]/g, "");
    const newValue = `K-${numbers}`;
    $form.numeroGafete = newValue;
    if (input.value !== newValue) {
      input.value = newValue;
      input.selectionStart = input.selectionEnd = newValue.length;
    }
  }

  function handleCedulaInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value.replace(/[^0-9-]/g, "");
    $form.cedula = newValue;
    if (input.value !== newValue) input.value = newValue;

    if (checkTimeout) clearTimeout(checkTimeout);
    checkTimeout = setTimeout(async () => {
      cedulaDuplicateError = await checkUniqueness("cedula", newValue);
    }, 400);
  }

  function handleEmailInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value;
    $form.email = newValue;

    if (checkTimeout) clearTimeout(checkTimeout);
    checkTimeout = setTimeout(async () => {
      emailDuplicateError = await checkUniqueness("email", newValue);
    }, 400);
  }

  function handleNameInput(event: Event, field: keyof CreateUserForm) {
    const input = event.target as HTMLInputElement;
    const newValue = input.value.replace(/[^a-zA-ZáéíóúÁÉÍÓÚñÑüÜ ]/g, "");
    // @ts-ignore
    $form[field] = newValue;
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
      $form[field] = "";
      return;
    }
    let formatted = "+";
    if (value.length > 0) formatted += value.substring(0, 3);
    if (value.length > 3) formatted += " " + value.substring(3, 7);
    if (value.length > 7) formatted += "-" + value.substring(7, 11);
    // @ts-ignore
    $form[field] = formatted;
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

      // 3. Update User
      const updateData = {
        ...$form,
        password: newPass,
        mustChangePassword: true,
      } as unknown as UpdateUserInput;

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

  // Standardized UI Pattern - CRUD Form Standard
  const inputClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white placeholder:text-gray-500 focus:outline-none focus:!border-blue-500/50 focus:!ring-1 focus:!ring-blue-500/20 disabled:opacity-50 transition-all";
  const selectClass =
    "w-full bg-black/20 border border-white/10 rounded-lg px-3 py-1.5 h-[34px] text-sm text-white focus:outline-none disabled:opacity-50 transition-all cursor-pointer appearance-none bg-no-repeat bg-right pr-8";
  const labelClass = "block text-xs font-medium text-secondary mb-1";
  const errorClass = "text-xs text-red-500 mt-0.5";
  const sectionClass =
    "text-xs font-semibold text-primary/80 uppercase tracking-wide border-b border-surface pb-1.5 mb-2";

  // Helper to determine field border color based on state
  function getFieldStateClass(field: string, value: any) {
    if (($errors as any)[field])
      return "!border-red-500/50 !ring-1 !ring-red-500/20";

    // Solo mostrar éxito si el campo ha sido "tocado" / cambiado
    const isTainted = $tainted && ($tainted as any)[field];
    if (
      isTainted &&
      value &&
      String(value).trim() !== "" &&
      value !== "Selec CDI" &&
      value !== "Selec Rol"
    ) {
      return "!border-green-500/50 !ring-1 !ring-green-500/20";
    }
    return "";
  }

  // Handler for custom Tab navigation in date inputs
  function handleDateTab(e: KeyboardEvent, nextId: string) {
    if (e.key === "Tab" && !e.shiftKey) {
      e.preventDefault();
      const next = document.getElementById(nextId);
      next?.focus();
    }
  }

  // Handler para Ctrl+S
  function handleKeydown(e: KeyboardEvent) {
    if (!show || readonly || loading) return;
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "s") {
      e.preventDefault();
      // Disparar submit del formulario
      const form = document.getElementById("user-form") as HTMLFormElement;
      if (form) {
        form.requestSubmit();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

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
      class="relative z-10 w-full {isChangingPassword
        ? 'max-w-sm'
        : 'max-w-2xl'} max-h-[95vh] overflow-hidden rounded-xl bg-surface-2 shadow-2xl border border-surface flex flex-col"
      transition:fly={{ y: 20, duration: 200 }}
    >
      <!-- Header -->
      <div
        class="flex-none flex items-center justify-between px-3 py-3 bg-surface-2 border-b border-surface"
      >
        <h2 class="text-xl font-semibold text-primary">
          {modalTitle}
        </h2>
        <button
          onclick={onClose}
          class="p-1.5 rounded-lg text-secondary hover:text-primary hover:bg-surface-3 transition-colors"
          aria-label="Cerrar"
        >
          <X size={20} />
        </button>
      </div>

      <div class="flex-1 flex flex-col">
        {#if isChangingPassword && user}
          <div class="flex justify-center items-start p-6 flex-1">
            <div class="w-full max-w-md">
              <ChangePasswordPanel
                userId={user.id}
                hideHeader={true}
                onSuccess={() => {
                  isChangingPassword = false;
                }}
                onCancel={() => (isChangingPassword = false)}
              />
            </div>
          </div>
        {:else}
          <form id="user-form" use:enhance class="contents">
            <!-- Form Area -->
            <div class="flex-1 p-6 space-y-4">
              {#if isEditMode}
                <div class="flex flex-col items-center justify-center mb-4">
                  <div class="relative group">
                    <div
                      class="w-24 h-24 rounded-full overflow-hidden bg-gray-100 dark:bg-gray-800 border-4 border-white dark:border-gray-700 shadow-lg flex items-center justify-center relative"
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
                            {$form.nombre
                              ? $form.nombre[0].toUpperCase()
                              : "?"}{$form.apellido
                              ? $form.apellido[0].toUpperCase()
                              : ""}
                          </span>
                        </div>
                      {/if}
                    </div>
                    {#if !readonly}
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
                </div>
              {/if}

              <div
                class="grid grid-cols-1 lg:grid-cols-2 gap-6 h-full p-7 bg-surface-1 rounded-lg border border-surface"
              >
                <!-- COL 1: Identidad -->
                <div class="space-y-3">
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label for="cedula" class={labelClass}
                        >Cédula <span class="text-red-500 ml-0.5">*</span
                        ></label
                      >
                      <input
                        id="cedula"
                        type="text"
                        value={$form.cedula}
                        oninput={handleCedulaInput}
                        placeholder="Ej: 1-1122-0333"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'cedula',
                          $form.cedula,
                        )} {cedulaDuplicateError
                          ? '!border-red-500/50 !ring-1 !ring-red-500/20'
                          : ''}"
                      />
                      {#if $errors.cedula || cedulaDuplicateError}
                        <p class={errorClass}>
                          {$errors.cedula || cedulaDuplicateError}
                        </p>
                      {/if}
                    </div>
                    <div>
                      <label for="numeroGafete" class={labelClass}>Gafete</label
                      >
                      <input
                        id="numeroGafete"
                        type="text"
                        value={$form.numeroGafete}
                        oninput={handleGafeteInput}
                        placeholder="K-123456"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'numeroGafete',
                          $form.numeroGafete,
                        )}"
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label for="nombre" class={labelClass}
                        >Nombre <span class="text-red-500 ml-0.5">*</span
                        ></label
                      >
                      <input
                        id="nombre"
                        type="text"
                        value={$form.nombre}
                        oninput={(e) => handleNameInput(e, "nombre")}
                        placeholder="Ej: Juan"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'nombre',
                          $form.nombre,
                        )}"
                      />
                      {#if $errors.nombre}<p class={errorClass}>
                          {$errors.nombre}
                        </p>{/if}
                    </div>
                    <div>
                      <label for="segundoNombre" class={labelClass}
                        >Segundo Nombre</label
                      >
                      <input
                        id="segundoNombre"
                        type="text"
                        value={$form.segundoNombre}
                        oninput={(e) => handleNameInput(e, "segundoNombre")}
                        placeholder="Ej: Carlos"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'segundoNombre',
                          $form.segundoNombre,
                        )}"
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label for="apellido" class={labelClass}
                        >Apellido <span class="text-red-500 ml-0.5">*</span
                        ></label
                      >
                      <input
                        id="apellido"
                        type="text"
                        value={$form.apellido}
                        oninput={(e) => handleNameInput(e, "apellido")}
                        placeholder="Ej: Pérez"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'apellido',
                          $form.apellido,
                        )}"
                      />
                      {#if $errors.apellido}<p class={errorClass}>
                          {$errors.apellido}
                        </p>{/if}
                    </div>
                    <div>
                      <label for="segundoApellido" class={labelClass}
                        >Segundo Apellido</label
                      >
                      <input
                        id="segundoApellido"
                        type="text"
                        value={$form.segundoApellido}
                        oninput={(e) => handleNameInput(e, "segundoApellido")}
                        placeholder="Ej: González"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'segundoApellido',
                          $form.segundoApellido,
                        )}"
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <!-- Operación / CDI (Moved from Col 2) -->
                    <div class="relative">
                      <label for="operacion" class={labelClass}
                        >Operación / CDI <span class="text-red-500 ml-0.5"
                          >*</span
                        ></label
                      >
                      <button
                        type="button"
                        disabled={loading || readonly}
                        onclick={() =>
                          (showOperacionDropdown = !showOperacionDropdown)}
                        class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left {showOperacionDropdown
                          ? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
                          : getFieldStateClass('operacion', $form.operacion)}"
                      >
                        <span class="truncate">
                          {$form.operacion || "Selec CDI"}
                        </span>
                        <ChevronDown size={16} class="text-secondary" />
                      </button>

                      {#if $errors.operacion}
                        <p class={errorClass}>{$errors.operacion}</p>
                      {/if}

                      {#if showOperacionDropdown && !readonly}
                        <!-- Backdrop -->
                        <div
                          class="fixed inset-0 z-40"
                          onclick={() => (showOperacionDropdown = false)}
                          role="presentation"
                          aria-hidden="true"
                        ></div>

                        <!-- Dropdown Menu -->
                        <div
                          class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
                          transition:fly={{ y: -10, duration: 300 }}
                        >
                          {#each Object.values(Operacion) as op}
                            <button
                              type="button"
                              onclick={() => {
                                $form.operacion = op;
                                showOperacionDropdown = false;
                              }}
                              class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
                            >
                              <span>{op}</span>
                              {#if $form.operacion === op}
                                <svg
                                  xmlns="http://www.w3.org/2000/svg"
                                  class="h-4 w-4 text-white"
                                  viewBox="0 0 20 20"
                                  fill="currentColor"
                                >
                                  <path
                                    fill-rule="evenodd"
                                    d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                    clip-rule="evenodd"
                                  />
                                </svg>
                              {/if}
                            </button>
                          {/each}
                        </div>
                      {/if}
                    </div>

                    <!-- Rol -->
                    <!-- Rol (Custom Select) -->
                    {#if !isSelf}
                      <div class="relative">
                        <label for="roleId" class={labelClass}
                          >Rol <span class="text-red-500 ml-0.5">*</span></label
                        >
                        <button
                          type="button"
                          disabled={loading || rolesLoading || readonly}
                          onclick={() => (showRoleDropdown = !showRoleDropdown)}
                          class="{inputClass} flex items-center justify-between cursor-pointer w-full text-left {showRoleDropdown
                            ? '!border-blue-500/50 !ring-1 !ring-blue-500/20'
                            : getFieldStateClass('roleId', $form.roleId)}"
                        >
                          <span class="truncate">
                            {#if rolesLoading}
                              Cargando...
                            {:else}
                              {availableRoles.find((r) => r.id === $form.roleId)
                                ?.name || "Selec Rol"}
                            {/if}
                          </span>
                          <ChevronDown size={16} class="text-secondary" />
                        </button>

                        {#if $errors.roleId}
                          <p class={errorClass}>{$errors.roleId}</p>
                        {/if}

                        {#if showRoleDropdown && !rolesLoading && !readonly}
                          <!-- Backdrop to close on outside click -->
                          <div
                            class="fixed inset-0 z-40"
                            onclick={() => (showRoleDropdown = false)}
                            role="presentation"
                            aria-hidden="true"
                          ></div>

                          <!-- Dropdown Menu -->
                          <div
                            class="absolute z-50 w-full mt-1 bg-[#1c2128] border border-white/10 rounded-lg shadow-xl overflow-hidden p-1 origin-top"
                            transition:fly={{ y: -10, duration: 300 }}
                          >
                            {#each availableRoles.filter((r) => r.isSystem) as role}
                              <button
                                type="button"
                                onclick={() => {
                                  $form.roleId = role.id;
                                  showRoleDropdown = false;
                                }}
                                class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
                              >
                                <span>{role.name}</span>
                                {#if $form.roleId === role.id}
                                  <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="h-4 w-4 text-white"
                                    viewBox="0 0 20 20"
                                    fill="currentColor"
                                  >
                                    <path
                                      fill-rule="evenodd"
                                      d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                      clip-rule="evenodd"
                                    />
                                  </svg>
                                {/if}
                              </button>
                            {/each}

                            {#if availableRoles.some((r) => !r.isSystem)}
                              {#each availableRoles.filter((r) => !r.isSystem) as role}
                                <button
                                  type="button"
                                  onclick={() => {
                                    $form.roleId = role.id;
                                    showRoleDropdown = false;
                                  }}
                                  class="w-full text-left px-3 py-1.5 text-sm text-gray-300 hover:bg-white/10 rounded-md transition-colors flex items-center justify-between group"
                                >
                                  <span>{role.name}</span>
                                  {#if $form.roleId === role.id}
                                    <svg
                                      xmlns="http://www.w3.org/2000/svg"
                                      class="h-4 w-4 text-white"
                                      viewBox="0 0 20 20"
                                      fill="currentColor"
                                    >
                                      <path
                                        fill-rule="evenodd"
                                        d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                        clip-rule="evenodd"
                                      />
                                    </svg>
                                  {/if}
                                </button>
                              {/each}
                            {/if}
                          </div>
                        {/if}
                      </div>
                    {:else}
                      <div>
                        <label for="role-readonly" class={labelClass}>Rol</label
                        >
                        <div
                          id="role-readonly"
                          class="flex items-center justify-center px-3 h-[34px] bg-black/20 rounded-lg border border-white/10 text-sm text-secondary text-center select-none"
                        >
                          {currentRoleName}
                        </div>
                      </div>
                    {/if}
                  </div>

                  <div>
                    <label for="email" class={labelClass}
                      >Correo Electrónico <span class="text-red-500 ml-0.5"
                        >*</span
                      ></label
                    >
                    <input
                      id="email"
                      type="email"
                      value={$form.email}
                      oninput={handleEmailInput}
                      placeholder="Ej: usuario@empresa.com"
                      disabled={loading || readonly}
                      class="{inputClass} {getFieldStateClass(
                        'email',
                        $form.email,
                      )} {emailDuplicateError
                        ? '!border-red-500/50 !ring-1 !ring-red-500/20'
                        : ''}"
                    />
                    {#if $errors.email || emailDuplicateError}
                      <p class={errorClass}>
                        {$errors.email || emailDuplicateError}
                      </p>
                    {/if}
                  </div>
                </div>

                <!-- COL 2: Institucional -->
                <div class="space-y-3">
                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label
                        for="vencimientoPortacion"
                        class="{labelClass} whitespace-nowrap"
                      >
                        Venc. Portación <span class="text-red-500 ml-0.5"
                          >*</span
                        >
                      </label>
                      <input
                        id="vencimientoPortacion"
                        type="date"
                        bind:value={$form.vencimientoPortacion}
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'vencimientoPortacion',
                          $form.vencimientoPortacion,
                        )}"
                        onkeydown={(e) =>
                          handleDateTab(e, "fechaInicioLabores")}
                      />
                      {#if $errors.vencimientoPortacion}
                        <p class="text-red-500 text-xs mt-1">
                          {$errors.vencimientoPortacion}
                        </p>
                      {/if}
                    </div>
                    <div>
                      <label for="fechaInicioLabores" class={labelClass}
                        >Inicio Labores</label
                      >
                      <input
                        id="fechaInicioLabores"
                        type="date"
                        bind:value={$form.fechaInicioLabores}
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'fechaInicioLabores',
                          $form.fechaInicioLabores,
                        )}"
                        onkeydown={(e) => handleDateTab(e, "fechaNacimiento")}
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <!-- Vencimiento Portación (Mandatory) -->
                    <div>
                      <label for="fechaNacimiento" class={labelClass}
                        >Fecha Nacimiento</label
                      >
                      <input
                        id="fechaNacimiento"
                        type="date"
                        bind:value={$form.fechaNacimiento}
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'fechaNacimiento',
                          $form.fechaNacimiento,
                        )}"
                        onkeydown={(e) => handleDateTab(e, "telefono")}
                      />
                    </div>
                    <div>
                      <label for="telefono" class={labelClass}
                        >Tel. Personal</label
                      >
                      <input
                        id="telefono"
                        type="tel"
                        value={$form.telefono}
                        oninput={(e) => handleGenericPhoneInput(e, "telefono")}
                        onkeydown={handlePhoneKeydown}
                        placeholder="+506 8888-8888"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'telefono',
                          $form.telefono,
                        )}"
                      />
                    </div>
                  </div>

                  <div class="grid grid-cols-2 gap-2">
                    <div>
                      <label for="contactoEmergenciaNombre" class={labelClass}
                        >Contact. Emergencia</label
                      >
                      <input
                        id="contactoEmergenciaNombre"
                        type="text"
                        value={$form.contactoEmergenciaNombre}
                        oninput={(e) =>
                          handleNameInput(e, "contactoEmergenciaNombre")}
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'contactoEmergenciaNombre',
                          $form.contactoEmergenciaNombre,
                        )}"
                        placeholder="Ej: María"
                      />
                    </div>
                    <div>
                      <label for="contactoEmergenciaTelefono" class={labelClass}
                        >Tel. Emergencia</label
                      >
                      <input
                        id="contactoEmergenciaTelefono"
                        type="tel"
                        value={$form.contactoEmergenciaTelefono}
                        oninput={(e) =>
                          handleGenericPhoneInput(
                            e,
                            "contactoEmergenciaTelefono",
                          )}
                        onkeydown={handlePhoneKeydown}
                        placeholder="+506 8888-8888"
                        disabled={loading || readonly}
                        class="{inputClass} {getFieldStateClass(
                          'contactoEmergenciaTelefono',
                          $form.contactoEmergenciaTelefono,
                        )}"
                      />
                    </div>
                  </div>

                  <!-- Dirección -->
                  <div>
                    <label for="direccion" class={labelClass}>Dirección</label>
                    <div
                      class="obs-container w-full bg-black/20 border border-white/10 rounded-lg transition-all outline-none {getFieldStateClass(
                        'direccion',
                        $form.direccion,
                      )} focus-within:!border-blue-500/50 focus-within:!ring-1 focus-within:!ring-blue-500/20"
                    >
                      <textarea
                        id="direccion"
                        bind:value={$form.direccion}
                        disabled={loading || readonly}
                        class="w-full bg-transparent px-3 py-2 text-sm text-white placeholder:text-gray-500 resize-none focus:outline-none outline-none border-none appearance-none ring-0 h-[93px]"
                        rows="4"
                        placeholder="Ej: San José, Calle 5, Av 3, Casa #123"
                      ></textarea>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Footer Actions -->
            <div
              class="flex-none flex items-center justify-end gap-3 px-6 py-4 border-t border-surface bg-surface-1"
            >
              <button
                type="button"
                onclick={onClose}
                class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-white/60 hover:text-white/80 text-sm"
              >
                Cancelar
              </button>

              {#if isSelf && !isChangingPassword && !readonly}
                <button
                  type="button"
                  onclick={() => (isChangingPassword = true)}
                  disabled={loading}
                  class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-accent hover:text-accent flex items-center gap-2 text-sm disabled:opacity-50"
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

              {#if isEditMode && !isSelf && $currentUser?.roleId === ROLE_ADMIN_ID && !isChangingPassword && !readonly}
                <button
                  type="button"
                  onclick={handleResetPasswordClick}
                  disabled={loading}
                  class="px-4 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-warning hover:text-warning text-sm disabled:opacity-50"
                >
                  Reset Password
                </button>
              {/if}

              {#if !isChangingPassword && !readonly}
                <button
                  type="submit"
                  disabled={loading ||
                    !!cedulaDuplicateError ||
                    !!emailDuplicateError}
                  class="px-6 py-2.5 rounded-lg border-2 border-surface text-secondary font-medium transition-all duration-200 hover:border-success hover:text-success text-sm disabled:opacity-50"
                >
                  {loading
                    ? "Guardando..."
                    : isEditMode
                      ? "Guardar Cambios"
                      : "Crear Usuario"}
                </button>
              {/if}
            </div>
          </form>
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

<style>
  /* Standardized input focus style */
  input:focus,
  textarea:focus {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
    outline: none !important;
  }

  /* Fix browser autofill white background */
  input:-webkit-autofill,
  input:-webkit-autofill:hover,
  input:-webkit-autofill:focus,
  textarea:-webkit-autofill,
  textarea:-webkit-autofill:hover,
  textarea:-webkit-autofill:focus {
    -webkit-text-fill-color: white !important;
    -webkit-box-shadow: 0 0 0px 1000px #1c2128 inset !important;
    transition: background-color 5000s ease-in-out 0s;
  }

  /* Force dark calendar picker */
  input[type="date"] {
    color-scheme: dark;
  }

  /* Select arrow styling */
  .select-arrow {
    background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%239ca3af' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
    background-position: right 0.5rem center;
    background-size: 1.25em 1.25em;
  }

  /* Address/Observations toggle container */
  .obs-container,
  .obs-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  .obs-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
