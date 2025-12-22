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
  const modalTitle = $derived(
    isEditMode ? `Editar: ${user?.nombre}` : "Nuevo Usuario",
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

  // Cargar datos del usuario cuando se abre en modo edición
  $effect(() => {
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

      <!-- Form -->
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
              <label for="numeroGafete" class={labelClass}>Número Gafete</label>
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
                oninput={(e) => handleNameInput(e, "contactoEmergenciaNombre")}
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
    </div>
  </div>
{/if}
