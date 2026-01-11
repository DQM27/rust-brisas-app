<script lang="ts">
  import { login as setAuth } from "$lib/stores/auth";
  import LoginForm from "$lib/components/LoginForm.svelte";
  import ChangePasswordPanel from "$lib/components/ChangePasswordPanel.svelte";
  import { authService } from "$lib/logic/auth/authService";
  import { toast } from "svelte-5-french-toast";
  import type { UserResponse } from "$lib/types/user";
  import { X } from "lucide-svelte";
  import { setWindowSize } from "$lib/services/keyringService";

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

  // Manage window size dynamically (Reduce 30% height for password view)
  $effect(() => {
    (async () => {
      if (view === "change_password") {
        await setWindowSize(350, 290); // 20% width reduction (450 -> 360)
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        await getCurrentWindow().center();
      } else {
        await setWindowSize(450, 500); // Standard login size
        const { getCurrentWindow } = await import("@tauri-apps/api/window");
        await getCurrentWindow().center();
      }
    })();
  });
</script>

<div
  class="w-full h-full flex flex-col items-center {view === 'login'
    ? 'justify-between py-8'
    : 'justify-center'} bg-surface-1 overflow-hidden"
>
  <div
    class="{view === 'login'
      ? 'flex-1 pb-20'
      : ''} flex items-center justify-center w-full"
  >
    {#if view === "login"}
      <div class="w-full max-w-[450px]">
        <LoginForm bind:this={formRef} {loading} onSubmit={handleLogin} />
      </div>
    {:else if view === "change_password" && tempUser}
      <div
        class="w-full max-w-[360px] bg-[#1e1e1e] rounded-xl shadow-2xl overflow-hidden animate-fade-in"
      >
        <!-- Header Banner matching premium modal style -->
        <div
          class="flex items-center justify-between px-6 py-3 border-b border-white/5 bg-white/5"
        >
          <div class="flex flex-col">
            <h2 class="text-lg font-bold text-primary leading-tight">
              Actualización de Contraseña
            </h2>
            <p
              class="text-[10px] text-secondary/60 uppercase tracking-widest font-semibold mt-0.5"
            >
              Seguridad Requerida
            </p>
          </div>
          <button
            onclick={handleCancelChange}
            class="p-1.5 rounded-lg text-secondary/60 hover:text-primary hover:bg-white/5 transition-all"
            aria-label="Cerrar"
          >
            <X size={20} />
          </button>
        </div>

        <ChangePasswordPanel
          userId={tempUser.id}
          currentPassword={tempPassword}
          hideHeader={true}
          onSuccess={handlePasswordChanged}
          onCancel={handleCancelChange}
        />
      </div>
    {/if}
  </div>
</div>
