<script lang="ts">
  import { Lock, AlertTriangle, ShieldAlert, Eye, EyeOff } from "lucide-svelte";
  import { preventDefault } from "svelte/legacy";
  import { fade, fly } from "svelte/transition";
  import { currentUser } from "$lib/stores/auth";
  import { auth as authApi } from "$lib/api/auth";
  import type { UserResponse } from "$lib/types/user";

  interface Props {
    show: boolean;
    title?: string;
    warningMessage: string;
    confirmButtonText?: string;
    variant?: "warning" | "danger";
    user?: UserResponse | null; // Optional user prop
    onConfirm: () => Promise<void> | void;
    onCancel: () => void;
  }

  let {
    show,
    title = "Confirmar Acción",
    warningMessage,
    confirmButtonText = "Confirmar",
    variant = "warning",
    user = null,
    onConfirm,
    onCancel,
  }: Props = $props();

  // Use provided user or fallback to store
  const activeUser = $derived(user || $currentUser);

  let password = $state("");
  let error = $state("");
  let loading = $state(false);
  let showPassword = $state(false);
  let inputRef: HTMLInputElement | null = $state(null);

  // Focus password input when modal opens
  $effect(() => {
    if (show && inputRef) {
      setTimeout(() => inputRef?.focus(), 150);
    }
  });

  // Reset state when modal closes
  $effect(() => {
    if (!show) {
      password = "";
      error = "";
      loading = false;
      showPassword = false;
    }
  });

  async function handleSubmit() {
    if (!activeUser?.email) {
      error = "Usuario no identificado";
      return;
    }

    if (!password.trim()) {
      error = "Ingresa tu contraseña";
      return;
    }

    error = "";
    loading = true;

    try {
      // Verify password by attempting login with current user credentials
      await authApi.login(activeUser.email, password);

      // Password correct - execute the confirmed action
      await onConfirm();
      password = "";
    } catch (err: any) {
      console.error("Password verification failed:", err);
      error = "Contraseña incorrecta";
      password = "";
      inputRef?.focus();
    } finally {
      loading = false;
    }
  }

  function handleCancel() {
    password = "";
    error = "";
    onCancel();
  }

  // Color variants
  const colors = {
    warning: {
      iconBg: "bg-amber-500/20",
      iconColor: "text-amber-400",
      border: "border-amber-500/30",
      bg: "bg-amber-500/10",
      button: "bg-amber-600 hover:bg-amber-700",
      glow: "shadow-amber-500/20",
    },
    danger: {
      iconBg: "bg-red-500/20",
      iconColor: "text-red-400",
      border: "border-red-500/30",
      bg: "bg-red-500/10",
      button: "bg-red-600 hover:bg-red-700",
      glow: "shadow-red-500/20",
    },
  };

  const c = $derived(colors[variant]);
</script>

