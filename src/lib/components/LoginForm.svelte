<script lang="ts">
  import { preventDefault } from "svelte/legacy";
  import { onMount } from "svelte";
  import { exitApp } from "$lib/services/keyringService";
  import { loginStore } from "$lib/stores/loginStore.svelte";
  import { validateLoginForm } from "$lib/logic/auth/loginValidation";

  interface Props {
    loading?: boolean;
    onSubmit: (data: { email: string; password: string }) => void;
  }

  let { loading = false, onSubmit }: Props = $props();

  let email = $state("");
  let password = $state("");
  let errors = $state<Record<string, string>>({});
  let rememberMe = $state(false);
  let rememberPassword = $state(false);

  let emailInput = $state<HTMLInputElement>();
  let passwordInput = $state<HTMLInputElement>();
  let mounted = $state(false);

  onMount(() => {
    if (loginStore.hasRememberedEmail) {
      email = loginStore.rememberedEmail;
      rememberMe = true;

      if (loginStore.hasRememberedPassword) {
        password = loginStore.rememberedPassword;
        rememberPassword = true;
        // Auto-submit logic could go here if desired, but user asked for "inicio de sesion con solo ingresar" which implies manual click or just pre-filled
      }

      // Focus password if email is remembered
      setTimeout(() => passwordInput?.focus(), 100);
    } else {
      // Focus email otherwise
      setTimeout(() => emailInput?.focus(), 100);
    }
    mounted = true;
  });

  $effect(() => {
    if (mounted && !rememberMe) {
      loginStore.clearRememberedEmail();
      rememberPassword = false;
      email = "";
      password = "";
      setTimeout(() => emailInput?.focus(), 50);
    }
  });

  $effect(() => {
    if (mounted && !rememberPassword && loginStore.hasRememberedPassword) {
      loginStore.clearRememberedPassword();
    }
  });

  function handleSubmit() {
    const result = validateLoginForm(email, password);

    if (!result.valid) {
      errors = result.errors;
      return;
    }

    errors = {};

    if (rememberMe) {
      loginStore.setRememberedEmail(email);
      if (rememberPassword) {
        loginStore.setRememberedPassword(password);
      } else {
        loginStore.clearRememberedPassword();
      }
    } else {
      loginStore.clearRememberedEmail();
    }

    onSubmit({ email, password });
  }

  export async function reset() {
    await loginStore.reload();
    email = loginStore.rememberedEmail;
    password = "";
    errors = {};
    rememberMe = loginStore.hasRememberedEmail;

    // Reset focus logic
    if (rememberMe) {
      setTimeout(() => passwordInput?.focus(), 100);
    } else {
      setTimeout(() => emailInput?.focus(), 100);
    }
  }

  async function minimizeWindow() {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    await getCurrentWindow().minimize();
  }
</script>

<div
  class="w-full flex flex-col justify-center pt-1 pb-14 px-8 bg-[#1e1e1e] rounded-xl shadow-2xl"
>
  <form onsubmit={preventDefault(handleSubmit)} class="flex flex-col gap-4">
    <div class="text-center flex flex-col items-center mb-5">
      <img
        src="/icono-brisas.png"
        alt="Logo"
        class="w-20 h-20 active:scale-95 transition-transform mb-2"
      />
      <h1 class="text-3xl font-bold text-primary">MegaBrisas</h1>
    </div>

    <!-- Email -->
    <div class="flex flex-col gap-1.5">
      <label for="email" class="text-sm font-medium text-secondary">
        Correo Electrónico
      </label>
      <input
        id="email"
        bind:this={emailInput}
        type="email"
        bind:value={email}
        placeholder="ejemplo@correo.com"
        disabled={loading}
        class="w-full rounded border bg-surface-1 px-3 py-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent transition-all {errors.email
          ? 'border-red-500 focus:border-red-500 focus:ring-red-500/20'
          : 'border-emphasis focus:border-accent'}"
      />
      {#if errors.email}
        <span class="text-xs text-red-500 animate-fade-in">{errors.email}</span>
      {/if}
    </div>

    <!-- Password -->
    <div class="flex flex-col gap-1.5">
      <div class="flex justify-between items-center">
        <label for="password" class="text-sm font-medium text-secondary">
          Contraseña
        </label>
      </div>
      <input
        id="password"
        bind:this={passwordInput}
        type="password"
        bind:value={password}
        placeholder="••••••••"
        disabled={loading}
        class="w-full rounded border bg-surface-1 px-3 py-2 text-primary focus:outline-none focus:ring-2 focus:ring-accent transition-all {errors.password
          ? 'border-red-500 focus:border-red-500 focus:ring-red-500/20'
          : 'border-emphasis focus:border-accent'}"
      />
      {#if errors.password}
        <span class="text-xs text-red-500 animate-fade-in"
          >{errors.password}</span
        >
      {/if}
    </div>

    <!-- Options -->
    <div class="flex flex-col gap-2 items-center justify-center">
      <label
        class="flex items-center gap-2 cursor-pointer text-sm text-secondary hover:text-primary transition-colors select-none"
      >
        <input
          type="checkbox"
          bind:checked={rememberMe}
          disabled={loading}
          class="rounded border-surface-tertiary text-accent focus:ring-accent w-4 h-4 cursor-pointer"
        />
        Recordar usuario
      </label>

      {#if rememberMe}
        <label
          class="flex items-center gap-2 cursor-pointer text-sm text-secondary hover:text-primary transition-colors select-none animate-fade-in"
        >
          <input
            type="checkbox"
            bind:checked={rememberPassword}
            disabled={loading}
            class="rounded border-surface-tertiary text-accent focus:ring-accent w-4 h-4 cursor-pointer"
          />
          Recordar contraseña
        </label>
      {/if}
    </div>

    <!-- Acciones -->
    <div class="flex gap-3 mt-4">
      <button
        type="button"
        onclick={exitApp}
        class="flex-1 py-2.5 px-4 rounded-lg border border-surface-tertiary text-secondary font-medium hover:bg-surface-3 transition-colors text-sm"
      >
        Cancelar
      </button>
      <button
        type="submit"
        disabled={loading}
        class="flex-1 rounded-lg bg-accent px-4 py-2.5 font-medium text-white transition-all hover:bg-accent-hover hover:shadow-lg hover:shadow-accent/20 disabled:cursor-not-allowed disabled:opacity-60 active:scale-[0.98]"
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
            ...
          </span>
        {:else}
          Ingresar
        {/if}
      </button>
    </div>
  </form>
</div>
