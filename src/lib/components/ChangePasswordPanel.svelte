<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import {
    ChangePasswordSchema,
    type ChangePasswordForm,
  } from "$lib/schemas/userSchema";
  import { authService } from "$lib/logic/auth/authService"; // Import authService
  import { toast } from "svelte-5-french-toast";

  interface Props {
    userId: string;
    currentPassword?: string; // Para pre-llenar si viene del login
    onSuccess: () => void;
    onCancel: () => void;
  }

  let { userId, currentPassword = "", onSuccess, onCancel }: Props = $props();

  let loading = $state(false);

  // Helper to get form data
  function getInitialFormData(pwd: string): ChangePasswordForm {
    return {
      currentPassword: pwd,
      newPassword: "",
      confirmPassword: "",
    };
  }

  // Estado del formulario - inicializar vacío, $effect sincroniza
  let formData = $state<ChangePasswordForm>({
    currentPassword: "",
    newPassword: "",
    confirmPassword: "",
  });

  // Sync if currentPassword prop changes
  $effect(() => {
    formData = getInitialFormData(currentPassword);
  });

  let errors = $state<Record<string, string>>({});

  // Refined Logic below
  async function handleSubmit(e: Event) {
    e.preventDefault();
    loading = true;
    errors = {};

    // 1. Validar con Zod
    const valResult = ChangePasswordSchema.safeParse(formData);

    if (!valResult.success) {
      const newErrors: Record<string, string> = {};
      valResult.error.issues.forEach((issue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
      loading = false;
      return;
    }

    // 2. Enviar al backend
    const serviceRes = await authService.changePassword(userId, valResult.data); // Use authService

    if (serviceRes.ok) {
      toast.success("Contraseña actualizada correctamente", { icon: "🔒" });
      onSuccess();
    } else {
      toast.error(serviceRes.error || "Error al actualizar contraseña");
    }
    loading = false;
  }

  const inputClass =
    "w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-[#0d1117] px-3 py-2 text-sm text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 focus:border-transparent focus:outline-none focus:ring-2 focus:ring-[#2da44e] disabled:opacity-60 transition-all";
  const errorClass = "text-xs text-red-500 mt-1";
</script>

<div
  class="w-full max-w-sm rounded-lg bg-white dark:bg-[#0d1117] p-8 shadow-xl border border-gray-200 dark:border-gray-700"
>
  <div class="mb-6 text-center">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
      Cambiar Contraseña
    </h2>
    <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
      Por seguridad, debes establecer una nueva contraseña.
    </p>
  </div>

  <form onsubmit={handleSubmit} class="space-y-4">
    <!-- Contraseña Actual (Oculta si ya se proveyó) -->
    <div class={currentPassword ? "hidden" : "block"}>
      <label
        for="current_password"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >Contraseña Actual</label
      >
      <input
        id="current_password"
        type="password"
        bind:value={formData.currentPassword}
        disabled={loading || !!currentPassword}
        class={inputClass}
      />
      {#if errors.currentPassword}<p class={errorClass}>
          {errors.currentPassword}
        </p>{/if}
    </div>

    <!-- Nueva Contraseña -->
    <div>
      <label
        for="new_password"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >Nueva Contraseña</label
      >
      <input
        id="new_password"
        type="password"
        bind:value={formData.newPassword}
        placeholder="Mínimo 6 caracteres"
        disabled={loading}
        class={inputClass}
      />
      {#if errors.newPassword}<p class={errorClass}>
          {errors.newPassword}
        </p>{/if}
    </div>

    <!-- Confirmar Contraseña -->
    <div>
      <label
        for="confirm_password"
        class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
        >Confirmar Contraseña</label
      >
      <input
        id="confirm_password"
        type="password"
        bind:value={formData.confirmPassword}
        placeholder="Repite la nueva contraseña"
        disabled={loading}
        class={inputClass}
      />
      {#if errors.confirmPassword}<p class={errorClass}>
          {errors.confirmPassword}
        </p>{/if}
    </div>

    <!-- Botones -->
    <div class="pt-2 flex flex-col gap-3">
      <button
        type="submit"
        disabled={loading}
        class="w-full rounded-md bg-[#2da44e] px-4 py-2 font-medium text-white transition-all hover:bg-[#2c974b] disabled:cursor-not-allowed disabled:opacity-60 shadow-sm"
      >
        {loading ? "Actualizando..." : "Confirmar Cambio"}
      </button>

      <button
        type="button"
        onclick={onCancel}
        disabled={loading}
        class="w-full text-center text-sm text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 transition-colors"
      >
        Cancelar
      </button>
    </div>
  </form>
</div>
