<script lang="ts">
  // @ts-nocheck - Svelte 5 runes not recognized by TS
  import {
    ChangePasswordSchema,
    type ChangePasswordForm,
  } from "$lib/schemas/userSchema";
  import { authService } from "$lib/logic/auth/authService"; // Import authService
  import { toast } from "svelte-5-french-toast";

  import { onMount } from "svelte"; // Added import

  interface Props {
    userId: string;
    currentPassword?: string; // Para pre-llenar si viene del login
    onSuccess: () => void;
    onCancel: () => void;
    hideHeader?: boolean;
  }

  let {
    userId,
    currentPassword = "",
    onSuccess,
    onCancel,
    hideHeader = false,
  }: Props = $props();

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

  let currentPasswordInput = $state<HTMLInputElement>();
  let newPasswordInput = $state<HTMLInputElement>();

  onMount(() => {
    setTimeout(() => {
      // Priorizar el foco en el campo de contraseña actual
      if (currentPasswordInput && !currentPassword) {
        currentPasswordInput.focus();
      } else {
        newPasswordInput?.focus();
      }
    }, 100);
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

  const errorClass = "text-xs text-red-500 mt-1";
</script>

<div class="w-full flex flex-col p-4 px-6">
  {#if !hideHeader}
    <div class="mb-4 text-center">
      <h2 class="text-lg font-bold text-gray-900 dark:text-gray-100">
        Cambiar Contraseña
      </h2>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
        Ingresa tu contraseña actual y la nueva contraseña.
      </p>
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="space-y-4">
    <!-- Contraseña Actual (Oculta si ya se proveyó) -->
    <div class={currentPassword ? "hidden" : "block"}>
      <label
        for="current_password"
        class="block text-sm font-medium text-secondary mb-1"
        >Contraseña Actual</label
      >
      <div
        class="input-container h-9 flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all"
      >
        <input
          id="current_password"
          bind:this={currentPasswordInput}
          type="password"
          bind:value={formData.currentPassword}
          disabled={loading || !!currentPassword}
          class="w-full bg-transparent px-3 text-white placeholder:text-gray-500 focus:outline-none outline-none border-none appearance-none ring-0 h-full"
        />
      </div>
      {#if errors.currentPassword}<p class={errorClass}>
          {errors.currentPassword}
        </p>{/if}
    </div>

    <!-- Nueva Contraseña -->
    <div>
      <label
        for="new_password"
        class="block text-sm font-medium text-secondary mb-1"
        >Nueva Contraseña</label
      >
      <div
        class="input-container h-9 flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all"
      >
        <input
          id="new_password"
          bind:this={newPasswordInput}
          type="password"
          bind:value={formData.newPassword}
          placeholder="Mínimo 6 caracteres"
          disabled={loading}
          class="w-full bg-transparent px-3 text-white placeholder:text-gray-500 focus:outline-none outline-none border-none appearance-none ring-0 h-full"
        />
      </div>
      {#if errors.newPassword}<p class={errorClass}>
          {errors.newPassword}
        </p>{/if}
    </div>

    <!-- Confirmar Contraseña -->
    <div>
      <label
        for="confirm_password"
        class="block text-sm font-medium text-secondary mb-1"
        >Confirmar Contraseña</label
      >
      <div
        class="input-container h-9 flex items-center bg-black/20 border border-white/10 rounded-lg focus-within:border-blue-500/50 focus-within:ring-1 focus-within:ring-blue-500/20 transition-all"
      >
        <input
          id="confirm_password"
          type="password"
          bind:value={formData.confirmPassword}
          placeholder="Repite la nueva contraseña"
          disabled={loading}
          class="w-full bg-transparent px-3 text-white placeholder:text-gray-500 focus:outline-none outline-none border-none appearance-none ring-0 h-full"
        />
      </div>
      {#if errors.confirmPassword}<p class={errorClass}>
          {errors.confirmPassword}
        </p>{/if}
    </div>

    <!-- Botones -->
    <div class="pt-3 flex gap-3">
      <button
        type="button"
        onclick={onCancel}
        disabled={loading}
        class="flex-1 h-9 flex items-center justify-center px-4 rounded-lg border border-surface text-secondary text-xs font-semibold transition-all duration-200 hover:border-white/40 hover:text-white disabled:opacity-50 whitespace-nowrap uppercase tracking-wider"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
        class="flex-1 h-9 flex items-center justify-center px-4 rounded-lg border border-surface text-secondary text-xs font-semibold transition-all duration-200 hover:border-success/50 hover:text-success disabled:opacity-50 whitespace-nowrap uppercase tracking-wider"
      >
        {loading ? "..." : "Guardar"}
      </button>
    </div>
  </form>
</div>

<style>
  /* Input container - mismo estilo que GafeteInput */
  .input-container,
  .input-container *:focus {
    outline: none !important;
    box-shadow: none !important;
  }

  .input-container:focus-within {
    border-color: rgba(59, 130, 246, 0.5) !important;
    box-shadow: 0 0 0 1px rgba(59, 130, 246, 0.2) !important;
  }
</style>
