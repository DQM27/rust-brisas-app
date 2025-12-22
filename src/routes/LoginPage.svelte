<script lang="ts">
  import { login as setAuth } from "$lib/stores/auth";
  import LoginForm from "$lib/components/LoginForm.svelte";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";
  import { authService } from "$lib/logic/auth/authService";
  import { toast } from "svelte-5-french-toast";
  import type { UserResponse } from "$lib/types/user";

  // Estado UI
  let view = $state<"login" | "change_password">("login");
  let loading = $state(false);

  // Estado Temporal (durante el flujo de cambio de pass)
  let tempUser = $state<UserResponse | null>(null);
  let tempPassword = $state<string>("");

  // Referencia al form para resetearlo
  let formRef = $state<any>();

  async function handleLogin({
    email,
    password,
  }: {
    email: string;
    password: string;
  }) {
    loading = true;

    // Usar authService centralizado con objeto tipado
    const result = await authService.login({ email, password });

    if (result.ok) {
      // 1. Revisar si debe cambiar contraseña
      if (result.data.mustChangePassword) {
        tempUser = result.data;
        tempPassword = password;
        view = "change_password";
        toast("Debes actualizar tu contraseña para continuar", { icon: "🔒" });
      } else {
        // 2. Login normal
        completeLogin(result.data);
      }
    } else {
      // Mensaje de error (si hay código específico, authService lo provee)
      toast.error(result.error || "Error al iniciar sesión", { icon: "✕" });
    }

    loading = false;
  }

  function completeLogin(user: UserResponse) {
    setAuth(user);
    formRef?.reset();
    toast.success("Sesión iniciada correctamente", { icon: "✓" });
  }

  function handlePasswordChanged() {
    if (tempUser) {
      // Actualizamos el flag localmente para que el store tenga el dato fresco
      const updatedUser = { ...tempUser, mustChangePassword: false };
      completeLogin(updatedUser);
    }
    view = "login";
    tempUser = null;
    tempPassword = "";
  }

  function handleCancelChange() {
    view = "login";
    tempUser = null;
    tempPassword = "";
  }
</script>

<!-- Usar bg-surface-1 para fondo consistente -->
<div class="flex h-screen w-full items-center justify-center bg-surface-1 p-4">
  {#if view === "login"}
    <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
  {:else if view === "change_password" && tempUser}
    <div class="animate-fade-in w-full max-w-sm">
      <ChangePasswordPanel
        userId={tempUser.id}
        currentPassword={tempPassword}
        onSuccess={handlePasswordChanged}
        onCancel={handleCancelChange}
      />
    </div>
  {/if}
</div>