{#if show}
  <!-- Backdrop with blur -->
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center"
    role="dialog"
    aria-modal="true"
    transition:fade={{ duration: 200 }}
  >
    <!-- Background overlay -->
    <button
      class="absolute inset-0 bg-black/70 backdrop-blur-md cursor-default border-0"
      onclick={handleCancel}
      tabindex="-1"
      aria-label="Cerrar modal"
    ></button>

    <!-- Modal Container -->
    <div
      class="relative w-full max-w-md mx-4 rounded-xl bg-gradient-to-b from-[#1a1a1a] to-[#0d0d0d] shadow-2xl border border-white/10 overflow-hidden"
      transition:fly={{ y: 30, duration: 250 }}
    >
      <!-- Decorative top glow -->
      <div
        class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-white/20 to-transparent"
      ></div>

      <!-- Header with icon -->
      <div class="relative px-6 pt-6 pb-4">
        <div class="flex items-start gap-4">
          <div
            class="flex-shrink-0 flex items-center justify-center w-12 h-12 rounded-xl {c.iconBg} {c.glow} shadow-lg"
          >
            <ShieldAlert class={c.iconColor} size={24} />
          </div>
          <div class="flex-1 min-w-0">
            <h2 class="text-lg font-bold text-white leading-tight">{title}</h2>
            <p class="text-sm text-gray-400 mt-0.5">
              Acción que requiere verificación
            </p>
          </div>
        </div>
      </div>

      <!-- Warning Message Box -->
      <div class="px-6 pb-4">
        <div class="p-4 rounded-lg {c.bg} border {c.border}">
          <div class="flex gap-3">
            <AlertTriangle
              class="{c.iconColor} flex-shrink-0 mt-0.5"
              size={18}
            />
            <p class="text-sm text-gray-200 leading-relaxed">
              {warningMessage}
            </p>
          </div>
        </div>
      </div>

      <!-- User Info Card -->
      <div class="px-6 pb-3">
        <div class="p-2.5 rounded-md bg-white/5 border border-white/10">
          <div class="flex items-center gap-2.5">
            <div
              class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-xs"
            >
              {activeUser?.nombre?.charAt(0)?.toUpperCase() || "U"}
            </div>
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium text-white truncate">
                {activeUser?.nombre || "Usuario"}
              </div>
              <div class="text-xs text-gray-500 truncate">
                {activeUser?.email || ""}
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Password Form -->
      <form onsubmit={preventDefault(handleSubmit)} class="px-6 pb-6">
        <!-- Password Input -->
        <div class="mb-4">
          <label
            for="confirm-password"
            class="block text-sm font-medium text-gray-300 mb-2"
          >
            Ingresa tu contraseña para confirmar
          </label>
          <div class="relative group">
            <div
              class="absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none"
            >
              <Lock
                class="text-gray-500 group-focus-within:text-blue-400 transition-colors"
                size={18}
              />
            </div>
            <input
              id="confirm-password"
              bind:this={inputRef}
              type={showPassword ? "text" : "password"}
              bind:value={password}
              placeholder="••••••••"
              disabled={loading}
              autocomplete="current-password"
              class="w-full pl-10 pr-12 py-3 rounded-lg border bg-[#0a0a0a] text-white placeholder:text-gray-600 focus:outline-none focus:ring-2 transition-all text-base {error
                ? 'border-red-500/50 focus:ring-red-500/20 focus:border-red-500'
                : 'border-white/10 focus:border-blue-500/50 focus:ring-blue-500/20'}"
            />
            <button
              type="button"
              onclick={() => (showPassword = !showPassword)}
              class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-500 hover:text-gray-300 transition-colors p-1"
              tabindex="-1"
            >
              {#if showPassword}
                <EyeOff size={18} />
              {:else}
                <Eye size={18} />
              {/if}
            </button>
          </div>
          {#if error}
            <p
              class="mt-2 text-sm text-red-400 flex items-center gap-1.5"
              transition:fade={{ duration: 150 }}
            >
              <AlertTriangle size={14} />
              {error}
            </p>
          {/if}
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-2">
          <button
            type="button"
            onclick={handleCancel}
            disabled={loading}
            class="flex-1 py-2 px-3 rounded-md border border-white/10 text-gray-300 font-medium hover:bg-white/5 hover:border-white/20 transition-all text-sm disabled:opacity-50"
          >
            Cancelar
          </button>
          <button
            type="submit"
            disabled={loading || !password.trim()}
            class="flex-1 rounded-md {c.button} px-3 py-2 font-medium text-white text-sm transition-all hover:shadow-lg {c.glow} disabled:cursor-not-allowed disabled:opacity-50 active:scale-[0.98]"
          >
            {#if loading}
              <span class="flex items-center justify-center gap-2">
                <svg
                  class="h-4 w-4 animate-spin text-white"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle>
                  <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  ></path>
                </svg>
                Verificando...
              </span>
            {:else}
              {confirmButtonText}
            {/if}
          </button>
        </div>

        <!-- Security Note -->
        <p class="mt-4 text-center text-xs text-gray-600">
          🔒 Tu contraseña se verifica localmente y no se almacena
        </p>
      </form>
    </div>
  </div>
{/if}
