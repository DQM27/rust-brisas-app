<script lang="ts">
  import {
    ChangePasswordSchema,
    type ChangePasswordForm,
  } from "$lib/schemas/userSchema";
  import { auth } from "$lib/api/auth";
  import { toast } from "svelte-5-french-toast";

  interface Props {
    userId: string;
    currentPassword?: string; // Para pre-llenar si viene del login
    onSuccess: () => void;
    onCancel: () => void;
  }

  let { userId, currentPassword = "", onSuccess, onCancel }: Props = $props();

  let loading = $state(false);

  // Estado del formulario
  let formData = $state<ChangePasswordForm>({
    currentPassword: currentPassword, // camelCase
    newPassword: "",
    confirmPassword: "",
  });

  let errors = $state<Record<string, string>>({});

  async function handleSubmit(e: Event) {
    e.preventDefault();
    loading = true;
    errors = {};

    // 1. Validar con Zod
    const result = ChangePasswordSchema.safeParse(formData);

    if (!result.success) {
      const newErrors: Record<string, string> = {};
      result.error.issues.forEach((issue) => {
        if (issue.path[0]) {
          newErrors[String(issue.path[0])] = issue.message;
        }
      });
      errors = newErrors;
      loading = false;
      return;
    }

    // 2. Enviar al backend
    try {
      await auth.changePassword(userId, result.data);

      toast.success("Contraseña actualizada correctamente", { icon: "🔒" });
      onSuccess();
    } catch (err: any) {
      console.error(err);
      toast.error(
        typeof err === "string" ? err : "Error al actualizar contraseña",
      );
      loading = false;
    }
  }

  const inputClass =
    "w-full rounded border border-emphasis bg-surface-1 px-3 py-2 text-sm text-primary placeholder:text-tertiary focus:border-accent focus:outline-none focus:ring-2 focus:ring-accent disabled:opacity-60";
  const errorClass = "text-xs text-red-500 mt-1";
</script>

<div
  class="w-full max-w-sm rounded-lg bg-surface-2 p-8 shadow-xl border border-surface-3"
>
  <div class="mb-6 text-center">
    <h2 class="text-2xl font-bold text-primary">Cambiar Contraseña</h2>
    <p class="text-sm text-tertiary mt-2">
      Por seguridad, debes establecer una nueva contraseña.
    </p>
  </div>

  <form onsubmit={handleSubmit} class="space-y-4">
    <!-- Contraseña Actual (Oculta si ya se proveyó) -->
    <!-- Si no se provee (ej: perfil), se muestra. Pero en flujo login usually viene. -->
    <div class={currentPassword ? "hidden" : "block"}>
      <label
        for="current_password"
        class="block text-sm font-medium text-primary mb-1"
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
        class="block text-sm font-medium text-primary mb-1"
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
        class="block text-sm font-medium text-primary mb-1"
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
        class="w-full rounded bg-accent px-4 py-2 font-medium text-white transition-colors hover:bg-accent-hover disabled:cursor-not-allowed disabled:opacity-60"
      >
        {loading ? "Actualizando..." : "Confirmar Cambio"}
      </button>

      <button
        type="button"
        onclick={onCancel}
        disabled={loading}
        class="w-full text-center text-sm text-tertiary hover:text-primary transition-colors"
      >
        Cancelar
      </button>
    </div>
  </form>
</div>
