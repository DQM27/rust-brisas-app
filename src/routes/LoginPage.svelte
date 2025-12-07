<script lang="ts">
  import { login as setAuth } from "$lib/stores/auth";
  import LoginForm from "$lib/components/LoginForm.svelte";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";
  import { submitLogin } from "$lib/logic/auth/submitLogin";
  import { toast } from "svelte-5-french-toast";
  import type { UserResponse } from "$lib/types/user";

  // Tipos para referencia del componente
  // import type LoginFormType from '$lib/components/LoginForm.svelte';

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

    const result = await submitLogin(email, password);

    if (result.ok) {
      // 1. Revisar si debe cambiar contraseña
      if (result.user.mustChangePassword) {
        tempUser = result.user;
        tempPassword = password;
        view = "change_password";
        toast("Debes actualizar tu contraseña para continuar", { icon: "🔒" });
      } else {
        // 2. Login normal
        completeLogin(result.user);
      }
    } else {
      toast.error(result.error, { icon: "✕" });
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

<div class="flex h-screen w-full items-center justify-center bg-[#1e1e1e]">
  {#if view === "login"}
    <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
  {:else if view === "change_password" && tempUser}
    <ChangePasswordPanel
      userId={tempUser.id}
      currentPassword={tempPassword}
      onSuccess={handlePasswordChanged}
      onCancel={handleCancelChange}
    />
  {/if}
</div>
