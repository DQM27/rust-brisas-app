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

    const result = await authService.login({ email, password });

    if (result.ok) {
      if (result.data.mustChangePassword) {
        tempUser = result.data;
        tempPassword = password;
        view = "change_password";
      } else {
        completeLogin(result.data);
      }
    } else {
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

<div
  class="w-full h-full flex flex-col items-center justify-between py-8 bg-surface-1 overflow-hidden"
>
  <div class="flex-1 flex items-center justify-center w-full pb-20">
    {#if view === "login"}
      <div class="w-full max-w-[450px]">
        <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
      </div>
    {:else if view === "change_password" && tempUser}
      <div
        class="w-full max-w-sm bg-white dark:bg-[#0d1117] rounded-xl shadow-2xl overflow-hidden"
      >
        <ChangePasswordPanel
          userId={tempUser.id}
          currentPassword={tempPassword}
          onSuccess={handlePasswordChanged}
          onCancel={handleCancelChange}
        />
      </div>
    {/if}
  </div>

  <div class="text-xs text-gray-500 font-medium opacity-60">
    &copy; 2025 Mega Brisas. Todos los derechos reservados.
  </div>
</div>
